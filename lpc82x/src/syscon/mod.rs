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
    _reserved0: [u8; 16usize],
    #[doc = "0x20 - System oscillator control"]
    pub sysoscctrl: SYSOSCCTRL,
    #[doc = "0x24 - Watchdog oscillator control"]
    pub wdtoscctrl: WDTOSCCTRL,
    #[doc = "0x28 - IRC control"]
    pub ircctrl: IRCCTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - System reset status register"]
    pub sysrststat: SYSRSTSTAT,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - System PLL clock source select"]
    pub syspllclksel: SYSPLLCLKSEL,
    #[doc = "0x44 - System PLL clock source update enable"]
    pub syspllclkuen: SYSPLLCLKUEN,
    _reserved3: [u8; 40usize],
    #[doc = "0x70 - Main clock source select"]
    pub mainclksel: MAINCLKSEL,
    #[doc = "0x74 - Main clock source update enable"]
    pub mainclkuen: MAINCLKUEN,
    #[doc = "0x78 - System clock divider"]
    pub sysahbclkdiv: SYSAHBCLKDIV,
    _reserved4: [u8; 4usize],
    #[doc = "0x80 - System clock control"]
    pub sysahbclkctrl: SYSAHBCLKCTRL,
    _reserved5: [u8; 16usize],
    #[doc = "0x94 - USART clock divider"]
    pub uartclkdiv: UARTCLKDIV,
    _reserved6: [u8; 72usize],
    #[doc = "0xe0 - CLKOUT clock source select"]
    pub clkoutsel: CLKOUTSEL,
    #[doc = "0xe4 - CLKOUT clock source update enable"]
    pub clkoutuen: CLKOUTUEN,
    #[doc = "0xe8 - CLKOUT clock divider"]
    pub clkoutdiv: CLKOUTDIV,
    _reserved7: [u8; 4usize],
    #[doc = "0xf0 - USART1 to USART4 common fractional generator divider value"]
    pub uartfrgdiv: UARTFRGDIV,
    #[doc = "0xf4 - USART1 to USART4 common fractional generator multiplier value"]
    pub uartfrgmult: UARTFRGMULT,
    _reserved8: [u8; 4usize],
    #[doc = "0xfc - External trace buffer command register"]
    pub exttracecmd: EXTTRACECMD,
    #[doc = "0x100 - POR captured PIO status 0"]
    pub pioporcap0: PIOPORCAP0,
    _reserved9: [u8; 48usize],
    #[doc = "0x134 - Peripheral clock 6 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv6: IOCONCLKDIV6,
    #[doc = "0x138 - Peripheral clock 5 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv5: IOCONCLKDIV5,
    #[doc = "0x13c - Peripheral clock 4 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv4: IOCONCLKDIV4,
    #[doc = "0x140 - Peripheral clock 3 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv3: IOCONCLKDIV3,
    #[doc = "0x144 - Peripheral clock 2 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv2: IOCONCLKDIV2,
    #[doc = "0x148 - Peripheral clock 1 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv1: IOCONCLKDIV1,
    #[doc = "0x14c - Peripheral clock 0 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv0: IOCONCLKDIV0,
    #[doc = "0x150 - Brown-Out Detect"]
    pub bodctrl: BODCTRL,
    #[doc = "0x154 - System tick counter calibration"]
    pub systckcal: SYSTCKCAL,
    _reserved10: [u8; 24usize],
    #[doc = "0x170 - IQR delay. Allows trade-off between interrupt latency and determinism."]
    pub irqlatency: IRQLATENCY,
    #[doc = "0x174 - NMI Source Control"]
    pub nmisrc: NMISRC,
    #[doc = "0x178 - GPIO Pin Interrupt Select register 0"]
    pub pintsel: [PINTSEL; 8],
    _reserved11: [u8; 108usize],
    #[doc = "0x204 - Start logic 0 pin wake-up enable register"]
    pub starterp0: STARTERP0,
    _reserved12: [u8; 12usize],
    #[doc = "0x214 - Start logic 1 interrupt wake-up enable register"]
    pub starterp1: STARTERP1,
    _reserved13: [u8; 24usize],
    #[doc = "0x230 - Power-down states in deep-sleep mode"]
    pub pdsleepcfg: PDSLEEPCFG,
    #[doc = "0x234 - Power-down states for wake-up from deep-sleep"]
    pub pdawakecfg: PDAWAKECFG,
    #[doc = "0x238 - Power configuration register"]
    pub pdruncfg: PDRUNCFG,
    _reserved14: [u8; 444usize],
    #[doc = "0x3f8 - Device ID"]
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
#[doc = "IRC control"]
pub struct IRCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRC control"]
pub mod ircctrl;
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
#[doc = "USART clock divider"]
pub struct UARTCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART clock divider"]
pub mod uartclkdiv;
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
#[doc = "USART1 to USART4 common fractional generator divider value"]
pub struct UARTFRGDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART1 to USART4 common fractional generator divider value"]
pub mod uartfrgdiv;
#[doc = "USART1 to USART4 common fractional generator multiplier value"]
pub struct UARTFRGMULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USART1 to USART4 common fractional generator multiplier value"]
pub mod uartfrgmult;
#[doc = "External trace buffer command register"]
pub struct EXTTRACECMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External trace buffer command register"]
pub mod exttracecmd;
#[doc = "POR captured PIO status 0"]
pub struct PIOPORCAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POR captured PIO status 0"]
pub mod pioporcap0;
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub struct IOCONCLKDIV6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv6;
#[doc = "Peripheral clock 5 to the IOCON block for programmable glitch filter"]
pub struct IOCONCLKDIV5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral clock 5 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv5;
#[doc = "Peripheral clock 4 to the IOCON block for programmable glitch filter"]
pub struct IOCONCLKDIV4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral clock 4 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv4;
#[doc = "Peripheral clock 3 to the IOCON block for programmable glitch filter"]
pub struct IOCONCLKDIV3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral clock 3 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv3;
#[doc = "Peripheral clock 2 to the IOCON block for programmable glitch filter"]
pub struct IOCONCLKDIV2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral clock 2 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv2;
#[doc = "Peripheral clock 1 to the IOCON block for programmable glitch filter"]
pub struct IOCONCLKDIV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral clock 1 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv1;
#[doc = "Peripheral clock 0 to the IOCON block for programmable glitch filter"]
pub struct IOCONCLKDIV0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral clock 0 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv0;
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
#[doc = "Start logic 0 pin wake-up enable register"]
pub struct STARTERP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic 0 pin wake-up enable register"]
pub mod starterp0;
#[doc = "Start logic 1 interrupt wake-up enable register"]
pub struct STARTERP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic 1 interrupt wake-up enable register"]
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
