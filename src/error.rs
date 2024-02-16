use std::fmt;

#[derive(Debug, Clone)]

pub(crate) enum Kind {
    REQWUEST,
    SERDE,
    INTERNAL,
}

pub(crate) struct BootError {
    pub(crate) kind: Kind,
    pub(crate) message: String,
}

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ERROR: {}", self.kind, self.message)
    }
}

impl From<reqwest::Error> for BootError {
    fn from(error: reqwest::Error) -> Self {
        BootError {
            kind: Kind::REQWUEST,
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for BootError {
    fn from(error: serde_json::Error) -> Self {
        BootError {
            kind: Kind::SERDE,
            message: error.to_string(),
        }
    }
}