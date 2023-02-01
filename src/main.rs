use std::{
    fs::create_dir,
    io::{self, Write},
    path::{Path, PathBuf},
};

use qrcode_generator::QrCodeEcc;

fn main() {
    // create /qrc as subfolder if it does not exist 
    if !Path::new("./qrc").is_dir() {
        match create_dir("./qrc") {
            Err(e) => println!("Err: [{e}]"), _ => {}
        }
    }

    println!("QR-Code Generator");

    // data to encoded into QR-Code
    let mut data = String::new();

    // path for new QR-Code
    let mut path_str = String::new();
    let mut path = PathBuf::new();

    print!("Data: ");
    io::stdout().flush().unwrap_or_else(|e| panic!("{e}"));

    io::stdin()
        .read_line(&mut data)
        .expect("Failed to get data to be bound.");

    print!("Name: ");
    io::stdout().flush().unwrap();

    // create path from path string
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

    // generate QR-Code
    match qrcode_generator::to_png_to_file(data, QrCodeEcc::Low, 1024, path) {
        Ok(_) => {}
        Err(e) => {
            println!("Err: [{:?}]", e)
        }
    };
}
