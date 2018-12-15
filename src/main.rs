extern crate hidapi;

use hidapi::HidApi;
use std::{thread, time};


const BLINK1_REPORT_ID: u8 = 1;
const BLINK1_REPORT_SIZE: usize = 8;
const BLINK1_REPORT2_ID: u8 = 2;
const BLINK1_REPORT2_SIZE: usize = 60;
const BLINK1_BUF_SIZE:usize = BLINK1_REPORT_SIZE + 1;
const BLINK1_BUF2_SIZE: usize = BLINK1_REPORT2_SIZE + 1;

fn main() {
    println!("Printing all available hid devices:");

    let ten_millis = time::Duration::from_millis(1000);

    match HidApi::new() {
        Ok(api) => {
            // Connect to device using its VID and PID (blink 1 controller)
            let (VID, PID) = (0x27B8, 0x01ED);
            let device = api.open(VID, PID).unwrap();
            let manufacturer = device.get_manufacturer_string().unwrap();
            let product = device.get_product_string().unwrap();
            let serial = device.get_serial_number_string().unwrap();
            println!("Product {:?} Device {:?} Serial {:?}", manufacturer, product, serial);

            fade_to_rgb(&device, 255, 0, 0, 0);
            fade_to_rgb(&device, 0, 0, 255, 1);
            thread::sleep(ten_millis);
            off(&device);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
}

fn off(device :&hidapi::HidDevice) {
    fade_to_rgb(device, 0,0,0, 0);
    fade_to_rgb(device, 0,0,0, 1);
}

fn fade_to_rgb(device :&hidapi::HidDevice, r :u8, g :u8, b :u8, index :u8) {
    let dms = 100; // 1000 ms

    let mut buf: [u8; BLINK1_BUF_SIZE] = [0; BLINK1_BUF_SIZE];

    buf[0] = BLINK1_REPORT_ID;     // report id
    buf[1] = 'c' as u8;   // command code for 'fade to rgb'
    buf[2] = r;
    buf[3] = g;
    buf[4] = b;
    buf[5] = 0;
    buf[6] = dms % 0xff;
    buf[7] = index;

    let result = device.send_feature_report(&buf);
    println!("{:?}", result)
}