use crate::controllers::cont_get_info::StrGetInfo;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug, Clone)]
pub struct WkClients {
    pub addr: Arc<Mutex<String>>,
    pub status: Arc<Mutex<i32>>,
    pub data: Arc<Mutex<StrClientData>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StrClientData {
    pub inc_addr: Option<String>,
    pub inc_status: Option<i32>,
    pub disk_data: Option<Vec<StrDiskInfo>>,
    pub cpu_info: Option<Vec<StrCpuInfo>>,
    pub ram_info: Option<StrRamInfo>,
    pub last_success: Option<String>,
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
            addr: Arc::new(Mutex::new(addr.to_string())),
            status: Arc::new(Mutex::new(0)),
            data: Arc::new(Mutex::new(StrClientData {
                cpu_info: None,
                disk_data: None,
                inc_addr: None,
                inc_status: None,
                ram_info: None,
                last_success: None,
            })),
        }
    }

    pub fn get_info(&self) -> StrGetInfo {
        StrGetInfo {
            addr: self.addr.lock().unwrap().clone(),
            status: self.status.lock().unwrap().clone(),
            data: self.data.lock().unwrap().clone(),
        }
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
                            println!("{:?}", e);
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

    pub fn get_current_time(&self) -> String {
        let now = Utc::now();
        let iso_string = now.to_rfc3339();
        let parts: Vec<&str> = iso_string.split('.').collect();
        let str_0: &str = parts[0];

        str_0.to_string()
    }

    fn map_data(&self, resp_res: HashMap<String, Value>) {
        let resp_res = Value::from(resp_res["data"].clone());

        let arr_disks: Option<Vec<StrDiskInfo>> =
            serde_json::from_value(resp_res["data"]["disk_info"].clone()).unwrap_or_default();
        let arr_cpus: Option<Vec<StrCpuInfo>> =
            serde_json::from_value(resp_res["data"]["cpu_info"].clone()).unwrap_or_default();
        let ram_info: Option<StrRamInfo> =
            serde_json::from_value(resp_res["data"]["mem_info"].clone()).unwrap_or_default();

        let last_success = { self.data.lock().unwrap().last_success.clone() };

        let final_data = if arr_disks.is_none() || arr_cpus.is_none() || ram_info.is_none() {
            StrClientData {
                inc_addr: Some(resp_res["addr"].to_string()),
                inc_status: None,
                cpu_info: arr_cpus,
                disk_data: arr_disks,
                ram_info: ram_info,
                last_success: last_success,
            }
        } else {
            StrClientData {
                inc_addr: Some(resp_res["addr"].to_string()),
                inc_status: None,
                cpu_info: arr_cpus,
                disk_data: arr_disks,
                ram_info: ram_info,
                last_success: Some(self.get_current_time()),
            }
        };

        {
            let mut init_data = self.data.lock().unwrap();
            *init_data = final_data.clone();
        }
    }
}
