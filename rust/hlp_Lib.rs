#[allow(non_camel_case_types)]
#[allow(dead_code)]
use std::ffi::{CString};
use std::os::raw::c_char;

pub enum ControlHex {
    With_X,
    WithOut_X,
    With_Space,
    WithOut_Space,
}


// It's a test function to call it from C/C++
#[no_mangle]
pub extern fn add_numbers(number1: i32, number2: i32) -> i32 {
    println!("Hello from rust!");
    number1 + number2
}



// Make control of pointers and add RC with controlling a `control`
#[no_mangle]
pub extern fn hex2ascii(hex_len : usize, hex_in: *const u8, ascii_len : &mut usize, ascii_out : *mut c_char, control : ControlHex) -> i32 {
    
    unsafe {
        if let Some(hex_in) = hex_in.as_ref() {
            if let Some(ascii_out) = ascii_out.as_mut() {

                let array = std::slice::from_raw_parts(hex_in, hex_len);

                let mut out_str = String::new();

                for i in array
                {
                     out_str.push_str(&(format!("0x{:02X?}, ", i)));
                }
                println!("{ }", out_str);
                out_str.pop();
                out_str.pop();
                *ascii_len = out_str.len();
                let c_str_out = CString::new(out_str).unwrap();
        
                std::ptr::copy(c_str_out.as_ptr(), ascii_out, *ascii_len); // Copy N bytes to dst from src
                
                return 0 
            }
            else{ 
                return 0
            }
        }
        else{ 
            return 0
        }
    }

    // if let Some(hex_in) = hex_in {
    //     println!("Here I'm");
    //     if let Some(ascii_out) = ascii_out {
    //         unsafe {
                
    //             let array = std::slice::from_raw_parts(hex_in, hex_len);

    //             let mut out_str = String::new();

    //             for i in array
    //             {
    //                  out_str.push_str(&(format!("0x{:02X?}, ", i)));
    //             }
    //             println!("{ }", out_str);
    //             out_str.pop();
    //             *ascii_len = out_str.len();
    //             let c_str_out = CString::new(out_str).unwrap();
        
    //             std::ptr::copy(c_str_out.as_ptr(), ascii_out, *ascii_len); // Copy N bytes to dst from src
                
    //             return 0 
    //         }

    //     }
    //     else{ 
    //         return 0
    //     }
        
    // }
    // else{ 
    //     return 0
    // }

    // match control{

    //     With_X => {

    //     }

    // }

    

}
