extern crate nng;

use std::ffi::CString;
use std::io::{self, Write};
use std::os::raw::{c_char, c_int, c_void};
use std::process::Command;
use std::ptr;

use std::mem;

mod obs;

static mut obs_module_pointer: *mut obs::obs_module_t = ptr::null_mut();

#[no_mangle]
pub extern "C" fn obs_module_set_pointer(module: *mut obs::obs_module_t) {
    unsafe { obs_module_pointer = module }
}

#[no_mangle]
pub extern "C" fn obs_module_ver() -> u32 {
    (22 << 24) | (0 << 16) | 3
}

#[no_mangle]
pub extern "C" fn obs_module_name() -> *const c_char {
    let s = CString::new("OBS Rust Module").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

#[no_mangle]
pub extern "C" fn frontend_callback(event: obs::obs_frontend_event, private_data: *mut c_void) {
    print!("callback!!\n");

    unsafe {
        let message = nng::Message::try_from(b"test");
        (*(private_data as *mut nng::Socket)).send(message.unwrap());
    }

    match event {
        obs::obs_frontend_event::OBS_FRONTEND_EVENT_RECORDING_STARTED => {}
        obs::obs_frontend_event::OBS_FRONTEND_EVENT_RECORDING_STOPPED => {}
        _ => println!("No idea what the fuck that event was :|"),
    }
}

#[no_mangle]
pub extern "C" fn obs_module_load() -> bool {
    print!("Hello from rust!\n");

    let mut socket = nng::Socket::new(nng::Protocol::Pair0).unwrap();
    socket.set_nonblocking(true);
    socket.listen("ipc:///tmp/obsprism.ipc");

    let tmp: Box<nng::Socket> = Box::new(socket);

    unsafe {
        let userData = Box::into_raw(tmp) as *mut c_void;
        obs::obs_frontend_add_event_callback(frontend_callback, userData);
    }

    return true;
}
