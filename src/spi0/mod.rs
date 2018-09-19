#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Configuration register"]
    pub cfg: CFG,
    #[doc = "0x04 - SPI Delay register"]
    pub dly: DLY,
    #[doc = "0x08 - SPI Status. Some status flags can be cleared by writing a 1 to that bit position"]
    pub stat: STAT,
    #[doc = "0x0c - SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    pub intenset: INTENSET,
    #[doc = "0x10 - SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
    pub intenclr: INTENCLR,
    #[doc = "0x14 - SPI Receive Data"]
    pub rxdat: RXDAT,
    #[doc = "0x18 - SPI Transmit Data with Control"]
    pub txdatctl: TXDATCTL,
    #[doc = "0x1c - SPI Transmit Data"]
    pub txdat: TXDAT,
    #[doc = "0x20 - SPI Transmit Control"]
    pub txctl: TXCTL,
    #[doc = "0x24 - SPI clock Divider"]
    pub div: DIV,
    #[doc = "0x28 - SPI Interrupt Status"]
    pub intstat: INTSTAT,
}
#[doc = "SPI Configuration register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Configuration register"]
pub mod cfg;
#[doc = "SPI Delay register"]
pub struct DLY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Delay register"]
pub mod dly;
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position"]
pub mod stat;
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub mod intenset;
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
pub mod intenclr;
#[doc = "SPI Receive Data"]
pub struct RXDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Receive Data"]
pub mod rxdat;
#[doc = "SPI Transmit Data with Control"]
pub struct TXDATCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Transmit Data with Control"]
pub mod txdatctl;
#[doc = "SPI Transmit Data"]
pub struct TXDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Transmit Data"]
pub mod txdat;
#[doc = "SPI Transmit Control"]
pub struct TXCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Transmit Control"]
pub mod txctl;
#[doc = "SPI clock Divider"]
pub struct DIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI clock Divider"]
pub mod div;
#[doc = "SPI Interrupt Status"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Interrupt Status"]
pub mod intstat;
