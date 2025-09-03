use std::thread;
use std::time::Duration;
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

fn main() {
    print_section_header("Ultimate Blinkenlights Simulation - Scene 1: 1-Bit Blinkenlight");

    let blink_char = '#'; // The oversized pixel/LED
    let blink_frequency_hz = 1.0; // 1 Hz blink
    let blink_duration_ms = (1000.0 / blink_frequency_hz / 2.0) as u64; // Half on, half off

    print_message("ACHTUNG! Das Blinkenlight ist born! One bit, pure power!");
    print_message("Press Ctrl+C to exit.");

    // Simulate blinking
    loop {
        // Turn on
        print!("\r{}", blink_char); // \r returns cursor to start of line
        io::stdout().flush().unwrap(); // Ensure it's printed immediately
        thread::sleep(Duration::from_millis(blink_duration_ms));

        // Turn off (clear character)
        print!("\r "); // Overwrite with space
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(blink_duration_ms));
    }
}