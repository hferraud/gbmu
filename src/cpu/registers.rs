use crate::error;
use std::io;

pub const B_REGISTER_CODE: u8 = 0x0;
pub const C_REGISTER_CODE: u8 = 0x1;
pub const D_REGISTER_CODE: u8 = 0x2;
pub const E_REGISTER_CODE: u8 = 0x3;
pub const H_REGISTER_CODE: u8 = 0x4;
pub const L_REGISTER_CODE: u8 = 0x5;
pub const HL_MEM_REGISTER_CODE: u8 = 0x6;
pub const A_REGISTER_CODE: u8 = 0x7;
pub const BC_REGISTER_CODE: u8 = 0x0;
pub const DE_REGISTER_CODE: u8 = 0x1;
pub const HL_REGISTER_CODE: u8 = 0x2;
pub const SP_REGISTER_CODE: u8 = 0x3;
pub const BC_MEM_REGISTER_CODE: u8 = 0x0;
pub const DE_MEM_REGISTER_CODE: u8 = 0x1;
pub const HL_INC_REGISTER_CODE: u8 = 0x2;
pub const HL_DEC_REGISTER_CODE: u8 = 0x3;

pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pub pc: u16,
    sp: u16,
}

pub enum Flags {
    Z = 0b10000000,
    N = 0b01000000,
    H = 0b00100000,
    C = 0b00010000,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value & 0xff00 >> 8) as u8;
        self.c = value as u8;
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value & 0xff00 >> 8) as u8;
        self.e = value as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value & 0xff00 >> 8) as u8;
        self.l = value as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_flags(&mut self, flag: Flags, value: bool) {
        match value {
            true => self.f |= flag as u8,
            false => self.f &= flag as u8,
        }
    }

    pub fn reset_flags(&mut self) {
        self.f = 0;
    }
    pub fn get_flag(&self, flag: Flags) -> bool {
        self.f & (flag as u8) != 0
    }

    pub fn get_word(&self, r8_code: u8) -> Result<u8, io::Error> {
        match r8_code {
            A_REGISTER_CODE => Ok(self.a),
            B_REGISTER_CODE => Ok(self.b),
            C_REGISTER_CODE => Ok(self.c),
            D_REGISTER_CODE => Ok(self.d),
            E_REGISTER_CODE => Ok(self.e),
            H_REGISTER_CODE => Ok(self.h),
            L_REGISTER_CODE => Ok(self.l),
            _ => Err(error::invalid_r8_code()),
        }
    }

    pub fn get_dword(&self, r16_code: u8) -> Result<u16, io::Error> {
        match r16_code {
            BC_REGISTER_CODE => Ok(self.get_bc()),
            HL_REGISTER_CODE => Ok(self.get_hl()),
            DE_REGISTER_CODE => Ok(self.get_de()),
            _ => Err(error::invalid_r16_code()),
        }
    }

    pub fn set_word(&mut self, binary_register: u8, value: u8) -> Result<(), io::Error> {
        match binary_register {
            A_REGISTER_CODE => self.a = value,
            B_REGISTER_CODE => self.b = value,
            C_REGISTER_CODE => self.c = value,
            D_REGISTER_CODE => self.d = value,
            E_REGISTER_CODE => self.e = value,
            H_REGISTER_CODE => self.h = value,
            L_REGISTER_CODE => self.l = value,
            _ => return Err(error::invalid_r8_code()),
        }
        Ok(())
    }

    pub fn set_dword(&mut self, binary_register: u8, value: u16) -> Result<(), io::Error> {
        match binary_register {
            BC_REGISTER_CODE => self.set_bc(value),
            HL_REGISTER_CODE => self.set_hl(value),
            DE_REGISTER_CODE => self.set_de(value),
            _ => return Err(error::invalid_r16_code()),
        }
        Ok(())
    }
}
