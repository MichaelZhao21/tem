# tem (Tagged Editor for Markdown)

> [!IMPORTANT]  
> This project is now discontinued. I originally wanted to make my own custom solution, but I have realized that using Obsidian + Git + a bunch of plugins can suffice for what I need. Will keep this repo here as I do have the custom version control system built, but this project will not be developed any longer.

I wanted to build my own system for markdown text editing. I didn't want to use Electron due to performance issues, so I opted for a [Tauri](https://tauri.app/) application for a balance between ease of use and low-level performance.

Although there are solutions out there (Google Drive, Obsidian, Vi), none of them were perfectly suited to my use case (Obsidian came close but it isn't open source sad face).

## Setup and Execution

Clone the repo, cd into the project, run `npm install`, then cd into `src-tauri/`, then run `cargo tauri dev`.

### Cargo tests

To run tests, use the `cargo test` command. This will run all tests. To run specific tests, you can run `cargo test <name>`, where it will match any tests that contain any part of name. (eg. a name of `tokenizer::` will run `temsync::tokenizer::tests::test_tokenizer`).

### Recommended Setup for VSCode (just extensions)

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Details

### Tech used

- Rust
- Tauri
- Vite
- Sveltekit

### Server synchronization

Diffing can be... tricky. I thought of using git to manage the file synchronization, but it feels quite overkill. Instead, I have opted to write my own system! This will be optimized for markdown text, so it _should_ be better.

To start, we need to figure out how to create diffs/patch files from the changed content. Git uses a modified version of the [Meyer's difference algorithm](https://www.nathaniel.ai/myers-diff/). We can implement this algorithm to create diffs. Before we do that, however, we need to figure out what is considered a singular "diffable unit". For Git, this is an **entire line**. Although line-wise diffing would be easier to implement, we want something a little more granular. The justification for this is because we are creating a text editor, and so we want to prioritize word-based changes. Line-based changes would also be bad because we are writing **long** lines (no newlines in paragraphs). This would be really annoying to read through edits because a one-word change would result in the entire paragraph being treated as modified.

### Current Features

- A window opens!
- temsync
    - Parsing tokens from file (word-based)
    - Create diff of 2 sets of tokens using Meyer's Diff Algo

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
