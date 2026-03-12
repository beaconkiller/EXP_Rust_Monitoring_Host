// use std::time;
use chrono;
use rand::{self, RngExt, distr::Alphanumeric};
use serde_json::to_string;

#[derive(Debug)]
pub struct SrvGenerate {}

impl SrvGenerate {
    pub fn gen_timestamp() -> i64 {
        let time = chrono::Utc::now().timestamp();
        time
    }

    pub fn gen_rand_strings(n: u32) -> String {
        let rng = rand::rng();
        let rand_str: String = rng
            .sample_iter(&Alphanumeric)
            .take(n as usize)
            .map(char::from)
            .collect();

        rand_str
    }

    pub fn gen_letters_ts() -> String {
        let x = Self::gen_rand_strings(14);
        let y = Self::gen_timestamp().to_string();
        let letters = format!("{x}_{y}");
        letters
    }
}
