use log::{Record, Metadata, Log, LevelFilter};
use lattice_types::{LatticePoint, LatticePointKind};
use serde_json;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use std::env;

pub struct LatticeLogger {
    log_dir: PathBuf,
}

impl LatticeLogger {
    pub fn new() -> Self {
        let project_root = env::current_dir().unwrap();
        let log_dir = project_root.join(".gemini").join("lattice_events");
        fs::create_dir_all(&log_dir).expect("Failed to create lattice_events directory for logger");
        LatticeLogger { log_dir }
    }

    fn log_lattice_point(&self, record: &Record) {
        let mut metadata = HashMap::new();
        metadata.insert("timestamp".to_string(), Utc::now().to_rfc3339());
        metadata.insert("level".to_string(), record.level().to_string());
        metadata.insert("target".to_string(), record.target().to_string());
        metadata.insert("message".to_string(), format!("{}", record.args()));
        if let Some(file) = record.file() {
            metadata.insert("file".to_string(), file.to_string());
        }
        if let Some(line) = record.line() {
            metadata.insert("line".to_string(), line.to_string());
        }

        let log_point = LatticePoint {
            id: format!("log_event_{}", Uuid::new_v4()),
            kind: LatticePointKind::LogEvent,
            metadata,
            relationships: Vec::new(), // Relationships can be added later
        };

        let file_name = format!("log_event_{}.json", log_point.id);
        let file_path = self.log_dir.join(file_name);

        let json_output = serde_json::to_string_pretty(&log_point).expect("Failed to serialize LogEvent LatticePoint");

        let mut file = fs::File::create(&file_path).expect("Failed to create log event file");
        file.write_all(json_output.as_bytes()).expect("Failed to write log event");
    }
}

impl Log for LatticeLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LevelFilter::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.log_lattice_point(record);
        }
    }

    fn flush(&self) {}
}

pub fn init_logger() {
    let logger = Box::new(LatticeLogger::new());
    log::set_logger(Box::leak(logger)).expect("Failed to set logger");
    log::set_max_level(LevelFilter::Info);
}
