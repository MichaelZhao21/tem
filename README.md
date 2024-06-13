# tem (Tagged Editor for Markdown)

I wanted to build my own system for markdown text editing. I didn't want to rely on traditional interfaces like Electron with web; instead, I decided to choose GTK for a balance between ease of use and low-level performance.

Although there are solutions out there (Google Drive, Obsidian, Vi), none of them were perfectly suited to my use case (Obsidian came close but it isn't open source sad face).

## Setup and Execution

You must have GTK4 installed! This varies in difficulty, but shouldn't be too bad on Linux/macOS. See the [gtk-rs book's installation guide](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html) for details.

Then just run `cargo run` to test the app!

## Details

### Tech used

- Rust
- GTK4
- [gtk-rs](https://gtk-rs.org/gtk4-rs/stable/latest/book/introduction.html)

### Current Features
- A window opens!
- Layout is getting laid... wait that's not-

### Future Features

**MVP**
- File manager
- Tagging system
- Text editor
- Markdown editor
- Vim keybindings with extra shortcuts
- Local storage method
- External repo syncing

**Future Goals**
- Thin client — edit files remotely
- Support for images
- Sharing files with others — readonly web view
- Multiline editing
