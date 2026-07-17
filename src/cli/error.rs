#[derive(Debug, Eq, PartialEq)]
pub struct Failure {
    pub exit_code: i32,
    pub code: &'static str,
    pub message: String,
    pub next_action: String,
}

impl Failure {
    pub fn usage(
        code: &'static str,
        message: impl Into<String>,
        next_action: impl Into<String>,
    ) -> Self {
        Self::new(2, code, message, next_action)
    }

    pub fn state(
        code: &'static str,
        message: impl Into<String>,
        next_action: impl Into<String>,
    ) -> Self {
        Self::new(3, code, message, next_action)
    }

    pub fn unavailable(
        code: &'static str,
        message: impl Into<String>,
        next_action: impl Into<String>,
    ) -> Self {
        Self::new(4, code, message, next_action)
    }

    pub fn safety(
        code: &'static str,
        message: impl Into<String>,
        next_action: impl Into<String>,
    ) -> Self {
        Self::new(5, code, message, next_action)
    }

    fn new(
        exit_code: i32,
        code: &'static str,
        message: impl Into<String>,
        next_action: impl Into<String>,
    ) -> Self {
        Self {
            exit_code,
            code,
            message: message.into(),
            next_action: next_action.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Failure>;
