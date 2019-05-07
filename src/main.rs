use std::env;
use std::fs;

// Match args for od
// -j XXX skip
// -A X address base d|o|x|n
// -N XXX go up to length xxx
// + offset 

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let path = &args[2];

    let bytes = fs::read(path).unwrap();

    let should_print_address = true;

    println!("decoding {}", path);
    let read_up_to = 17;
    let skip = 0; // Need to add a skip arg
    for i in 0..read_up_to {
        if should_print_address {
            print!("{:03x} ", i * 4);
        }
        read_pair(&bytes, skip + (i*2));
    }
}

fn read_pair(bytes: &Vec<u8>, index: usize) {
    let b0 = bytes[index];
    let b1 = bytes[index + 1];
    // println!("{:02x} {:02x}", b0, b1);
    //println!("{:02x}", b1);
    // let opcode = decode(b0 >> 4);
    let opcode = decode(b0, b1);
    println!("{}", opcode);
}

fn decode(b0: u8, b1: u8) -> &'static str {
    let opcode = b0 >> 4;
    let arg0 = b0 & 0x0F;
    let arg1 = b1 >> 4;
    let arg2 = b1 & 0x0F;
    // println!("{} {:01x} {:01x} {:01x} ", opcode, arg0, arg1, arg2);
    if opcode == 0 { 
        if arg0 == 0 && arg1 == 0xE {
            if arg2 == 0 { return "CLS" }
            else if arg2 == 0xE { return "RET" }
        }
        return "SYS" 
    }
    else if opcode == 1 { return "JP" }
    else if opcode == 2 { 
        let nnn = ((b0 as u16) << 12) | b1 as u16;
        print!("{:03x} ", nnn);
        return "CALL" 
    }
    else if opcode == 3 { return "SE" }
    else if opcode == 4 { return "SNE" }
    else if opcode == 5 { return "SE" }
    else if opcode == 6 { return "LD" }
    else if opcode == 7 { return "ADD" }
    else if opcode == 8 { 
        if arg2 == 0 { 
            print!("V{:01x} = V{:01x} ", arg1, arg2);
            return "LD" 
        }
        else if arg2 == 1 { return "OR" }
        else if arg2 == 2 { return "AND" }
        else if arg2 == 3 { return "XOR" }
        else if arg2 == 4 { return "ADD" }
        else if arg2 == 5 { return "SUB" }
        else if arg2 == 6 { return "SHR" }
        else if arg2 == 7 { return "SUBN" }
        else if arg2 == 0xE { return "SHL" }
        return "MATH" 
    }
    else if opcode == 9 { return "SNE" }
    else if opcode == 0xA { return "LD I," }
    else if opcode == 0xB { return "JP" }
    else if opcode == 0xC { return "RND" }
    else if opcode == 0xD { return "DRW" }
    else if opcode == 0xE { return "SKP" }
    else if opcode == 0xF { return "EXTLD" }
    else { return "??" }
}


fn decode_op(opcode: u8) -> &'static str {
    if opcode == 0 { return "SYS" }
    else if opcode == 1 { return "JP" }
    else if opcode == 2 { return "CALL" }
    else if opcode == 3 { return "SE" }
    else if opcode == 4 { return "SNE" }
    else if opcode == 5 { return "SE" }
    else if opcode == 6 { return "LD" }
    else if opcode == 7 { return "MATH" }
    else if opcode == 8 { return "LD" }
    else if opcode == 9 { return "SNE" }
    else if opcode == 0xA { return "LD" }
    else if opcode == 0xB { return "JP" }
    else if opcode == 0xC { return "RND" }
    else if opcode == 0xD { return "DRW" }
    else if opcode == 0xE { return "SKP" }
    else if opcode == 0xF { return "EXTLD" }
    else { return "??" }
}

