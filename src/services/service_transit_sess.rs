use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::{Engine, engine::general_purpose};
use chrono::Utc;
use image::{ColorType, ImageEncoder, Luma};
use qrcode::QrCode;
use rand::seq::index;
use tokio::{
    sync::Mutex,
    time::{Instant, sleep_until},
};

use crate::{global::LL_global::GL_SRV_TRANSIT_SESS, services::service_generate::SrvGenerate};

#[derive(Debug, Clone)]
pub struct resp_session_transit {
    sess_id: String,
    start_time: i64,
    exp_time: i64,
    data: Option<String>,
}

#[derive(Debug)]
pub struct session_transit {
    sess_id: String,
    start_time: i64,
    exp_time: i64,
    data: Mutex<Option<String>>,
}

#[derive(Debug)]
pub struct SrvTransitSess {
    pub transit_data: Mutex<Vec<session_transit>>,
    pub sess_lifespan: u64,
}

impl SrvTransitSess {
    pub fn get_qr_code(&self, data: &str) -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
        let code = QrCode::new(data).unwrap();

        // println!("{:?}", "Time");
        // println!("{:?}", SrvGenerate::gen_timestamp());
        // println!("{:?}", SrvGenerate::gen_rand_strings(14));

        let image: image::ImageBuffer<Luma<u8>, Vec<u8>> =
            code.render::<Luma<u8>>().max_dimensions(500, 500).build();

        // image
        //     .save(format!("tmp/tmp_qr_session/qrcode{data}.png"))
        //     .unwrap();

        image
    }

    pub async fn add_new_session(&self, id: String) -> String {
        let lifespan = self.sess_lifespan;
        println!("{:?}", format!(" -------------------------------------- "));
        println!("{:?}", format!(" Creating a new session."));
        println!("{:?}", format!(" Id       : {id} "));
        println!("{:?}", format!(" Lifespan : {lifespan}s "));
        println!("{:?}", format!(" -------------------------------------- "));

        let time = chrono::Utc::now();
        let id_temp = id.clone();

        // ====== WE ADD SELF DELETING IN THE SESSION INITIALIZATION =======

        tokio::spawn(async move {
            sleep_until(
                tokio::time::Instant::now()
                    + tokio::time::Duration::from_secs(GL_SRV_TRANSIT_SESS.sess_lifespan),
            )
            .await;
            GL_SRV_TRANSIT_SESS.delete_sess(id_temp).await;
        });

        // =================================================================

        let new_sess = session_transit {
            data: Mutex::new(None),
            sess_id: id.clone(),
            start_time: time.timestamp_millis(),
            exp_time: 123,
        };

        {
            let mut init_data = self.transit_data.lock().await;
            init_data.push(new_sess);
        }

        id
    }

    pub async fn delete_sess(&self, sess_id: String) {
        println!("{:?}", format!(" -------------------------------------- "));
        println!("{:?}", format!(" Deleting {sess_id}"));
        println!("{:?}", format!(" -------------------------------------- "));

        let mut data = self.transit_data.lock().await;
        let index: Option<usize> = match data.iter().position(|el| el.sess_id == sess_id) {
            Some(i) => Some(i),
            None => None,
        };
        if let Some(i) = index {
            data.remove(i);
        };
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

    // pub async fn add_new_session_auto(&self, data: &str) -> Vec<u8> {
    //     let id = SrvGenerate::gen_letters_ts();
    //     self.add_new_session(id.clone()).await;

    //     let qr_img: image::ImageBuffer<image::Luma<u8>, Vec<u8>> =
    //         GL_SRV_TRANSIT_SESS.get_qr_code(&id);

    //     let mut png_bytes: Vec<u8> = Vec::new();

    //     qr_img.as_raw().clone()
    //     // let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
    //     // encoder
    //     //     .write_image(
    //     //         &qr_img.as_raw(),
    //     //         qr_img.width(),
    //     //         qr_img.height(),
    //     //         ColorType::L8.into(),
    //     //     )
    //     //     .unwrap();

    //     // let base64_str = general_purpose::STANDARD.encode(png_bytes);

    //     // base64_str
    // }

    pub async fn add_new_session_auto(&self, data: &str) -> String {
        let id = SrvGenerate::gen_letters_ts();
        self.add_new_session(id.clone()).await;

        let qr_img: image::ImageBuffer<image::Luma<u8>, Vec<u8>> =
            GL_SRV_TRANSIT_SESS.get_qr_code(&id);

        let mut png_bytes: Vec<u8> = Vec::new();

        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        encoder
            .write_image(
                &qr_img.as_raw(),
                qr_img.width(),
                qr_img.height(),
                ColorType::L8.into(),
            )
            .unwrap();

        let base64_str = general_purpose::STANDARD.encode(png_bytes);

        base64_str
    }
}
