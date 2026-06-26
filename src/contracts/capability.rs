#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Capability {
    Download,
    Update,
    Headless,
    Version,
    Stats,
    Models,
    Security,
    Yolo,
    Ui,
}

impl Capability {
    pub const ALL: [Capability; 9] = [
        Capability::Download,
        Capability::Update,
        Capability::Headless,
        Capability::Version,
        Capability::Stats,
        Capability::Models,
        Capability::Security,
        Capability::Yolo,
        Capability::Ui,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Capability::Download => "download",
            Capability::Update => "update",
            Capability::Headless => "headless",
            Capability::Version => "version",
            Capability::Stats => "stats",
            Capability::Models => "models",
            Capability::Security => "security",
            Capability::Yolo => "yolo",
            Capability::Ui => "ui",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "download" => Some(Capability::Download),
            "update" => Some(Capability::Update),
            "headless" => Some(Capability::Headless),
            "version" => Some(Capability::Version),
            "stats" => Some(Capability::Stats),
            "models" => Some(Capability::Models),
            "security" => Some(Capability::Security),
            "yolo" => Some(Capability::Yolo),
            "ui" => Some(Capability::Ui),
            _ => None,
        }
    }
}

impl std::fmt::Display for Capability {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
