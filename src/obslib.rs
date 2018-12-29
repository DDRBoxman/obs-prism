extern crate nng;
extern crate hidapi;

use hidapi::HidApi;
use std::os::raw::{c_int, c_char, c_void};
use std::ptr;
use std::ffi::CString;
use std::io::{self, Write};
use std::process::Command;

use std::mem;

const BLINK1_REPORT_ID: u8 = 1;
const BLINK1_REPORT_SIZE: usize = 8;
const BLINK1_REPORT2_ID: u8 = 2;
const BLINK1_REPORT2_SIZE: usize = 60;
const BLINK1_BUF_SIZE:usize = BLINK1_REPORT_SIZE + 1;
const BLINK1_BUF2_SIZE: usize = BLINK1_REPORT2_SIZE + 1;

mod obs;

static mut obs_module_pointer: *mut obs::obs_module_t = ptr::null_mut();

#[no_mangle]
pub extern fn obs_module_set_pointer(module: *mut obs::obs_module_t) {
    unsafe {
        obs_module_pointer = module
    }
}

#[no_mangle]
pub extern fn obs_module_ver() -> u32 {
    (22 << 24) | (0 << 16) | 3
}

#[no_mangle]
pub extern fn obs_module_name() -> *const c_char {
    let s = CString::new("OBS Rust Module").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

#[no_mangle]
pub extern fn frontend_callback(event: obs::obs_frontend_event, private_data: *mut c_void) {
    print!("callback!!\n");

    unsafe {
        let message = nng::Message::try_from(b"test");
        (*(private_data as *mut nng::Socket)).send(message.unwrap());
    }

    match event {
        obs::obs_frontend_event::OBS_FRONTEND_EVENT_RECORDING_STARTED => {
            open_and_fade_to_rgb(255, 0, 0)
        },
        obs::obs_frontend_event::OBS_FRONTEND_EVENT_RECORDING_STOPPED => {
            open_and_fade_to_rgb(0, 0, 0)
        },
        _ => println!("No idea what the fuck that event was :|"),
    }
}

#[no_mangle]
pub extern fn obs_module_load() -> bool {
    print!("Hello from rust!\n");


    let mut socket = nng::Socket::new(nng::Protocol::Pair0).unwrap();
    socket.set_nonblocking(true);
    socket.listen("ipc:///tmp/obsprism.ipc");

    let tmp: Box<nng::Socket> = Box::new(socket);

    open_and_fade_to_rgb(0,0,0);

    unsafe {
        let userData = Box::into_raw(tmp) as *mut c_void;
        obs::obs_frontend_add_event_callback(frontend_callback, userData);
    }

    return true
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

fn open_and_fade_to_rgb(r :u8, g :u8, b :u8) {
    match HidApi::new() {
        Ok(api) => {
            // Connect to device using its VID and PID (blink 1 controller)
            let (VID, PID) = (0x27B8, 0x01ED);
            let device = api.open(VID, PID).unwrap();
            fade_to_rgb(&device, r, g, b, 0);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
}