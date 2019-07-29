#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC mode register"]
    pub mode: MODE,
    #[doc = "0x04 - CRC seed register"]
    pub seed: SEED,
    #[doc = "CRC data register CRC checksum register"]
    pub sum: SUM_UNION,
}
#[doc = "CRC data register CRC checksum register"]
#[repr(C)]
pub union SUM_UNION {
    #[doc = "0x08 - CRC data register"]
    pub wr_data: WR_DATA,
    #[doc = "0x08 - CRC checksum register"]
    pub sum: SUM,
}
#[doc = "CRC mode register"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC mode register"]
pub mod mode;
#[doc = "CRC seed register"]
pub struct SEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC seed register"]
pub mod seed;
#[doc = "CRC checksum register"]
pub struct SUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC checksum register"]
pub mod sum;
#[doc = "CRC data register"]
pub struct WR_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC data register"]
pub mod wr_data;
