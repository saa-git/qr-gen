use std::{
    fs::create_dir,
    io::{self, Write},
    path::{Path, PathBuf},
};

use qrcode_generator::QrCodeEcc;

fn main() {
    if !Path::new("./qrc").is_dir() {
        match create_dir("./qrc") {
            Ok(_) => {}
            Err(e) => println!("Err: [{e}]"),
        }
    }

    println!("QR-Code Generator");

    let mut data = String::new();
    let mut path_str = String::new();
    let mut path = PathBuf::new();

    print!("Data: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut data)
        .expect("Failed to get data to be bound.");

    print!("Name: ");
    io::stdout().flush().unwrap();

    // match replaced the string concat
    match io::stdin().read_line(&mut path_str) {
        Ok(_) => {
            path.push(
                format!("./qrc/{path_str}.png")
                    .replace(' ', "_")
                    .replace("\r\n", ""),
            );
        }
        Err(e) => println!("Err: [{e}]"),
    }

    match qrcode_generator::to_png_to_file(data, QrCodeEcc::Low, 1024, path) {
        Ok(_) => {}
        Err(e) => {
            println!("Err: [{:?}]", e)
        }
    };
}
