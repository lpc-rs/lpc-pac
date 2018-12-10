#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
    pub cfg: CFG,
    #[doc = "0x04 - USART Control register. USART control settings that are more likely to change during operation."]
    pub ctl: CTL,
    #[doc = "0x08 - USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
    pub stat: STAT,
    #[doc = "0x0c - Interrupt Enable read and Set register. Contains an individual interrupt enable bit for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    pub intenset: INTENSET,
    #[doc = "0x10 - Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
    pub intenclr: INTENCLR,
    #[doc = "0x14 - Receiver Data register. Contains the last character received."]
    pub rxdat: RXDAT,
    #[doc = "0x18 - Receiver Data with Status register. Combines the last character received with the current USART receive status. Allows DMA or software to recover incoming data and status together."]
    pub rxdatstat: RXDATSTAT,
    #[doc = "0x1c - Transmit Data register. Data to be transmitted is written here."]
    pub txdat: TXDAT,
    #[doc = "0x20 - Baud Rate Generator register. 16-bit integer baud rate divisor value."]
    pub brg: BRG,
    #[doc = "0x24 - Interrupt status register. Reflects interrupts that are currently enabled."]
    pub intstat: INTSTAT,
    #[doc = "0x28 - Oversample selection register for asynchronous communication."]
    pub osr: OSR,
    #[doc = "0x2c - Address register for automatic address matching."]
    pub addr: ADDR,
}
#[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
pub mod cfg;
#[doc = "USART Control register. USART control settings that are more likely to change during operation."]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Control register. USART control settings that are more likely to change during operation."]
pub mod ctl;
#[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
pub mod stat;
#[doc = "Interrupt Enable read and Set register. Contains an individual interrupt enable bit for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable read and Set register. Contains an individual interrupt enable bit for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub mod intenset;
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
pub mod intenclr;
#[doc = "Receiver Data register. Contains the last character received."]
pub struct RXDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Data register. Contains the last character received."]
pub mod rxdat;
#[doc = "Receiver Data with Status register. Combines the last character received with the current USART receive status. Allows DMA or software to recover incoming data and status together."]
pub struct RXDATSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Data with Status register. Combines the last character received with the current USART receive status. Allows DMA or software to recover incoming data and status together."]
pub mod rxdatstat;
#[doc = "Transmit Data register. Data to be transmitted is written here."]
pub struct TXDAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data register. Data to be transmitted is written here."]
pub mod txdat;
#[doc = "Baud Rate Generator register. 16-bit integer baud rate divisor value."]
pub struct BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Generator register. 16-bit integer baud rate divisor value."]
pub mod brg;
#[doc = "Interrupt status register. Reflects interrupts that are currently enabled."]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status register. Reflects interrupts that are currently enabled."]
pub mod intstat;
#[doc = "Oversample selection register for asynchronous communication."]
pub struct OSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oversample selection register for asynchronous communication."]
pub mod osr;
#[doc = "Address register for automatic address matching."]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Address register for automatic address matching."]
pub mod addr;
