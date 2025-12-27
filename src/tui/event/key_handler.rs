use crossterm::event::{KeyCode, KeyEvent};
use crate::tui::{App, Mode};

pub enum KeyAction {
    None,
    OpenFile,
    DeleteFile,
    DeleteMarked,
}

pub fn handle_key_event(key_event: KeyEvent, app: &mut App) -> KeyAction {
    // 处理引导模式
    if app.mode == Mode::Tutorial {
        match key_event.code {
            KeyCode::Up | KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('k') => {
                app.next_tutorial_step();
                KeyAction::None
            }
            KeyCode::Tab => {
                app.next_tutorial_step();
                KeyAction::None
            }
            KeyCode::Char(' ') => {
                app.next_tutorial_step();
                KeyAction::None
            }
            KeyCode::Enter => {
                app.exit_tutorial();
                KeyAction::None
            }
            KeyCode::Char('q') => {
                app.exit_tutorial();
                KeyAction::None
            }
            _ => KeyAction::None,
        }
    } else if app.mode == Mode::Help {
        // 任意键关闭帮助
        app.hide_help();
        KeyAction::None
    } else {
        // 正常模式
        match key_event.code {
            // 退出
            KeyCode::Char('q') => {
                app.quit();
                KeyAction::None
            }

            // 帮助
            KeyCode::Char('?') => {
                app.show_help();
                KeyAction::None
            }

            // 导航 - 在重复组之间移动
            KeyCode::Down | KeyCode::Char('j') => {
                app.next_group();
                KeyAction::None
            }

            KeyCode::Up | KeyCode::Char('k') => {
                app.previous_group();
                KeyAction::None
            }

            // Tab - 在同一组的文件间循环切换
            KeyCode::Tab => {
                app.next_file();
                KeyAction::None
            }

            // Shift+Tab (或 h) - 反向切换文件
            KeyCode::BackTab => {
                app.previous_file();
                KeyAction::None
            }

            KeyCode::Char('h') => {
                app.previous_file();
                KeyAction::None
            }

            // 标记/取消标记
            KeyCode::Char(' ') => {
                app.toggle_mark();
                KeyAction::None
            }

            // 打开文件
            KeyCode::Char('o') => {
                KeyAction::OpenFile
            }

            // 删除文件
            KeyCode::Char('d') => {
                KeyAction::DeleteFile
            }

            // 删除所有标记
            KeyCode::Char('D') => {
                if app.marked_count() > 0 {
                    KeyAction::DeleteMarked
                } else {
                    KeyAction::None
                }
            }

            // 清除标记
            KeyCode::Char('u') => {
                app.clear_marks();
                KeyAction::None
            }

            // Page Down - 跳转5组
            KeyCode::PageDown => {
                for _ in 0..5 {
                    app.next_group();
                }
                KeyAction::None
            }

            // Page Up - 回退5组
            KeyCode::PageUp => {
                for _ in 0..5 {
                    app.previous_group();
                }
                KeyAction::None
            }

            // Home - 第一组
            KeyCode::Home => {
                app.selected_group = 0;
                app.selected_file = 0;
                KeyAction::None
            }

            // End - 最后一组
            KeyCode::End => {
                app.selected_group = app.group_count().saturating_sub(1);
                app.selected_file = 0;
                KeyAction::None
            }

            // Enter - 退出引导模式
            KeyCode::Enter => {
                if app.show_tutorial {
                    app.exit_tutorial();
                }
                KeyAction::None
            }

            _ => KeyAction::None,
        }
    }
}
