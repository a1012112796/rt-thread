/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-20     foxglove     micro rust log component
 */
/* Basic logging primitives and console output helpers */
use rt_rust::print;
use rt_rust::time;

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Level {
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

fn level_tag(level: Level) -> &'static str {
    match level {
        Level::Error => "ERROR",
        Level::Warn => "WARN",
        Level::Info => "INFO",
        Level::Debug => "DEBUG",
        Level::Trace => "TRACE",
    }
}

fn level_color(level: Level) -> &'static str {
    match level {
        Level::Error => "\x1b[1;31m",
        Level::Warn => "\x1b[1;33m",
        Level::Info => "\x1b[1;32m",
        Level::Debug => "\x1b[1;34m",
        Level::Trace => "\x1b[1;30m",
    }
}

fn u32_to_str(mut value: u32, buf: &mut [u8; 10]) -> &str {
    let mut index = buf.len();

    if value == 0 {
        index -= 1;
        buf[index] = b'0';
    }
    else {
        while value != 0 {
            index -= 1;
            buf[index] = b'0' + (value % 10) as u8;
            value /= 10;
        }
    }

    unsafe { core::str::from_utf8_unchecked(&buf[index..]) }
}

fn millis_to_str(value: u32, buf: &mut [u8; 3]) -> &str {
    buf[0] = b'0' + ((value / 100) % 10) as u8;
    buf[1] = b'0' + ((value / 10) % 10) as u8;
    buf[2] = b'0' + (value % 10) as u8;
    unsafe { core::str::from_utf8_unchecked(&buf[..]) }
}

fn print_prefix(level: Level, secs: u32, millis: u32) {
    let mut secs_buf = [0u8; 10];
    let mut millis_buf = [0u8; 3];

    print!("{}[{}][", level_color(level), level_tag(level));
    print!("{}", u32_to_str(secs, &mut secs_buf));
    print!(".");
    print!("{}", millis_to_str(millis, &mut millis_buf));
    print!("] ");
}

/* Top-level logging macro */
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => ({
        $crate::logging::_log($level, format_args!($($arg)*));
    });
}

/* Error-level logging */
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::logging::_log($crate::logging::Level::Error, format_args!($($arg)*));
    });
}

/* Warning-level logging */
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::logging::_log($crate::logging::Level::Warn, format_args!($($arg)*));
    });
}

/* Info-level logging */
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        $crate::logging::_log($crate::logging::Level::Info, format_args!($($arg)*));
    });
}

/* Debug-level logging */
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::logging::_log($crate::logging::Level::Debug, format_args!($($arg)*));
    });
}

/* Trace-level logging */
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => ({
        $crate::logging::_log($crate::logging::Level::Trace, format_args!($($arg)*));
    });
}

/* Internal logging handler: format directly without heap allocation */
pub fn _log(level: Level, args: core::fmt::Arguments) {
    let now = time::get_time();
    let secs = now.as_secs() as u32;
    let millis = now.subsec_millis();

    print_prefix(level, secs, millis);

    print!("{}", args);
    print!("\x1b[0m\n");
}
