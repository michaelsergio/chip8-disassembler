use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

fn parse_hex(arg: &str) -> usize {
        if arg.starts_with("0x") {
            let hex = &arg[2..];
            usize::from_str_radix(hex, 16).unwrap_or(0)
        } else {
            arg.parse().unwrap_or(0 as usize)
        }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "chip8-disasm")]
struct Opt {

    #[structopt(short = "j", long = "skip", default_value = "0", parse(from_str = parse_hex))]
    skip: usize,

    // choices d|o|x|n
    // #[structopt(short = "A", long = "base", parse(try_from_string = parse_base_option))]
    // address: char,

    #[structopt(short = "N", long = "length", default_value = "16")]
    length: u32,

    // #[structopt(short = "+", long = "offset")]
    // offset: u32,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

// Match args for od
// -j XXX skip
// -A X address base d|o|x|n
// -N XXX go up to length xxx
// + offset 

fn main() {
    let opt: Opt = Opt::from_args();
    //println!("{:#?}", opt);

    // check if file exist
    if !opt.file.is_file() {
        println!("Not a valid file");
        return;
    }

    println!("decoding {}", opt.file.to_str().unwrap_or("error"));
    let bytes = fs::read(opt.file).unwrap();

    let should_print_address = true;
    let address_offset = 0x200;

    for i in 0..opt.length {

        let read_addr = opt.skip + (i as usize * 2);
        if read_addr >= bytes.len() {
            return
        }

        if should_print_address {
            // print!("{:03x} ", (i * 4) + address_offset + skip);
            print!("{:03x} ", address_offset + read_addr);
        }
        read_pair(&bytes, read_addr);
    }
}

fn read_pair(bytes: &Vec<u8>, index: usize) {
    let b0 = bytes[index];
    let b1 = bytes[index + 1];
    // println!("{:02x} {:02x}", b0, b1);
    //println!("{:02x}", b1);
    // let opcode = decode(b0 >> 4);
    let opcode = decode(b0, b1);
    let should_show_ascii = true;
    if should_show_ascii 
        && (b0 == b' ' || b0.is_ascii_alphanumeric()) 
        && (b1 == b' ' || b1.is_ascii_alphanumeric()) 
    {
        println!("{} \"{}{}\"", opcode, b0 as char, b1 as char);
    }
    else {
        println!("{}", opcode);
    }
}

fn decode(b0: u8, b1: u8) -> String {
    // b1 is also known as kk
    let opcode = b0 >> 4;
    let arg0 = b0 & 0x0F;
    let arg1 = b1 >> 4;
    let arg2 = b1 & 0x0F;
    let x = b0 & 0x0F;
    let y = b1 >> 4;
    let n = b1 & 0x0F;
    // println!("{} {:01x} {:01x} {:01x} ", opcode, arg0, arg1, arg2);
    if opcode == 0 { 
        if arg0 == 0 && arg1 == 0xE {
            if arg2 == 0 { return String::from("CLS") }
            else if arg2 == 0xE { return String::from("RET") }
        }
        return String::from("SYS???") 
    }
    else if opcode == 1 { return format!("JP {:#X}", arg3(b0, b1)) }
    else if opcode == 2 { return format!("CALL {:#X}", arg3(b0, b1)) }
    else if opcode == 3 { return format!("SE V{:X} == {:#X} ({})", x, b1, b1) }
    else if opcode == 4 { return format!("SNE V{:X}, {:#X} ({})", x, b1, b1) }
    else if opcode == 5 { return format!("SE V{:X}, V{:X}", x, y) }
    else if opcode == 6 { return format!("LD V{:X}, {:#X} ({})", x, b1, b1) }
    else if opcode == 7 { return format!("ADD V{:X}, {:#X} ({})", x, b1, b1) }
    else if opcode == 8 { 
        if arg2 == 0 { return format!("LD V{:X} = V{:X} ", x, y); }
        else if arg2 == 1 { return format!("OR, V{:X}, V{:X}", x, y) }
        else if arg2 == 2 { return format!("AND, V{:X} V{:X}", x, y) }
        else if arg2 == 3 { return format!("XOR, V{:X} V{:X}", x, y) }
        else if arg2 == 4 { return format!("ADD, V{:X} V{:X}", x, y) }
        else if arg2 == 5 { return format!("SUB, V{:X} V{:X}", x, y) }
        else if arg2 == 6 { return format!("SHR, V{:X} >> 1", x) }
        else if arg2 == 7 { return format!("SUBN, V{:X} V{:X}", x, y) }
        else if arg2 == 0xE { return format!("SHL, V{:X} << 1", x) }
        return String::from("MATH???") 
    }
    else if opcode == 9 { return format!("SNE V{:X}, V{:X}", x, y) }
    else if opcode == 0xA { return format!("LD I, {:#X}", arg3(b0, b1)) }
    else if opcode == 0xB { return format!("JP V0, {:#X}", arg3(b0, b1)) }
    else if opcode == 0xC { return format!("RND V{:X}, RND & {:#X}", x, b1) }
    else if opcode == 0xD { return format!("DRW V{:X} V{:X} {:X}", x, y, n) }
    else if opcode == 0xE { return format!("SKP V{:X}", x) }
    else if opcode == 0xF { 
        if b1 == 0x07 { return format!("LD V{:X}, DT", x) }
        if b1 == 0x0A { return format!("LD V{:X}, KEY", x) }
        if b1 == 0x15 { return format!("LD DT, V{:X}", x) }
        if b1 == 0x18 { return format!("LD, ST, V{:X}", x) }
        if b1 == 0x1E { return format!("ADD I, V{:X}", x) }
        if b1 == 0x29 { return format!("LD F, V{:X}", x) }
        if b1 == 0x33 { return format!("LD BCD, V{:X}", x) }
        if b1 == 0x55 { return format!("LD [I], V{:X}", x) }
        if b1 == 0x65 { return format!("LD V{:X}, [I]", x) }
        else { return String::from("??")}
    }
    else { return String::from("??") }
}

// Used for getting back nnn of Xnnn
fn arg3(b0: u8, b1: u8) -> u16 {
        return (((b0 & 0x0F) as u16) << 8) | b1 as u16;
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

