# Version Caching System

Terminal Jarvis now includes an intelligent version caching system that reduces API calls and improves startup performance.

## How It Works

The version caching system stores NPM distribution tag information locally to avoid repeated API calls every time you access the Terminal Jarvis home page.

### Cache Features

- **Automatic Caching**: Version information is cached automatically on first access
- **TTL Support**: Configurable Time-To-Live (default: 1 hour)
- **Automatic Expiration**: Expired cache entries are automatically cleaned up
- **Fallback**: If cache fails, falls back to direct API calls gracefully

### Cache Storage

Cache files are stored in your system's standard configuration directory:

- **Linux/macOS**: `~/.config/terminal-jarvis/version_cache.toml`
- **Windows**: `%APPDATA%\terminal-jarvis\version_cache.toml`

### CLI Commands

#### Check Cache Status

```bash
terminal-jarvis cache status
```

Shows current cache state, including:

- Cached version information
- Cache timestamp
- TTL settings
- Time remaining until expiration

#### Refresh Cache

```bash
terminal-jarvis cache refresh --ttl 3600
```

Manually refresh the cache with a custom TTL (in seconds).

#### Clear Cache

```bash
terminal-jarvis cache clear
```

Remove all cached version information.

### Performance Benefits

- **Faster Startup**: Interactive mode loads instantly without waiting for API calls
- **Reduced Network Traffic**: Fewer calls to NPM registry
- **Better Offline Experience**: Shows cached version info when network is unavailable
- **Graceful Degradation**: Falls back cleanly if caching fails

### Implementation Details

The caching system is implemented with the following components:

1. **VersionCache struct**: Stores version info with timestamp and TTL
2. **ConfigManager**: Handles cache file I/O operations
3. **Cached API methods**: Provide caching layer over existing API calls

### Cache File Format

The cache is stored as TOML:

```toml
version_info = "stable, beta"
cached_at = 1754850485
ttl_seconds = 3600
```

### Error Handling

- Cache failures are non-fatal and won't break the application
- Warning messages are logged for debugging
- Automatic cleanup of corrupted cache files
- Graceful fallback to direct API calls when needed

This caching system significantly improves the user experience while maintaining reliability and data freshness.
