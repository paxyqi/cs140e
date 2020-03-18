#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
// #![feature(repr_align)] // stable since 1.25.0
#![feature(attr_literals)]

#![feature(exclusive_range_pattern)]
//#![feature(alloc, allocator_api, global_allocator)]
#![feature(alloc, allocator_api)]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;
extern crate core;
extern crate pi;
extern crate stack_vec;
extern crate fat32;

pub mod allocator;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;
pub mod fs;

#[cfg(not(test))]
use allocator::Allocator;
use fs::FileSystem;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

extern crate compiler_builtins;

use pi::gpio::Gpio;
use pi::uart::MiniUart;


//old version is 0x3F000000
const GPIO_BASE: usize = 0xFE000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

use pi::timer::Timer;

#[cfg(not(test))]
use pi::timer::spin_sleep_ms;
use console::{CONSOLE,kprint,kprintln};

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn kmain() {

    ALLOCATOR.initialize();

}

