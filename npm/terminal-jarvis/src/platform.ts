export type SupportedPlatform = NodeJS.Platform | "android";

export function getBundledBinaryName(
  currentPlatform: SupportedPlatform,
  currentArch: string,
): string {
  let binaryName = "terminal-jarvis";

  if (currentPlatform === "linux" || currentPlatform === "android") {
    binaryName += "-linux";
  } else if (currentPlatform === "darwin") {
    binaryName += "-macos";
  } else if (currentPlatform === "win32") {
    binaryName += "-windows.exe";
  } else {
    binaryName += "-linux";
  }

  if (currentPlatform !== "win32") {
    if (currentArch === "x64" || currentArch === "x86_64") {
      binaryName += "-x64";
    } else if (currentArch === "arm64") {
      binaryName += "-arm64";
    } else {
      binaryName += "-x64";
    }
  }

  return binaryName;
}
