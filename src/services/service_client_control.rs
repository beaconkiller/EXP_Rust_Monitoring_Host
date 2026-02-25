use crate::{
    LL_global::{GL_ARR_CLIENTS, GL_ARR_GET_DATA},
    helper::service_helper::SrvHelper,
};
use reqwest::Response;
use serde_json::Value;
use std::{collections::HashMap, time::Duration};

pub struct SrvCLientControl {}

pub struct StrRespSysinfo {
    status: i16,
    data: HashMap<String, String>,
}

impl SrvCLientControl {
    pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
        println!();
        println!(" --------- INIT SERVICE --------- ");
        println!();

        let mut arr = GL_ARR_CLIENTS.lock().unwrap();

        Self::get_data_loop(arr.to_vec()).await;

        Ok(())
    }

    pub fn add_client(str_ip: &str) {
        let mut arr = GL_ARR_CLIENTS.lock().unwrap();

        let mut hm: HashMap<String, Value> = HashMap::new();
        hm.insert("addr".to_string(), Value::from(str_ip.to_string()));

        arr.push(hm);
    }

    pub async fn get_monitoring_data(
        str_addr: String,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        let mut str_addr_sysinfo = format!("http://{str_addr}/get_sysinfo");
        let resp = reqwest::get(str_addr_sysinfo).await?;
        let response = resp.json::<HashMap<String, Value>>().await?;

        Ok(response)
    }

    async fn get_data_loop(arr_data: Vec<HashMap<String, Value>>) {
        // let arr_clients = GL_ARR_CLIENTS.lock().unwrap();
        println!("{:?}", arr_data);

        SrvHelper::_get_i_by_key(
            arr_data,
            "addr".to_string(),
            "192.168.100.205:2109".to_string(),
        );

        let arr_data = GL_ARR_GET_DATA.lock().unwrap();
        println!("{:?}", arr_data);

        // while true {
        //     let mut arr_all_data: Vec<HashMap<String, Value>> = Vec::new();
        //     arr_all_data.clear();

        //     GL_ARR_GET_DATA.lock().unwrap().clear();

        //     for el in arr_data.to_vec() {
        //         let addr = el["addr"].to_string();

        //         let mut new_hm: HashMap<String, Value> = HashMap::new();
        //         new_hm.insert("addr".to_string(), Value::from(addr.clone()));

        //         // println!("{:?}", el);
        //         let resp = SrvCLientControl::get_monitoring_data(addr).await;

        //         match resp {
        //             Ok(data) => {
        //                 let obj_data = &data["data"].clone();
        //                 new_hm.insert("status".to_string(), Value::from(200));
        //                 new_hm.insert("disks".to_string(), obj_data.clone());

        //                 let mut global = GL_ARR_GET_DATA.lock().unwrap();
        //                 global.push(new_hm);
        //             }
        //             Err(e) => {
        //                 new_hm.insert("status".to_string(), Value::from(404));
        //                 let mut global = GL_ARR_GET_DATA.lock().unwrap();
        //                 global.push(new_hm);
        //             }
        //         };
        //     }

        //     println!("{:?}", GL_ARR_GET_DATA.lock().unwrap());

        //     for el in arr_all_data {
        //         println!("{:#?}", el);
        //     }

        //     tokio::time::sleep(Duration::from_secs(1)).await;
        // }

        // println!("{:?}", arr_all_data);
    }
}
