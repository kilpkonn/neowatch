use libc::{SIGINT, c_int, sighandler_t, signal};

extern "C" fn sigint_handler(_sig: c_int) {
    println!("\x1B[?1049l");
    std::process::exit(0)
}

unsafe fn set_os_handler(sig: c_int, handler: extern "C" fn(c_int)) {
    signal(sig, handler as sighandler_t);
}

pub fn setup_handlers()
{
    unsafe { set_os_handler(SIGINT, sigint_handler) }
}
