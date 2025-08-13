use crate::models::Connection;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

#[derive(Default)]
pub struct AppState {
    pub connections: Vec<Connection>,
    pub filtered_indices: Vec<usize>,
    pub selected: usize,
    pub search: String,
    pub mode: Mode,
    pub show_help: bool,
    pub pending_delete: bool,
    pub add_buffer: String,
    pub status: String,
}

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Search,
    Add,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

impl AppState {
    pub fn apply_filter(&mut self) {
        let matcher = SkimMatcherV2::default();
        let q = self.search.trim();
        if q.is_empty() {
            self.filtered_indices = (0..self.connections.len()).collect();
            self.selected = 0.min(self.filtered_indices.len().saturating_sub(1));
            return;
        }
        let mut scored: Vec<(i64, usize)> = self
            .connections
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                let hay = c.label();
                matcher.fuzzy_match(&hay, q).map(|score| (score, i))
            })
            .collect();
        scored.sort_by(|a, b| b.0.cmp(&a.0));
        self.filtered_indices = scored.into_iter().map(|(_, i)| i).collect();
        self.selected = 0.min(self.filtered_indices.len().saturating_sub(1));
    }
}