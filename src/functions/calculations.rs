pub fn gather_average(contents: String, target: String) -> u128 {
    let json = serde_json::from_str::<serde_json::Value>(contents.as_str()).unwrap();

    let mut total_resources = 0;

    for object in json.as_array().unwrap() {
        total_resources += object[target.clone()].as_u64().unwrap();
    }

    let average_resources = (total_resources as f64) / (json.as_array().unwrap().len() as f64);

    let average_resources = average_resources as u128;

    average_resources
}

pub fn growth_rate(population_start: u128, population: u128) -> f64 {
    let population = population as f64;

    let population_start = population_start as f64;

    let growth_rate = (population / population_start - 1.0) * 100.0;

    growth_rate
}
