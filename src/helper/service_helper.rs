use std::collections::HashMap;

use serde_json::Value;

pub struct SrvHelper;

impl SrvHelper {
    pub fn _get_i_by_key(arr: Vec<HashMap<String, Value>>, key: String, val: String) {
        for el in &arr {
            let to_search = Value::from(val.clone());

            println!("{:?}", el);
            println!("{:?}", to_search);
            println!("{:?}", el[&key]);
        }
    }
}
