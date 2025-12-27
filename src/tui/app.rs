use crate::models::DuplicateGroup;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Help,
    Tutorial,  // 新增：引导模式
}

pub struct App {
    pub should_quit: bool,
    pub mode: Mode,
    pub duplicate_groups: Vec<DuplicateGroup>,
    pub selected_group: usize,
    pub selected_file: usize,
    pub marked_files: HashSet<usize>,
    pub show_tutorial: bool,  // 是否显示引导
    pub tutorial_step: usize,  // 引导步骤
}

impl App {
    pub fn new(duplicate_groups: Vec<DuplicateGroup>) -> Self {
        let show_tutorial = !duplicate_groups.is_empty(); // 如果有重复文件，显示引导
        Self {
            should_quit: false,
            mode: if show_tutorial { Mode::Tutorial } else { Mode::Normal },
            duplicate_groups,
            selected_group: 0,
            selected_file: 0,
            marked_files: HashSet::new(),
            show_tutorial,
            tutorial_step: 0,
        }
    }

    pub fn current_group(&self) -> Option<&DuplicateGroup> {
        self.duplicate_groups.get(self.selected_group)
    }

    pub fn group_count(&self) -> usize {
        self.duplicate_groups.len()
    }

    pub fn file_count(&self) -> usize {
        self.current_group()
            .map(|g| g.file_count())
            .unwrap_or(0)
    }

    pub fn next_group(&mut self) {
        if self.selected_group < self.group_count().saturating_sub(1) {
            self.selected_group += 1;
            self.selected_file = 0; // 重置文件选择
        }
    }

    pub fn previous_group(&mut self) {
        if self.selected_group > 0 {
            self.selected_group -= 1;
            self.selected_file = 0; // 重置文件选择
        }
    }

    pub fn next_file(&mut self) {
        if let Some(group) = self.current_group() {
            if group.file_count() > 0 {
                self.selected_file = (self.selected_file + 1) % group.file_count();
                // 循环到第一个文件
            }
        }
    }

    pub fn previous_file(&mut self) {
        if let Some(group) = self.current_group() {
            if group.file_count() > 0 {
                if self.selected_file == 0 {
                    self.selected_file = group.file_count() - 1; // 循环到最后一个
                } else {
                    self.selected_file -= 1;
                }
            }
        }
    }

    pub fn toggle_mark(&mut self) {
        if self.current_group().is_some() {
            let global_index = self.get_global_file_index(self.selected_group, self.selected_file);
            if self.marked_files.contains(&global_index) {
                self.marked_files.remove(&global_index);
            } else {
                self.marked_files.insert(global_index);
            }
        }
    }

    pub fn is_current_file_marked(&self) -> bool {
        if self.current_group().is_some() {
            let global_index = self.get_global_file_index(self.selected_group, self.selected_file);
            self.marked_files.contains(&global_index)
        } else {
            false
        }
    }

    pub fn marked_count(&self) -> usize {
        self.marked_files.len()
    }

    pub fn marked_count_in_group(&self, group_idx: usize) -> usize {
        let mut count = 0;
        let mut global_idx = 0;
        for (i, group) in self.duplicate_groups.iter().enumerate() {
            for _ in &group.files {
                if i == group_idx && self.marked_files.contains(&global_idx) {
                    count += 1;
                }
                global_idx += 1;
            }
        }
        count
    }

    pub fn clear_marks(&mut self) {
        self.marked_files.clear();
    }

    pub fn show_help(&mut self) {
        self.mode = Mode::Help;
    }

    pub fn hide_help(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn exit_tutorial(&mut self) {
        self.mode = Mode::Normal;
        self.show_tutorial = false;
    }

    pub fn next_tutorial_step(&mut self) {
        self.tutorial_step += 1;
        if self.tutorial_step > 3 {
            self.exit_tutorial();
        }
    }

    pub fn get_tutorial_hint(&self) -> &'static str {
        match self.tutorial_step {
            0 => "👋 欢迎！使用 ↑↓ 键选择重复文件组",
            1 => "📌 按 Tab 键在文件之间切换（会循环）",
            2 => "✅ 按 Space（空格）标记要删除的文件",
            3 => "🗑️  按 d 键删除标记的文件，按 ? 查看帮助",
            _ => "",
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    fn get_global_file_index(&self, group_idx: usize, file_idx: usize) -> usize {
        let mut index = 0;
        for (i, group) in self.duplicate_groups.iter().enumerate() {
            if i == group_idx {
                return index + file_idx;
            }
            index += group.file_count();
        }
        index
    }

    // 获取当前操作提示
    pub fn get_action_hint(&self) -> &'static str {
        if self.marked_count() > 0 {
            "已标记文件，按 D 删除全部"
        } else if self.is_current_file_marked() {
            "文件已标记，按 Space 取消"
        } else {
            "按 Space 标记文件"
        }
    }
}
