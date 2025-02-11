use core::panic::PanicInfo;

/// Method called upon panics
///
/// Does nothing
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::println!("{info}");
    loop {}
}
