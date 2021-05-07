#[allow(non_camel_case_types)]
#[allow(dead_code)]
use std::ffi::{CString};
use std::os::raw::c_char;

#[repr(u8)]
pub enum ControlHex {
    With_X          = 1,
    With_Space      = 2,
    WithOut_Space   = 3,
}

#[repr(u8)]
pub enum ControlError {
    OK           = 0,
    ERRDat       = 1,
    ERRLen       = 2,
    FATErr       = 255,
}


// It's a test function to call it from C/C++
#[no_mangle]
pub extern fn add_numbers(number1: i32, number2: i32) -> i32 {
    println!("Hello from rust!");
    number1 + number2
}



/****************************************************************************************/
/*                This function was made to conver bytes in string                      */
/*  Input:                                                                              */
/*      -- hex_len      - length of bytearray                                           */
/*      -- hex_in       - bytearray                                                     */
/*      -- ascii_len    - length of allocated memmory                                   */
/*      -- control      - type of output string                                         */
/*  Output:                                                                             */
/*      -- ascii_out    - pointer on output string                                      */
/*      -- ascii_len    - length of output string                                       */
/****************************************************************************************/

#[no_mangle]
pub extern fn hex2ascii(hex_len : usize, hex_in: *const u8, ascii_len : &mut usize, ascii_out : *mut c_char, control : ControlHex) -> ControlError {
    
    unsafe {
        if let Some(hex_in) = hex_in.as_ref() {
            if let Some(ascii_out) = ascii_out.as_mut() {

                let array = std::slice::from_raw_parts(hex_in, hex_len);

                let mut out_str = String::new();

                match control {
                    ControlHex::With_X => 
                    {
                        for i in array
                        {
                             out_str.push_str(&(format!("0x{:02X?}, ", i)));
                        }
                        out_str.pop();
                        out_str.pop();
                    }
                    ControlHex::With_Space => 
                    {
                        for i in array
                        {
                             out_str.push_str(&(format!("{:02X?} ", i)));
                        }
                        out_str.pop();
                    }
                    ControlHex::WithOut_Space =>
                    {
                        for i in array
                        {
                             out_str.push_str(&(format!("{:02X?}", i)));
                        }
                    }
                    _ => return ControlError::ERRDat
                }

                
                //println!("{ }", out_str);
                if *ascii_len < out_str.len() {
                    //println!("Len smaller { } < { }", *ascii_len, out_str.len());
                    return ControlError::ERRLen
                }
                else
                {
                    *ascii_len = out_str.len();
                    let c_str_out = CString::new(out_str).unwrap();
        
                    std::ptr::copy(c_str_out.as_ptr(), ascii_out, *ascii_len); // Copy N bytes to dst from src
                
                    return ControlError::OK 
                }
                
            }
            else{ 
                return ControlError::ERRDat
            }
        }
        else{ 
            return ControlError::ERRDat
        }
    }
}
