use libc::{SIGINT, SIGTERM, SIGHUP, c_int, sighandler_t, signal};

extern "C" fn die_handler(_sig: c_int) {
    println!("\x1B[?1049l");
    std::process::exit(0)
}

unsafe fn set_os_handler(sig: c_int, handler: extern "C" fn(c_int)) {
    signal(sig, handler as sighandler_t);
}

pub fn setup_handlers()
{
    unsafe { 
        set_os_handler(SIGINT, die_handler) ;
        set_os_handler(SIGTERM, die_handler);
        set_os_handler(SIGHUP, die_handler);
    }
}
