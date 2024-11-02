// src/braille.rs
use crate::mapping;
use std::collections::HashMap;

pub struct BrailleConverter {
    mapping: HashMap<char, [bool; 6]>,
}

impl BrailleConverter {
    pub fn new() -> Self {
        Self {
            mapping: mapping::create_mapping(),
        }
    }

    pub fn convert_text(&self, text: &str) -> Vec<(char, [bool; 6])> {
        let mut result: Vec<(char, [bool; 6])> = Vec::new();
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if mapping::is_number(c) {
                // まだ数字モードでない場合、数符を追加
                if result.is_empty() || !mapping::is_number(result.last().unwrap().0) {
                    result.push(('#', mapping::get_number_prefix()));
                }
            }

            if let Some(pattern) = self.mapping.get(&c) {
                result.push((c, *pattern));
            }
        }
        result
    }

    pub fn display_text(&self, patterns: &[(char, [bool; 6])]) -> String {
        if patterns.is_empty() {
            return String::new();
        }

        let mut result = String::new();

        // 原文を表示（数符は表示しない）
        result.push_str("原文: ");
        for (char, _) in patterns {
            if *char != '#' {
                // 数符のマーカーはスキップ
                result.push(*char);
                result.push(' ');
            }
        }
        result.push('\n');
        result.push('\n');

        // 点字を表示（上段）
        for (_, pattern) in patterns {
            result.push_str(&format!(
                "{} {} ",
                if pattern[0] { "●" } else { "○" },
                if pattern[1] { "●" } else { "○" }
            ));
        }
        result.push('\n');

        // 点字を表示（中段）
        for (_, pattern) in patterns {
            result.push_str(&format!(
                "{} {} ",
                if pattern[2] { "●" } else { "○" },
                if pattern[3] { "●" } else { "○" }
            ));
        }
        result.push('\n');

        // 点字を表示（下段）
        for (_, pattern) in patterns {
            result.push_str(&format!(
                "{} {} ",
                if pattern[4] { "●" } else { "○" },
                if pattern[5] { "●" } else { "○" }
            ));
        }
        result.push('\n');

        result
    }

    pub fn is_supported(&self, c: char) -> bool {
        self.mapping.contains_key(&c)
    }
}
