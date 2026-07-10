const wrapper = require("./bin/terminal-jarvis");

const status = wrapper.postinstallPathStatus();
if (status.diagnostic) {
  console.warn(status.diagnostic);
}
