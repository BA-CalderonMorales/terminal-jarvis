# Known Limitations and Issues

This document outlines current limitations, known issues, and workarounds for Terminal Jarvis.

## Authentication Issues

### ~~Gemini and Qwen Login Problems~~ (RESOLVED in v0.0.40)

- **Previous Issue**: Login would fail on first attempt due to browser opening in headless environments
- **Resolution**: Added comprehensive environment detection system that prevents unwanted browser opening
- **Current Behavior**: Tools now properly prompt for API keys instead of opening browsers in terminal environments
- **Status**: **FIXED** - Browser authentication issues resolved with robust environment detection

## Tool-Specific Issues

### ~~OpenCode Input Focus~~ (RESOLVED in v0.0.41)

- **Previous Issue**: Input box lacked focus on fresh installs, requiring manual clicking before typing
- **Root Cause**: Terminal Jarvis progress indicators and clearing sequences interfered with opencode's terminal initialization
- **Resolution**: Implemented careful terminal state preparation with minimal escape sequences and 75ms initialization delay
- **Current Behavior**: Input box is automatically focused and ready for immediate typing on startup
- **Status**: **FIXED** - Input focus works immediately with comprehensive test coverage

### New Tool Testing

- **LLxprt**: Recently added multi-provider AI coding assistant, actively seeking community feedback
- **Feedback**: Please report any issues or unexpected behavior with tools via GitHub issues
- **Testing**: All tools undergo comprehensive integration testing, but real-world usage patterns help identify edge cases

## Platform-Specific Requirements

### NPM channel prerequisites

- NPM installs require `tar` to extract the downloaded binary during postinstall.
- Internet access is required at install time to fetch the binary from GitHub Releases.
- Offline or air-gapped NPM installs are not supported. Use Cargo or pre-distributed binaries for offline workflows.

### Windows support

- Native Windows support for the NPM package is not yet available. Use WSL2 for a seamless Linux environment.

> For complete installation instructions and prerequisites, see [INSTALLATION.md](INSTALLATION.md)

## Performance Considerations

### Package Size

- Current NPM package is ~1.2MB compressed / ~2.9MB unpacked
- See [INSTALLATION.md](INSTALLATION.md#package-information) for detailed package information and distribution channels

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
