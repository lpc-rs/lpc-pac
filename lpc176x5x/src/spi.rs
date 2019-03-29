#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register. This register controls the operation of the SPI."]
    pub cr: CR,
    #[doc = "0x04 - SPI Status Register. This register shows the status of the SPI."]
    pub sr: SR,
    #[doc = "0x08 - SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register."]
    pub dr: DR,
    #[doc = "0x0c - SPI Clock Counter Register. This register controls the frequency of a master's SCK0."]
    pub ccr: CCR,
    _reserved4: [u8; 12usize],
    #[doc = "0x1c - SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface."]
    pub int: INT,
}
#[doc = "SPI Control Register. This register controls the operation of the SPI."]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Control Register. This register controls the operation of the SPI."]
pub mod cr;
#[doc = "SPI Status Register. This register shows the status of the SPI."]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Status Register. This register shows the status of the SPI."]
pub mod sr;
#[doc = "SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register."]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register."]
pub mod dr;
#[doc = "SPI Clock Counter Register. This register controls the frequency of a master's SCK0."]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Clock Counter Register. This register controls the frequency of a master's SCK0."]
pub mod ccr;
#[doc = "SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface."]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface."]
pub mod int;
