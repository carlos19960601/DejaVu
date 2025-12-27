use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub struct HelpWidget;

impl HelpWidget {
    pub fn render(f: &mut Frame) {
        let size = f.area();

        // Create a centered rectangle for the help dialog
        let popup_area = Rect {
            x: size.width / 10,
            y: size.height / 10,
            width: size.width * 8 / 10,
            height: size.height * 8 / 10,
        };

        f.render_widget(Clear, popup_area);

        let help_text = vec![
            Line::from("🎯 DejaVu 快捷键指南").style(Style::default().fg(Color::Cyan).bold()),
            Line::from(""),
            Line::from(vec![
                Span::styled("━━━ 导航操作 ━━━",
                    Style::default().fg(Color::Yellow).bold()),
            ]),
            Line::from("  ↑ / ↓ 或 j / k     在重复组之间上下移动"),
            Line::from("  Tab                在当前组的文件间切换"),
            Line::from("  Shift + Tab         反向切换文件"),
            Line::from("  Page Up / Down     快速跳转 5 个重复组"),
            Line::from("  Home / End         跳转到第一个 / 最后一个组"),
            Line::from(""),
            Line::from(vec![
                Span::styled("━━━ 文件操作 ━━━",
                    Style::default().fg(Color::Yellow).bold()),
            ]),
            Line::from("  o                  用系统默认应用打开选中的文件"),
            Line::from("  d                  删除当前选中的文件（需确认）"),
            Line::from("  Space (空格)        标记/取消标记文件"),
            Line::from("  D                  删除所有已标记的文件（需确认）"),
            Line::from("  u                  取消所有标记"),
            Line::from(""),
            Line::from(vec![
                Span::styled("━━━ 其他操作 ━━━",
                    Style::default().fg(Color::Yellow).bold()),
            ]),
            Line::from("  q                  退出程序"),
            Line::from("  ?                  显示/隐藏此帮助"),
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                Span::styled("💡 使用提示:",
                    Style::default().fg(Color::Green).bold()),
            ]),
            Line::from("  • 绿色 ✓ 标记表示推荐的原始文件"),
            Line::from("  • 只删除重复文件，保留原始文件以节省空间"),
            Line::from("  • 可以先标记多个文件，然后按 D 批量删除"),
            Line::from("  • 删除操作需要输入 y 确认，请谨慎操作"),
            Line::from(""),
            Line::from(""),
            Line::from(vec![
                Span::styled("按任意键关闭此帮助",
                    Style::default().fg(Color::Cyan).bold()),
            ]),
        ];

        let paragraph = Paragraph::new(help_text)
            .block(
                Block::default()
                    .title(" 📖 帮助 ")
                    .title_style(Style::default().fg(Color::Cyan).bold())
                    .borders(Borders::ALL),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        f.render_widget(paragraph, popup_area);
    }
}
