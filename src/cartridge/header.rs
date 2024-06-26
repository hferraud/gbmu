pub const HEADER_OFFSET: usize = 0x100;
pub const HEADER_END: usize = 0x14F;

pub const ENTRY_POINT_OFFSET: u16 = 0x0;
pub const ENTRY_POINT_WIDTH: usize = 0x4;
pub const NINTENDO_LOGO_OFFSET: u16 = 0x4;
pub const NINTENDO_LOGO_WIDTH: usize = 0x30;
pub const TITLE_OFFSET: u16 = 0x34;
pub const TITLE_WIDTH: usize = 0x10;
pub const MANUFACTURER_CODE_OFFSET: u16 = 0x3F;
pub const MANUFACTURER_CODE_WIDTH: usize = 0x1;
pub const CGB_FLAG_OFFSET: u16 = 0x43;
pub const CGB_FLAG_WIDTH: usize = 0x1;
pub const NEW_LICENSEE_CODE_OFFSET: u16 = 0x44;
pub const NEW_LICENSEE_CODE_WIDTH: usize = 0x2;
pub const SGB_FLAG_OFFSET: u16 = 0x46;
pub const SGB_FLAG_WIDTH: usize = 0x1;
pub const CARTRIDGE_TYPE_OFFSET: u16 = 0x47;
pub const CARTRIDGE_TYPE_WIDTH: usize = 0x1;
pub const ROM_SIZE_OFFSET: u16 = 0x48;
pub const ROM_SIZE_WIDTH: usize = 0x1;
pub const RAM_SIZE_OFFSET: u16 = 0x49;
pub const RAM_SIZE_WIDTH: usize = 0x1;
pub const DESTINATION_CODE_OFFSET: u16 = 0x4A;
pub const DESTINATION_CODE_WIDTH: usize = 0x1;
pub const OLD_LICENSEE_CODE_OFFSET: u16 = 0x4B;
pub const OLD_LICENSEE_CODE_WIDTH: usize = 0x1;
pub const MASK_ROM_VERSION_OFFSET: u16 = 0x4C;
pub const MASK_ROM_VERSION_WIDTH: usize = 0x1;
pub const HEADER_CHECKSUM_OFFSET: u16 = 0x4D;
pub const HEADER_CHECKSUM_WIDTH: usize = 0x1;
pub const GLOBAL_CHECKSUM_OFFSET: u16 = 0x4E;
pub const GLOBAL_CHECKSUM_WIDTH: usize = 0x2;

#[repr(C)]
#[derive(Debug)]
pub struct CartridgeHeader {
    pub entry_point: [u8; ENTRY_POINT_WIDTH],
    pub nintendo_logo: [u8; NINTENDO_LOGO_WIDTH],
    pub title: [u8; TITLE_WIDTH],
    pub new_licensee_code: [u8; NEW_LICENSEE_CODE_WIDTH],
    pub sgb_flag: [u8; SGB_FLAG_WIDTH],
    pub cartridge_type: [u8; CARTRIDGE_TYPE_WIDTH],
    pub rom_size: [u8; ROM_SIZE_WIDTH],
    pub ram_size: [u8; RAM_SIZE_WIDTH],
    pub destination_code: [u8; DESTINATION_CODE_WIDTH],
    pub old_licensee_code: [u8; OLD_LICENSEE_CODE_WIDTH],
    pub mask_rom_version: [u8; MASK_ROM_VERSION_WIDTH],
    pub header_checksum: [u8; HEADER_CHECKSUM_WIDTH],
    pub global_checksum: [u8; GLOBAL_CHECKSUM_WIDTH],
}

impl CartridgeHeader {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::io::Error> {
        if bytes.len() < std::mem::size_of::<Self>() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Insufficient bytes to parse the cartridge header",
            ));
        }
        let ptr = bytes.as_ptr() as *const Self;
        let cartridge_header = unsafe { ptr.read() };
        Ok(cartridge_header)
    }
}
