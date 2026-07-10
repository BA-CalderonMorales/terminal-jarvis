const wrapper = require("./bin/terminal-jarvis");

const status = wrapper.postinstallPathStatus();
if (status.kind === "shadow") {
  console.warn(status.diagnostic);
}
