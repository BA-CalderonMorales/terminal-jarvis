const wrapper = require("./bin/terminal-jarvis");

const status = wrapper.postinstallPathStatus();
if (status.diagnostic) console.warn(status.diagnostic);
if (status.kind === "shadow") process.exitCode = 1;
