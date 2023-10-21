use std::time::{Duration, Instant};

const PROFILER_ENABLED: bool = false;

pub struct Profiler {
    events: Vec<(String, Instant)>,
}

impl Profiler {
    pub fn new() -> Profiler {
        Profiler { events: Vec::new() }
    }

    pub fn start(&mut self, name: &str) {
        if PROFILER_ENABLED {
            let indentation = "  ".repeat(self.events.len());
            println!("{}{} {{", indentation, name);
            self.events.push((name.to_string(), Instant::now()));
        }
    }

    pub fn end(&mut self, name: &str) {
        if PROFILER_ENABLED {
            if let Some(index) = self
                .events
                .iter()
                .rposition(|(event_name, _)| event_name == name)
            {
                let (event_name, start_time) = self.events.remove(index);
                let elapsed = start_time.elapsed();
                let indentation = "  ".repeat(self.events.len());

                println!(
                    "{}}} {} {} nanoseconds;",
                    indentation,
                    event_name,
                    elapsed.as_nanos() as f64
                );
            }
        }
    }

    pub fn finish(&mut self) {
        if PROFILER_ENABLED {
            println!(); // Print a newline
            self.events.clear(); // Clear the vector
        }
    }
}
