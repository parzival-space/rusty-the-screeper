use js_sys::{Error, JsString};
use log::{Level, LevelFilter, Log, Metadata, Record, error};
use screeps::console;
use simplelog::{CombinedLogger, Config, SharedLogger};
use std::fmt::Write;
use std::panic;
use std::panic::PanicHookInfo;

fn get_level_colour(level: Level) -> String {
    match level {
        Level::Error => "#FF0000".to_string(),
        Level::Warn => "#FFFF00".to_string(),
        Level::Info => "#005fff".to_string(),
        Level::Debug => "#00FFFF".to_string(),
        Level::Trace => "#FFFFFF".to_string(),
    }
}

pub struct ConsoleLogger {
    pub level: LevelFilter,
}

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.level <= metadata.level()
    }

    fn log(&self, record: &Record) {
        let mut message_string = String::new();

        // level
        write!(
            &mut message_string,
            "<span style=\"color:{};\">[{}] </span>",
            get_level_colour(record.level()),
            record.level()
        )
        .unwrap_or(());

        // target
        write!(
            &mut message_string,
            "<span>{target:<pad$}: </span>",
            target = record.target(),
            pad = 10
        )
        .unwrap_or(());

        // file: line
        let file = record.file().unwrap_or("<unknown>");
        if let Some(line) = record.line() {
            write!(&mut message_string, "<span>[{}:{}] </span>", file, line).unwrap_or(());
        } else {
            write!(&mut message_string, "<span>[{}] </span>", file).unwrap_or(());
        }

        // module path
        write!(
            &mut message_string,
            "<span>[{}] </span>",
            record.module_path().unwrap_or("<unknown>")
        )
        .unwrap_or(());

        // message
        write!(&mut message_string, "<span>{}</span>", record.args()).unwrap_or(());

        console::log_unsafe(&JsString::from(message_string));
    }

    fn flush(&self) {}
}

impl SharedLogger for ConsoleLogger {
    fn level(&self) -> LevelFilter {
        self.level
    }

    fn config(&self) -> Option<&Config> {
        None
    }

    fn as_log(self: Box<Self>) -> Box<dyn Log> {
        Box::new(*self)
    }
}

impl ConsoleLogger {
    pub fn new(log_level: LevelFilter) -> Box<ConsoleLogger> {
        Box::new(ConsoleLogger { level: log_level })
    }
}

pub fn setup_logging() {
    CombinedLogger::init(vec![ConsoleLogger::new(LevelFilter::Trace)])
        .map_err(|e| {
            console::log_unsafe(&JsString::from(format!(
                "Failed to initialize logger: {}",
                e
            )))
        })
        .unwrap_or(());
    panic::set_hook(Box::new(|info| {
        let mut fmt_error = String::new();
        write!(&mut fmt_error, "{:#?}", info).unwrap_or(());
        error!("{}", fmt_error);
    }))
}
