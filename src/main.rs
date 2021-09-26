use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use c8_disasm_lib::decode;

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

    let should_show_ascii = true;
    let should_print_address = true;
    let address_offset = 0x200;

    for i in 0..opt.length {

        let read_addr = opt.skip + (i as usize * 2);
        if read_addr >= bytes.len() { return }

        if should_print_address {
            print!("{:03x} ", address_offset + read_addr);
        }

        let b0 = bytes[read_addr];
        let b1 = bytes[read_addr + 1];
        decode_print_byte(b0, b1, should_show_ascii);
    }

    fn decode_print_byte(b0: u8, b1: u8, should_show_ascii: bool) {
        let opcode = decode(b0, b1);

        let b0_printable = b0 == b' ' || b0.is_ascii_alphanumeric();
        let b1_printable = b1 == b' ' || b1.is_ascii_alphanumeric();
        if should_show_ascii && b0_printable && b1_printable {
            println!("{} \"{}{}\"", opcode, b0 as char, b1 as char);
        } else {
            println!("{}", opcode);
        }
    }
}

