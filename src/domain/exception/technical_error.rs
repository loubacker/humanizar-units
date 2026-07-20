use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TechnicalError {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl TechnicalError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    pub(crate) fn with_source(
        message: impl Into<String>,
        source: impl Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for TechnicalError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for TechnicalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn Error + 'static))
    }
}
