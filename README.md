# Nvim-Utils: Utilities for writing Neovim plugins in Rust

[![DeepSource](https://deepsource.io/gh/willothy/nvim-utils.svg/?label=active+issues&show_trend=true&token=Z6ZpietODcwGH8IaieqJ7Z60)](https://deepsource.io/gh/willothy/nvim-utils/?ref=repository-badge)
[![DeepSource](https://deepsource.io/gh/willothy/nvim-utils.svg/?label=resolved+issues&show_trend=true&token=Z6ZpietODcwGH8IaieqJ7Z60)](https://deepsource.io/gh/willothy/nvim-utils/?ref=repository-badge)

Interacting with Neovim's lua api in Rust can be a bit of a pain. `nvim-utils` aims to make it easier and quicker to develop Neovim plugins in Rust by removing as much of the required boilerplate as possible, and implementing bindings for the `vim.*` lua api.

## Installation

```sh
cargo add nvim-utils
```

## Features

`nvim-utils` provides utilities for:

- Declaratively building lua modules using `mlua`
- Interacting with Neovim's lua api
- Logging using `vim.notify`
- Accessing common lua builtin functions like `require` and `print`
- And more to come!

## Documentation

Check out the examples for a simple example plugin, or my plugin [moveline](https://github.com/willothy/moveline.nvim) for a slightly more complex one.

Open [docs.rs/nvim-utils](https://docs.rs/nvim-utils) for the full documentation.

## Plugins using nvim-utils

If you build a plugin using `nvim-utils`, submit a PR or let me know and I'll list it here!

- [willothy/moveline.nvim](https://github.com/willothy/moveline.nvim): Easily move lines up and down

