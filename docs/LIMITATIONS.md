# Known Limitations and Issues

This document outlines current limitations, known issues, and workarounds for Terminal Jarvis.

## Authentication Issues

### ~~Gemini and Qwen Login Problems~~ (RESOLVED in v0.0.40)

- **Previous Issue**: Login would fail on first attempt due to browser opening in headless environments
- **Resolution**: Added AuthManager system that detects CI/headless environments and prevents unwanted browser opening
- **Current Behavior**: Tools now properly prompt for API keys instead of opening browsers in terminal environments
- **Status**: ✅ **FIXED** - Browser authentication issues resolved

## Tool-Specific Issues

### Opencode Input Edge Case

- **Issue**: Rare edge case where Opencode opens but typing doesn't register in the actual input field
- **Potential Cause**: May be related to the runtime/thin layer interaction between Terminal Jarvis and the underlying tools
- **Workaround**: Restart the tool if input becomes unresponsive
- **Status**: Investigating root cause in the wrapper layer

### New Tool Testing

- **Opencode**: Relatively new addition, actively seeking user feedback and testing
- **LLxprt**: Recently added tool, looking for community testers to validate functionality
- **Feedback**: Please report any issues or unexpected behavior with these tools via GitHub issues

## Platform-Specific Requirements

### macOS Prerequisites

- **Requirement**: Rust toolchain must be installed before using Terminal Jarvis
- **Installation**:

  ```bash
  # Install Rust via rustup
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source ~/.cargo/env

  # Then install Terminal Jarvis
  npm install -g terminal-jarvis
  ```

- **Why**: Some tools require compilation or Rust-specific dependencies on macOS

### General Prerequisites

- **Node.js and NPM**: Required for most AI coding tools
- **Internet Connection**: Required for tool installation and updates
- **Terminal Support**: Best experience with modern terminal emulators that support Unicode and colors

## Performance Considerations

### Package Size

- Current NPM package is ~1.2MB compressed / ~2.9MB unpacked
- Includes pre-compiled binaries for immediate functionality
- Future optimizations planned for platform-specific packages

### Tool Detection

- Multi-method verification may cause slight delays during initial tool detection
- Tools are cached after first detection to improve subsequent performance

## Reporting Issues

If you encounter any of these issues or discover new ones:

1. Check this document for known workarounds
2. Search existing [GitHub Issues](https://github.com/BA-CalderonMorales/terminal-jarvis/issues)
3. Create a new issue with:
   - Your operating system and version
   - Terminal Jarvis version (`terminal-jarvis --version`)
   - Steps to reproduce the issue
   - Expected vs actual behavior

## Future Improvements

We're actively working on:

- Streamlining authentication flows for all tools
- Improving the wrapper layer stability
- Platform-specific installation optimizations
- Enhanced error handling and user feedback
