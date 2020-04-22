#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Remap register"]
    pub sysmemremap: SYSMEMREMAP,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - PLL control"]
    pub syspllctrl: SYSPLLCTRL,
    #[doc = "0x0c - PLL status"]
    pub syspllstat: SYSPLLSTAT,
    _reserved3: [u8; 16usize],
    #[doc = "0x20 - system oscillator control"]
    pub sysoscctrl: SYSOSCCTRL,
    #[doc = "0x24 - Watchdog oscillator control"]
    pub wdtoscctrl: WDTOSCCTRL,
    #[doc = "0x28 - FRO oscillator control"]
    pub frooscctrl: FROOSCCTRL,
    _reserved6: [u8; 4usize],
    #[doc = "0x30 - FRO direct clock source update enable register"]
    pub frodirectclkuen: FRODIRECTCLKUEN,
    _reserved7: [u8; 4usize],
    #[doc = "0x38 - System reset status register"]
    pub sysrststat: SYSRSTSTAT,
    _reserved8: [u8; 4usize],
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
    _reserved15: [u8; 4usize],
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
    _reserved21: [u8; 8usize],
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
    _reserved26: [u8; 20usize],
    #[doc = "0xd0 - no description available"]
    pub frg0: FRG,
    _reserved27: [u8; 4usize],
    #[doc = "0xe0 - no description available"]
    pub frg1: FRG,
    _reserved28: [u8; 4usize],
    #[doc = "0xf0 - CLKOUT clock source select register"]
    pub clkoutsel: CLKOUTSEL,
    #[doc = "0xf4 - CLKOUT clock divider registers"]
    pub clkoutdiv: CLKOUTDIV,
    _reserved30: [u8; 4usize],
    #[doc = "0xfc - External trace buffer command register"]
    pub exttracecmd: EXTTRACECMD,
    #[doc = "0x100 - POR captured PIO N status register(PIO0 has 32 PIOs, PIO1 has 22 PIOs)"]
    pub pioporcap: [PIOPORCAP; 2],
    _reserved32: [u8; 44usize],
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
    _reserved41: [u8; 24usize],
    #[doc = "0x170 - IRQ latency register"]
    pub irqlatency: IRQLATENCY,
    #[doc = "0x174 - NMI source selection register"]
    pub nmisrc: NMISRC,
    #[doc = "0x178 - Pin interrupt select registers N"]
    pub pintsel: [PINTSEL; 8],
    _reserved44: [u8; 108usize],
    #[doc = "0x204 - Start logic 0 pin wake-up enable register 0"]
    pub starterp0: STARTERP0,
    _reserved45: [u8; 12usize],
    #[doc = "0x214 - Start logic 0 pin wake-up enable register 1"]
    pub starterp1: STARTERP1,
    _reserved46: [u8; 24usize],
    #[doc = "0x230 - Deep-sleep configuration register"]
    pub pdsleepcfg: PDSLEEPCFG,
    #[doc = "0x234 - Wake-up configuration register"]
    pub pdawakecfg: PDAWAKECFG,
    #[doc = "0x238 - Power configuration register"]
    pub pdruncfg: PDRUNCFG,
    _reserved49: [u8; 444usize],
    #[doc = "0x3f8 - Part ID register"]
    pub device_id: DEVICE_ID,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct FRG {
    #[doc = "0x00 - fractional generator N divider value register"]
    pub frgdiv: self::frg::FRGDIV,
    #[doc = "0x04 - fractional generator N multiplier value register"]
    pub frgmult: self::frg::FRGMULT,
    #[doc = "0x08 - FRG N clock source select register"]
    pub frgclksel: self::frg::FRGCLKSEL,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod frg;
#[doc = "System Remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysmemremap](sysmemremap) module"]
pub type SYSMEMREMAP = crate::Reg<u32, _SYSMEMREMAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSMEMREMAP;
#[doc = "`read()` method returns [sysmemremap::R](sysmemremap::R) reader structure"]
impl crate::Readable for SYSMEMREMAP {}
#[doc = "`write(|w| ..)` method takes [sysmemremap::W](sysmemremap::W) writer structure"]
impl crate::Writable for SYSMEMREMAP {}
#[doc = "System Remap register"]
pub mod sysmemremap;
#[doc = "PLL control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllctrl](syspllctrl) module"]
pub type SYSPLLCTRL = crate::Reg<u32, _SYSPLLCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLCTRL;
#[doc = "`read()` method returns [syspllctrl::R](syspllctrl::R) reader structure"]
impl crate::Readable for SYSPLLCTRL {}
#[doc = "`write(|w| ..)` method takes [syspllctrl::W](syspllctrl::W) writer structure"]
impl crate::Writable for SYSPLLCTRL {}
#[doc = "PLL control"]
pub mod syspllctrl;
#[doc = "PLL status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllstat](syspllstat) module"]
pub type SYSPLLSTAT = crate::Reg<u32, _SYSPLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLSTAT;
#[doc = "`read()` method returns [syspllstat::R](syspllstat::R) reader structure"]
impl crate::Readable for SYSPLLSTAT {}
#[doc = "PLL status"]
pub mod syspllstat;
#[doc = "system oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysoscctrl](sysoscctrl) module"]
pub type SYSOSCCTRL = crate::Reg<u32, _SYSOSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSOSCCTRL;
#[doc = "`read()` method returns [sysoscctrl::R](sysoscctrl::R) reader structure"]
impl crate::Readable for SYSOSCCTRL {}
#[doc = "`write(|w| ..)` method takes [sysoscctrl::W](sysoscctrl::W) writer structure"]
impl crate::Writable for SYSOSCCTRL {}
#[doc = "system oscillator control"]
pub mod sysoscctrl;
#[doc = "Watchdog oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtoscctrl](wdtoscctrl) module"]
pub type WDTOSCCTRL = crate::Reg<u32, _WDTOSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTOSCCTRL;
#[doc = "`read()` method returns [wdtoscctrl::R](wdtoscctrl::R) reader structure"]
impl crate::Readable for WDTOSCCTRL {}
#[doc = "`write(|w| ..)` method takes [wdtoscctrl::W](wdtoscctrl::W) writer structure"]
impl crate::Writable for WDTOSCCTRL {}
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "FRO oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frooscctrl](frooscctrl) module"]
pub type FROOSCCTRL = crate::Reg<u32, _FROOSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FROOSCCTRL;
#[doc = "`read()` method returns [frooscctrl::R](frooscctrl::R) reader structure"]
impl crate::Readable for FROOSCCTRL {}
#[doc = "`write(|w| ..)` method takes [frooscctrl::W](frooscctrl::W) writer structure"]
impl crate::Writable for FROOSCCTRL {}
#[doc = "FRO oscillator control"]
pub mod frooscctrl;
#[doc = "FRO direct clock source update enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frodirectclkuen](frodirectclkuen) module"]
pub type FRODIRECTCLKUEN = crate::Reg<u32, _FRODIRECTCLKUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRODIRECTCLKUEN;
#[doc = "`read()` method returns [frodirectclkuen::R](frodirectclkuen::R) reader structure"]
impl crate::Readable for FRODIRECTCLKUEN {}
#[doc = "`write(|w| ..)` method takes [frodirectclkuen::W](frodirectclkuen::W) writer structure"]
impl crate::Writable for FRODIRECTCLKUEN {}
#[doc = "FRO direct clock source update enable register"]
pub mod frodirectclkuen;
#[doc = "System reset status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysrststat](sysrststat) module"]
pub type SYSRSTSTAT = crate::Reg<u32, _SYSRSTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSRSTSTAT;
#[doc = "`read()` method returns [sysrststat::R](sysrststat::R) reader structure"]
impl crate::Readable for SYSRSTSTAT {}
#[doc = "`write(|w| ..)` method takes [sysrststat::W](sysrststat::W) writer structure"]
impl crate::Writable for SYSRSTSTAT {}
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "System PLL clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllclksel](syspllclksel) module"]
pub type SYSPLLCLKSEL = crate::Reg<u32, _SYSPLLCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLCLKSEL;
#[doc = "`read()` method returns [syspllclksel::R](syspllclksel::R) reader structure"]
impl crate::Readable for SYSPLLCLKSEL {}
#[doc = "`write(|w| ..)` method takes [syspllclksel::W](syspllclksel::W) writer structure"]
impl crate::Writable for SYSPLLCLKSEL {}
#[doc = "System PLL clock source select register"]
pub mod syspllclksel;
#[doc = "System PLL clock source update enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllclkuen](syspllclkuen) module"]
pub type SYSPLLCLKUEN = crate::Reg<u32, _SYSPLLCLKUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLCLKUEN;
#[doc = "`read()` method returns [syspllclkuen::R](syspllclkuen::R) reader structure"]
impl crate::Readable for SYSPLLCLKUEN {}
#[doc = "`write(|w| ..)` method takes [syspllclkuen::W](syspllclkuen::W) writer structure"]
impl crate::Writable for SYSPLLCLKUEN {}
#[doc = "System PLL clock source update enable register"]
pub mod syspllclkuen;
#[doc = "Main clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclkpllsel](mainclkpllsel) module"]
pub type MAINCLKPLLSEL = crate::Reg<u32, _MAINCLKPLLSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINCLKPLLSEL;
#[doc = "`read()` method returns [mainclkpllsel::R](mainclkpllsel::R) reader structure"]
impl crate::Readable for MAINCLKPLLSEL {}
#[doc = "`write(|w| ..)` method takes [mainclkpllsel::W](mainclkpllsel::W) writer structure"]
impl crate::Writable for MAINCLKPLLSEL {}
#[doc = "Main clock source select register"]
pub mod mainclkpllsel;
#[doc = "Main clock source update enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclkplluen](mainclkplluen) module"]
pub type MAINCLKPLLUEN = crate::Reg<u32, _MAINCLKPLLUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINCLKPLLUEN;
#[doc = "`read()` method returns [mainclkplluen::R](mainclkplluen::R) reader structure"]
impl crate::Readable for MAINCLKPLLUEN {}
#[doc = "`write(|w| ..)` method takes [mainclkplluen::W](mainclkplluen::W) writer structure"]
impl crate::Writable for MAINCLKPLLUEN {}
#[doc = "Main clock source update enable register"]
pub mod mainclkplluen;
#[doc = "Main clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclksel](mainclksel) module"]
pub type MAINCLKSEL = crate::Reg<u32, _MAINCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINCLKSEL;
#[doc = "`read()` method returns [mainclksel::R](mainclksel::R) reader structure"]
impl crate::Readable for MAINCLKSEL {}
#[doc = "`write(|w| ..)` method takes [mainclksel::W](mainclksel::W) writer structure"]
impl crate::Writable for MAINCLKSEL {}
#[doc = "Main clock source select register"]
pub mod mainclksel;
#[doc = "Main clock source update enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclkuen](mainclkuen) module"]
pub type MAINCLKUEN = crate::Reg<u32, _MAINCLKUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINCLKUEN;
#[doc = "`read()` method returns [mainclkuen::R](mainclkuen::R) reader structure"]
impl crate::Readable for MAINCLKUEN {}
#[doc = "`write(|w| ..)` method takes [mainclkuen::W](mainclkuen::W) writer structure"]
impl crate::Writable for MAINCLKUEN {}
#[doc = "Main clock source update enable register"]
pub mod mainclkuen;
#[doc = "System clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysahbclkdiv](sysahbclkdiv) module"]
pub type SYSAHBCLKDIV = crate::Reg<u32, _SYSAHBCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSAHBCLKDIV;
#[doc = "`read()` method returns [sysahbclkdiv::R](sysahbclkdiv::R) reader structure"]
impl crate::Readable for SYSAHBCLKDIV {}
#[doc = "`write(|w| ..)` method takes [sysahbclkdiv::W](sysahbclkdiv::W) writer structure"]
impl crate::Writable for SYSAHBCLKDIV {}
#[doc = "System clock divider register"]
pub mod sysahbclkdiv;
#[doc = "CAPT clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [captclksel](captclksel) module"]
pub type CAPTCLKSEL = crate::Reg<u32, _CAPTCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPTCLKSEL;
#[doc = "`read()` method returns [captclksel::R](captclksel::R) reader structure"]
impl crate::Readable for CAPTCLKSEL {}
#[doc = "`write(|w| ..)` method takes [captclksel::W](captclksel::W) writer structure"]
impl crate::Writable for CAPTCLKSEL {}
#[doc = "CAPT clock source select register"]
pub mod captclksel;
#[doc = "ADC clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclksel](adcclksel) module"]
pub type ADCCLKSEL = crate::Reg<u32, _ADCCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCLKSEL;
#[doc = "`read()` method returns [adcclksel::R](adcclksel::R) reader structure"]
impl crate::Readable for ADCCLKSEL {}
#[doc = "`write(|w| ..)` method takes [adcclksel::W](adcclksel::W) writer structure"]
impl crate::Writable for ADCCLKSEL {}
#[doc = "ADC clock source select register"]
pub mod adcclksel;
#[doc = "ADC clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclkdiv](adcclkdiv) module"]
pub type ADCCLKDIV = crate::Reg<u32, _ADCCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCLKDIV;
#[doc = "`read()` method returns [adcclkdiv::R](adcclkdiv::R) reader structure"]
impl crate::Readable for ADCCLKDIV {}
#[doc = "`write(|w| ..)` method takes [adcclkdiv::W](adcclkdiv::W) writer structure"]
impl crate::Writable for ADCCLKDIV {}
#[doc = "ADC clock divider register"]
pub mod adcclkdiv;
#[doc = "SCT clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctclksel](sctclksel) module"]
pub type SCTCLKSEL = crate::Reg<u32, _SCTCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCLKSEL;
#[doc = "`read()` method returns [sctclksel::R](sctclksel::R) reader structure"]
impl crate::Readable for SCTCLKSEL {}
#[doc = "`write(|w| ..)` method takes [sctclksel::W](sctclksel::W) writer structure"]
impl crate::Writable for SCTCLKSEL {}
#[doc = "SCT clock source select register"]
pub mod sctclksel;
#[doc = "SCT clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctclkdiv](sctclkdiv) module"]
pub type SCTCLKDIV = crate::Reg<u32, _SCTCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCLKDIV;
#[doc = "`read()` method returns [sctclkdiv::R](sctclkdiv::R) reader structure"]
impl crate::Readable for SCTCLKDIV {}
#[doc = "`write(|w| ..)` method takes [sctclkdiv::W](sctclkdiv::W) writer structure"]
impl crate::Writable for SCTCLKDIV {}
#[doc = "SCT clock divider register"]
pub mod sctclkdiv;
#[doc = "external clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extclksel](extclksel) module"]
pub type EXTCLKSEL = crate::Reg<u32, _EXTCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTCLKSEL;
#[doc = "`read()` method returns [extclksel::R](extclksel::R) reader structure"]
impl crate::Readable for EXTCLKSEL {}
#[doc = "`write(|w| ..)` method takes [extclksel::W](extclksel::W) writer structure"]
impl crate::Writable for EXTCLKSEL {}
#[doc = "external clock source select register"]
pub mod extclksel;
#[doc = "System clock group 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysahbclkctrl0](sysahbclkctrl0) module"]
pub type SYSAHBCLKCTRL0 = crate::Reg<u32, _SYSAHBCLKCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSAHBCLKCTRL0;
#[doc = "`read()` method returns [sysahbclkctrl0::R](sysahbclkctrl0::R) reader structure"]
impl crate::Readable for SYSAHBCLKCTRL0 {}
#[doc = "`write(|w| ..)` method takes [sysahbclkctrl0::W](sysahbclkctrl0::W) writer structure"]
impl crate::Writable for SYSAHBCLKCTRL0 {}
#[doc = "System clock group 0 control register"]
pub mod sysahbclkctrl0;
#[doc = "System clock group 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysahbclkctrl1](sysahbclkctrl1) module"]
pub type SYSAHBCLKCTRL1 = crate::Reg<u32, _SYSAHBCLKCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSAHBCLKCTRL1;
#[doc = "`read()` method returns [sysahbclkctrl1::R](sysahbclkctrl1::R) reader structure"]
impl crate::Readable for SYSAHBCLKCTRL1 {}
#[doc = "`write(|w| ..)` method takes [sysahbclkctrl1::W](sysahbclkctrl1::W) writer structure"]
impl crate::Writable for SYSAHBCLKCTRL1 {}
#[doc = "System clock group 1 control register"]
pub mod sysahbclkctrl1;
#[doc = "Peripheral reset group 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl0](presetctrl0) module"]
pub type PRESETCTRL0 = crate::Reg<u32, _PRESETCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRL0;
#[doc = "`read()` method returns [presetctrl0::R](presetctrl0::R) reader structure"]
impl crate::Readable for PRESETCTRL0 {}
#[doc = "`write(|w| ..)` method takes [presetctrl0::W](presetctrl0::W) writer structure"]
impl crate::Writable for PRESETCTRL0 {}
#[doc = "Peripheral reset group 0 control register"]
pub mod presetctrl0;
#[doc = "Peripheral reset group 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl1](presetctrl1) module"]
pub type PRESETCTRL1 = crate::Reg<u32, _PRESETCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRL1;
#[doc = "`read()` method returns [presetctrl1::R](presetctrl1::R) reader structure"]
impl crate::Readable for PRESETCTRL1 {}
#[doc = "`write(|w| ..)` method takes [presetctrl1::W](presetctrl1::W) writer structure"]
impl crate::Writable for PRESETCTRL1 {}
#[doc = "Peripheral reset group 1 control register"]
pub mod presetctrl1;
#[doc = "peripheral clock source select register. FCLK0SEL~FCLK4SEL are for UART0~UART4 clock source select register. FCLK5SEL~FCLK8SEL are for I2C0~I2C3 clock source select register. FCLK9SEL~FCLK10SEL are for SPI0~SPI1 clock source select register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fclksel](fclksel) module"]
pub type FCLKSEL = crate::Reg<u32, _FCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCLKSEL;
#[doc = "`read()` method returns [fclksel::R](fclksel::R) reader structure"]
impl crate::Readable for FCLKSEL {}
#[doc = "`write(|w| ..)` method takes [fclksel::W](fclksel::W) writer structure"]
impl crate::Writable for FCLKSEL {}
#[doc = "peripheral clock source select register. FCLK0SEL~FCLK4SEL are for UART0~UART4 clock source select register. FCLK5SEL~FCLK8SEL are for I2C0~I2C3 clock source select register. FCLK9SEL~FCLK10SEL are for SPI0~SPI1 clock source select register."]
pub mod fclksel;
#[doc = "CLKOUT clock source select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutsel](clkoutsel) module"]
pub type CLKOUTSEL = crate::Reg<u32, _CLKOUTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTSEL;
#[doc = "`read()` method returns [clkoutsel::R](clkoutsel::R) reader structure"]
impl crate::Readable for CLKOUTSEL {}
#[doc = "`write(|w| ..)` method takes [clkoutsel::W](clkoutsel::W) writer structure"]
impl crate::Writable for CLKOUTSEL {}
#[doc = "CLKOUT clock source select register"]
pub mod clkoutsel;
#[doc = "CLKOUT clock divider registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutdiv](clkoutdiv) module"]
pub type CLKOUTDIV = crate::Reg<u32, _CLKOUTDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTDIV;
#[doc = "`read()` method returns [clkoutdiv::R](clkoutdiv::R) reader structure"]
impl crate::Readable for CLKOUTDIV {}
#[doc = "`write(|w| ..)` method takes [clkoutdiv::W](clkoutdiv::W) writer structure"]
impl crate::Writable for CLKOUTDIV {}
#[doc = "CLKOUT clock divider registers"]
pub mod clkoutdiv;
#[doc = "External trace buffer command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exttracecmd](exttracecmd) module"]
pub type EXTTRACECMD = crate::Reg<u32, _EXTTRACECMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTTRACECMD;
#[doc = "`read()` method returns [exttracecmd::R](exttracecmd::R) reader structure"]
impl crate::Readable for EXTTRACECMD {}
#[doc = "`write(|w| ..)` method takes [exttracecmd::W](exttracecmd::W) writer structure"]
impl crate::Writable for EXTTRACECMD {}
#[doc = "External trace buffer command register"]
pub mod exttracecmd;
#[doc = "POR captured PIO N status register(PIO0 has 32 PIOs, PIO1 has 22 PIOs)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pioporcap](pioporcap) module"]
pub type PIOPORCAP = crate::Reg<u32, _PIOPORCAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIOPORCAP;
#[doc = "`read()` method returns [pioporcap::R](pioporcap::R) reader structure"]
impl crate::Readable for PIOPORCAP {}
#[doc = "POR captured PIO N status register(PIO0 has 32 PIOs, PIO1 has 22 PIOs)"]
pub mod pioporcap;
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconclkdiv6](ioconclkdiv6) module"]
pub type IOCONCLKDIV6 = crate::Reg<u32, _IOCONCLKDIV6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCONCLKDIV6;
#[doc = "`read()` method returns [ioconclkdiv6::R](ioconclkdiv6::R) reader structure"]
impl crate::Readable for IOCONCLKDIV6 {}
#[doc = "`write(|w| ..)` method takes [ioconclkdiv6::W](ioconclkdiv6::W) writer structure"]
impl crate::Writable for IOCONCLKDIV6 {}
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv6;
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconclkdiv5](ioconclkdiv5) module"]
pub type IOCONCLKDIV5 = crate::Reg<u32, _IOCONCLKDIV5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCONCLKDIV5;
#[doc = "`read()` method returns [ioconclkdiv5::R](ioconclkdiv5::R) reader structure"]
impl crate::Readable for IOCONCLKDIV5 {}
#[doc = "`write(|w| ..)` method takes [ioconclkdiv5::W](ioconclkdiv5::W) writer structure"]
impl crate::Writable for IOCONCLKDIV5 {}
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv5;
#[doc = "Peripheral clock 4 to the IOCON block for programmable glitch filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconclkdiv4](ioconclkdiv4) module"]
pub type IOCONCLKDIV4 = crate::Reg<u32, _IOCONCLKDIV4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCONCLKDIV4;
#[doc = "`read()` method returns [ioconclkdiv4::R](ioconclkdiv4::R) reader structure"]
impl crate::Readable for IOCONCLKDIV4 {}
#[doc = "`write(|w| ..)` method takes [ioconclkdiv4::W](ioconclkdiv4::W) writer structure"]
impl crate::Writable for IOCONCLKDIV4 {}
#[doc = "Peripheral clock 4 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv4;
#[doc = "Peripheral clock 3 to the IOCON block for programmable glitch filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconclkdiv3](ioconclkdiv3) module"]
pub type IOCONCLKDIV3 = crate::Reg<u32, _IOCONCLKDIV3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCONCLKDIV3;
#[doc = "`read()` method returns [ioconclkdiv3::R](ioconclkdiv3::R) reader structure"]
impl crate::Readable for IOCONCLKDIV3 {}
#[doc = "`write(|w| ..)` method takes [ioconclkdiv3::W](ioconclkdiv3::W) writer structure"]
impl crate::Writable for IOCONCLKDIV3 {}
#[doc = "Peripheral clock 3 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv3;
#[doc = "Peripheral clock 2 to the IOCON block for programmable glitch filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconclkdiv2](ioconclkdiv2) module"]
pub type IOCONCLKDIV2 = crate::Reg<u32, _IOCONCLKDIV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCONCLKDIV2;
#[doc = "`read()` method returns [ioconclkdiv2::R](ioconclkdiv2::R) reader structure"]
impl crate::Readable for IOCONCLKDIV2 {}
#[doc = "`write(|w| ..)` method takes [ioconclkdiv2::W](ioconclkdiv2::W) writer structure"]
impl crate::Writable for IOCONCLKDIV2 {}
#[doc = "Peripheral clock 2 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv2;
#[doc = "Peripheral clock 1 to the IOCON block for programmable glitch filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconclkdiv1](ioconclkdiv1) module"]
pub type IOCONCLKDIV1 = crate::Reg<u32, _IOCONCLKDIV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCONCLKDIV1;
#[doc = "`read()` method returns [ioconclkdiv1::R](ioconclkdiv1::R) reader structure"]
impl crate::Readable for IOCONCLKDIV1 {}
#[doc = "`write(|w| ..)` method takes [ioconclkdiv1::W](ioconclkdiv1::W) writer structure"]
impl crate::Writable for IOCONCLKDIV1 {}
#[doc = "Peripheral clock 1 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv1;
#[doc = "Peripheral clock 0 to the IOCON block for programmable glitch filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconclkdiv0](ioconclkdiv0) module"]
pub type IOCONCLKDIV0 = crate::Reg<u32, _IOCONCLKDIV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCONCLKDIV0;
#[doc = "`read()` method returns [ioconclkdiv0::R](ioconclkdiv0::R) reader structure"]
impl crate::Readable for IOCONCLKDIV0 {}
#[doc = "`write(|w| ..)` method takes [ioconclkdiv0::W](ioconclkdiv0::W) writer structure"]
impl crate::Writable for IOCONCLKDIV0 {}
#[doc = "Peripheral clock 0 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv0;
#[doc = "BOD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodctrl](bodctrl) module"]
pub type BODCTRL = crate::Reg<u32, _BODCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BODCTRL;
#[doc = "`read()` method returns [bodctrl::R](bodctrl::R) reader structure"]
impl crate::Readable for BODCTRL {}
#[doc = "`write(|w| ..)` method takes [bodctrl::W](bodctrl::W) writer structure"]
impl crate::Writable for BODCTRL {}
#[doc = "BOD control register"]
pub mod bodctrl;
#[doc = "System tick timer calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systckcal](systckcal) module"]
pub type SYSTCKCAL = crate::Reg<u32, _SYSTCKCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTCKCAL;
#[doc = "`read()` method returns [systckcal::R](systckcal::R) reader structure"]
impl crate::Readable for SYSTCKCAL {}
#[doc = "`write(|w| ..)` method takes [systckcal::W](systckcal::W) writer structure"]
impl crate::Writable for SYSTCKCAL {}
#[doc = "System tick timer calibration register"]
pub mod systckcal;
#[doc = "IRQ latency register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqlatency](irqlatency) module"]
pub type IRQLATENCY = crate::Reg<u32, _IRQLATENCY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQLATENCY;
#[doc = "`read()` method returns [irqlatency::R](irqlatency::R) reader structure"]
impl crate::Readable for IRQLATENCY {}
#[doc = "`write(|w| ..)` method takes [irqlatency::W](irqlatency::W) writer structure"]
impl crate::Writable for IRQLATENCY {}
#[doc = "IRQ latency register"]
pub mod irqlatency;
#[doc = "NMI source selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmisrc](nmisrc) module"]
pub type NMISRC = crate::Reg<u32, _NMISRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMISRC;
#[doc = "`read()` method returns [nmisrc::R](nmisrc::R) reader structure"]
impl crate::Readable for NMISRC {}
#[doc = "`write(|w| ..)` method takes [nmisrc::W](nmisrc::W) writer structure"]
impl crate::Writable for NMISRC {}
#[doc = "NMI source selection register"]
pub mod nmisrc;
#[doc = "Pin interrupt select registers N\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintsel](pintsel) module"]
pub type PINTSEL = crate::Reg<u32, _PINTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINTSEL;
#[doc = "`read()` method returns [pintsel::R](pintsel::R) reader structure"]
impl crate::Readable for PINTSEL {}
#[doc = "`write(|w| ..)` method takes [pintsel::W](pintsel::W) writer structure"]
impl crate::Writable for PINTSEL {}
#[doc = "Pin interrupt select registers N"]
pub mod pintsel;
#[doc = "Start logic 0 pin wake-up enable register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterp0](starterp0) module"]
pub type STARTERP0 = crate::Reg<u32, _STARTERP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTERP0;
#[doc = "`read()` method returns [starterp0::R](starterp0::R) reader structure"]
impl crate::Readable for STARTERP0 {}
#[doc = "`write(|w| ..)` method takes [starterp0::W](starterp0::W) writer structure"]
impl crate::Writable for STARTERP0 {}
#[doc = "Start logic 0 pin wake-up enable register 0"]
pub mod starterp0;
#[doc = "Start logic 0 pin wake-up enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterp1](starterp1) module"]
pub type STARTERP1 = crate::Reg<u32, _STARTERP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTERP1;
#[doc = "`read()` method returns [starterp1::R](starterp1::R) reader structure"]
impl crate::Readable for STARTERP1 {}
#[doc = "`write(|w| ..)` method takes [starterp1::W](starterp1::W) writer structure"]
impl crate::Writable for STARTERP1 {}
#[doc = "Start logic 0 pin wake-up enable register 1"]
pub mod starterp1;
#[doc = "Deep-sleep configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsleepcfg](pdsleepcfg) module"]
pub type PDSLEEPCFG = crate::Reg<u32, _PDSLEEPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSLEEPCFG;
#[doc = "`read()` method returns [pdsleepcfg::R](pdsleepcfg::R) reader structure"]
impl crate::Readable for PDSLEEPCFG {}
#[doc = "`write(|w| ..)` method takes [pdsleepcfg::W](pdsleepcfg::W) writer structure"]
impl crate::Writable for PDSLEEPCFG {}
#[doc = "Deep-sleep configuration register"]
pub mod pdsleepcfg;
#[doc = "Wake-up configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdawakecfg](pdawakecfg) module"]
pub type PDAWAKECFG = crate::Reg<u32, _PDAWAKECFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAWAKECFG;
#[doc = "`read()` method returns [pdawakecfg::R](pdawakecfg::R) reader structure"]
impl crate::Readable for PDAWAKECFG {}
#[doc = "`write(|w| ..)` method takes [pdawakecfg::W](pdawakecfg::W) writer structure"]
impl crate::Writable for PDAWAKECFG {}
#[doc = "Wake-up configuration register"]
pub mod pdawakecfg;
#[doc = "Power configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfg](pdruncfg) module"]
pub type PDRUNCFG = crate::Reg<u32, _PDRUNCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFG;
#[doc = "`read()` method returns [pdruncfg::R](pdruncfg::R) reader structure"]
impl crate::Readable for PDRUNCFG {}
#[doc = "`write(|w| ..)` method takes [pdruncfg::W](pdruncfg::W) writer structure"]
impl crate::Writable for PDRUNCFG {}
#[doc = "Power configuration register"]
pub mod pdruncfg;
#[doc = "Part ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_id](device_id) module"]
pub type DEVICE_ID = crate::Reg<u32, _DEVICE_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_ID;
#[doc = "`read()` method returns [device_id::R](device_id::R) reader structure"]
impl crate::Readable for DEVICE_ID {}
#[doc = "Part ID register"]
pub mod device_id;
