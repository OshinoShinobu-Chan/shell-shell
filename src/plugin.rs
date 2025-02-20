//! # Plugin Module
//! This module defines the plugin, and its operations. The plugin is a core part of the shell-shell.
//! A **plugin** is a excutable file or a script that can extend the functionality of the shell under the
//! framwork of the shell-shell.
//!
//! ## Basic Description
//! Plugins extend the shell by modifying the input of the user and the output of the shell.
//! More specifically, plugins can read the input of the user, modify it and print something
//! to the console, and then send the modified input to the shell. This creates an illusion for
//! the user that their input has printed some text on the console and the shell has responded
//! in a certain way, even though in reality, such a phenomenon would not occur with their
//! input in the original shell. Similar goals can be achieved by modifying the output of the
//! shell. The plugin that modifies the input is called a **pre-run-plugin**, and the plugin
//! that modifies the output is called a **post-run-plugin**.
//!
//! For example, if you want to acheive the grammar hightlighting on the bash, you can catch
//! output of the bash, and then modify it with the color codes.
//!
//! ## Working Details
//! The **pre-run-plugin** is working in the following way:
//! 1. The plugin is invoked by the shell-shell at a some point, and the plugin is supposed to
//! wait for the input from the shell-shell.
//! 2. All plugins work like a pipeline. When user types something, the shell-shell reads the
//! input, and then sends it to the first plugin.
//! 3. The plugin reads the input, modifies it (and may print something at the same time),
//! and then sends it to the next plugin. After all plugins are done, the output should be a
//! command that can be executed by the shell.

/// PluginMode is the running mode of the plugin
/// - `Always`: The plugin is invoke every time shell-shell is started (default)
/// - `Invoke`: The plugin is only invoke when the plugin is called
pub enum PluginInvokeMode {
    Always,
    Invoke,
}

/// PluginErrorMode is the error handling mode of the plugin
/// - `Stop`: Stop the shell-shell
/// - `ImmediateRestart`: Restart the plugin immediately, and retry `isize` times.
///     if retry times is exhausted, stop the shell-shell. If `isize` is `-1`,
///     retry until success
/// - `DelayedRestart`: Restart the plugin the next time plugin is called
pub enum PluginErrorMode {
    Stop,
    ImmdiateRestart(isize),
    DelayedRestart,
}

/// PluginConfig is the configuration of the plugin
pub struct PluginConfig {
    pub invoke_mode: PluginInvokeMode,
    pub error_mode: PluginErrorMode,
}

/// Plugin is the struct that represents the plugin
pub struct Plugin {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub path: String,
    pub config: PluginConfig,
    pub log_path: Option<String>,
}
