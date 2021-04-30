use std::ffi::{CStr, CString};
use std::os::raw::c_uchar;

#[cfg(all(target_os = "win32", target_arch = "x86"))]
#[link(name = "kernel32")]
#[no_mangle]
pub fn hex2ascii(/*hex_in : CString*/) -> *const u8{
    //let mut ascii_ch : u8 = 0;
    let mut out_str = String::new();
    //let mut ascii_ch = format!("0x{:X?}, ", hex_in[0]);
    //println!("hex2ascii fn {}, {:#X}", hex_len, hex_in[0]);
    
    
    // for i in hex_in.to_bytes()
    // {
    //     //ascii_ch = format!("0x{:02X?}, ", element);
    //     out_str.push_str(&(format!("0x{:02X?}, ", i)));
    // }

    // //println!("{ }", out_str.len());
    // out_str.pop();
    // //println!("{ }", out_str.len());
    // out_str.pop();
    // //println!("{ }", out_str);

    // let s = CString::new(out_str).unwrap();
    
    return "s".as_ptr();
}