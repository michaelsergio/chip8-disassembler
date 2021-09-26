pub fn decode(b0: u8, b1: u8) -> String {
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

/*
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
*/
