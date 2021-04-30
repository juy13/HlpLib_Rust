use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn hex2ascii(hex_in : &[u8]) -> CString{
    //let mut ascii_ch : u8 = 0;
    let mut out_str = String::new();
    //let mut ascii_ch = format!("0x{:X?}, ", hex_in[0]);
    //println!("hex2ascii fn {}, {:#X}", hex_len, hex_in[0]);
    
    
    for element in hex_in.iter()
    {
        //ascii_ch = format!("0x{:02X?}, ", element);
        out_str.push_str(&(format!("0x{:02X?}, ", element)));
    }

    //println!("{ }", out_str.len());
    out_str.pop();
    //println!("{ }", out_str.len());
    out_str.pop();
    //println!("{ }", out_str);

    let s = CString::new(out_str).unwrap();
    
    return s;
}