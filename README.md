# tem (Tagged Editor for Markdown)

I wanted to build my own system for markdown text editing. I didn't want to use Electron due to performance issues, so I opted for a [Tauri](https://tauri.app/) application for a balance between ease of use and low-level performance.

Although there are solutions out there (Google Drive, Obsidian, Vi), none of them were perfectly suited to my use case (Obsidian came close but it isn't open source sad face).

## Setup and Execution

Clone the repo, cd into the project, run `npm install`, then cd into `src-tauri/`, then run `cargo tauri dev`.

### Recommended Setup for VSCode (just extensions)

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Details

### Tech used

- Rust
- Tauri
- Vite
- Sveltekit

### Current Features
- A window opens!

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
