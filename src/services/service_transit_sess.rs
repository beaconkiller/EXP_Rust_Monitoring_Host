use std::time::{SystemTime, UNIX_EPOCH};

use image::Luma;
use qrcode::QrCode;
use tokio::sync::Mutex;

use crate::{global::LL_global::GL_SRV_TRANSIT_SESS, services::service_generate::SrvGenerate};

#[derive(Debug, Clone)]
pub struct resp_session_transit {
    sess_id: String,
    start_time: u64,
    exp_time: u64,
    data: Option<String>,
}

#[derive(Debug)]
pub struct session_transit {
    sess_id: String,
    start_time: u64,
    exp_time: u64,
    data: Mutex<Option<String>>,
}

#[derive(Debug)]
pub struct SrvTransitSess {
    pub transit_data: Mutex<Vec<session_transit>>,
}

impl SrvTransitSess {
    pub fn get_qr_code(&self, data: &str) {
        let str_utf8: &[u8] = data.as_bytes();
        let code = QrCode::new(data).unwrap();

        println!("{:?}", "Time");
        println!("{:?}", SrvGenerate::gen_timestamp());
        println!("{:?}", SrvGenerate::gen_rand_strings(14));

        let image: image::ImageBuffer<Luma<u8>, Vec<u8>> =
            code.render::<Luma<u8>>().max_dimensions(500, 500).build();

        image.save("tmp/tmp_qr_session/qrcode.png").unwrap();
    }

    pub async fn add_new_session(&self, id: String) {
        let exp_time = chrono::Utc::now();

        println!("{:?}", exp_time);
        println!("{:?}", exp_time.timestamp_millis());

        
        // let time_from = SystemTime::from(exp_time);



        let new_sess = session_transit {
            data: Mutex::new(None),
            exp_time: 123,
            sess_id: id,
            start_time: 123,
        };

        {
            let mut init_data = GL_SRV_TRANSIT_SESS.transit_data.lock().await;
            init_data.push(new_sess);
        }
    }

    pub async fn get_all_sess(&self) -> Vec<resp_session_transit> {
        let data = self.transit_data.lock().await;
        let mut arr: Vec<resp_session_transit> = vec![];

        for el in data.iter() {
            arr.push(resp_session_transit {
                sess_id: el.sess_id.clone(),
                start_time: el.start_time.clone(),
                exp_time: el.exp_time.clone(),
                data: el.data.lock().await.clone(),
            });
        }
        println!("{:?}", arr);
        arr
    }
}
