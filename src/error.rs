use std::io;

pub fn unsupported_instruction() -> io::Error {
    io::Error::new(io::ErrorKind::Unsupported, "Instruction is unsupported")
}

pub fn invalid_r8_code() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Invalid r8 code")
}

pub fn invalid_r16_code() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Invalid r16 code")
}

pub fn invalid_instruction_type() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Invalid instruction type")
}

pub fn invalid_condition_code() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Invalid condition code")
}

pub fn invalid_flag() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Invalid flag")
}

pub fn invalid_opcode() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Invalid opcode")
}

pub fn invalid_address() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "Invalid address")
}
