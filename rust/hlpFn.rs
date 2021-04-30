fn hex2ascii(hex_in : &[u8]) -> String{
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

    return out_str;
}

fn main() {
    let arr: [u8; 5] = [0x22, 0x03, 0x34, 0x05, 0x06];
    println!("{ }", hex2ascii(&arr));
    //println!("Hello, world!");
}