use std::env;

pub fn use_variable_TSM_API_KEY() -> String {
    let TSM_API_KEY: String = env::var("TSM_API_KEY").expect("TSM_API_KEY must be set").to_string();
    TSM_API_KEY
}