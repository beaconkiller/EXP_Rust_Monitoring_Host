use crate::controllers::cont_get_info::StrGetInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug)]
pub struct WkClients {
    pub addr: Mutex<String>,
    pub status: Mutex<i32>,
    pub data: Mutex<StrClientData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StrClientData {
    pub inc_addr: Option<String>,
    pub inc_status: Option<i32>,
    pub disk_data: Option<Vec<StrDiskInfo>>,
    pub cpu_info: Option<Vec<StrCpuInfo>>,
    pub ram_info: Option<StrRamInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StrDiskInfo {
    pub mounted_on: String,
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrCpuInfo {
    pub i: i32,
    pub cpu_name: String,
    pub cpu_n: i32,
    pub cpu_usage: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrRamInfo {
    pub memory_total: u64,
    pub memory_used: u64,
    pub percent: String,
}

impl WkClients {
    pub fn new(addr: String) -> Self {
        Self {
            addr: Mutex::new(addr.to_string()),
            status: Mutex::new(0),
            data: Mutex::new(StrClientData {
                cpu_info: None,
                disk_data: None,
                inc_addr: None,
                inc_status: None,
                ram_info: None,
            }),
        }
    }

    pub fn get_info(&self) -> StrGetInfo {
        StrGetInfo {
            addr: self.addr.lock().unwrap().clone(),
            status: self.status.lock().unwrap().clone(),
            data: self.data.lock().unwrap().clone(),
        }
    }

    pub fn get_latest_data(&self) -> &WkClients {
        println!("{:?}", self);
        self
    }

    pub fn init_worker(self: Arc<Self>) {
        tokio::spawn(async move {
            println!("Worker started for {}", self.addr.lock().unwrap());
            loop {
                {
                    let x = { self.addr.lock().unwrap().clone() };

                    let resp = match self::WkClients::get_monitoring_data(x.to_string()).await {
                        Ok(data) => data,
                        Err(e) => {
                            let mut init_data = self.status.lock().unwrap();
                            *init_data = i32::from(500);
                            HashMap::from([("data".to_string(), Value::from(()))])
                        }
                    };
                    Self::map_data(&self, resp);
                }
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        });
    }

    pub async fn get_monitoring_data(
        str_addr: String,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        let str_addr_sysinfo = format!("http://{str_addr}/get_sysinfo");
        let resp = reqwest::get(str_addr_sysinfo).await?;
        let response = resp.json::<HashMap<String, Value>>().await?;

        Ok(response)
    }

    fn map_data(&self, resp_res: HashMap<String, Value>) {
        // println!("{:?}", resp_res);

        let resp_res = Value::from(resp_res["data"].clone());
        let arr_disks: Vec<StrDiskInfo> =
            serde_json::from_value(resp_res["data"]["disk_info"].clone()).unwrap_or_default();
        let arr_cpus: Vec<StrCpuInfo> =
            serde_json::from_value(resp_res["data"]["cpu_info"].clone()).unwrap_or_default();
        let ram_info: Option<StrRamInfo> =
            serde_json::from_value(resp_res["data"]["mem_info"].clone()).unwrap_or_default();

        // println!("{:#?}", resp_res);

        let final_data: StrClientData = StrClientData {
            inc_addr: Some(resp_res["addr"].to_string()),
            inc_status: None,
            cpu_info: Some(arr_cpus),
            disk_data: Some(arr_disks),
            ram_info: ram_info,
        };

        let mut init_data = self.data.lock().unwrap();
        *init_data = final_data
    }
}
