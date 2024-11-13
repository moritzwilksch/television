use crate::ui::input::Input;
use ratatui::widgets::ListState;
use television_utils::strings::EMPTY_STRING;

#[derive(Debug)]
pub struct Picker {
    pub(crate) state: ListState,
    pub(crate) relative_state: ListState,
    pub(crate) view_offset: usize,
    _inverted: bool,
    pub(crate) input: Input,
}

impl Default for Picker {
    fn default() -> Self {
        Self::new()
    }
}

impl Picker {
    fn new() -> Self {
        Self {
            state: ListState::default(),
            relative_state: ListState::default(),
            view_offset: 0,
            _inverted: false,
            input: Input::new(EMPTY_STRING.to_string()),
        }
    }

    pub(crate) fn inverted(mut self) -> Self {
        self._inverted = !self._inverted;
        self
    }

    pub(crate) fn reset_selection(&mut self) {
        self.state.select(Some(0));
        self.relative_state.select(Some(0));
        self.view_offset = 0;
    }

    pub(crate) fn reset_input(&mut self) {
        self.input.reset();
    }

    pub(crate) fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub(crate) fn select(&mut self, index: Option<usize>) {
        self.state.select(index);
    }

    fn relative_selected(&self) -> Option<usize> {
        self.relative_state.selected()
    }

    pub(crate) fn relative_select(&mut self, index: Option<usize>) {
        self.relative_state.select(index);
    }

    pub(crate) fn select_next(&mut self, total_items: usize, height: usize) {
        if self._inverted {
            self._select_prev(total_items, height);
        } else {
            self._select_next(total_items, height);
        }
    }

    pub(crate) fn select_prev(&mut self, total_items: usize, height: usize) {
        if self._inverted {
            self._select_next(total_items, height);
        } else {
            self._select_prev(total_items, height);
        }
    }

    fn _select_next(&mut self, total_items: usize, height: usize) {
        let selected = self.selected().unwrap_or(0);
        let relative_selected = self.relative_selected().unwrap_or(0);
        if selected > 0 {
            self.select(Some(selected - 1));
            self.relative_select(Some(relative_selected.saturating_sub(1)));
            if relative_selected == 0 {
                self.view_offset = self.view_offset.saturating_sub(1);
            }
        } else {
            self.view_offset =
                total_items.saturating_sub(height.saturating_sub(2));
            self.select(Some(total_items.saturating_sub(1)));
            self.relative_select(Some(height.saturating_sub(3)));
        }
    }

    fn _select_prev(&mut self, total_items: usize, height: usize) {
        let new_index = (self.selected().unwrap_or(0) + 1) % total_items;
        self.select(Some(new_index));
        if new_index == 0 {
            self.view_offset = 0;
            self.relative_select(Some(0));
            return;
        }
        if self.relative_selected().unwrap_or(0) == height.saturating_sub(3) {
            self.view_offset += 1;
            self.relative_select(Some(
                self.selected().unwrap_or(0).min(height.saturating_sub(3)),
            ));
        } else {
            self.relative_select(Some(
                (self.relative_selected().unwrap_or(0) + 1)
                    .min(self.selected().unwrap_or(0)),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::used_underscore_binding)]
    fn test_picker_new() {
        let picker = Picker::new();
        assert_eq!(picker.state.selected(), None);
        assert_eq!(picker.relative_state.selected(), None);
        assert_eq!(picker.view_offset, 0);
        assert!(!picker._inverted);
        assert_eq!(picker.input.value(), EMPTY_STRING);
    }

    #[test]
    #[allow(clippy::used_underscore_binding)]
    fn test_picker_inverted() {
        let picker = Picker::new().inverted();
        assert!(picker._inverted);
    }

    #[test]
    fn test_picker_reset_selection() {
        let mut picker = Picker::new();
        picker.state.select(Some(1));
        picker.relative_state.select(Some(1));
        picker.view_offset = 1;
        picker.reset_selection();
        assert_eq!(picker.state.selected(), Some(0));
        assert_eq!(picker.relative_state.selected(), Some(0));
        assert_eq!(picker.view_offset, 0);
    }

    #[test]
    fn test_picker_reset_input() {
        let mut picker = Picker::new();
        picker.input = Input::new("test".to_string());
        assert_eq!(picker.input.value(), "test");
        picker.reset_input();
        assert_eq!(picker.input.value(), EMPTY_STRING);
    }

    #[test]
    fn test_picker_select() {
        let mut picker = Picker::new();
        assert_eq!(picker.selected(), None);
        picker.state.select(Some(1));
        assert_eq!(picker.selected(), Some(1));
    }

    #[test]
    fn test_picker_relative_select() {
        let mut picker = Picker::new();
        assert_eq!(picker.relative_selected(), None);
        picker.relative_state.select(Some(1));
        assert_eq!(picker.relative_selected(), Some(1));
    }

    #[test]
    fn test_picker_select_next() {
        let mut picker = Picker::new();
        picker.select(Some(1));
        picker.relative_select(Some(1));
        picker.select_next(5, 3);
        assert_eq!(picker.selected(), Some(0));
        assert_eq!(picker.relative_selected(), Some(0));
        assert_eq!(picker.view_offset, 0);
    }
}
