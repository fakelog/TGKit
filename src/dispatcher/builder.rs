use crate::{handlers::EventHandler, middleware::Middleware};
use anyhow::Result;

use super::EventDispatcher;

/// Builder для создания и настройки EventDispatcher
pub struct EventDispatcherBuilder {
    handlers: Vec<Box<dyn EventHandler>>,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl EventDispatcherBuilder {
    /// Создает новый билдер
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    /// Добавляет обработчик событий
    pub fn with_handler(mut self, handler: Box<dyn EventHandler>) -> Self {
        self.handlers.push(handler);
        self
    }

    /// Добавляет middleware
    pub fn with_middleware(mut self, middleware: Box<dyn Middleware>) -> Self {
        self.middlewares.push(middleware);
        self
    }

    /// Добавляет несколько обработчиков
    pub fn with_handlers(mut self, handlers: Vec<Box<dyn EventHandler>>) -> Self {
        self.handlers.extend(handlers);
        self
    }

    /// Добавляет несколько middleware
    pub fn with_middlewares(mut self, middlewares: Vec<Box<dyn Middleware>>) -> Self {
        self.middlewares.extend(middlewares);
        self
    }

    /// Строит EventDispatcher
    pub async fn build(self) -> Result<EventDispatcher> {
        let mut dispatcher = EventDispatcher::new();

        for handler in self.handlers {
            dispatcher.register_handler(handler);
        }

        for middleware in self.middlewares {
            dispatcher.register_middleware(middleware).await;
        }

        Ok(dispatcher)
    }
}

impl Default for EventDispatcherBuilder {
    fn default() -> Self {
        Self::new()
    }
}
