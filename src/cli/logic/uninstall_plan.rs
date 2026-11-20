//! Which command removes a harness. The tool's own documented remover wins;
//! otherwise removal inverts the channel that installed it (npm, uv, cargo).
//! A script installer has no honest inverse, so it has no plan here.

use super::error;
use crate::contracts::{CapabilityPlan, Harness};

/// `source` is the download row's own evidence URL; a script install has no
/// derivable inverse, so the vendor page is the honest pointer.
pub(super) fn unavailable(name: &str, source: Option<&str>) -> error::Failure {
    let hint = match source.filter(|url| url.starts_with("https://")) {
        Some(url) => format!("{name} was installed by a script; follow the vendor docs at {url}"),
        None => "remove it with the tool that installed it".to_string(),
    };
    error::Failure::state(
        "uninstall_unavailable",
        format!("no uninstaller derivable for {name}"),
        hint,
    )
}

/// The removal plan: the tool's own uninstaller when its catalog entry
/// declares one, otherwise the inverse of the channel that installed it --
/// npm uninstall -g p, uv tool uninstall p, cargo uninstall bin.
pub(super) fn derive(
    download: &CapabilityPlan,
    harness: &Harness,
) -> Option<crate::contracts::CommandPlan> {
    if let Some(declared) = &download.uninstall {
        return Some(declared.clone());
    }
    let command = download.command.command.as_str();
    let args = &download.command.args;
    let package = download.package.as_deref().or_else(|| {
        args.iter()
            .rev()
            .find(|arg| !arg.starts_with('-') && !arg.contains('/'))
            .map(|arg| arg.split('@').next().unwrap_or(arg))
    })?;
    if command == "npm" && args.iter().any(|arg| arg == "install") {
        return Some(crate::contracts::CommandPlan::new(
            "npm".into(),
            vec!["uninstall".into(), "-g".into(), package.into()],
        ));
    }
    if command == "uv" && args.iter().any(|arg| arg == "tool") {
        return Some(crate::contracts::CommandPlan::new(
            "uv".into(),
            vec!["tool".into(), "uninstall".into(), package.into()],
        ));
    }
    if command == "cargo" && args.first().map(String::as_str) == Some("install") {
        return Some(crate::contracts::CommandPlan::new(
            "cargo".into(),
            vec!["uninstall".into(), harness.binary.clone()],
        ));
    }
    None
}
