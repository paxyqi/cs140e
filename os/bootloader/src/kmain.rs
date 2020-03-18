#![feature(asm, lang_items)]

extern crate core;
extern crate xmodem;
extern crate pi;

pub mod lang_items;
//pub mod mutex;
//pub mod console;


use std::io;
use std::io::Write;
use xmodem::Xmodem;
use pi::uart::MiniUart;
use pi::gpio::Gpio;

//use std::fmt::Write;

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.

fn jump_to(addr: *mut u8) -> ! {
    unsafe {
        asm!("br $0" : : "r"(addr as usize));
        loop { asm!("nop" :::: "volatile")  }
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    // FIXME: Implement the bootloader.
    
    let mut ready_led = Gpio::new(16).into_output();
    ready_led.set();
    
    let mut uart=MiniUart::new();
    uart.set_read_timeout(750);

    
    loop{
        let dest = unsafe { std::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE) };
        match xmodem::Xmodem::receive(&mut uart, io::Cursor::new(dest)) {
            Ok(_) => {
                // Succeed
                ready_led.clear();
                jump_to(BINARY_START)
            }
            Err(err) => match err.kind() {
                io::ErrorKind::TimedOut => continue,
                io::ErrorKind::InvalidData => continue, // might receive 0x00 when no input
                _ => uart.write_fmt(format_args!("Error: {:?}\r\n", err))
                    .unwrap(),
            },
        }
    }
    

}
