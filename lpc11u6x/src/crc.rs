#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC mode register"]
    pub mode: MODE,
    #[doc = "0x04 - CRC seed register"]
    pub seed: SEED,
    _reserved_2_sum: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x08 - CRC data register"]
    #[inline(always)]
    pub fn wr_data(&self) -> &WR_DATA {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const WR_DATA) }
    }
    #[doc = "0x08 - CRC data register"]
    #[inline(always)]
    pub fn wr_data_mut(&self) -> &mut WR_DATA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut WR_DATA) }
    }
    #[doc = "0x08 - CRC checksum register"]
    #[inline(always)]
    pub fn sum(&self) -> &SUM {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const SUM) }
    }
    #[doc = "0x08 - CRC checksum register"]
    #[inline(always)]
    pub fn sum_mut(&self) -> &mut SUM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut SUM) }
    }
}
#[doc = "CRC mode register"]
pub struct MODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CRC mode register"]
pub mod mode;
#[doc = "CRC seed register"]
pub struct SEED {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CRC seed register"]
pub mod seed;
#[doc = "CRC checksum register"]
pub struct SUM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CRC checksum register"]
pub mod sum;
#[doc = "CRC data register"]
pub struct WR_DATA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CRC data register"]
pub mod wr_data;
