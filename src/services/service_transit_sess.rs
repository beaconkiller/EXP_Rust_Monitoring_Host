use image::Luma;
use qrcode::QrCode;

use crate::services::service_generate::SrvGenerate;

#[derive(Debug)]
pub struct SrvTransitSess {}

impl SrvTransitSess {
    pub fn get_qr_code(data: &str) {
        let str_utf8: &[u8] = data.as_bytes();
        let code = QrCode::new(data).unwrap();

        println!("{:?}", "Time");
        println!("{:?}", SrvGenerate::gen_timestamp());
        println!("{:?}", SrvGenerate::gen_rand_strings(14));

        let image: image::ImageBuffer<Luma<u8>, Vec<u8>> =
            code.render::<Luma<u8>>().max_dimensions(500, 500).build();

        image.save("tmp/tmp_qr_session/qrcode.png").unwrap();
    }
}
