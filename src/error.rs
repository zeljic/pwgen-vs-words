use std;

#[derive(Debug, Clone)]
pub enum GenericErrorKind {
    PWGEN,
    WORDS,
    PIPEREAD,
}

impl std::fmt::Display for GenericErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                GenericErrorKind::PWGEN => "PWGEN",
                GenericErrorKind::WORDS => "WORDS",
                GenericErrorKind::PIPEREAD => "PIPEREAD",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct GenericError {
    pub kind: GenericErrorKind,
    pub message: String,
}

impl std::error::Error for GenericError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] Error: {}", self.kind, self.message)
    }
}
