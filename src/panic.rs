use core::panic::PanicInfo;

/// Method called upon panics
///
/// Does nothing
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
