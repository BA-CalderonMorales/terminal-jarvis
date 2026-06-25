fn main() {
    let home = terminal_jarvis::context::default_home();
    let catalog = terminal_jarvis::context::catalog_root();
    let code = terminal_jarvis::cli::run(std::env::args(), &catalog, &home);
    std::process::exit(code);
}
