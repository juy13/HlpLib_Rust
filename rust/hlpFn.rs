fn hex2ascii(hex_in : &[i8], hex_len: usize){
    let mut ascii_ch : i8 = 0;
    ascii_ch = hex_in[0];
    println!("hex2ascii fn {}, {:#X}", hex_len, hex_in[0]);
    
}

fn main() {
    let arr: [i8; 5] = [0x22, 0x03, 0x04, 0x05, 0x06];
    hex2ascii(&arr, arr.len());
    //println!("Hello, world!");
}