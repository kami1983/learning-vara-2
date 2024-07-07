#![no_std]

use gstd::msg;

static mut COUNTER: i32 = 0;

#[no_mangle]
unsafe extern "C" fn handle() {
    let command = msg::load_bytes().expect("Invalid message");

    match command.as_slice() {
        b"inc" /* 0x696e63 */ => COUNTER += 1,
        b"dec" /* 0x646563 */ => COUNTER -= 1,
        b"get" /* 0x676574 */ => (),
        _ => panic!("Invalid command"),
    };

    msg::reply(COUNTER, 0).expect("Unable to reply");
}
