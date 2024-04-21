const B_REGISTER_CODE: u8 = 0x0;
const C_REGISTER_CODE: u8 = 0x1;
const D_REGISTER_CODE: u8 = 0x2;
const E_REGISTER_CODE: u8 = 0x3;
const H_REGISTER_CODE: u8 = 0x4;
const L_REGISTER_CODE: u8 = 0x5;
const HL_REGISTER_CODE: u8 = 0x6;
const A_REGISTER_CODE: u8 = 0x7;

pub struct Registers {
    pub a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
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
        (self.b as u16) <<  8 | self.c as u16
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

    pub fn get_flag(&self, flag: Flags) -> bool {
        self.f & (flag as u8) != 0
    }

    pub fn get_register_value(&self, binary_register: u8) -> Option<u8> {
        match binary_register {
            A_REGISTER_CODE => Some(self.a),
            B_REGISTER_CODE => Some(self.b),
            C_REGISTER_CODE => Some(self.c),
            D_REGISTER_CODE => Some(self.d),
            E_REGISTER_CODE => Some(self.e),
            HL_REGISTER_CODE => Some(self.h),
            L_REGISTER_CODE => Some(self.l),
            _ => None
            // HL_REGISTER_CODE =>
        }
    }
}
