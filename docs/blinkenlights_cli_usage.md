# Ultimate Blinkenlights Simulation CLI Usage

This document describes how to run and interact with the `ultimate_blinkenlights_simulation` program using its command-line interface.

## Running the Simulation

To run the simulation, navigate to the project root directory and use the `cargo run` command, specifying the `ultimate_blinkenlights_simulation` package:

```bash
cargo run --package ultimate_blinkenlights_simulation
```

## Controlling Simulation Steps with `--step-limit`

The simulation now accepts a `--step-limit` argument to control the number of steps it will execute before automatically stopping. This is useful for testing, debugging, or running the simulation for a fixed duration.

### Usage

To specify a step limit, use the `--step-limit` flag followed by the desired number of steps:

```bash
cargo run --package ultimate_blinkenlights_simulation -- --step-limit=<NUMBER_OF_STEPS>
```

Replace `<NUMBER_OF_STEPS>` with an integer value. For example, to run the simulation for 5 steps:

```bash
cargo run --package ultimate_blinkenlights_simulation -- --step-limit=5
```

### Default Behavior

If the `--step-limit` argument is not provided, the simulation will default to running for 10 steps.

## Example Output

When running with a `step-limit` of 2, the output will be similar to this:

```
=== Ultimate Blinkenlights Simulation - Scene 1: 1-Bit Blinkenlight ===
Step 1: Blinkenlight ON
#Step 2: Blinkenlight OFF
 
=== Simulation stopped after 2 steps. ===
```

This verbose output shows the current step and the state of the blinkenlight (ON/OFF) for each step.