use std;

#[derive(Debug, Clone)]
pub enum GeneralErrorKind {
    PWGEN,
    WORDS,
}

impl std::fmt::Display for GeneralErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                GeneralErrorKind::PWGEN => "PWGEN",
                GeneralErrorKind::WORDS => "WORDS",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct GeneralError {
    pub kind: GeneralErrorKind,
    pub message: String,
}

impl std::error::Error for GeneralError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for GeneralError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] Error: {}", self.kind, self.message)
    }
}
