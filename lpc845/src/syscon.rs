#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Remap register"]
    pub sysmemremap: SYSMEMREMAP,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - PLL control"]
    pub syspllctrl: SYSPLLCTRL,
    #[doc = "0x0c - PLL status"]
    pub syspllstat: SYSPLLSTAT,
    _reserved1: [u8; 16usize],
    #[doc = "0x20 - system oscillator control"]
    pub sysoscctrl: SYSOSCCTRL,
    #[doc = "0x24 - Watchdog oscillator control"]
    pub wdtoscctrl: WDTOSCCTRL,
    #[doc = "0x28 - FRO oscillator control"]
    pub frooscctrl: FROOSCCTRL,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - FRO direct clock source update enable register"]
    pub frodirectclkuen: FRODIRECTCLKUEN,
    _reserved3: [u8; 4usize],
    #[doc = "0x38 - System reset status register"]
    pub sysrststat: SYSRSTSTAT,
    _reserved4: [u8; 4usize],
    #[doc = "0x40 - System PLL clock source select register"]
    pub syspllclksel: SYSPLLCLKSEL,
    #[doc = "0x44 - System PLL clock source update enable register"]
    pub syspllclkuen: SYSPLLCLKUEN,
    #[doc = "0x48 - Main clock source select register"]
    pub mainclkpllsel: MAINCLKPLLSEL,
    #[doc = "0x4c - Main clock source update enable register"]
    pub mainclkplluen: MAINCLKPLLUEN,
    #[doc = "0x50 - Main clock source select register"]
    pub mainclksel: MAINCLKSEL,
    #[doc = "0x54 - Main clock source update enable register"]
    pub mainclkuen: MAINCLKUEN,
    #[doc = "0x58 - System clock divider register"]
    pub sysahbclkdiv: SYSAHBCLKDIV,
    _reserved5: [u8; 4usize],
    #[doc = "0x60 - CAPT clock source select register"]
    pub captclksel: CAPTCLKSEL,
    #[doc = "0x64 - ADC clock source select register"]
    pub adcclksel: ADCCLKSEL,
    #[doc = "0x68 - ADC clock divider register"]
    pub adcclkdiv: ADCCLKDIV,
    #[doc = "0x6c - SCT clock source select register"]
    pub sctclksel: SCTCLKSEL,
    #[doc = "0x70 - SCT clock divider register"]
    pub sctclkdiv: SCTCLKDIV,
    #[doc = "0x74 - external clock source select register"]
    pub extclksel: EXTCLKSEL,
    _reserved6: [u8; 8usize],
    #[doc = "0x80 - System clock group 0 control register"]
    pub sysahbclkctrl0: SYSAHBCLKCTRL0,
    #[doc = "0x84 - System clock group 1 control register"]
    pub sysahbclkctrl1: SYSAHBCLKCTRL1,
    #[doc = "0x88 - Peripheral reset group 0 control register"]
    pub presetctrl0: PRESETCTRL0,
    #[doc = "0x8c - Peripheral reset group 1 control register"]
    pub presetctrl1: PRESETCTRL1,
    #[doc = "0x90 - peripheral clock source select register. FCLK0SEL~FCLK4SEL are for UART0~UART4 clock source select register. FCLK5SEL~FCLK8SEL are for I2C0~I2C3 clock source select register. FCLK9SEL~FCLK10SEL are for SPI0~SPI1 clock source select register."]
    pub fclksel: [FCLKSEL; 11],
    _reserved7: [u8; 20usize],
    #[doc = "0xd0 - no description available"]
    pub frg0: FRG,
    _reserved8: [u8; 4usize],
    #[doc = "0xe0 - no description available"]
    pub frg1: FRG,
    _reserved9: [u8; 4usize],
    #[doc = "0xf0 - CLKOUT clock source select register"]
    pub clkoutsel: CLKOUTSEL,
    #[doc = "0xf4 - CLKOUT clock divider registers"]
    pub clkoutdiv: CLKOUTDIV,
    _reserved10: [u8; 4usize],
    #[doc = "0xfc - External trace buffer command register"]
    pub exttracecmd: EXTTRACECMD,
    #[doc = "0x100 - POR captured PIO N status register(PIO0 has 32 PIOs, PIO1 has 22 PIOs)"]
    pub pioporcap: [PIOPORCAP; 2],
    _reserved11: [u8; 44usize],
    #[doc = "0x134 - Peripheral clock 6 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv6: IOCONCLKDIV6,
    #[doc = "0x138 - Peripheral clock 6 to the IOCON block for programmable glitch filter"]
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
    #[doc = "0x150 - BOD control register"]
    pub bodctrl: BODCTRL,
    #[doc = "0x154 - System tick timer calibration register"]
    pub systckcal: SYSTCKCAL,
    _reserved12: [u8; 24usize],
    #[doc = "0x170 - IRQ latency register"]
    pub irqlatency: IRQLATENCY,
    #[doc = "0x174 - NMI source selection register"]
    pub nmisrc: NMISRC,
    #[doc = "0x178 - Pin interrupt select registers N"]
    pub pintsel: [PINTSEL; 8],
    _reserved13: [u8; 108usize],
    #[doc = "0x204 - Start logic 0 pin wake-up enable register 0"]
    pub starterp0: STARTERP0,
    _reserved14: [u8; 12usize],
    #[doc = "0x214 - Start logic 0 pin wake-up enable register 1"]
    pub starterp1: STARTERP1,
    _reserved15: [u8; 24usize],
    #[doc = "0x230 - Deep-sleep configuration register"]
    pub pdsleepcfg: PDSLEEPCFG,
    #[doc = "0x234 - Wake-up configuration register"]
    pub pdawakecfg: PDAWAKECFG,
    #[doc = "0x238 - Power configuration register"]
    pub pdruncfg: PDRUNCFG,
    _reserved16: [u8; 444usize],
    #[doc = "0x3f8 - Part ID register"]
    pub device_id: DEVICE_ID,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct FRG {
    #[doc = "0x00 - fractional generator N divider value register"]
    pub frgdiv: self::frg::FRGDIV,
    #[doc = "0x04 - fractional generator N multiplier value register"]
    pub frgmult: self::frg::FRGMULT,
    #[doc = "0x08 - FRG N clock source select register"]
    pub frgclksel: self::frg::FRGCLKSEL,
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod frg;
#[doc = "System Remap register"]
pub struct SYSMEMREMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Remap register"]
pub mod sysmemremap;
#[doc = "PLL control"]
pub struct SYSPLLCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL control"]
pub mod syspllctrl;
#[doc = "PLL status"]
pub struct SYSPLLSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL status"]
pub mod syspllstat;
#[doc = "system oscillator control"]
pub struct SYSOSCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "system oscillator control"]
pub mod sysoscctrl;
#[doc = "Watchdog oscillator control"]
pub struct WDTOSCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "FRO oscillator control"]
pub struct FROOSCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FRO oscillator control"]
pub mod frooscctrl;
#[doc = "FRO direct clock source update enable register"]
pub struct FRODIRECTCLKUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FRO direct clock source update enable register"]
pub mod frodirectclkuen;
#[doc = "System reset status register"]
pub struct SYSRSTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "System PLL clock source select register"]
pub struct SYSPLLCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL clock source select register"]
pub mod syspllclksel;
#[doc = "System PLL clock source update enable register"]
pub struct SYSPLLCLKUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL clock source update enable register"]
pub mod syspllclkuen;
#[doc = "Main clock source select register"]
pub struct MAINCLKPLLSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main clock source select register"]
pub mod mainclkpllsel;
#[doc = "Main clock source update enable register"]
pub struct MAINCLKPLLUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main clock source update enable register"]
pub mod mainclkplluen;
#[doc = "Main clock source select register"]
pub struct MAINCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main clock source select register"]
pub mod mainclksel;
#[doc = "Main clock source update enable register"]
pub struct MAINCLKUEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Main clock source update enable register"]
pub mod mainclkuen;
#[doc = "System clock divider register"]
pub struct SYSAHBCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System clock divider register"]
pub mod sysahbclkdiv;
#[doc = "CAPT clock source select register"]
pub struct CAPTCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAPT clock source select register"]
pub mod captclksel;
#[doc = "ADC clock source select register"]
pub struct ADCCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC clock source select register"]
pub mod adcclksel;
#[doc = "ADC clock divider register"]
pub struct ADCCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC clock divider register"]
pub mod adcclkdiv;
#[doc = "SCT clock source select register"]
pub struct SCTCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT clock source select register"]
pub mod sctclksel;
#[doc = "SCT clock divider register"]
pub struct SCTCLKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCT clock divider register"]
pub mod sctclkdiv;
#[doc = "external clock source select register"]
pub struct EXTCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "external clock source select register"]
pub mod extclksel;
#[doc = "System clock group 0 control register"]
pub struct SYSAHBCLKCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System clock group 0 control register"]
pub mod sysahbclkctrl0;
#[doc = "System clock group 1 control register"]
pub struct SYSAHBCLKCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System clock group 1 control register"]
pub mod sysahbclkctrl1;
#[doc = "Peripheral reset group 0 control register"]
pub struct PRESETCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset group 0 control register"]
pub mod presetctrl0;
#[doc = "Peripheral reset group 1 control register"]
pub struct PRESETCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset group 1 control register"]
pub mod presetctrl1;
#[doc = "peripheral clock source select register. FCLK0SEL~FCLK4SEL are for UART0~UART4 clock source select register. FCLK5SEL~FCLK8SEL are for I2C0~I2C3 clock source select register. FCLK9SEL~FCLK10SEL are for SPI0~SPI1 clock source select register."]
pub struct FCLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "peripheral clock source select register. FCLK0SEL~FCLK4SEL are for UART0~UART4 clock source select register. FCLK5SEL~FCLK8SEL are for I2C0~I2C3 clock source select register. FCLK9SEL~FCLK10SEL are for SPI0~SPI1 clock source select register."]
pub mod fclksel;
#[doc = "CLKOUT clock source select register"]
pub struct CLKOUTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock source select register"]
pub mod clkoutsel;
#[doc = "CLKOUT clock divider registers"]
pub struct CLKOUTDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock divider registers"]
pub mod clkoutdiv;
#[doc = "External trace buffer command register"]
pub struct EXTTRACECMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External trace buffer command register"]
pub mod exttracecmd;
#[doc = "POR captured PIO N status register(PIO0 has 32 PIOs, PIO1 has 22 PIOs)"]
pub struct PIOPORCAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POR captured PIO N status register(PIO0 has 32 PIOs, PIO1 has 22 PIOs)"]
pub mod pioporcap;
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub struct IOCONCLKDIV6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv6;
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub struct IOCONCLKDIV5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
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
#[doc = "BOD control register"]
pub struct BODCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BOD control register"]
pub mod bodctrl;
#[doc = "System tick timer calibration register"]
pub struct SYSTCKCAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System tick timer calibration register"]
pub mod systckcal;
#[doc = "IRQ latency register"]
pub struct IRQLATENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IRQ latency register"]
pub mod irqlatency;
#[doc = "NMI source selection register"]
pub struct NMISRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NMI source selection register"]
pub mod nmisrc;
#[doc = "Pin interrupt select registers N"]
pub struct PINTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt select registers N"]
pub mod pintsel;
#[doc = "Start logic 0 pin wake-up enable register 0"]
pub struct STARTERP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic 0 pin wake-up enable register 0"]
pub mod starterp0;
#[doc = "Start logic 0 pin wake-up enable register 1"]
pub struct STARTERP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start logic 0 pin wake-up enable register 1"]
pub mod starterp1;
#[doc = "Deep-sleep configuration register"]
pub struct PDSLEEPCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep-sleep configuration register"]
pub mod pdsleepcfg;
#[doc = "Wake-up configuration register"]
pub struct PDAWAKECFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up configuration register"]
pub mod pdawakecfg;
#[doc = "Power configuration register"]
pub struct PDRUNCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power configuration register"]
pub mod pdruncfg;
#[doc = "Part ID register"]
pub struct DEVICE_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Part ID register"]
pub mod device_id;
