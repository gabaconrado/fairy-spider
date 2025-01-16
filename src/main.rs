//! Main binary of the Fairy Spider OS

#![no_std]
#![no_main]

/// Panic behavior implementation
mod panic;
/// Screen implementation
mod vga;

/// A test message to be printed in the screen
pub const WELCOME_MSG: &[u8] = b"Welcome to Fairy Spider";

/// Entrypoint of the OS
///
/// This is necessary since we cannot rely on the standard C runtime or Rust minimal setup
///
/// The entrypoint loops forever
///
/// # Notes
///
/// - "No mangle" is used so the name keeps its original name
/// - "extern C" is used so the compiler uses the C calling convention for this method
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::print_vga(WELCOME_MSG);
    loop {}
}
