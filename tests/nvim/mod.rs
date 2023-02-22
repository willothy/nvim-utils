//! This module contains the tests for interacting with Neovim
//! This should only be run in CI, as it requires a headless Neovim instance

use std::{
    env::var,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use fs_extra::dir::CopyOptions;

static CARGO_TARGET_DIR: &str = "CARGO_TARGET_DIR";

#[test]
#[ignore = "This test requires a headless Neovim instance, and is only intended to be run in CI"]
pub fn test_plugin() -> Result<(), Box<dyn std::error::Error>> {
    let crate_name = "test_plugin".to_owned();
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tmp_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));

    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    let target_dir = var(CARGO_TARGET_DIR)
        .map(|p| PathBuf::from(p))
        .unwrap_or(manifest_dir.join("target"))
        .join(profile)
        .join("deps");

    let in_ext;
    let mut out_ext = None;
    if cfg!(target_os = "windows") {
        in_ext = "dll";
    } else if cfg!(target_os = "macos") {
        in_ext = "dylib";
        out_ext = Some("so");
    } else {
        in_ext = "so";
    };
    let built_name = format!("lib{}.{}", crate_name, in_ext);
    let plugin_name = format!("{}.{}", crate_name, out_ext.unwrap_or(in_ext));

    let built_plugin = target_dir.join(&built_name);

    let lua_dir = tmp_dir.join("lua").join(&crate_name);
    let deps_dir = lua_dir.join("lua").join("deps");

    let plugin_dest = lua_dir.join("lua").join(&plugin_name);

    fs::create_dir_all(&deps_dir)
        .map_err(|e| format!("Could not create {:?}: {}", &deps_dir, e))?;
    fs::copy(&built_plugin, &plugin_dest).map_err(|e| {
        format!(
            "Could not copy {:?} to {:?}: {}",
            &built_plugin, &plugin_dest, e
        )
    })?;

    fs::remove_file(&built_plugin)
        .map_err(|e| format!("Could not remove {:?}: {}", &built_plugin, e))?;

    let copy_opt = CopyOptions::new().copy_inside(true).overwrite(true);
    fs_extra::copy_items(&[&target_dir], &deps_dir, &copy_opt)?;

    let init_lua = tmp_dir.join("init.lua");
    let mut init_file = File::create(&init_lua)?;
    let mut writer = BufWriter::new(&mut init_file);

    let init_text = format!(
        "\
require('{0}')
require('{0}').hello()
local info = require('{0}').get_plugin_info()
print(info.author)
vim.cmd('qa!')
    ",
        &crate_name
    );

    writeln!(writer, "{}", init_text)?;
    writer.flush()?;
    drop(writer);

    let mut nvim = Command::new("nvim");
    let output = nvim
        .arg("--clean")
        .arg("--headless")
        .args([
            "--cmd",
            &format!(
                "let &runtimepath.=','.escape('{}', '\\,')",
                &lua_dir.to_string_lossy().to_string()
            ),
        ])
        .arg("-u")
        .arg(&init_lua)
        //.stdout(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    assert!(
        output.status.success(),
        "Neovim exited with non-zero status"
    );
    assert!(!output.stderr.is_empty(), "Neovim output was empty");
    assert!(output.stdout.is_empty(), "Neovim stdout was not empty");

    // Remove carriage returns from stderr to make it easier to compare
    let stderr = String::from_utf8(output.stderr)?.replace("\r", "");
    assert_eq!(
        stderr, "Hello from Rust and NeoVim!\nExample Author",
        "out:{}",
        stderr
    );

    Ok(())
}
