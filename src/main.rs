// This is the main entry point of the program.
// It defines two functions, `simulate_world` and `debug_simulate_world`.
// The first function runs the simulation in an infinite loop,
// while the second function runs the simulation a fixed number of times
// and saves the results to a file.
//
// The main function then calls either `simulate_world` or `debug_simulate_world`
// depending on the value of the `global_debug_mode` variable.
mod data {
    pub mod global_vars;
}

mod functions {
    pub mod file;
    pub mod calculations;
}

use data::global_vars::{ * };
fn main() {
    // Whether to run the simulation in debug mode or not.
    // If this is true, the simulation will run a fixed number of times
    // and save the results to a file.
    let global_debug_mode = true;

    // Run the simulation in an infinite loop.
    fn simulate_world() {
        loop {
            // Update the global variables.
            update_global_vars();

            // Get a copy of the current global variables.
            let global_vars = get_global_vars();

            // Print out the current state of the simulation.
            println!(
                "Tick: {}, Hours: {}, Minutes: {}, Population: {}, Resources: {}, Day: {}, Growth Rate: {}%",
                global_vars.tick,
                global_vars.hours,
                global_vars.minutes,
                global_vars.population,
                global_vars.resources,
                global_vars.day,
                global_vars.growth_rate
            );

            // If we're in debug mode, sleep for a second and print out a debug message.
            if global_vars.sim_debug_mode {
                std::thread::sleep(std::time::Duration::from_millis(500));
                println!("[DEBUG]");
            } else {
                // Otherwise, sleep for half of the tick modifier time.
                std::thread::sleep(
                    std::time::Duration::from_millis(global_vars.tick_modifier_ms / 2)
                );
            }
        }
    }

    // Run the simulation a fixed number of times and save the results to a file.
    fn debug_simulate_world() {
        // Create a vector to store the results of the simulation.
        let mut results = Vec::new();
        let ticks = 96;
        let day_modifier = 7;
        let total = ticks * day_modifier;
        // Run the simulation a fixed number of times. (24 hours * 4 ticks / 15 minutes) = 96 total ticks.
        for _ in 0..total {
            // Update the global variables.
            update_global_vars();

            // Get a copy of the current global variables.
            let global_vars = get_global_vars();

            let obj =
                serde_json::json!({
                "tick": global_vars.tick,
                "hours": global_vars.hours,
                "minutes": global_vars.minutes,
                "population": global_vars.population,
                "resources": global_vars.resources,
                "day": global_vars.day,
                "growth_rate": format!("{:.2}", global_vars.growth_rate)
            });
            // Store the current global variables in the results vector.
            results.push(obj);
        }

        // Get the number of files in the `sets` directory.
        let count = functions::file::file_count("./src/data/sets");

        // Create a file name for the results file.
        let file_name = format!("./src/data/sets/set_{}.json", count);

        // Convert the results vector to a JSON string.
        let contents = serde_json::to_string_pretty(&results).unwrap();

        // Write the JSON string to the file.
        functions::file::write_file(&file_name, &contents);
    }

    fn display_averages() {
        // Get the number of files in the `sets` directory.
        let count = functions::file::file_count("./src/data/sets");
        println!("Iteration: {}", count);

        // Create a file name for the results file.
        let file_name = format!("./src/data/sets/set_{}.json", count - 1);

        println!("File Name: {}", file_name);

        let contents = functions::file::target_file(file_name.as_str());

        let res_average = functions::calculations::gather_average(
            contents.clone(),
            "resources".to_string()
        );

        let pop_average = functions::calculations::gather_average(
            contents.clone(),
            "population".to_string()
        );

        println!("Average Pop: {}", pop_average);
        println!("Average Resources: {}", res_average);
        let pop_start = 100;
        println!("Growth Rate: {} %", functions::calculations::growth_rate(pop_start, pop_average));
    }

    // Call either `simulate_world` or `debug_simulate_world` depending on the value of `global_debug_mode`.
    if global_debug_mode {
        debug_simulate_world();
        std::thread::sleep(std::time::Duration::from_millis(100));
        display_averages();
    } else {
        simulate_world();
    }
}
