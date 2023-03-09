#![allow( non_upper_case_globals
    , non_camel_case_types
    , non_snake_case
    , dead_code)]

use std::ffi::c_void;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::{env, process::exit};
use std::fs;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("[!] args err");
        exit(-1);
    }

    let filepath = args.get(1).unwrap();
    println!("[*] filepath: {}", filepath);
    let pathbuf = PathBuf::from(filepath);
    let abspath = fs::canonicalize(pathbuf).unwrap();
    let rawdata = fs::read(abspath).unwrap();

    let mut writer = BufWriter::new(fs::File::create("./test.raw").unwrap());

    const BUF_SIZE: usize = 10;
    let mut ibuf: [u8; BUF_SIZE] = Default::default();
    let obuf: [i16; BUF_SIZE*2] = Default::default();

    // libg722
    unsafe {
        let g722_dctx: *mut c_void = g722_decoder_new(64000, G722_SAMPLE_RATE_8000 as i32);
        if g722_dctx.is_null() {
            eprintln!("[!] g722_decoder_new failed");
            exit(-1);
        }
        
        for i in (0..rawdata.len()).step_by(10) {
        
            // copy every 10 bytes
            ibuf.copy_from_slice(&rawdata[i..i+10]);

            g722_decode(g722_dctx, ibuf.as_ptr(), BUF_SIZE as i32, obuf.as_ptr() as *mut i16);
            
            // i16 to u8
            let (_, shorts, _) = obuf.align_to::<u8>();
            
            writer.write(&shorts[0..20]).unwrap();
        }
        writer.flush().unwrap();
    }

}
