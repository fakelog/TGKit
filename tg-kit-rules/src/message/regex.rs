use async_trait::async_trait;
use grammers_client::types::Message;
use regex::Regex;
use std::borrow::Cow;
use tg_kit::{rules::MessageRule, types::PayloadItem};

#[derive(Debug)]
pub struct RegexResult {
    pub matches: Vec<String>,
}

pub struct RegexRule {
    pattern: Regex,
}

impl RegexRule {
    pub fn new(pattern: impl Into<Cow<'static, str>>) -> Result<Self, regex::Error> {
        let pattern = pattern.into();
        let regex = Regex::new(&pattern)?;

        Ok(Self { pattern: regex })
    }
}

#[async_trait]
impl MessageRule for RegexRule {
    async fn matches(&self, message: &Message) -> PayloadItem {
        let message = message.text().to_lowercase();

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
