# SigmaOS Nemo

Rust data models for file-manager navigation, bookmarks, view settings, and
Nemo-style context actions. The exported `NemoFileManager` currently stores
navigation and file-list state in memory.

## Current behavior and limits

`refresh_files()` clears the in-memory file list; it does not scan a directory.
Copy, move, and delete operations are represented as enum values but do not
perform filesystem changes. Opening a terminal and opening as root are
placeholders. This crate is not a usable file manager or privilege broker.

`NemoAction::format_exec()` constructs a command string by substituting file
paths. It is not an argv parser or a safe shell-escaping boundary; do not pass
untrusted paths through a shell using this output. A future executor should
build an executable plus argument vector and enforce action permissions.

File operations should be keyboard-accessible, predictable, and explicit about
permissions and destructive changes. Validate filesystem behavior and recovery
before claiming those operations are implemented.

## Development

The crate depends on `SigmaOS-kernel` through a sibling path dependency. CI
resolves SigmaOS path dependencies and runs `cargo test --all-targets --locked`.
