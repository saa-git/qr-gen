use std::io::{stdin, stdout, Write};

fn main() {
    println!("QR-Code Generator");

    let mut qr_data = String::new();
    print!("\nData: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut qr_data).unwrap();

    let qr = qrcode::QrCode::new(&qr_data).unwrap();
    let image = qr.render::<image::Luma<u8>>().build();
    image.save("./qr.png").unwrap();

    println!("\nSuccess!");
}
