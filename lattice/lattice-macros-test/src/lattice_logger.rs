use log::{Record, Metadata, Log, LevelFilter, set_logger, set_max_level};
use lattice_types::{LatticePoint, LatticePointKind};
use serde_json;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use std::env;

use tracing_subscriber::{self, fmt::MakeWriter, prelude::*, registry::LookupSpan, util::SubscriberInitExt};
use tracing::{Event, Subscriber, Level, span, field::Visit};
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatFields;

pub struct LatticeLogger {
    log_dir: PathBuf,
    actual_execution_id: Option<String>,
}

impl LatticeLogger {
    pub fn new(actual_execution_id: Option<String>) -> Self {
        let project_root = env::current_dir().unwrap();
        let log_dir = project_root.join(".gemini").join("lattice_events");
        fs::create_dir_all(&log_dir).expect("Failed to create lattice_events directory for logger");
        LatticeLogger { log_dir, actual_execution_id }
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
            hero_status: None,
        };

        self.write_lattice_point(log_point, "log_event");
    }

    fn trace_lattice_point(&self, event: &Event) {
        let mut metadata = HashMap::new();
        metadata.insert("timestamp".to_string(), Utc::now().to_rfc3339());
        metadata.insert("level".to_string(), event.metadata().level().to_string());
        metadata.insert("target".to_string(), event.metadata().target().to_string());

        // Extract fields from the event
        let mut visitor = EventFieldVisitor { fields: HashMap::new() };
        event.record(&mut visitor);
        for (key, value) in visitor.fields {
            metadata.insert(key, value);
        }

        if let Some(file) = event.metadata().file() {
            metadata.insert("file".to_string(), file.to_string());
        }
        if let Some(line) = event.metadata().line() {
            metadata.insert("line".to_string(), line.to_string());
        }

        let trace_point = LatticePoint {
            id: format!("trace_event_{}", Uuid::new_v4()),
            kind: LatticePointKind::TraceEvent,
            metadata,
            relationships: Vec::new(),
            hero_status: None,
        };

        self.write_lattice_point(trace_point, "trace_event");
    }

    fn write_lattice_point(&self, point: LatticePoint, prefix: &str) {
        let file_name = format!("{}_{}.json", prefix, point.id);
        let file_path = self.log_dir.join(file_name);

        let json_output = serde_json::to_string_pretty(&point).expect(&format!("Failed to serialize {} LatticePoint", prefix));

        let mut file = fs::File::create(&file_path).expect(&format!("Failed to create {} file", prefix));
        file.write_all(json_output.as_bytes()).expect(&format!("Failed to write {} event", prefix));
    }
}

impl Log for LatticeLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LevelFilter::Info
    }

    fn log(&self, record: &Record) {
        if log::Log::enabled(self, record.metadata()) {
            self.log_lattice_point(record);
        }
    }

    fn flush(&self) {}
}

// Custom visitor to extract fields from tracing events
struct EventFieldVisitor {
    fields: HashMap<String, String>,
}

impl Visit for EventFieldVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.fields.insert(field.name().to_string(), format!("{:?}", value));
    }
}

impl Subscriber for LatticeLogger {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        metadata.level() <= &Level::INFO
    }

    fn new_span(&self, span_attributes: &span::Attributes<'_>) -> span::Id {
        let span_id_uuid = Uuid::new_v4();
        let span_id_u64 = span_id_uuid.as_u64_pair().0; // Use one part of the UUID as u64

        let mut metadata = HashMap::new();
        metadata.insert("timestamp".to_string(), Utc::now().to_rfc3339());
        metadata.insert("name".to_string(), span_attributes.metadata().name().to_string());
        metadata.insert("target".to_string(), span_attributes.metadata().target().to_string());
        metadata.insert("level".to_string(), span_attributes.metadata().level().to_string());

        // Extract fields from span attributes
        let mut visitor = EventFieldVisitor { fields: HashMap::new() };
        span_attributes.record(&mut visitor);
        for (key, value) in visitor.fields {
            metadata.insert(key, value);
        }

        let mut relationships = Vec::new();
        if let Some(actual_exec_id) = &self.actual_execution_id {
            relationships.push(actual_exec_id.clone());
        }

        let span_point = LatticePoint {
            id: format!("span_event_{}", span_id_uuid),
            kind: LatticePointKind::TraceEvent, // Assuming TraceEvent can represent spans
            metadata,
            relationships,
            hero_status: None,
        };

        self.write_lattice_point(span_point, "span_event");

        span::Id::from_u64(span_id_u64)
    }

    fn record(&self, _span: &span::Id, _values: &tracing::span::Record<'_>) {
        // Not currently recording span fields as separate events
    }

    fn record_follows_from(&self, _span: &span::Id, _follows: &span::Id) {
        // Not currently recording follows_from relationships
    }

    fn event(&self, event: &Event<'_>) {
        self.trace_lattice_point(event);
    }

    fn enter(&self, _span: &span::Id) {
        // Not currently recording span enter events
    }

    fn exit(&self, _span: &span::Id) {
        // Not currently recording span exit events
    }
}

pub fn init_logger(actual_execution_id: Option<String>) {
    // Initialize log crate logger
    let logger = Box::new(LatticeLogger::new(actual_execution_id.clone()));
    set_logger(Box::leak(logger)).expect("Failed to set log logger");
    set_max_level(LevelFilter::Info);

    // Initialize tracing subscriber
    let lattice_logger_for_tracing = LatticeLogger::new(actual_execution_id);
    tracing::subscriber::set_global_default(lattice_logger_for_tracing).expect("Failed to set tracing subscriber");
}
