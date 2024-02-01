use std::ffi::CString;
use std::os::raw::{c_char, c_int}; 
use clap::Parser; 

extern "C" {
    fn c_main(log: *const c_char, store: *const c_char) -> c_int;
}

#[derive(Parser, Debug)]
struct Opts {
    /// Store file 
    #[clap(short, long)]
    store: String,

    /// Log directory 
    #[clap(short, long)]
    log: String, 
}

fn main() {
    let opts = Opts::parse(); 
    let clog = CString::new(opts.log).expect("Log dir invalid CString");
    let cstore = CString::new(opts.store).expect("Store file invalid CString");
    unsafe {
        let rc = c_main(clog.as_ptr(), cstore.as_ptr()); 
        std::process::exit(rc);
    }
}
