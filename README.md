## nrmrs - npm registry manager with Rust 🦀

## Install

```bash
npm install -g nrmrs
```

`npm` installs a prebuilt Rust binary for your platform.

Check installation:

```bash
nrmrs --help
```

## Usage

List registries (`*` marks current):

```bash
nrmrs ls
```

Use a registry:

```bash
nrmrs use --name npm
nrmrs use -n taobao
```

Add a registry:

```bash
nrmrs add --name company --url https://registry.example.com/
nrmrs add -n company -u https://registry.example.com/
```

Delete a registry:

```bash
nrmrs delete --name company
nrmrs delete -n company
```

`delete` cannot remove the currently active registry.

## Built-in registries

On first run, these registries are inserted automatically:

- `npm` -> `https://registry.npmjs.org/`
- `yarn` -> `https://registry.yarnpkg.com/`
- `taobao` -> `https://registry.npmmirror.com/`
- `tencent` -> `https://mirrors.tencent.com/npm/`
- `cnpm` -> `https://r.cnpmjs.org/`
- `huawei` -> `https://repo.huaweicloud.com/repository/npm/`

## Data file

`nrmrs` stores registry data in `npm-registry.db`:

- Windows: `%APPDATA%\nrmrs\npm-registry.db`
- macOS: `~/Library/Application Support/nrmrs/npm-registry.db`
- Linux: `~/.local/share/nrmrs/npm-registry.db`
