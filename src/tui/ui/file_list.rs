// This file is intentionally left minimal as the list rendering
// is handled in main_layout.rs for better layout coordination

use ratatui::Frame;
use crate::tui::App;

pub struct FileListWidget;

impl FileListWidget {
    pub fn render(_f: &mut Frame, _app: &App) {
        // Rendering handled in main_layout.rs
    }
}
