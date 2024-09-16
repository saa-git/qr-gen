use qrcode_generator::QrCodeEcc;
use std::{
    io::{stdin, stdout, Write},
    path::Path,
};

fn main() {
    println!("QR-Code Generator");

    let mut qr_data = String::new();
    print!("\nData: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut qr_data).unwrap();

    qrcode_generator::to_png_to_file(qr_data, QrCodeEcc::Low, 1024, Path::new("./qr.png")).unwrap();

    println!("\nSuccess!");
}
