use crate::{rules::MessageRule, types::Payload};
use grammers_client::types::Message;

#[derive(Default)]
pub struct RuleChecker {
    strict_mode: bool,
}

impl RuleChecker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn strict(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    pub async fn check(
        &self,
        rules: Vec<Box<dyn MessageRule>>,
        message: &Message,
    ) -> RuleCheckResult {
        let mut result = RuleCheckResult::new();

        for rule in rules {
            let rule_result = rule.matches(message).await;

            match rule_result.downcast_ref::<bool>() {
                Some(&success) => {
                    if !success {
                        result.mark_failed();
                        if self.strict_mode {
                            break;
                        }
                    }
                }
                None => {
                    if rule_result.downcast_ref::<()>().is_some() {
                        result.mark_failed();
                        if self.strict_mode {
                            break;
                        }
                    } else {
                        result.add_payload(rule_result);
                    }
                }
            }
        }

        result
    }
}

pub struct RuleCheckResult {
    passed: bool,
    payload: Payload,
}

impl RuleCheckResult {
    pub fn new() -> Self {
        Self {
            passed: true,
            payload: Vec::new(),
        }
    }

    pub fn mark_failed(&mut self) {
        self.passed = false;
    }

    pub fn add_payload(&mut self, item: Box<dyn std::any::Any + Send>) {
        self.payload.push(item);
    }

    pub fn all_passed(&self) -> bool {
        self.passed
    }

    pub fn into_payload(self) -> Payload {
        self.payload
    }
}
