
<span style="font-size: 2.25rem; font-weight: bold;">
    nvim-utils:
</span>
<span style="font-size: 1.6rem">Utilities for writing neovim plugins in rust</span>

Interacting with Neovim's lua api in Rust can be a bit of a pain. `nvim-utils` aims to make it easier and quicker to develop Neovim plugins in Rust by removing as much of the required boilerplate as possible, and implementing bindings for the `vim.*` lua api.

Using `nvim-utils` is as simple as adding it to your `Cargo.toml`:

```toml
[dependencies]
nvim-utils = "0.1.0"
```

## Features

`nvim-utils` provides utilities for:

- Declaratively building lua modules using `mlua`
- Interacting with Neovim's lua api
- Logging using `vim.notify`
- Accessing common lua builtin functions like `require` and `print`
- And more to come!

## Documentation

Open [docs.rs/nvim-utils](https://docs.rs/nvim-utils) for the full documentation.
