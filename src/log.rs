use std::io::{stdout, Write};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use chrono::{DateTime, Local};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

pub struct Logger {
    pub level: LogLevel,
}

impl Logger {
    fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if level < self.level {
            return;
        }

        let timestamp: DateTime<Local> = Local::now();
        let formatted_message = format!(
            "[{}] {:?}: {}\n",
            timestamp.format("%H:%M:%S.%6f %d-%m-%Y").to_string(), // Correct format
            level,
            message
        );

        let stdout = stdout();
        let mut handle = stdout.lock(); // Acquire lock here, inside the log function
        handle.write_all(formatted_message.as_bytes()).unwrap();
        handle.flush().unwrap();
        drop(handle); // Important to release the lock
    }
}

pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new(LogLevel::Error)));

pub fn init_logger(level: LogLevel) {
    let mut logger = LOGGER.lock().unwrap();
    logger.level = level;
}

#[macro_export]
macro_rules! log_level {
    ($level:expr, $($arg:tt)*) => {
        {
            let logger = crate::log::LOGGER.lock().unwrap();
            logger.log($level, &format!($($arg)*));
            drop(logger);
        }
    };
}

#[macro_export]
macro_rules! LOG_TRC {
    ($($arg:tt)*) => {
        log_level!(crate::log::LogLevel::Trace, $($arg)*)
    };
}
#[macro_export]
macro_rules! LOG_DBG {
    ($($arg:tt)*) => {
        log_level!(crate::log::LogLevel::Debug, $($arg)*)
    };
}
#[macro_export]
macro_rules! LOG_INF {
    ($($arg:tt)*) => {
        log_level!(crate::log::LogLevel::Info, $($arg)*)
    };
}
#[macro_export]
macro_rules! LOG_WRN {
    ($($arg:tt)*) => {
        log_level!(crate::log::LogLevel::Warning, $($arg)*)
    };
}
#[macro_export]
macro_rules! LOG_ERR {
    ($($arg:tt)*) => {
        log_level!(crate::log::LogLevel::Error, $($arg)*)
    };
}
