use core::panic::PanicInfo;
#[no_mangle]
#[cfg(not(test))]
//#[lang = "panic_fmt"]
//pub extern fn panic_fmt(fmt: ::std::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
#[lang = "panic_impl"]
pub extern fn rust_begin_panic(info: &PanicInfo) -> ! {  
    // FIXME: Print `fmt`, `file`, and `line` to the console.

    loop { unsafe { asm!("wfe") } }
}

#[cfg(not(test))] #[lang = "eh_personality"] pub extern fn eh_personality() {}
