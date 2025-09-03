use clap::Parser;
use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write}; // For flushing stdout

// Custom output functions
fn print_section_header(title: &str) {
    println!("\n=== {} ===", title);
}

fn print_subsection_header(title: &str) {
    println!("\n--- {} ---", title);
}

fn print_message(message: &str) {
    println!("{}", message);
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of steps to simulate
    #[arg(long, default_value_t = 10)]
    step_limit: u64,
}

fn main() {
    print_section_header("Ultimate Blinkenlights Simulation - Scene 1: 1-Bit Blinkenlight");

    let args = Args::parse();
    let max_steps = args.step_limit;

    let blink_char = '#'; // The oversized pixel/LED
    let blink_frequency_hz = 1.0; // 1 Hz blink
    let blink_duration_ms = (1000.0 / blink_frequency_hz / 2.0) as u64; // Half on, half off

    print_message("Press Ctrl+C to exit.");

    let start_time = Instant::now();
    print_message(&format!("Simulation started at: {:?}", start_time));

    let mut step_count = 0;

    loop {
        step_count += 1;
        let current_time = Instant::now();
        print_message(&format!("[{:?}] Step {}: Blinkenlight ON", current_time - start_time, step_count));
        // Turn on
        print!("\r{}", blink_char); // \r returns cursor to start of line
        io::stdout().flush().unwrap(); // Ensure it's printed immediately
        thread::sleep(Duration::from_millis(blink_duration_ms));

        if step_count >= max_steps {
            break;
        }

        step_count += 1;
        let current_time = Instant::now();
        print_message(&format!("[{:?}] Step {}: Blinkenlight OFF", current_time - start_time, step_count));
        // Turn off (clear character)
        print!("\r "); // Overwrite with space
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(blink_duration_ms));

        if step_count >= max_steps {
            break;
        }
    }
    let end_time = Instant::now();
    print_section_header(&format!("Simulation stopped after {} steps. Total duration: {:?}", max_steps, end_time - start_time));
}
