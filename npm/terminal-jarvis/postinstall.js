const wrapper = require("./bin/terminal-jarvis");

const status = wrapper.postinstallPathStatus();
if (status.diagnostic) {
  const write = status.kind === "shadow" ? console.error : console.warn;
  write(status.diagnostic);
}
if (status.kind === "shadow") {
  console.error(
    "terminal-jarvis: refusing to complete a global install because " +
    "the installed npm package would not be the command resolved on PATH. " +
    "Move the npm prefix earlier in PATH, remove the stale binary, or set " +
    "TERMINAL_JARVIS_SKIP_PATH_DIAGNOSTIC=1 to bypass this check."
  );
  process.exit(1);
}
