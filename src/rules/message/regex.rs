use async_trait::async_trait;
use regex::Regex;

use crate::types::PayloadItem;

use super::MessageRule;

#[derive(Debug)]
pub struct RegexResult {
    pub matches: Vec<String>,
}

pub struct RegexRule {
    pattern: Regex,
}

impl RegexRule {
    pub fn new(pattern: &str) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern)?;
        Ok(RegexRule { pattern: regex })
    }
}

#[async_trait]
impl MessageRule for RegexRule {
    async fn matches(&self, message: &str) -> PayloadItem {
        let message = message.to_lowercase();

        if let Some(captures) = self.pattern.captures(&message) {
            // Преобразуем найденные группы в вектор строк
            let matches: Vec<String> = captures
                .iter()
                .filter_map(|m| m.map(|mat| mat.as_str().to_string()))
                .collect();

            // Возвращаем найденные группы как PayloadItem
            Box::new(RegexResult { matches }) as PayloadItem
        } else {
            // Если совпадений нет, возвращаем false
            Box::new(false) as PayloadItem
        }
    }
}
