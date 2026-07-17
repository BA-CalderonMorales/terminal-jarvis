#[derive(Clone, Copy)]
pub enum Route {
    Command {
        name: &'static str,
        command: &'static str,
        args: &'static [&'static str],
    },
    Manual {
        name: &'static str,
        guidance: &'static str,
    },
}

pub fn selected() -> Route {
    if wrapper_path().is_some() {
        return npm();
    }
    match crate::distribution::channel() {
        Some("npm") => npm(),
        Some("homebrew") => command("homebrew", "brew", &["upgrade", "terminal-jarvis"]),
        Some("cargo" | "source") => command("cargo", "cargo", &["install", "terminal-jarvis"]),
        Some("direct") => Route::Manual {
            name: "direct",
            guidance: "download and checksum-verify the matching direct release asset",
        },
        _ => Route::Manual {
            name: "unknown",
            guidance: "identify the install channel before updating",
        },
    }
}

pub fn name() -> &'static str {
    match selected() {
        Route::Command { name, .. } | Route::Manual { name, .. } => name,
    }
}

fn npm() -> Route {
    command("npm", "npm", &["install", "-g", "terminal-jarvis@latest"])
}

fn command(name: &'static str, command: &'static str, args: &'static [&'static str]) -> Route {
    Route::Command {
        name,
        command,
        args,
    }
}

pub(super) fn wrapper_path() -> Option<std::path::PathBuf> {
    let wrapper = std::env::var("TERMINAL_JARVIS_WRAPPER").ok()?;
    let package = std::path::Path::new(&wrapper)
        .parent()
        .and_then(std::path::Path::parent)?
        .join("package.json");
    package.exists().then_some(package)
}

#[cfg(test)]
pub(super) fn homebrew_path(path: &str) -> bool {
    crate::distribution::homebrew_path(path)
}
