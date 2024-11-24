use std::io::BufRead;
use std::path::Path;

use devicons::FileIcon;

use super::OnAir;
use crate::entry::{Entry, PreviewType};
use television_fuzzy::matcher::{config::Config, Matcher};

pub struct Channel {
    matcher: Matcher<String>,
    icon: FileIcon,
    preview_type: PreviewType,
}

const NUM_THREADS: usize = 2;

impl Channel {
    pub fn new(preview_type: Option<PreviewType>) -> Self {
        let mut lines = Vec::new();
        for line in std::io::stdin().lock().lines().map_while(Result::ok) {
            if !line.trim().is_empty() {
                lines.push(line);
            }
        }
        let matcher = Matcher::new(Config::default().n_threads(NUM_THREADS));
        let injector = matcher.injector();
        for line in lines.iter().rev() {
            let () = injector.push(line.clone(), |e, cols| {
                cols[0] = e.clone().into();
            });
        }
        Self {
            matcher,
            icon: FileIcon::from("nu"),
            preview_type: preview_type.unwrap_or(PreviewType::Basic),
        }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self::new(None)
    }
}

impl OnAir for Channel {
    fn find(&mut self, pattern: &str) {
        self.matcher.find(pattern);
    }

    fn results(&mut self, num_entries: u32, offset: u32) -> Vec<Entry> {
        self.matcher.tick();
        self.matcher
            .results(num_entries, offset)
            .into_iter()
            .map(|item| {
                let path = Path::new(&item.matched_string);
                let icon = if path.try_exists().unwrap_or(false) {
                    FileIcon::from(path)
                } else {
                    self.icon
                };
                Entry::new(item.matched_string, self.preview_type.clone())
                    .with_name_match_ranges(item.match_indices)
                    .with_icon(icon)
            })
            .collect()
    }

    fn get_result(&self, index: u32) -> Option<Entry> {
        self.matcher.get_result(index).map(|item| {
            Entry::new(item.matched_string.clone(), self.preview_type.clone())
                .with_icon(self.icon)
        })
    }

    fn result_count(&self) -> u32 {
        self.matcher.matched_item_count
    }

    fn total_count(&self) -> u32 {
        self.matcher.total_item_count
    }

    fn running(&self) -> bool {
        self.matcher.status.running
    }

    fn shutdown(&self) {}
}
