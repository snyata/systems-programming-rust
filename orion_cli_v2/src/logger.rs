use log::{Level, LevelFilter, Metadata, Record};
use colored::*;
use env_logger::Builder;
use std::env;

pub struct ColoredLogger;

impl log::Log for ColoredLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => {
                    println!("{} - {}", "ERROR".red().bold(), record.args());
                }
                Level::Warn => {
                    println!("{} - {}", "WARN".yellow().bold(), record.args());
                }
                Level::Info => {
                    println!("{} - {}", "INFO".green().bold(), record.args());
                }
                Level::Debug => {
                    println!("{} - {}", "DEBUG".blue().bold(), record.args());
                }
                Level::Trace => {
                    println!("{} - {}", "TRACE".magenta().bold(), record.args());
                }
            }
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    let mut builder = Builder::from_default_env();
    builder.filter(None, LevelFilter::Info).init();

    log::set_boxed_logger(Box::new(ColoredLogger)).unwrap();
    log::set_max_level(LevelFilter::Trace);
}