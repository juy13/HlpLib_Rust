#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[allow(unused_assignments)]

//use DES::*;
use std::ffi::{CString};
use std::os::raw::{c_char, c_uchar};
use std::panic;
use std::mem;

mod DES;
//use crate::DES;
// use DES::*;

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

// TODO here is some troubles with string
// "57 49 41 33 25 17 09 01 58 50 42 34 26 18 10 02 59 51 43 35 27 19 11 03 60 52 44 36 63 55 47 39 31 23 15 07 62 54 46 38 30 22 14 06 61 53 45 37 29 21 13 05 28 20 12 04""
fn translate_a2h(a1 : i8, a2 : i8, a_out : &mut i8) -> ControlError {

    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));
    
        let result = panic::catch_unwind(|| {
                let mut num;

                if a1 <= '9' as i8{
                    num = (a1 - ('0' as i8)) * 16;
                }
                else{
                    num = ((a1 - ('A' as i8)) + 10) << 4;
                }
    
                if a2 <= '9'  as i8{
                    num |= a2 - ('0' as i8);
                }
                else{
                    num |= (a2 - ('A' as i8)) + 10;
                }
                num
        });

        match result {
            Ok(res) => {
                *a_out = res; 
                return ControlError::OK;
            },
            Err(_) => return ControlError::FATErr,
        }
}

fn translate_a2na(ascii_in : &[i8], ascii_len : &mut usize, ascii_out : &mut Vec<i8>, control : ControlHex) -> ControlError{


    let mut out_str: Vec<i8> = Vec::new(); 
    let mut i = 0;
    let mut flag = 0;

    match control {

        ControlHex::With_Space => {

            while i < *ascii_len {

                if i % 1 == 0 {                 //I think it's a wrong way....
                    out_str.push(ascii_in[i]);
                    i += 1;
                    continue;
                }
                if i % 2 == 0 {
                    out_str.push(ascii_in[i]);
                    i += 1;
                    continue;
                }
                if i % 3 == 0 {
                    i += 1;
                    continue;
                }

            }       
            println!("Out of With_Space: {:?}", out_str);
            *ascii_len = out_str.len();
            *ascii_out = out_str;
            return ControlError::OK
        }

        ControlHex::With_X => {

            println!("Len: {:?}", *ascii_len);
            while i < *ascii_len {

                match flag {

                    0 => {
                        flag += 1;
                        i += 1;
                        continue;
                    }
                    1 => {
                        flag += 1;
                        i += 1;
                        continue;
                    }
                    2 => {
                        flag += 1;
                        out_str.push(ascii_in[i]);
                        i += 1;
                        continue;
                    }
                    3 => {
                        flag += 1;
                        out_str.push(ascii_in[i]);
                        i += 1;
                        continue;
                    }
                    4 => {
                        flag += 1;
                        i += 1;
                        continue;
                    }
                    5 => {
                        flag = 0;
                        i += 1;
                        continue;
                    }
                    _ => return ControlError::FATErr
                }

            }            
            println!("Out of With_X: {}", out_str.len());
            *ascii_len = out_str.len();
            *ascii_out = out_str;
            return ControlError::OK
        }

        ControlHex::WithOut_Space => {

            *ascii_out = ascii_in.to_vec();
            return ControlError::OK
        }

        _ => return ControlError::ERRDat

    } 
}


/****************************************************************************************/
/*          This function was made to convert ascii hex string in hexdemical            */
/*  Input:                                                                              */
/*      -- ascii_len    - length of ascii hex string                                    */
/*      -- ascii_in     - ascii hex string                                              */
/*      -- hex_len      - length of allocated memmory                                   */
/*      -- control      - type of output string                                         */
/*  Output:                                                                             */
/*      -- hex_out    - pointer on output bytearray                                     */
/*      -- hex_len    - length of output bytearray                                      */
/****************************************************************************************/

#[no_mangle]
pub extern fn ascii2hex(ascii_len : usize, ascii_in : *const c_char, hex_len : &mut usize, hex_out : *mut u8, control : ControlHex) -> ControlError {

    let mut vec: Vec<u8> = Vec::new(); 
    let mut array_norm: Vec<i8> = Vec::new();
    let mut rc;
    let mut vec_len = ascii_len;

    unsafe {

        if let Some(ascii_in) = ascii_in.as_ref() {
            if let Some(hex_out) = hex_out.as_mut() {

                let array = std::slice::from_raw_parts(ascii_in, ascii_len);
                rc = translate_a2na(array, &mut vec_len, &mut array_norm, control);
                match rc {

                    ControlError::OK => {

                        println!("{:#?}", array);
                        let mut i = 0;
                
                        while i < vec_len
                        {
                            let mut num : i8 = 0;
                            rc = translate_a2h(array_norm[i], array_norm[i+1], &mut num);
                            match rc {
                
                                ControlError::OK => { 
                                    vec.push(num as u8);
                                    i += 2;
                                }
                
                                _ => return rc
                
                            }
                            
                        }
                
                        if *hex_len < vec.len() {
                            return ControlError::ERRLen
                        }
                        else
                        {
                            *hex_len = vec.len();
                            //println!("{ }", vec.len());
                            std::ptr::copy(vec.as_ptr(), hex_out, *hex_len);
                            return ControlError::OK 
                        }

                    }
                    _ => return rc

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

#[test]
fn testDES() {
    println!("Result {:#?}", DES::DesSbox[0][0][1]);
}
