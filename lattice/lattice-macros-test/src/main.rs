use lattice_macros::LatticePointDerive; // Changed to derive macro import
use lattice_types::{LatticePoint, LatticePointKind};
use chrono::Utc;
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use std::env;
use std::time::Instant; // Import Instant for timing
mod lattice_logger;
use tracing::{info, warn, error, span, Level};
use construction;
use construction::lattice::get_global_lattice; // Import the function to get the global lattice

#[derive(LatticePointDerive)] // Apply the derive macro
struct MyTestStruct {
    field1: u32,
    field2: String,
}

#[derive(LatticePointDerive)]
enum MyTestEnum {
    Variant1,
    Variant2(u32),
    Variant3 { name: String, value: bool },
}

#[derive(LatticePointDerive)]
struct MyUnitTestStruct;

fn main() {
    let start_time = Instant::now(); // Record start time

    // --- Actual Execution Lattice Point --- 
    let mut metadata = HashMap::new();
    metadata.insert("timestamp".to_string(), Utc::now().to_rfc3339());
    metadata.insert("command_args".to_string(), env::args().collect::<Vec<String>>().join(" "));
    metadata.insert("current_dir".to_string(), env::current_dir().unwrap().to_string_lossy().to_string());

    let actual_execution_point = LatticePoint {
        id: format!("actual_execution_{}", Utc::now().timestamp_nanos_opt().unwrap_or_default()),
        kind: LatticePointKind::ActualExecution,
        metadata,
        relationships: vec!["predicted_lattice_macros_test_execution".to_string()], // Relate to the predicted point
        hero_status: None,
    };

    // Define the output directory for lattice events
    let project_root = env::current_dir().unwrap(); // Assuming current_dir is project root for simplicity
    let lattice_events_dir = project_root.join(".gemini").join("lattice_events");
    fs::create_dir_all(&lattice_events_dir).expect("Failed to create lattice_events directory");

    let file_name = format!("actual_execution_{}.json", actual_execution_point.id);
    let file_path = lattice_events_dir.join(file_name);

    println!("DEBUG: lattice_events_dir = {}", lattice_events_dir.display());
    println!("DEBUG: file_path = {}", file_path.display());

    let json_output = serde_json::to_string_pretty(&actual_execution_point).expect("Failed to serialize LatticePoint");

    let mut file = fs::File::create(&file_path).expect("Failed to create actual execution log file");
    file.write_all(json_output.as_bytes()).expect("Failed to write actual execution log");

    println!("Actual execution lattice point logged to: {}", file_path.display());

    lattice_logger::init_logger(Some(actual_execution_point.id.clone()));
    info!("Lattice macros test application started.");

    let _my_struct = MyTestStruct {
        field1: 42,
        field2: "hello".to_string(),
    };
    info!("MyTestStruct created with field1: {} and field2: {}.", _my_struct.field1, _my_struct.field2);
    println!("Macro applied to MyTestStruct. Check build output for generated code details.");

    warn!("This is a warning message from the test application.");
    error!("This is an error message from the test application.");

    let my_span = span!(Level::INFO, "my_test_span", custom_field = "custom_value");
    let _enter = my_span.enter();
    info!("Inside my_test_span.");

    let another_span = span!(Level::DEBUG, "another_test_span");
    let _enter2 = another_span.enter();
    info!("Inside another_test_span.");

    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(100));

    info!("Application finished.");

    // Call the poem conversion function
    construction::run_poem_conversion();

    // --- Debug: Print all LatticePoints in Global Lattice ---
    let global_lattice = get_global_lattice();
    println!("\n--- All LatticePoints in Global Lattice ---");
    for (id, point) in &global_lattice.points {
        println!("ID: {}, Kind: {:?}", id, point.kind);
    }
    println!("-------------------------------------------\n");

    // --- Log ZosPoemElement LatticePoints ---
    let global_lattice = get_global_lattice();
    for (id, point) in &global_lattice.points {
        if point.kind == LatticePointKind::ZosPoemElement {
            let zos_poem_file_name = format!("zos_poem_element_{}.json", id);
            let zos_poem_file_path = lattice_events_dir.join(zos_poem_file_name);
            let zos_poem_json_output = serde_json::to_string_pretty(point).expect("Failed to serialize ZosPoemElement LatticePoint");
            let mut zos_poem_file = fs::File::create(&zos_poem_file_path).expect("Failed to create ZosPoemElement log file");
            zos_poem_file.write_all(zos_poem_json_output.as_bytes()).expect("Failed to write ZosPoemElement log");
            println!("ZosPoemElement lattice point logged to: {}", zos_poem_file_path.display());
        }
    }

    let end_time = Instant::now(); // Record end time
    let duration = end_time.duration_since(start_time); // Calculate duration

    // Create ExecutionTrace LatticePoint
    let mut execution_trace_metadata = HashMap::new();
    execution_trace_metadata.insert("duration_ms".to_string(), duration.as_millis().to_string());
    execution_trace_metadata.insert("command".to_string(), env::args().collect::<Vec<String>>().join(" "));
    execution_trace_metadata.insert("status".to_string(), "completed".to_string()); // Assuming completion for now

    let execution_trace_point = LatticePoint {
        id: format!("execution_trace_{}", Utc::now().timestamp_nanos_opt().unwrap_or_default()),
        kind: LatticePointKind::ExecutionTrace,
        metadata: execution_trace_metadata,
        relationships: vec![actual_execution_point.id.clone()], // Relate to the actual execution point
        hero_status: None,
    };

    // Log ExecutionTrace LatticePoint to file
    let execution_trace_file_name = format!("execution_trace_{}.json", execution_trace_point.id);
    let execution_trace_file_path = lattice_events_dir.join(execution_trace_file_name);
    let execution_trace_json_output = serde_json::to_string_pretty(&execution_trace_point).expect("Failed to serialize ExecutionTrace LatticePoint");
    let mut execution_trace_file = fs::File::create(&execution_trace_file_path).expect("Failed to create ExecutionTrace log file");
    execution_trace_file.write_all(execution_trace_json_output.as_bytes()).expect("Failed to write ExecutionTrace log");
    println!("ExecutionTrace lattice point logged to: {}", execution_trace_file_path.display());
}