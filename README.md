# Skryvola

Skryvola is an extremely basic Linux-only desktop web browser written in Rust with GTK 3 and WebKitGTK.

It is intentionally small:
- one window
- one tab
- one address bar
- back, forward, and reload/stop controls
- inline error pages

## Linux Only

Skryvola only supports Linux.

## Requirements

You need these system capabilities installed:
- `pkg-config`
- GTK 3 development files
- WebKitGTK 4.1 development files
- libsoup 3 development files

## Run

```bash
cargo run
```

```bash
cargo run -- https://example.com
```

## Intentionally Not Implemented

- tabs
- bookmarks
- persistent history UI
- download manager
- extensions
- devtools UI
- search fallback in the address bar
- cross-platform support
