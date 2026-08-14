#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnvMode {
    None,
    Optional,
    Any,
    All,
}

impl EnvMode {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "none" => Ok(EnvMode::None),
            "optional" => Ok(EnvMode::Optional),
            "any" => Ok(EnvMode::Any),
            "all" => Ok(EnvMode::All),
            other => Err(format!("unknown env mode '{other}'")),
        }
    }
}
