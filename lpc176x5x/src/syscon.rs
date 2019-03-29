#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Accelerator Configuration Register. Controls flash access timing."]
    pub flashcfg: FLASHCFG,
    _reserved1: [u8; 124usize],
    #[doc = "0x80 - PLL0 Control Register"]
    pub pll0con: PLL0CON,
    #[doc = "0x84 - PLL0 Configuration Register"]
    pub pll0cfg: PLL0CFG,
    #[doc = "0x88 - PLL0 Status Register"]
    pub pll0stat: PLL0STAT,
    #[doc = "0x8c - PLL0 Feed Register"]
    pub pll0feed: PLL0FEED,
    _reserved5: [u8; 16usize],
    #[doc = "0xa0 - PLL1 Control Register"]
    pub pll1con: PLL1CON,
    #[doc = "0xa4 - PLL1 Configuration Register"]
    pub pll1cfg: PLL1CFG,
    #[doc = "0xa8 - PLL1 Status Register"]
    pub pll1stat: PLL1STAT,
    #[doc = "0xac - PLL1 Feed Register"]
    pub pll1feed: PLL1FEED,
    _reserved9: [u8; 16usize],
    #[doc = "0xc0 - Power Control Register"]
    pub pcon: PCON,
    #[doc = "0xc4 - Power Control for Peripherals Register"]
    pub pconp: PCONP,
    _reserved11: [u8; 60usize],
    #[doc = "0x104 - CPU Clock Configuration Register"]
    pub cclkcfg: CCLKCFG,
    #[doc = "0x108 - USB Clock Configuration Register"]
    pub usbclkcfg: USBCLKCFG,
    #[doc = "0x10c - Clock Source Select Register"]
    pub clksrcsel: CLKSRCSEL,
    #[doc = "0x110 - Allows clearing the current CAN channel sleep state as well as reading that state."]
    pub cansleepclr: CANSLEEPCLR,
    #[doc = "0x114 - Allows reading the wake-up state of the CAN channels."]
    pub canwakeflags: CANWAKEFLAGS,
    _reserved16: [u8; 40usize],
    #[doc = "0x140 - External Interrupt Flag Register"]
    pub extint: EXTINT,
    _reserved17: [u8; 4usize],
    #[doc = "0x148 - External Interrupt Mode register"]
    pub extmode: EXTMODE,
    #[doc = "0x14c - External Interrupt Polarity Register"]
    pub extpolar: EXTPOLAR,
    _reserved19: [u8; 48usize],
    #[doc = "0x180 - Reset Source Identification Register"]
    pub rsid: RSID,
    _reserved20: [u8; 28usize],
    #[doc = "0x1a0 - System control and status"]
    pub scs: SCS,
    _reserved21: [u8; 4usize],
    #[doc = "0x1a8 - Peripheral Clock Selection register 0."]
    pub pclksel0: PCLKSEL0,
    #[doc = "0x1ac - Peripheral Clock Selection register 1."]
    pub pclksel1: PCLKSEL1,
    _reserved23: [u8; 16usize],
    #[doc = "0x1c0 - USB Interrupt Status"]
    pub usbintst: USBINTST,
    #[doc = "0x1c4 - Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
    pub dmacreqsel: DMACREQSEL,
    #[doc = "0x1c8 - Clock Output Configuration Register"]
    pub clkoutcfg: CLKOUTCFG,
}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing."]
pub struct FLASHCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing."]
pub mod flashcfg;
#[doc = "PLL0 Control Register"]
pub struct PLL0CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Control Register"]
pub mod pll0con;
#[doc = "PLL0 Configuration Register"]
pub struct PLL0CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Configuration Register"]
pub mod pll0cfg;
#[doc = "PLL0 Status Register"]
pub struct PLL0STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Status Register"]
pub mod pll0stat;
#[doc = "PLL0 Feed Register"]
pub struct PLL0FEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL0 Feed Register"]
pub mod pll0feed;
#[doc = "PLL1 Control Register"]
pub struct PLL1CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 Control Register"]
pub mod pll1con;
#[doc = "PLL1 Configuration Register"]
pub struct PLL1CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 Configuration Register"]
pub mod pll1cfg;
#[doc = "PLL1 Status Register"]
pub struct PLL1STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 Status Register"]
pub mod pll1stat;
#[doc = "PLL1 Feed Register"]
pub struct PLL1FEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL1 Feed Register"]
pub mod pll1feed;
#[doc = "Power Control Register"]
pub struct PCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Control Register"]
pub mod pcon;
#[doc = "Power Control for Peripherals Register"]
pub struct PCONP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Control for Peripherals Register"]
pub mod pconp;
#[doc = "CPU Clock Configuration Register"]
pub struct CCLKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Clock Configuration Register"]
pub mod cclkcfg;
#[doc = "USB Clock Configuration Register"]
pub struct USBCLKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Clock Configuration Register"]
pub mod usbclkcfg;
#[doc = "Clock Source Select Register"]
pub struct CLKSRCSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Source Select Register"]
pub mod clksrcsel;
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state."]
pub struct CANSLEEPCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state."]
pub mod cansleepclr;
#[doc = "Allows reading the wake-up state of the CAN channels."]
pub struct CANWAKEFLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Allows reading the wake-up state of the CAN channels."]
pub mod canwakeflags;
#[doc = "External Interrupt Flag Register"]
pub struct EXTINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Flag Register"]
pub mod extint;
#[doc = "External Interrupt Mode register"]
pub struct EXTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Mode register"]
pub mod extmode;
#[doc = "External Interrupt Polarity Register"]
pub struct EXTPOLAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Interrupt Polarity Register"]
pub mod extpolar;
#[doc = "Reset Source Identification Register"]
pub struct RSID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Source Identification Register"]
pub mod rsid;
#[doc = "System control and status"]
pub struct SCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System control and status"]
pub mod scs;
#[doc = "Peripheral Clock Selection register 0."]
pub struct PCLKSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Selection register 0."]
pub mod pclksel0;
#[doc = "Peripheral Clock Selection register 1."]
pub struct PCLKSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Selection register 1."]
pub mod pclksel1;
#[doc = "USB Interrupt Status"]
pub struct USBINTST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Interrupt Status"]
pub mod usbintst;
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
pub struct DMACREQSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
pub mod dmacreqsel;
#[doc = "Clock Output Configuration Register"]
pub struct CLKOUTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Output Configuration Register"]
pub mod clkoutcfg;
