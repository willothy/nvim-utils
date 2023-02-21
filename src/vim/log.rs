//! Corresponds to `vim.log`

/// The log level of a message.
/// Corresponds to `vim.log.levels`
#[repr(u8)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Off = 5,
}