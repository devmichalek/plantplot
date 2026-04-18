
struct Plant {
    database_id: u64,
    box_id: u64,
    name: String,
    binomial_name: String,
    last_time_watered: u64,
    next_time_to_be_watered: u64,
    position: (f64, f64),
}
