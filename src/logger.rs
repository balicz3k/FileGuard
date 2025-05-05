pub(crate) enum LogLevel {
    ERR,
    WRN,
    INF,
    DBG
}

pub(crate) struct Logger;

impl Logger {
    pub(crate) fn log(level: LogLevel, args: std::fmt::Arguments<'_>) {
        let timestamp = chrono::Local::now().format("%H:%M:%S");
        let level_prefix = match level {
            LogLevel::ERR => "\x1b[31m[ERR]\x1b[0m",    /*Red prefix color*/
            LogLevel::WRN => "\x1b[33m[WRN]\x1b[0m",    /*Yellow prefix color*/
            LogLevel::INF => "\x1b[34m[INF]\x1b[0m",    /*Blue prefix color*/
            LogLevel::DBG => "\x1b[35m[DBG]\x1b[0m"     /*Purple prefix color*/
        };
        println!("{} {} {}", timestamp, level_prefix, args);
    }
}

#[macro_export]
macro_rules! log_dbg {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log(
            $crate::logger::LogLevel::DBG,
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! log_inf {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log(
            $crate::logger::LogLevel::INF,
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! log_wrn {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log(
            $crate::logger::LogLevel::WRN,
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {
        $crate::logger::Logger::log(
            $crate::logger::LogLevel::ERR,
            format_args!($($arg)*)
        )
    };
}
