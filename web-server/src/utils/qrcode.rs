use fast_qr::qr::QRBuilder;
use log::info;

pub async fn generate_qrcode(_content: &str) -> () {
    let qrcode = QRBuilder::new("test").build().unwrap();
    info!("{:?}", qrcode.data);
    let str = qrcode.to_str(); // .print() exists
    println!("{}", str);
}
