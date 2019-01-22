#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin function select register 0."]
    pub pinsel0: PINSEL0,
    #[doc = "0x04 - Pin function select register 1."]
    pub pinsel1: PINSEL1,
    #[doc = "0x08 - Pin function select register 2."]
    pub pinsel2: PINSEL2,
    #[doc = "0x0c - Pin function select register 3."]
    pub pinsel3: PINSEL3,
    #[doc = "0x10 - Pin function select register 4"]
    pub pinsel4: PINSEL4,
    _reserved5: [u8; 8usize],
    #[doc = "0x1c - Pin function select register 7"]
    pub pinsel7: PINSEL7,
    _reserved6: [u8; 4usize],
    #[doc = "0x24 - Pin function select register 9"]
    pub pinsel9: PINSEL9,
    #[doc = "0x28 - Pin function select register 10"]
    pub pinsel10: PINSEL10,
    _reserved8: [u8; 20usize],
    #[doc = "0x40 - Pin mode select register 0"]
    pub pinmode0: PINMODE0,
    #[doc = "0x44 - Pin mode select register 1"]
    pub pinmode1: PINMODE1,
    #[doc = "0x48 - Pin mode select register 2"]
    pub pinmode2: PINMODE2,
    #[doc = "0x4c - Pin mode select register 3."]
    pub pinmode3: PINMODE3,
    #[doc = "0x50 - Pin mode select register 4"]
    pub pinmode4: PINMODE4,
    _reserved13: [u8; 8usize],
    #[doc = "0x5c - Pin mode select register 7"]
    pub pinmode7: PINMODE7,
    _reserved14: [u8; 4usize],
    #[doc = "0x64 - Pin mode select register 9"]
    pub pinmode9: PINMODE9,
    #[doc = "0x68 - Open drain mode control register 0"]
    pub pinmode_od0: PINMODE_OD0,
    #[doc = "0x6c - Open drain mode control register 1"]
    pub pinmode_od1: PINMODE_OD1,
    #[doc = "0x70 - Open drain mode control register 2"]
    pub pinmode_od2: PINMODE_OD2,
    #[doc = "0x74 - Open drain mode control register 3"]
    pub pinmode_od3: PINMODE_OD3,
    #[doc = "0x78 - Open drain mode control register 4"]
    pub pinmode_od4: PINMODE_OD4,
    #[doc = "0x7c - I2C Pin Configuration register"]
    pub i2cpadcfg: I2CPADCFG,
}
#[doc = "Pin function select register 0."]
pub struct PINSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin function select register 0."]
pub mod pinsel0;
#[doc = "Pin function select register 1."]
pub struct PINSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin function select register 1."]
pub mod pinsel1;
#[doc = "Pin function select register 2."]
pub struct PINSEL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin function select register 2."]
pub mod pinsel2;
#[doc = "Pin function select register 3."]
pub struct PINSEL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin function select register 3."]
pub mod pinsel3;
#[doc = "Pin function select register 4"]
pub struct PINSEL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin function select register 4"]
pub mod pinsel4;
#[doc = "Pin function select register 7"]
pub struct PINSEL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin function select register 7"]
pub mod pinsel7;
#[doc = "Pin function select register 9"]
pub struct PINSEL9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin function select register 9"]
pub mod pinsel9;
#[doc = "Pin function select register 10"]
pub struct PINSEL10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin function select register 10"]
pub mod pinsel10;
#[doc = "Pin mode select register 0"]
pub struct PINMODE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin mode select register 0"]
pub mod pinmode0;
#[doc = "Pin mode select register 1"]
pub struct PINMODE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin mode select register 1"]
pub mod pinmode1;
#[doc = "Pin mode select register 2"]
pub struct PINMODE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin mode select register 2"]
pub mod pinmode2;
#[doc = "Pin mode select register 3."]
pub struct PINMODE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin mode select register 3."]
pub mod pinmode3;
#[doc = "Pin mode select register 4"]
pub struct PINMODE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin mode select register 4"]
pub mod pinmode4;
#[doc = "Pin mode select register 7"]
pub struct PINMODE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin mode select register 7"]
pub mod pinmode7;
#[doc = "Pin mode select register 9"]
pub struct PINMODE9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin mode select register 9"]
pub mod pinmode9;
#[doc = "Open drain mode control register 0"]
pub struct PINMODE_OD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open drain mode control register 0"]
pub mod pinmode_od0;
#[doc = "Open drain mode control register 1"]
pub struct PINMODE_OD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open drain mode control register 1"]
pub mod pinmode_od1;
#[doc = "Open drain mode control register 2"]
pub struct PINMODE_OD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open drain mode control register 2"]
pub mod pinmode_od2;
#[doc = "Open drain mode control register 3"]
pub struct PINMODE_OD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open drain mode control register 3"]
pub mod pinmode_od3;
#[doc = "Open drain mode control register 4"]
pub struct PINMODE_OD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Open drain mode control register 4"]
pub mod pinmode_od4;
#[doc = "I2C Pin Configuration register"]
pub struct I2CPADCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Pin Configuration register"]
pub mod i2cpadcfg;
