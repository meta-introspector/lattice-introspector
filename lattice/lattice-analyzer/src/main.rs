use lattice_types::{LatticePoint, LatticePointKind};
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::env;
use chrono::{Utc, DateTime, Local};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Lattice Analyzer started.");

    // 1. Read the PredictedExecution point
    // This is tricky as it's generated code. For now, we'll assume its ID and structure.
    // In a real scenario, you might parse the generated_lattice_registration.rs or have a way to query the lattice.
    let predicted_point_id = "predicted_lattice_macros_test_execution";
    println!("Attempting to retrieve predicted execution point with ID: {}", predicted_point_id);

    // For demonstration, let's manually construct a predicted point that matches what build.rs generates
    let mut predicted_metadata = HashMap::new();
    predicted_metadata.insert("binary_path".to_string(), "/data/data/com.termux/files/home/storage/github/rustc/target/debug/lattice-macros-test".to_string()); // Hardcoded for now
    predicted_metadata.insert("expected_args".to_string(), "".to_string());
    predicted_metadata.insert("prediction_timestamp".to_string(), "".to_string()); // Will be filled by build.rs

    let predicted_point = LatticePoint {
        id: predicted_point_id.to_string(),
        kind: LatticePointKind::PredictedExecution,
        metadata: predicted_metadata,
        relationships: vec!["binary_binary_lattice_macros_test".to_string()], // Relate to the binary point
    };

    // 2. Read the ActualExecution point from the latest JSON file
    let lattice_events_dir = env::current_dir().unwrap().join(".gemini").join("lattice_events");
    println!("DEBUG: lattice_events_dir path: {}", lattice_events_dir.display());
    println!("DEBUG: lattice_events_dir exists: {}", lattice_events_dir.exists());
    println!("DEBUG: lattice_events_dir is_dir: {}", lattice_events_dir.is_dir());

    let mut found_file: Option<PathBuf> = None;

    if lattice_events_dir.exists() && lattice_events_dir.is_dir() {
        println!("DEBUG: Scanning directory: {}", lattice_events_dir.display());
        for entry in fs::read_dir(&lattice_events_dir)? {
            let entry = entry?;
            let path = entry.path();
            println!("DEBUG: Found file: {}", path.display());
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") && path.file_name().map_or(false, |name| name.to_string_lossy().starts_with("actual_execution_")) {
                println!("DEBUG: File matches pattern: {}", path.display());
                found_file = Some(path);
                break; // Just take the first one for now
            }
        }
    }

    let actual_point: LatticePoint;
    let mut log_events: Vec<LatticePoint> = Vec::new();

    if let Some(file_path) = found_file {
        println!("Found actual execution log: {}", file_path.display());
        let file_content = fs::read_to_string(&file_path)?;
        println!("DEBUG: File content read. Attempting to deserialize.");
        actual_point = serde_json::from_str(&file_content)?;
        println!("DEBUG: Deserialization successful.");
    } else {
        // If no actual execution log found, try to find log events
        let mut found_log_events = false;
        if lattice_events_dir.exists() && lattice_events_dir.is_dir() {
            for entry in fs::read_dir(&lattice_events_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") && path.file_name().map_or(false, |name| name.to_string_lossy().starts_with("log_event_")) {
                    let file_content = fs::read_to_string(&path)?;
                    let log_point: LatticePoint = serde_json::from_str(&file_content)?;
                    if log_point.kind == LatticePointKind::LogEvent {
                        log_events.push(log_point);
                        found_log_events = true;
                    }
                }
            }
        }
        if !found_log_events {
            return Err("No actual execution log or log events found.".into());
        }
        // If only log events are found, we can still proceed with displaying them
        // but the distance calculation will be skipped.
        println!("No actual execution log found, but found {} log events.", log_events.len());
        // Create a dummy actual_point to avoid compilation errors for now
        actual_point = LatticePoint {
            id: "dummy_actual_execution".to_string(),
            kind: LatticePointKind::ActualExecution,
            metadata: HashMap::new(),
            relationships: Vec::new(),
        };
    }

    // Display Log Events
    if !log_events.is_empty() {
        println!("\n--- Log Events ---");
        for event in log_events {
            println!("  Level: {}", event.metadata.get("level").unwrap_or(&"N/A".to_string()));
            println!("  Message: {}", event.metadata.get("message").unwrap_or(&"N/A".to_string()));
            if let Some(file) = event.metadata.get("file") {
                if let Some(line) = event.metadata.get("line") {
                    println!("  Source: {}:{}", file, line);
                }
            }
            println!("  Timestamp: {}", event.metadata.get("timestamp").unwrap_or(&"N/A".to_string()));
            println!("--------------------");
        }
    }

    // 3. Calculate distance (only if actual_point is not dummy) 
    if actual_point.id != "dummy_actual_execution" {
        println!("\n--- Comparing Predicted and Actual Execution ---");
        println!("Predicted ID: {}", predicted_point.id);
        println!("Actual ID:    {}", actual_point.id);

        let mut distance = 0.0;

        // Compare binary paths
        let predicted_binary_path = predicted_point.metadata.get("binary_path").unwrap_or(&"N/A".to_string()).clone();
        let actual_binary_path = actual_point.metadata.get("command_args").unwrap_or(&"N/A".to_string()).split_whitespace().next().unwrap_or("N/A").to_string();

        if predicted_binary_path != actual_binary_path {
            println!("  Binary Path Mismatch: Predicted='{}', Actual='{}'", predicted_binary_path, actual_binary_path);
            distance += 1.0;
        } else {
            println!("  Binary Paths Match: '{}'", predicted_binary_path);
        }

        // Compare expected arguments (simple check for now)
        let predicted_args = predicted_point.metadata.get("expected_args").unwrap_or(&"N/A".to_string()).clone();
        let actual_args = actual_point.metadata.get("command_args").unwrap_or(&"N/A".to_string()).clone();

        // A very simplistic argument comparison: just check if the predicted args are a substring of actual args
        if !predicted_args.is_empty() && !actual_args.contains(&predicted_args) {
            println!("  Arguments Mismatch: Predicted='{}', Actual='{}'", predicted_args, actual_args);
            distance += 0.5; // Smaller penalty for args mismatch
        } else if predicted_args.is_empty() {
            println!("  No specific predicted arguments.");
        } else {
            println!("  Arguments Match (or predicted is substring): Predicted='{}', Actual='{}'", predicted_args, actual_args);
        }

        // Compare kinds
        if predicted_point.kind != LatticePointKind::PredictedExecution || actual_point.kind != LatticePointKind::ActualExecution {
            println!("  Kind Mismatch: Predicted='{:?}', Actual='{:?}'", predicted_point.kind, actual_point.kind);
            distance += 2.0; // High penalty for kind mismatch
        } else {
            println!("  Kinds are as expected: PredictedExecution vs ActualExecution");
        }

        println!("\nCalculated Distance (e): {}", distance);

        // Define a threshold for 'e'
        let threshold_e = 0.1; // For example, a very small threshold for exact match

        if distance <= threshold_e {
            println!("The predicted and actual execution points are within distance e ({} <= {}).", distance, threshold_e);
        } else {
            println!("The predicted and actual execution points are NOT within distance e ({} > {}).", distance, threshold_e);
        }
    } else {
        println!("Skipping distance calculation as no actual execution log was found.");
    }

    Ok(())
}
