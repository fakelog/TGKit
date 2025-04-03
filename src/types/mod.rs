use std::any::Any;

pub type PayloadItem = Box<dyn Any + Send>;
pub type Payload = Vec<PayloadItem>;
