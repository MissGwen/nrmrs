## nrmrs - npm registry manager with Rust 🦀

nrmrs is an npm registry manager written in Rust:
- Stores the registry list in SQLite (on first run it creates `nrmrs/npm-registry.db` under your user data directory)
- Switches the active registry via npm (equivalent to `npm config set registry <url>`)

### Requirements
- Node.js / npm (must be able to run `npm` from your shell; on Windows it is usually `npm.cmd`)
- Rust (required when running from source or building)

### Installation

#### Run from source

```bash
cargo run -- --help
```

#### Build a binary

```bash
cargo build --release
```

- Windows: `target/release/nrmrs.exe`
- macOS/Linux: `target/release/nrmrs`

#### Install to cargo bin

```bash
cargo install --path .
```

### Maintainer Release
Create and push a version tag (for example `v0.1.0`) to trigger GitHub Actions release:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The workflow publishes binary assets to GitHub Releases for:
- Windows x64 (`x86_64-pc-windows-msvc`, `.zip`)
- macOS arm64 (`aarch64-apple-darwin`, `.tar.gz`)
- Linux x64 (`x86_64-unknown-linux-gnu`, `.tar.gz`)

Asset naming format:
- `nrmrs-{version}-{target}.{zip|tar.gz}`
- `nrmrs-{version}-{target}.{zip|tar.gz}.sha256`

### Commands

Show help:

```bash
nrmrs --help
nrmrs <command> --help
```

#### ls
List all registries, marking the currently used one with `*`:

```bash
nrmrs ls
```

#### use
Switch the current registry (`name` must already exist in the list):

```bash
nrmrs use --name taobao
nrmrs use -n npm
```

#### add
Add a new registry:

```bash
nrmrs add --name company --url https://registry.example.com/
nrmrs add -n company -u https://registry.example.com/
```

#### delete
Delete a registry (you cannot delete the currently used registry):

```bash
nrmrs delete --name company
nrmrs delete -n company
```

### Default registries
On first run, these registries are inserted automatically:
- npm -> https://registry.npmjs.org/
- yarn -> https://registry.yarnpkg.com/
- taobao -> https://registry.npmmirror.com/

### Data file
- `npm-registry.db`: SQLite database under the `nrmrs` subdirectory in your OS user data directory
  - Windows: `%APPDATA%\nrmrs\npm-registry.db` (exact path depends on the resolved system data directory)
  - macOS: `~/Library/Application Support/nrmrs/npm-registry.db`
  - Linux: `~/.local/share/nrmrs/npm-registry.db`
- To reset, simply delete this file
