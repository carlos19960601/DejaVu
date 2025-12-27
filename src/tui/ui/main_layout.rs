use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::tui::App;
use crate::utils::format_size;

pub struct MainLayout;

impl MainLayout {
    pub fn render(f: &mut Frame, app: &App) {
        // 如果是引导模式，显示引导界面
        if app.mode == crate::tui::Mode::Tutorial {
            Self::render_tutorial(f, app);
            return;
        }

        let size = f.area();

        // Split into 4 parts: stats (top), main content (middle), help (bottom)
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),  // Stats panel + tutorial hint
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Help text
            ])
            .split(size);

        // Render stats panel at top
        Self::render_stats_panel(f, app, main_chunks[0]);

        // Split main content into left (groups) and right (details)
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35),  // Group list
                Constraint::Percentage(65),  // File details
            ])
            .split(main_chunks[1]);

        // Render group list on left
        Self::render_group_list(f, app, content_chunks[0]);

        // Render file details on right
        Self::render_file_details(f, app, content_chunks[1]);

        // Render help text at bottom
        Self::render_help_text(f, main_chunks[2]);
    }

    fn render_tutorial(f: &mut Frame, app: &App) {
        let size = f.area();

        // 创建居中的引导面板
        let tutorial_area = Rect {
            x: size.width / 4,
            y: size.height / 4,
            height: size.height / 2,
            width: size.width / 2,
        };

        f.render_widget(Clear, tutorial_area);

        let tutorial_text = vec![
            Line::from("🎯 DejaVu 使用指南").style(Style::default().fg(Color::Cyan).bold()),
            Line::from(""),
            Line::from("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━").style(Style::default().fg(Color::Yellow)),
            Line::from(""),
            Line::from(app.get_tutorial_hint()).style(Style::default().fg(Color::Green).bold()),
            Line::from(""),
            Line::from(""),
            Line::from("📖 操作说明:").style(Style::default().fg(Color::Cyan)),
            Line::from("  第1步: 用 ↑↓ 键选择重复文件组（左侧列表）"),
            Line::from("  第2步: 按 Tab 键在同一组的文件间循环切换"),
            Line::from("  第3步: 按 Space（空格）标记要删除的重复文件"),
            Line::from("  第4步: 按 d 删除，或按 D 删除所有标记的文件"),
            Line::from(""),
            Line::from(""),
            Line::from("💡 提示: 绿色✓表示推荐的原始文件，请保留它"),
            Line::from(""),
            Line::from(""),
            Line::from("按任意键继续，按 q 退出，按 Enter 跳过引导")
                .style(Style::default().fg(Color::Yellow)),
        ];

        let paragraph = Paragraph::new(tutorial_text)
            .block(
                Block::default()
                    .title(" 👋 新手引导 ")
                    .title_style(Style::default().fg(Color::Cyan).bold())
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, tutorial_area);
    }

    fn render_stats_panel(f: &mut Frame, app: &App, area: Rect) {
        let total_groups = app.group_count();
        let total_files: usize = app.duplicate_groups.iter().map(|g| g.file_count()).sum();
        let marked_count = app.marked_count();
        let total_wasted: u64 = app.duplicate_groups.iter().map(|g| g.wasted_space()).sum();

        let duplicate_files = total_files.saturating_sub(total_groups);

        let stats = vec![
            Line::from(vec![
                Span::styled("📊 找到 ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{} 个重复组", total_groups),
                    Style::default().fg(Color::Yellow).bold(),
                ),
                Span::raw(" • "),
                Span::styled(format!("{} 个文件", total_files),
                    Style::default().fg(Color::White)),
                Span::raw(" • "),
                Span::styled("重复:",
                    Style::default().fg(Color::Red)),
                Span::styled(
                    format!("{}", duplicate_files),
                    Style::default().fg(Color::Red).bold(),
                ),
            ]),
            Line::from(vec![
                Span::styled("💾 可释放: ",
                    Style::default().fg(Color::Green)),
                Span::styled(
                    format_size(total_wasted),
                    Style::default().fg(Color::Yellow).bold(),
                ),
                Span::raw(" • "),
                Span::styled(
                    if marked_count > 0 {
                        format!("已标记 {} 个", marked_count)
                    } else {
                        "未标记".to_string()
                    },
                    Style::default().fg(if marked_count > 0 {
                        Color::Magenta
                    } else {
                        Color::DarkGray
                    }),
                ),
            ]),
            Line::from(vec![
                Span::styled("💡 ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    app.get_action_hint(),
                    Style::default().fg(Color::Green).bold(),
                ),
                Span::raw(" • "),
                Span::styled(
                    "按 ? 查看帮助",
                    Style::default().fg(Color::White),
                ),
            ]),
        ];

        let paragraph = Paragraph::new(stats)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" 📈 统计 ")
                    .title_style(Style::default().fg(Color::Cyan).bold()),
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_group_list(f: &mut Frame, app: &App, area: Rect) {
        let title = format!(" 📁 重复文件组 ({}) ", app.group_count());

        let mut lines = Vec::new();

        // Add header
        lines.push(Line::from(vec![
            Span::styled(" 序号    文件数    大小      标记", Style::default().fg(Color::Cyan).bold()),
        ]));
        lines.push(Line::from("─".repeat(area.width.saturating_sub(2) as usize)));

        if app.duplicate_groups.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  暂无重复文件",
                    Style::default().fg(Color::DarkGray)),
            ]));
        } else {
            for (i, group) in app.duplicate_groups.iter().enumerate() {
                let is_selected = i == app.selected_group;

                let style = if is_selected {
                    Style::default().bg(Color::Blue).fg(Color::White).bold()
                } else {
                    Style::default()
                };

                let prefix = if is_selected { "▶" } else { "▪" };

                let marked_in_group = app.marked_count_in_group(i);
                let mark_indicator = if marked_in_group > 0 {
                    format!("[{}]", marked_in_group)
                } else {
                    "   ".to_string()
                };

                let duplicate_count = group.file_count().saturating_sub(1);

                let line = Line::from(vec![
                    Span::styled(format!("{} ", prefix), style),
                    Span::styled(
                        format!("#{:2}", i + 1),
                        Style::default().fg(Color::Yellow).bold(),
                    ),
                    Span::styled(
                        format!("   {:>4}", group.file_count()),
                        style,
                    ),
                    Span::styled(
                        format!("  {:>8}", format_size(group.total_size())),
                        style,
                    ),
                    Span::styled(
                        format!("   {}", mark_indicator),
                        Style::default()
                            .fg(if marked_in_group > 0 {
                                Color::Magenta
                            } else {
                                Color::DarkGray
                            })
                            .bold(),
                    ),
                    Span::styled(
                        format!("  重复{:>2}个", duplicate_count),
                        Style::default().fg(Color::Red),
                    ),
                ]);

                lines.push(line);
            }
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(paragraph, area);
    }

    fn render_file_details(f: &mut Frame, app: &App, area: Rect) {
        if let Some(group) = app.current_group() {
            let title = format!(
                " 📄 组 #{} - 共 {} 个文件 ",
                app.selected_group + 1,
                group.file_count()
            );

            // Split into file list and action hints
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(7)])
                .split(area);

            // Render file list
            let mut lines = Vec::new();

            // Add header with file numbers
            for (i, file) in group.files.iter().enumerate() {
                let is_selected = i == app.selected_file;
                let is_original = i == group.recommended_original;
                let is_marked = app.is_current_file_marked() && is_selected;

                // 不同的背景色表示不同状态
                let style = if is_selected {
                    if is_marked {
                        Style::default().bg(Color::Magenta).fg(Color::White).bold()
                    } else {
                        Style::default().bg(Color::Blue).fg(Color::White).bold()
                    }
                } else if is_original {
                    Style::default().fg(Color::Green).bold()
                } else {
                    Style::default()
                };

                let prefix = if is_selected { "▶" } else { " " };

                // 状态标记
                let status_mark = if is_original {
                    "✓原始"
                } else if is_marked {
                    "[✓标记]"
                } else {
                    " 重复"
                };

                // 文件名和大小
                let max_name_len = chunks[0]
                    .width
                    .saturating_sub(30) as usize;
                let filename = if file.filename().len() > max_name_len {
                    format!("...{}", &file.filename()[file.filename().len().saturating_sub(max_name_len)..])
                } else {
                    file.filename().to_string()
                };

                let file_num = format!("{}/{}", i + 1, group.file_count());

                let line = Line::from(vec![
                    Span::styled(format!("{} ", prefix), style),
                    Span::styled(
                        format!("{:<6}", file_num),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(status_mark,
                        Style::default().fg(if is_original {
                            Color::Green
                        } else if is_marked {
                            Color::Magenta
                        } else {
                            Color::DarkGray
                        }).bold()),
                    Span::styled(
                        format!(" {:<width$}", filename, width = max_name_len),
                        style,
                    ),
                    Span::styled(
                        format!(" {:>8}", format_size(file.size)),
                        style,
                    ),
                ]);

                lines.push(line);
            }

            let paragraph = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title(title))
                .wrap(Wrap { trim: false });
            f.render_widget(paragraph, chunks[0]);

            // Render action hints
            if let Some(file) = group.files.get(app.selected_file) {
                let is_marked = app.is_current_file_marked();
                let file_type_name = if file.is_image() {
                    "图片"
                } else {
                    "视频"
                };

                let hints = vec![
                    Line::from(vec![
                        Span::styled("【当前选中】", Style::default().fg(Color::Cyan).bold()),
                        Span::styled(
                            file.filename(),
                            Style::default().fg(Color::White).bold(),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("▶ 快捷操作: ", Style::default().fg(Color::Cyan)),
                        Span::styled("[Tab]", Style::default().fg(Color::Yellow).bold()),
                        Span::styled("切换文件 ", Style::default()),
                        Span::styled("[Space]", Style::default().fg(Color::Yellow).bold()),
                        Span::styled(
                            if is_marked { "取消标记" } else { "标记文件" },
                            Style::default().fg(if is_marked {
                                Color::Red
                            } else {
                                Color::Green
                            }).bold()
                        ),
                        Span::styled(" ", Style::default()),
                        Span::styled("[o]打开", Style::default().fg(Color::Green).bold()),
                        Span::styled(" ", Style::default()),
                        Span::styled("[d]删除", Style::default().fg(Color::Red).bold()),
                    ]),
                    Line::from(vec![
                        Span::styled("📊 文件信息: ", Style::default().fg(Color::Cyan)),
                        Span::styled("类型=", Style::default()),
                        Span::styled(
                            file_type_name,
                            Style::default().fg(Color::Magenta),
                        ),
                        Span::styled("  •  大小=", Style::default()),
                        Span::styled(
                            format_size(file.size),
                            Style::default().fg(Color::White).bold(),
                        ),
                    ]),
                    Line::from(vec![
                        Span::styled("📁 完整路径: ", Style::default().fg(Color::Cyan)),
                        Span::styled(
                            format!("{}", file.path.display()),
                            Style::default().fg(Color::DarkGray)),
                    ]),
                    Line::from(vec![
                        Span::styled("💡 提示: ", Style::default().fg(Color::Green)),
                        Span::styled(
                            if is_marked {
                                "文件已标记，按 Space 取消标记"
                            } else {
                                "按 Space 标记此文件为待删除"
                            },
                            Style::default().fg(Color::White),
                        ),
                    ]),
                ];

                let hint_paragraph = Paragraph::new(hints)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" 🛠️  操作面板 ")
                            .title_style(Style::default().fg(Color::Cyan)),
                    )
                    .wrap(Wrap { trim: true });
                f.render_widget(hint_paragraph, chunks[1]);
            }
        } else {
            let paragraph = Paragraph::new("  请选择左侧的文件组")
                .block(Block::default().borders(Borders::ALL).title(" 文件详情 "));
            f.render_widget(paragraph, area);
        }
    }

    fn render_help_text(f: &mut Frame, area: Rect) {
        let help_text = vec![
            Line::from(vec![
                Span::styled("📍 当前: ", Style::default().fg(Color::Cyan)),
                Span::styled("组#", Style::default().fg(Color::Yellow).bold()),
                Span::styled(" | ", Style::default()),
                Span::styled("操作: ", Style::default().fg(Color::Cyan)),
                Span::styled("↑↓选组", Style::default().fg(Color::Green).bold()),
                Span::styled(" ", Style::default()),
                Span::styled("Tab换文件", Style::default().fg(Color::Green).bold()),
                Span::styled(" ", Style::default()),
                Span::styled("Space标记", Style::default().fg(Color::Green).bold()),
                Span::styled(" ", Style::default()),
                Span::styled("d删除", Style::default().fg(Color::Red).bold()),
                Span::styled(" | ", Style::default()),
                Span::styled("q退出", Style::default().fg(Color::Yellow).bold()),
                Span::styled(" ", Style::default()),
                Span::styled("?帮助", Style::default().fg(Color::Cyan).bold()),
            ]),
        ];

        let paragraph = Paragraph::new(help_text)
            .alignment(Alignment::Center)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White).bold());
        f.render_widget(paragraph, area);
    }
}
