use std::thread;
use std::time;
use hidapi::HidApi;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let api = HidApi::new().unwrap();

    let (vid, pid) = (0x16C0, 0x0486);

    // Find the specific device's first interface (In our case this is the RawHID interface)
    match api.devices().iter().find(|d| d.vendor_id == vid && d.product_id == pid && d.interface_number == 0) {
        Some(device) => {
            open_device(&api, device);
        },
        None => {
            eprintln!("Could not find compatible device.");
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}

fn open_device(api: &hidapi::HidApi, device_info: &hidapi::HidDeviceInfo) {
    loop {
        // Connect to device using its VID and PID
        println!("Attempting to connect to device...");
        match device_info.open_device(&api) {
            Ok(device) => {
                println!("Device connected!");
                match device.get_manufacturer_string() {
                    Ok(manufacturer) => println!("Manufacturer: {}", manufacturer.unwrap()),
                    _ => {}
                }
                match device.get_product_string() {
                    Ok(product) => println!("Product: {}", product.unwrap()),
                    _ => {}
                }
                device_poll(&device);
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                thread::sleep(time::Duration::from_millis(1000));
            }
        };
    }
}

fn device_poll(device: &hidapi::HidDevice) {
    loop {
        // TODO
        let pipe_path = "D:\\Games\\Stepmania 5.1\\pipe\\StepMania-Lights-SextetStream";

        let mut pipe: Option<File> = None;

        println!("Reading pipe at {}", pipe_path);
        while !pipe.is_some() {
            match File::open(pipe_path) {
                Ok(file) => pipe = Some(file),
                Err(error) => {
                    println!("Could not open pipe: {}", error);
                    println!("Retrying in 3 seconds...");
                    thread::sleep(time::Duration::from_millis(3000));
                }
            }
        }
        println!("Pipe is open!");
        let pipe = pipe.unwrap();
        let mut last_line: String = "AAA".to_string();

        loop {
            let reader = BufReader::new(&pipe);
            let mut current_line: String = String::from("");

            for (_index, line) in reader.lines().enumerate() {
                current_line = line.unwrap(); // Ignore errors.
            }

            if current_line.len() == 0 || current_line == last_line {
                continue;
            }

            last_line = current_line.clone();

            let mut data: [u8; 65] = [0; 65];
            for (i, byte) in current_line.as_bytes().iter().enumerate() {
                data[i+1] = *byte;
            }

            device.write(&data).unwrap();
        }
    }
}
