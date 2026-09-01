use std::collections::{HashMap, HashSet};

pub struct ConversionContext {
    pub warned_messages: HashSet<String>,
    pub substitution_counts: HashMap<String, i64>,
}

impl ConversionContext {
    pub fn new() -> Self {
        ConversionContext {
            warned_messages: HashSet::new(),
            substitution_counts: HashMap::from([
                ("keys".to_string(), 0),
                ("events".to_string(), 0),
                ("layers".to_string(), 0),
                ("directions".to_string(), 0),
            ]),
        }
    }

    pub fn warn(&mut self, message: &str, strict: bool, once: bool) {
        if strict {
            panic!("{}", message);
        }
        if once {
            if self.warned_messages.contains(message) {
                return;
            }
            self.warned_messages.insert(message.to_string());
        }
        eprintln!("warning: {}", message);
    }

    pub fn bump(&mut self, category: &str) {
        if let Some(c) = self.substitution_counts.get_mut(category) {
            *c += 1;
        }
    }

    pub fn substitution_summary(&self) -> Option<String> {
        let total: i64 = self.substitution_counts.values().sum();
        if total == 0 {
            return None;
        }
        Some(format!(
            "conversion substitutions: keys={}, events={}, layers={}, directions={}",
            self.substitution_counts["keys"],
            self.substitution_counts["events"],
            self.substitution_counts["layers"],
            self.substitution_counts["directions"],
        ))
    }
}

impl Default for ConversionContext {
    fn default() -> Self {
        Self::new()
    }
}
