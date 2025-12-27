mod cli;
mod dedup;
mod error;
mod file_ops;
mod hashing;
mod models;
mod scanner;
mod tui;
mod utils;

use crate::tui::KeyAction;
use crate::utils::format_size;
use clap::Parser as _;

use crate::cli::Args;
use crate::dedup::HashGrouper;
use crate::error::{DejaVuError, Result};
use crate::file_ops::{FileDeleter, FileOpener};
use crate::models::DuplicateGroup;
use crate::scanner::{FileCollector, MediaFilter};
use crate::tui::event::handle_key_event;
use crate::tui::{App, MainLayout};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use indicatif::{ProgressBar, ProgressStyle};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Check if directory exists
    if !args.directory.exists() {
        return Err(DejaVuError::PathNotFound(
            args.directory.display().to_string(),
        ));
    }

    // Step 1: Scan for files
    println!("🔍 正在扫描目录: {}", args.directory.display());
    let filter = MediaFilter::new(!args.videos_only, !args.images_only);
    let collector = FileCollector::new(filter, args.min_size);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg:.dim}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    let files = collector.collect_with_progress(&args.directory, |found, _total| {
        pb.set_message(format!("已找到 {} 个媒体文件", found));
    })?;

    pb.finish_with_message(format!("✓ 扫描完成，共找到 {} 个媒体文件", files.len()));

    if files.is_empty() {
        println!("❌ 指定目录中未找到媒体文件");
        return Ok(());
    }

    // Step 2: Find duplicates
    println!("🔄 正在计算哈希值并查找重复文件...");
    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg:.dim}")
            .unwrap()
            .progress_chars("##-"),
    );

    let grouper = HashGrouper::new(args.threshold);
    let duplicate_groups = grouper.find_duplicates(files, Some(&pb))?;

    pb.finish_with_message(format!("✓ 发现 {} 个重复文件组", duplicate_groups.len()));

    if duplicate_groups.is_empty() {
        println!("✅ 太棒了！没有发现重复文件");
        return Ok(());
    }

    let total_wasted: u64 = duplicate_groups.iter().map(|g| g.wasted_space()).sum();
    println!("💾 可释放空间: {}", format_size(total_wasted));

    // Step 3: Launch TUI
    println!("\n🚀 正在启动图形界面...");
    println!("💡 提示: 按 ? 键可查看帮助");
    run_tui(duplicate_groups)?;

    Ok(())
}

fn run_tui(duplicate_groups: Vec<DuplicateGroup>) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(duplicate_groups);

    // Run event loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Print summary
    if app.marked_count() > 0 {
        println!("Marked {} files for deletion", app.marked_count());
    }

    // Convert Box<dyn Error> to DejaVuError if needed
    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            // Try to downcast to io::Error or create a generic error
            // Use a safer approach that doesn't require unwrap()
            if e.is::<std::io::Error>() {
                // Safe to unwrap since we just checked the type
                let io_err = e.downcast::<std::io::Error>().unwrap();
                Err(DejaVuError::Io(*io_err))
            } else {
                Err(DejaVuError::FileOperationFailed(e.to_string()))
            }
        }
    }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> std::result::Result<(), Box<dyn std::error::Error + 'static>>
where
    <B as Backend>::Error: 'static,
{
    loop {
        // Draw UI
        terminal.draw(|f| {
            if app.mode == crate::tui::Mode::Help {
                crate::tui::ui::HelpWidget::render(f);
            } else {
                MainLayout::render(f, app);
            }
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                let action = handle_key_event(key, app);

                match action {
                    KeyAction::OpenFile => {
                        if let Some(group) = app.current_group() {
                            if let Some(file) = group.files.get(app.selected_file) {
                                // Leave raw mode temporarily to open file
                                disable_raw_mode()?;
                                if let Err(e) = FileOpener::open(&file.path) {
                                    enable_raw_mode()?;
                                    return Err(Box::new(e) as Box<dyn std::error::Error>);
                                }
                                enable_raw_mode()?;
                            }
                        }
                    }
                    KeyAction::DeleteFile => {
                        if let Some(group) = app.current_group() {
                            if let Some(file) = group.files.get(app.selected_file) {
                                // Confirm deletion
                                disable_raw_mode()?;
                                println!(
                                    "\n⚠️  确定要删除文件 '{}' 吗? (y/n)",
                                    file.filename()
                                );
                                println!("💡 此操作不可撤销，请谨慎操作！");
                                let mut input = String::new();
                                std::io::stdin().read_line(&mut input)?;
                                enable_raw_mode()?;

                                if input.trim().to_lowercase() == "y" {
                                    if let Err(e) = FileDeleter::delete(&file.path) {
                                        enable_raw_mode()?;
                                        eprintln!("❌ 删除失败: {}", e);
                                        enable_raw_mode()?;
                                        return Err(Box::new(e) as Box<dyn std::error::Error>);
                                    }
                                    println!("✓ 文件已删除");
                                }
                            }
                        }
                    }
                    KeyAction::DeleteMarked => {
                        // Delete all marked files
                        disable_raw_mode()?;
                        println!("\n⚠️  确定要删除已标记的 {} 个文件吗? (y/n)", app.marked_count());
                        println!("💡 此操作不可撤销，请谨慎操作！");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input)?;
                        enable_raw_mode()?;

                        if input.trim().to_lowercase() == "y" {
                            // Collect files to delete
                            let files_to_delete: Vec<_> = app
                                .marked_files
                                .iter()
                                .filter_map(|&idx| {
                                    let mut count = 0;
                                    for group in &app.duplicate_groups {
                                        for file in &group.files {
                                            if count == idx {
                                                return Some(file.path.clone());
                                            }
                                            count += 1;
                                        }
                                    }
                                    None
                                })
                                .collect();

                            let mut deleted_count = 0;
                            for path in &files_to_delete {
                                if let Err(e) = FileDeleter::delete(path) {
                                    eprintln!("❌ 删除失败 {}: {}", path.display(), e);
                                } else {
                                    deleted_count += 1;
                                }
                            }

                            println!("✓ 成功删除 {} 个文件", deleted_count);
                            app.clear_marks();
                        }
                    }
                    KeyAction::None => {}
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
