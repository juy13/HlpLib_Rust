// mod hlp_Lib;
// mod DES;

// use hlp_Lib;

// use crate::hlp_Lib::*;
use std::ffi::{CStr, CString};


fn main() {
    let arr: [u8; 5] = [0x22, 0x03, 0x34, 0x05, 0x06];

    let arr2 = arr.to_vec();

    unsafe {
        // let a : i64 = 1383827165325090801;
        // hlp_Lib::testDES2();
        let c_string = CString::from_vec_unchecked(arr2);
        //let out = hex2ascii(c_string).into_string();
       // println!("{:?}", out);
    }

   

    
    
    //println!("{ }", );
    //println!("Hello, world!");
}