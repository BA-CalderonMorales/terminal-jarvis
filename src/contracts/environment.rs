#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnvMode {
    None,
    Any,
    All,
}

impl EnvMode {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "none" => Ok(EnvMode::None),
            "any" => Ok(EnvMode::Any),
            "all" => Ok(EnvMode::All),
            other => Err(format!("unknown env mode '{other}'")),
        }
    }
}
