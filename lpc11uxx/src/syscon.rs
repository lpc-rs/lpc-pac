#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System memory remap"]
    pub sysmemremap: SYSMEMREMAP,
    #[doc = "0x04 - Peripheral reset control"]
    pub presetctrl: PRESETCTRL,
    #[doc = "0x08 - System PLL control"]
    pub syspllctrl: SYSPLLCTRL,
    #[doc = "0x0c - System PLL status"]
    pub syspllstat: SYSPLLSTAT,
    #[doc = "0x10 - USB PLL control"]
    pub usbpllctrl: USBPLLCTRL,
    #[doc = "0x14 - USB PLL status"]
    pub usbpllstat: USBPLLSTAT,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - System oscillator control"]
    pub sysoscctrl: SYSOSCCTRL,
    #[doc = "0x24 - Watchdog oscillator control"]
    pub wdtoscctrl: WDTOSCCTRL,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - System reset status register"]
    pub sysrststat: SYSRSTSTAT,
    _reserved9: [u8; 12usize],
    #[doc = "0x40 - System PLL clock source select"]
    pub syspllclksel: SYSPLLCLKSEL,
    #[doc = "0x44 - System PLL clock source update enable"]
    pub syspllclkuen: SYSPLLCLKUEN,
    #[doc = "0x48 - USB PLL clock source select"]
    pub usbpllclksel: USBPLLCLKSEL,
    #[doc = "0x4c - USB PLL clock source update enable"]
    pub usbpllclkuen: USBPLLCLKUEN,
    _reserved13: [u8; 32usize],
    #[doc = "0x70 - Main clock source select"]
    pub mainclksel: MAINCLKSEL,
    #[doc = "0x74 - Main clock source update enable"]
    pub mainclkuen: MAINCLKUEN,
    #[doc = "0x78 - System clock divider"]
    pub sysahbclkdiv: SYSAHBCLKDIV,
    _reserved16: [u8; 4usize],
    #[doc = "0x80 - System clock control"]
    pub sysahbclkctrl: SYSAHBCLKCTRL,
    _reserved17: [u8; 16usize],
    #[doc = "0x94 - SSP0 clock divider"]
    pub ssp0clkdiv: SSP0CLKDIV,
    #[doc = "0x98 - UART clock divider"]
    pub uartclkdiv: UARTCLKDIV,
    #[doc = "0x9c - SSP1 clock divider"]
    pub ssp1clkdiv: SSP1CLKDIV,
    _reserved20: [u8; 32usize],
    #[doc = "0xc0 - USB clock source select"]
    pub usbclksel: USBCLKSEL,
    #[doc = "0xc4 - USB clock source update enable"]
    pub usbclkuen: USBCLKUEN,
    #[doc = "0xc8 - USB clock source divider"]
    pub usbclkdiv: USBCLKDIV,
    _reserved23: [u8; 20usize],
    #[doc = "0xe0 - CLKOUT clock source select"]
    pub clkoutsel: CLKOUTSEL,
    #[doc = "0xe4 - CLKOUT clock source update enable"]
    pub clkoutuen: CLKOUTUEN,
    #[doc = "0xe8 - CLKOUT clock divider"]
    pub clkoutdiv: CLKOUTDIV,
    _reserved26: [u8; 20usize],
    #[doc = "0x100 - POR captured PIO status 0"]
    pub pioporcap0: PIOPORCAP0,
    #[doc = "0x104 - POR captured PIO status 1"]
    pub pioporcap1: PIOPORCAP1,
    _reserved28: [u8; 72usize],
    #[doc = "0x150 - Brown-Out Detect"]
    pub bodctrl: BODCTRL,
    #[doc = "0x154 - System tick counter calibration"]
    pub systckcal: SYSTCKCAL,
    _reserved30: [u8; 24usize],
    #[doc = "0x170 - IQR delay. Allows trade-off between interrupt latency and determinism."]
    pub irqlatency: IRQLATENCY,
    #[doc = "0x174 - NMI Source Control"]
    pub nmisrc: NMISRC,
    #[doc = "0x178 - GPIO Pin Interrupt Select register 0"]
    pub pintsel: [PINTSEL; 8],
    #[doc = "0x198 - USB clock control"]
    pub usbclkctrl: USBCLKCTRL,
    #[doc = "0x19c - USB clock status"]
    pub usbclkst: USBCLKST,
    _reserved35: [u8; 100usize],
    #[doc = "0x204 - Start logic 0 interrupt wake-up enable register 0"]
    pub starterp0: STARTERP0,
    _reserved36: [u8; 12usize],
    #[doc = "0x214 - Start logic 1 interrupt wake-up enable register 1"]
    pub starterp1: STARTERP1,
    _reserved37: [u8; 24usize],
    #[doc = "0x230 - Power-down states in deep-sleep mode"]
    pub pdsleepcfg: PDSLEEPCFG,
    #[doc = "0x234 - Power-down states for wake-up from deep-sleep"]
    pub pdawakecfg: PDAWAKECFG,
    #[doc = "0x238 - Power configuration register"]
    pub pdruncfg: PDRUNCFG,
    _reserved40: [u8; 440usize],
    #[doc = "0x3f4 - Device ID"]
    pub device_id: DEVICE_ID,
}
#[doc = "System memory remap"]
pub struct SYSMEMREMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System memory remap"]
pub mod sysmemremap;
#[doc = "Peripheral reset control"]
pub struct PRESETCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control"]
pub mod presetctrl;
#[doc = "System PLL control"]
pub struct SYSPLLCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL control"]
pub mod syspllctrl;
#[doc = "System PLL status"]
pub struct SYSPLLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL status"]
pub mod syspllstat;
#[doc = "USB PLL control"]
pub struct USBPLLCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PLL control"]
pub mod usbpllctrl;
#[doc = "USB PLL status"]
pub struct USBPLLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PLL status"]
pub mod usbpllstat;
#[doc = "System oscillator control"]
pub struct SYSOSCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System oscillator control"]
pub mod sysoscctrl;
#[doc = "Watchdog oscillator control"]
pub struct WDTOSCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "System reset status register"]
pub struct SYSRSTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "System PLL clock source select"]
pub struct SYSPLLCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL clock source select"]
pub mod syspllclksel;
#[doc = "System PLL clock source update enable"]
pub struct SYSPLLCLKUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL clock source update enable"]
pub mod syspllclkuen;
#[doc = "USB PLL clock source select"]
pub struct USBPLLCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PLL clock source select"]
pub mod usbpllclksel;
#[doc = "USB PLL clock source update enable"]
pub struct USBPLLCLKUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PLL clock source update enable"]
pub mod usbpllclkuen;
#[doc = "Main clock source select"]
pub struct MAINCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main clock source select"]
pub mod mainclksel;
#[doc = "Main clock source update enable"]
pub struct MAINCLKUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main clock source update enable"]
pub mod mainclkuen;
#[doc = "System clock divider"]
pub struct SYSAHBCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System clock divider"]
pub mod sysahbclkdiv;
#[doc = "System clock control"]
pub struct SYSAHBCLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System clock control"]
pub mod sysahbclkctrl;
#[doc = "SSP0 clock divider"]
pub struct SSP0CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSP0 clock divider"]
pub mod ssp0clkdiv;
#[doc = "UART clock divider"]
pub struct UARTCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART clock divider"]
pub mod uartclkdiv;
#[doc = "SSP1 clock divider"]
pub struct SSP1CLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SSP1 clock divider"]
pub mod ssp1clkdiv;
#[doc = "USB clock source select"]
pub struct USBCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB clock source select"]
pub mod usbclksel;
#[doc = "USB clock source update enable"]
pub struct USBCLKUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB clock source update enable"]
pub mod usbclkuen;
#[doc = "USB clock source divider"]
pub struct USBCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB clock source divider"]
pub mod usbclkdiv;
#[doc = "CLKOUT clock source select"]
pub struct CLKOUTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock source select"]
pub mod clkoutsel;
#[doc = "CLKOUT clock source update enable"]
pub struct CLKOUTUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock source update enable"]
pub mod clkoutuen;
#[doc = "CLKOUT clock divider"]
pub struct CLKOUTDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "POR captured PIO status 0"]
pub struct PIOPORCAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POR captured PIO status 0"]
pub mod pioporcap0;
#[doc = "POR captured PIO status 1"]
pub struct PIOPORCAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POR captured PIO status 1"]
pub mod pioporcap1;
#[doc = "Brown-Out Detect"]
pub struct BODCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Brown-Out Detect"]
pub mod bodctrl;
#[doc = "System tick counter calibration"]
pub struct SYSTCKCAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System tick counter calibration"]
pub mod systckcal;
#[doc = "IQR delay. Allows trade-off between interrupt latency and determinism."]
pub struct IRQLATENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IQR delay. Allows trade-off between interrupt latency and determinism."]
pub mod irqlatency;
#[doc = "NMI Source Control"]
pub struct NMISRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NMI Source Control"]
pub mod nmisrc;
#[doc = "GPIO Pin Interrupt Select register 0"]
pub struct PINTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Pin Interrupt Select register 0"]
pub mod pintsel;
#[doc = "USB clock control"]
pub struct USBCLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB clock control"]
pub mod usbclkctrl;
#[doc = "USB clock status"]
pub struct USBCLKST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB clock status"]
pub mod usbclkst;
#[doc = "Start logic 0 interrupt wake-up enable register 0"]
pub struct STARTERP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic 0 interrupt wake-up enable register 0"]
pub mod starterp0;
#[doc = "Start logic 1 interrupt wake-up enable register 1"]
pub struct STARTERP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic 1 interrupt wake-up enable register 1"]
pub mod starterp1;
#[doc = "Power-down states in deep-sleep mode"]
pub struct PDSLEEPCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power-down states in deep-sleep mode"]
pub mod pdsleepcfg;
#[doc = "Power-down states for wake-up from deep-sleep"]
pub struct PDAWAKECFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power-down states for wake-up from deep-sleep"]
pub mod pdawakecfg;
#[doc = "Power configuration register"]
pub struct PDRUNCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power configuration register"]
pub mod pdruncfg;
#[doc = "Device ID"]
pub struct DEVICE_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device ID"]
pub mod device_id;
