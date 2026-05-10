import { describe, expect, test } from "vitest";
import { getBundledBinaryName } from "../src/platform";

const { getPlatformInfo } = require("../scripts/postinstall.js");

describe("platform resolution", () => {
  test("launcher maps Android arm64 to Linux ARM64 bundled binary", () => {
    expect(getBundledBinaryName("android", "arm64")).toBe(
      "terminal-jarvis-linux-arm64",
    );
  });

  test("launcher maps Android x64 to Linux x64 bundled binary", () => {
    expect(getBundledBinaryName("android", "x64")).toBe(
      "terminal-jarvis-linux-x64",
    );
  });

  test("postinstall maps Android arm64 to Linux release archive", () => {
    expect(getPlatformInfo("android", "arm64")).toEqual({
      name: "Android/Termux",
      file: "terminal-jarvis-linux.tar.gz",
      isWindows: false,
    });
  });
});
