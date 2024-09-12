// src/data/global_vars.rs
use std::sync::{ Arc, Mutex };
use lazy_static::lazy_static;
use rand::Rng;
use serde::{ Deserialize, Serialize };

use crate::functions;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalVars {
    pub sim_debug_mode: bool,
    pub tick_modifier_ms: u64,
    pub tick: u64,
    max_ticks: u64,
    pub hours: u64,
    pub minutes: u64,
    pub population: u128,
    pub resources: u128,
    pub day: u64,
    pub growth_rate: f64,
}

impl GlobalVars {
    pub fn new() -> Self {
        GlobalVars {
            sim_debug_mode: true,
            tick_modifier_ms: 30000,
            max_ticks: 96,
            tick: 0,
            hours: 0,
            minutes: 0,
            population: 100,
            resources: 1000,
            day: 0,
            growth_rate: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.tick += 1;

        if self.tick >= self.max_ticks {
            self.tick = 0;
            self.minutes += 15;

            if self.minutes >= 60 {
                self.minutes = 0;
                self.hours += 1;

                if self.hours >= 24 {
                    self.hours = 0;
                    self.day += 1;
                }
            }
        } else {
            self.minutes += 15;

            if self.minutes >= 60 {
                self.minutes = 0;
                self.hours += 1;

                if self.hours >= 24 {
                    self.hours = 0;
                }
            }
        }

        if self.hours % 2 == 1 && self.minutes == 0 {
            let seed = rand::thread_rng().gen_range(0..1000);
            if seed % 2 == 0 {
                let random_number = rand::thread_rng().gen_range(0..self.population / 3);
                let random_number_2 = rand::thread_rng().gen_range(0..self.resources / 3);
                self.population += random_number;
                self.resources += random_number_2;

                self.growth_rate = functions::calculations::growth_rate(100, self.population);
            } else {
                let random_number = rand::thread_rng().gen_range(0..self.population / 4);
                let random_number_2 = rand::thread_rng().gen_range(0..self.resources / 4);
                self.population = self.population.saturating_sub(random_number);
                self.resources = self.resources.saturating_sub(random_number_2);

                self.growth_rate = functions::calculations::growth_rate(100, self.population);
            }
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_VARS: Arc<Mutex<GlobalVars>> = Arc::new(Mutex::new(GlobalVars::new()));
}

pub fn update_global_vars() {
    let mut global_vars = GLOBAL_VARS.lock().unwrap();
    global_vars.update();
}

pub fn get_global_vars() -> GlobalVars {
    let global_vars = GLOBAL_VARS.lock().unwrap();
    (*global_vars).clone()
}
