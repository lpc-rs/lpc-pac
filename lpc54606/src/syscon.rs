#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - AHB multilayer matrix priority control"]
    pub ahbmatprio: AHBMATPRIO,
    _reserved1: [u8; 44usize],
    #[doc = "0x40 - System tick counter calibration"]
    pub systckcal: SYSTCKCAL,
    _reserved2: [u8; 4usize],
    #[doc = "0x48 - NMI Source Select"]
    pub nmisrc: NMISRC,
    #[doc = "0x4c - Asynchronous APB Control"]
    pub asyncapbctrl: ASYNCAPBCTRL,
    _reserved4: [u8; 112usize],
    #[doc = "0xc0 - POR captured value of port n"]
    pub pioporcap: [PIOPORCAP; 2],
    _reserved5: [u8; 8usize],
    #[doc = "0xd0 - Reset captured value of port n"]
    pub piorescap: [PIORESCAP; 2],
    _reserved6: [u8; 40usize],
    #[doc = "0x100 - Peripheral reset control n"]
    pub presetctrl0: PRESETCTRL0,
    #[doc = "0x104 - Peripheral reset control n"]
    pub presetctrl1: PRESETCTRL1,
    #[doc = "0x108 - Peripheral reset control n"]
    pub presetctrl2: PRESETCTRL2,
    _reserved9: [u8; 20usize],
    #[doc = "0x120 - Set bits in PRESETCTRLn"]
    pub presetctrlset: [PRESETCTRLSET; 3],
    _reserved10: [u8; 20usize],
    #[doc = "0x140 - Clear bits in PRESETCTRLn"]
    pub presetctrlclr: [PRESETCTRLCLR; 3],
    _reserved11: [u8; 164usize],
    #[doc = "0x1f0 - System reset status register"]
    pub sysrststat: SYSRSTSTAT,
    _reserved12: [u8; 12usize],
    #[doc = "0x200 - AHB Clock control n"]
    pub ahbclkctrl0: AHBCLKCTRL0,
    #[doc = "0x204 - AHB Clock control n"]
    pub ahbclkctrl1: AHBCLKCTRL1,
    #[doc = "0x208 - AHB Clock control n"]
    pub ahbclkctrl2: AHBCLKCTRL2,
    _reserved15: [u8; 20usize],
    #[doc = "0x220 - Set bits in AHBCLKCTRLn"]
    pub ahbclkctrlset: [AHBCLKCTRLSET; 3],
    _reserved16: [u8; 20usize],
    #[doc = "0x240 - Clear bits in AHBCLKCTRLn"]
    pub ahbclkctrlclr: [AHBCLKCTRLCLR; 3],
    _reserved17: [u8; 52usize],
    #[doc = "0x280 - Main clock source select A"]
    pub mainclksela: MAINCLKSELA,
    #[doc = "0x284 - Main clock source select B"]
    pub mainclkselb: MAINCLKSELB,
    #[doc = "0x288 - CLKOUT clock source select A"]
    pub clkoutsela: CLKOUTSELA,
    _reserved20: [u8; 4usize],
    #[doc = "0x290 - PLL clock source select"]
    pub syspllclksel: SYSPLLCLKSEL,
    _reserved21: [u8; 4usize],
    #[doc = "0x298 - Audio PLL clock source select"]
    pub audpllclksel: AUDPLLCLKSEL,
    _reserved22: [u8; 4usize],
    #[doc = "0x2a0 - SPIFI clock source select"]
    pub spificlksel: SPIFICLKSEL,
    #[doc = "0x2a4 - ADC clock source select"]
    pub adcclksel: ADCCLKSEL,
    #[doc = "0x2a8 - USB0 clock source select"]
    pub usb0clksel: USB0CLKSEL,
    #[doc = "0x2ac - USB1 clock source select"]
    pub usb1clksel: USB1CLKSEL,
    #[doc = "0x2b0 - Flexcomm 0 clock source select"]
    pub fclksel: [FCLKSEL; 10],
    _reserved27: [u8; 8usize],
    #[doc = "0x2e0 - MCLK clock source select"]
    pub mclkclksel: MCLKCLKSEL,
    _reserved28: [u8; 4usize],
    #[doc = "0x2e8 - Fractional Rate Generator clock source select"]
    pub frgclksel: FRGCLKSEL,
    #[doc = "0x2ec - Digital microphone (DMIC) subsystem clock select"]
    pub dmicclksel: DMICCLKSEL,
    #[doc = "0x2f0 - SCTimer/PWM clock source select"]
    pub sctclksel: SCTCLKSEL,
    #[doc = "0x2f4 - LCD clock source select"]
    pub lcdclksel: LCDCLKSEL,
    #[doc = "0x2f8 - SDIO clock source select"]
    pub sdioclksel: SDIOCLKSEL,
    _reserved33: [u8; 4usize],
    #[doc = "0x300 - SYSTICK clock divider"]
    pub systickclkdiv: SYSTICKCLKDIV,
    #[doc = "0x304 - ARM Trace clock divider"]
    pub armtraceclkdiv: ARMTRACECLKDIV,
    #[doc = "0x308 - MCAN0 clock divider"]
    pub can0clkdiv: CAN0CLKDIV,
    #[doc = "0x30c - MCAN1 clock divider"]
    pub can1clkdiv: CAN1CLKDIV,
    #[doc = "0x310 - Smartcard0 clock divider"]
    pub sc0clkdiv: SC0CLKDIV,
    #[doc = "0x314 - Smartcard1 clock divider"]
    pub sc1clkdiv: SC1CLKDIV,
    _reserved39: [u8; 104usize],
    #[doc = "0x380 - AHB clock divider"]
    pub ahbclkdiv: AHBCLKDIV,
    #[doc = "0x384 - CLKOUT clock divider"]
    pub clkoutdiv: CLKOUTDIV,
    #[doc = "0x388 - FROHF clock divider"]
    pub frohfclkdiv: FROHFCLKDIV,
    _reserved42: [u8; 4usize],
    #[doc = "0x390 - SPIFI clock divider"]
    pub spificlkdiv: SPIFICLKDIV,
    #[doc = "0x394 - ADC clock divider"]
    pub adcclkdiv: ADCCLKDIV,
    #[doc = "0x398 - USB0 clock divider"]
    pub usb0clkdiv: USB0CLKDIV,
    #[doc = "0x39c - USB1 clock divider"]
    pub usb1clkdiv: USB1CLKDIV,
    #[doc = "0x3a0 - Fractional rate divider"]
    pub frgctrl: FRGCTRL,
    _reserved47: [u8; 4usize],
    #[doc = "0x3a8 - DMIC clock divider"]
    pub dmicclkdiv: DMICCLKDIV,
    #[doc = "0x3ac - I2S MCLK clock divider"]
    pub mclkdiv: MCLKDIV,
    #[doc = "0x3b0 - LCD clock divider"]
    pub lcdclkdiv: LCDCLKDIV,
    #[doc = "0x3b4 - SCT/PWM clock divider"]
    pub sctclkdiv: SCTCLKDIV,
    #[doc = "0x3b8 - EMC clock divider"]
    pub emcclkdiv: EMCCLKDIV,
    #[doc = "0x3bc - SDIO clock divider"]
    pub sdioclkdiv: SDIOCLKDIV,
    _reserved53: [u8; 64usize],
    #[doc = "0x400 - Flash wait states configuration"]
    pub flashcfg: FLASHCFG,
    _reserved54: [u8; 8usize],
    #[doc = "0x40c - USB0 clock control"]
    pub usb0clkctrl: USB0CLKCTRL,
    #[doc = "0x410 - USB0 clock status"]
    pub usb0clkstat: USB0CLKSTAT,
    _reserved56: [u8; 4usize],
    #[doc = "0x418 - Frequency measure register"]
    pub freqmectrl: FREQMECTRL,
    _reserved57: [u8; 4usize],
    #[doc = "0x420 - MCLK input/output control"]
    pub mclkio: MCLKIO,
    #[doc = "0x424 - USB1 clock control"]
    pub usb1clkctrl: USB1CLKCTRL,
    #[doc = "0x428 - USB1 clock status"]
    pub usb1clkstat: USB1CLKSTAT,
    _reserved60: [u8; 24usize],
    #[doc = "0x444 - EMC system control"]
    pub emcsysctrl: EMCSYSCTRL,
    #[doc = "0x448 - EMC clock delay control"]
    pub emcdlyctrl: EMCDLYCTRL,
    #[doc = "0x44c - EMC delay chain calibration control"]
    pub emcdlycal: EMCDLYCAL,
    #[doc = "0x450 - Ethernet PHY Selection"]
    pub ethphysel: ETHPHYSEL,
    #[doc = "0x454 - Ethernet SBD flow control"]
    pub ethsbdctrl: ETHSBDCTRL,
    _reserved65: [u8; 8usize],
    #[doc = "0x460 - SDIO CCLKIN phase and delay control"]
    pub sdioclkctrl: SDIOCLKCTRL,
    _reserved66: [u8; 156usize],
    #[doc = "0x500 - FRO oscillator control"]
    pub froctrl: FROCTRL,
    #[doc = "0x504 - System oscillator control"]
    pub sysoscctrl: SYSOSCCTRL,
    #[doc = "0x508 - Watchdog oscillator control"]
    pub wdtoscctrl: WDTOSCCTRL,
    #[doc = "0x50c - RTC oscillator 32 kHz output control"]
    pub rtcoscctrl: RTCOSCCTRL,
    _reserved70: [u8; 12usize],
    #[doc = "0x51c - USB PLL control"]
    pub usbpllctrl: USBPLLCTRL,
    #[doc = "0x520 - USB PLL status"]
    pub usbpllstat: USBPLLSTAT,
    _reserved72: [u8; 92usize],
    #[doc = "0x580 - System PLL control"]
    pub syspllctrl: SYSPLLCTRL,
    #[doc = "0x584 - PLL status"]
    pub syspllstat: SYSPLLSTAT,
    #[doc = "0x588 - PLL N divider"]
    pub syspllndec: SYSPLLNDEC,
    #[doc = "0x58c - PLL P divider"]
    pub syspllpdec: SYSPLLPDEC,
    #[doc = "0x590 - System PLL M divider"]
    pub syspllmdec: SYSPLLMDEC,
    _reserved77: [u8; 12usize],
    #[doc = "0x5a0 - Audio PLL control"]
    pub audpllctrl: AUDPLLCTRL,
    #[doc = "0x5a4 - Audio PLL status"]
    pub audpllstat: AUDPLLSTAT,
    #[doc = "0x5a8 - Audio PLL N divider"]
    pub audpllndec: AUDPLLNDEC,
    #[doc = "0x5ac - Audio PLL P divider"]
    pub audpllpdec: AUDPLLPDEC,
    #[doc = "0x5b0 - Audio PLL M divider"]
    pub audpllmdec: AUDPLLMDEC,
    #[doc = "0x5b4 - Audio PLL fractional divider control"]
    pub audpllfrac: AUDPLLFRAC,
    _reserved83: [u8; 72usize],
    #[doc = "0x600 - Sleep configuration register"]
    pub pdsleepcfg0: PDSLEEPCFG0,
    #[doc = "0x604 - Sleep configuration register"]
    pub pdsleepcfg1: PDSLEEPCFG1,
    _reserved85: [u8; 8usize],
    #[doc = "0x610 - Power configuration register"]
    pub pdruncfg0: PDRUNCFG0,
    #[doc = "0x614 - Power configuration register"]
    pub pdruncfg1: PDRUNCFG1,
    _reserved87: [u8; 8usize],
    #[doc = "0x620 - Power configuration set register"]
    pub pdruncfgset0: PDRUNCFGSET0,
    #[doc = "0x624 - Power configuration set register"]
    pub pdruncfgset1: PDRUNCFGSET1,
    _reserved89: [u8; 8usize],
    #[doc = "0x630 - Power configuration clear register"]
    pub pdruncfgclr0: PDRUNCFGCLR0,
    #[doc = "0x634 - Power configuration clear register"]
    pub pdruncfgclr1: PDRUNCFGCLR1,
    _reserved91: [u8; 72usize],
    #[doc = "0x680 - Start logic 0 wake-up enable register"]
    pub starter0: STARTER0,
    #[doc = "0x684 - Start logic 0 wake-up enable register"]
    pub starter1: STARTER1,
    _reserved93: [u8; 24usize],
    #[doc = "0x6a0 - Set bits in STARTER"]
    pub starterset: [STARTERSET; 2],
    _reserved94: [u8; 24usize],
    #[doc = "0x6c0 - Clear bits in STARTER0"]
    pub starterclr: [STARTERCLR; 2],
    _reserved95: [u8; 184usize],
    #[doc = "0x780 - Configures special cases of hardware wake-up"]
    pub hwwake: HWWAKE,
    _reserved96: [u8; 1664usize],
    #[doc = "0xe04 - Auto Clock-Gate Override Register"]
    pub autocgor: AUTOCGOR,
    _reserved97: [u8; 492usize],
    #[doc = "0xff4 - JTAG ID code register"]
    pub jtagidcode: JTAGIDCODE,
    #[doc = "0xff8 - Part ID register"]
    pub device_id0: DEVICE_ID0,
    #[doc = "0xffc - Boot ROM and die revision register"]
    pub device_id1: DEVICE_ID1,
    _reserved100: [u8; 127044usize],
    #[doc = "0x20044 - Brown-Out Detect control"]
    pub bodctrl: BODCTRL,
}
#[doc = "AHB multilayer matrix priority control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbmatprio](ahbmatprio) module"]
pub type AHBMATPRIO = crate::Reg<u32, _AHBMATPRIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBMATPRIO;
#[doc = "`read()` method returns [ahbmatprio::R](ahbmatprio::R) reader structure"]
impl crate::Readable for AHBMATPRIO {}
#[doc = "`write(|w| ..)` method takes [ahbmatprio::W](ahbmatprio::W) writer structure"]
impl crate::Writable for AHBMATPRIO {}
#[doc = "AHB multilayer matrix priority control"]
pub mod ahbmatprio;
#[doc = "System tick counter calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systckcal](systckcal) module"]
pub type SYSTCKCAL = crate::Reg<u32, _SYSTCKCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTCKCAL;
#[doc = "`read()` method returns [systckcal::R](systckcal::R) reader structure"]
impl crate::Readable for SYSTCKCAL {}
#[doc = "`write(|w| ..)` method takes [systckcal::W](systckcal::W) writer structure"]
impl crate::Writable for SYSTCKCAL {}
#[doc = "System tick counter calibration"]
pub mod systckcal;
#[doc = "NMI Source Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmisrc](nmisrc) module"]
pub type NMISRC = crate::Reg<u32, _NMISRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMISRC;
#[doc = "`read()` method returns [nmisrc::R](nmisrc::R) reader structure"]
impl crate::Readable for NMISRC {}
#[doc = "`write(|w| ..)` method takes [nmisrc::W](nmisrc::W) writer structure"]
impl crate::Writable for NMISRC {}
#[doc = "NMI Source Select"]
pub mod nmisrc;
#[doc = "Asynchronous APB Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncapbctrl](asyncapbctrl) module"]
pub type ASYNCAPBCTRL = crate::Reg<u32, _ASYNCAPBCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCAPBCTRL;
#[doc = "`read()` method returns [asyncapbctrl::R](asyncapbctrl::R) reader structure"]
impl crate::Readable for ASYNCAPBCTRL {}
#[doc = "`write(|w| ..)` method takes [asyncapbctrl::W](asyncapbctrl::W) writer structure"]
impl crate::Writable for ASYNCAPBCTRL {}
#[doc = "Asynchronous APB Control"]
pub mod asyncapbctrl;
#[doc = "POR captured value of port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pioporcap](pioporcap) module"]
pub type PIOPORCAP = crate::Reg<u32, _PIOPORCAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIOPORCAP;
#[doc = "`read()` method returns [pioporcap::R](pioporcap::R) reader structure"]
impl crate::Readable for PIOPORCAP {}
#[doc = "POR captured value of port n"]
pub mod pioporcap;
#[doc = "Reset captured value of port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [piorescap](piorescap) module"]
pub type PIORESCAP = crate::Reg<u32, _PIORESCAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIORESCAP;
#[doc = "`read()` method returns [piorescap::R](piorescap::R) reader structure"]
impl crate::Readable for PIORESCAP {}
#[doc = "Reset captured value of port n"]
pub mod piorescap;
#[doc = "Peripheral reset control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl0](presetctrl0) module"]
pub type PRESETCTRL0 = crate::Reg<u32, _PRESETCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRL0;
#[doc = "`read()` method returns [presetctrl0::R](presetctrl0::R) reader structure"]
impl crate::Readable for PRESETCTRL0 {}
#[doc = "`write(|w| ..)` method takes [presetctrl0::W](presetctrl0::W) writer structure"]
impl crate::Writable for PRESETCTRL0 {}
#[doc = "Peripheral reset control n"]
pub mod presetctrl0;
#[doc = "Peripheral reset control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl1](presetctrl1) module"]
pub type PRESETCTRL1 = crate::Reg<u32, _PRESETCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRL1;
#[doc = "`read()` method returns [presetctrl1::R](presetctrl1::R) reader structure"]
impl crate::Readable for PRESETCTRL1 {}
#[doc = "`write(|w| ..)` method takes [presetctrl1::W](presetctrl1::W) writer structure"]
impl crate::Writable for PRESETCTRL1 {}
#[doc = "Peripheral reset control n"]
pub mod presetctrl1;
#[doc = "Peripheral reset control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl2](presetctrl2) module"]
pub type PRESETCTRL2 = crate::Reg<u32, _PRESETCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRL2;
#[doc = "`read()` method returns [presetctrl2::R](presetctrl2::R) reader structure"]
impl crate::Readable for PRESETCTRL2 {}
#[doc = "`write(|w| ..)` method takes [presetctrl2::W](presetctrl2::W) writer structure"]
impl crate::Writable for PRESETCTRL2 {}
#[doc = "Peripheral reset control n"]
pub mod presetctrl2;
#[doc = "Set bits in PRESETCTRLn\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrlset](presetctrlset) module"]
pub type PRESETCTRLSET = crate::Reg<u32, _PRESETCTRLSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRLSET;
#[doc = "`write(|w| ..)` method takes [presetctrlset::W](presetctrlset::W) writer structure"]
impl crate::Writable for PRESETCTRLSET {}
#[doc = "Set bits in PRESETCTRLn"]
pub mod presetctrlset;
#[doc = "Clear bits in PRESETCTRLn\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrlclr](presetctrlclr) module"]
pub type PRESETCTRLCLR = crate::Reg<u32, _PRESETCTRLCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESETCTRLCLR;
#[doc = "`write(|w| ..)` method takes [presetctrlclr::W](presetctrlclr::W) writer structure"]
impl crate::Writable for PRESETCTRLCLR {}
#[doc = "Clear bits in PRESETCTRLn"]
pub mod presetctrlclr;
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
#[doc = "AHB Clock control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl0](ahbclkctrl0) module"]
pub type AHBCLKCTRL0 = crate::Reg<u32, _AHBCLKCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKCTRL0;
#[doc = "`read()` method returns [ahbclkctrl0::R](ahbclkctrl0::R) reader structure"]
impl crate::Readable for AHBCLKCTRL0 {}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl0::W](ahbclkctrl0::W) writer structure"]
impl crate::Writable for AHBCLKCTRL0 {}
#[doc = "AHB Clock control n"]
pub mod ahbclkctrl0;
#[doc = "AHB Clock control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl1](ahbclkctrl1) module"]
pub type AHBCLKCTRL1 = crate::Reg<u32, _AHBCLKCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKCTRL1;
#[doc = "`read()` method returns [ahbclkctrl1::R](ahbclkctrl1::R) reader structure"]
impl crate::Readable for AHBCLKCTRL1 {}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl1::W](ahbclkctrl1::W) writer structure"]
impl crate::Writable for AHBCLKCTRL1 {}
#[doc = "AHB Clock control n"]
pub mod ahbclkctrl1;
#[doc = "AHB Clock control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl2](ahbclkctrl2) module"]
pub type AHBCLKCTRL2 = crate::Reg<u32, _AHBCLKCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKCTRL2;
#[doc = "`read()` method returns [ahbclkctrl2::R](ahbclkctrl2::R) reader structure"]
impl crate::Readable for AHBCLKCTRL2 {}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl2::W](ahbclkctrl2::W) writer structure"]
impl crate::Writable for AHBCLKCTRL2 {}
#[doc = "AHB Clock control n"]
pub mod ahbclkctrl2;
#[doc = "Set bits in AHBCLKCTRLn\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrlset](ahbclkctrlset) module"]
pub type AHBCLKCTRLSET = crate::Reg<u32, _AHBCLKCTRLSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKCTRLSET;
#[doc = "`write(|w| ..)` method takes [ahbclkctrlset::W](ahbclkctrlset::W) writer structure"]
impl crate::Writable for AHBCLKCTRLSET {}
#[doc = "Set bits in AHBCLKCTRLn"]
pub mod ahbclkctrlset;
#[doc = "Clear bits in AHBCLKCTRLn\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrlclr](ahbclkctrlclr) module"]
pub type AHBCLKCTRLCLR = crate::Reg<u32, _AHBCLKCTRLCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKCTRLCLR;
#[doc = "`write(|w| ..)` method takes [ahbclkctrlclr::W](ahbclkctrlclr::W) writer structure"]
impl crate::Writable for AHBCLKCTRLCLR {}
#[doc = "Clear bits in AHBCLKCTRLn"]
pub mod ahbclkctrlclr;
#[doc = "Main clock source select A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclksela](mainclksela) module"]
pub type MAINCLKSELA = crate::Reg<u32, _MAINCLKSELA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINCLKSELA;
#[doc = "`read()` method returns [mainclksela::R](mainclksela::R) reader structure"]
impl crate::Readable for MAINCLKSELA {}
#[doc = "`write(|w| ..)` method takes [mainclksela::W](mainclksela::W) writer structure"]
impl crate::Writable for MAINCLKSELA {}
#[doc = "Main clock source select A"]
pub mod mainclksela;
#[doc = "Main clock source select B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclkselb](mainclkselb) module"]
pub type MAINCLKSELB = crate::Reg<u32, _MAINCLKSELB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINCLKSELB;
#[doc = "`read()` method returns [mainclkselb::R](mainclkselb::R) reader structure"]
impl crate::Readable for MAINCLKSELB {}
#[doc = "`write(|w| ..)` method takes [mainclkselb::W](mainclkselb::W) writer structure"]
impl crate::Writable for MAINCLKSELB {}
#[doc = "Main clock source select B"]
pub mod mainclkselb;
#[doc = "CLKOUT clock source select A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutsela](clkoutsela) module"]
pub type CLKOUTSELA = crate::Reg<u32, _CLKOUTSELA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTSELA;
#[doc = "`read()` method returns [clkoutsela::R](clkoutsela::R) reader structure"]
impl crate::Readable for CLKOUTSELA {}
#[doc = "`write(|w| ..)` method takes [clkoutsela::W](clkoutsela::W) writer structure"]
impl crate::Writable for CLKOUTSELA {}
#[doc = "CLKOUT clock source select A"]
pub mod clkoutsela;
#[doc = "PLL clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllclksel](syspllclksel) module"]
pub type SYSPLLCLKSEL = crate::Reg<u32, _SYSPLLCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLCLKSEL;
#[doc = "`read()` method returns [syspllclksel::R](syspllclksel::R) reader structure"]
impl crate::Readable for SYSPLLCLKSEL {}
#[doc = "`write(|w| ..)` method takes [syspllclksel::W](syspllclksel::W) writer structure"]
impl crate::Writable for SYSPLLCLKSEL {}
#[doc = "PLL clock source select"]
pub mod syspllclksel;
#[doc = "Audio PLL clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllclksel](audpllclksel) module"]
pub type AUDPLLCLKSEL = crate::Reg<u32, _AUDPLLCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDPLLCLKSEL;
#[doc = "`read()` method returns [audpllclksel::R](audpllclksel::R) reader structure"]
impl crate::Readable for AUDPLLCLKSEL {}
#[doc = "`write(|w| ..)` method takes [audpllclksel::W](audpllclksel::W) writer structure"]
impl crate::Writable for AUDPLLCLKSEL {}
#[doc = "Audio PLL clock source select"]
pub mod audpllclksel;
#[doc = "SPIFI clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spificlksel](spificlksel) module"]
pub type SPIFICLKSEL = crate::Reg<u32, _SPIFICLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPIFICLKSEL;
#[doc = "`read()` method returns [spificlksel::R](spificlksel::R) reader structure"]
impl crate::Readable for SPIFICLKSEL {}
#[doc = "`write(|w| ..)` method takes [spificlksel::W](spificlksel::W) writer structure"]
impl crate::Writable for SPIFICLKSEL {}
#[doc = "SPIFI clock source select"]
pub mod spificlksel;
#[doc = "ADC clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclksel](adcclksel) module"]
pub type ADCCLKSEL = crate::Reg<u32, _ADCCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCLKSEL;
#[doc = "`read()` method returns [adcclksel::R](adcclksel::R) reader structure"]
impl crate::Readable for ADCCLKSEL {}
#[doc = "`write(|w| ..)` method takes [adcclksel::W](adcclksel::W) writer structure"]
impl crate::Writable for ADCCLKSEL {}
#[doc = "ADC clock source select"]
pub mod adcclksel;
#[doc = "USB0 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0clksel](usb0clksel) module"]
pub type USB0CLKSEL = crate::Reg<u32, _USB0CLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB0CLKSEL;
#[doc = "`read()` method returns [usb0clksel::R](usb0clksel::R) reader structure"]
impl crate::Readable for USB0CLKSEL {}
#[doc = "`write(|w| ..)` method takes [usb0clksel::W](usb0clksel::W) writer structure"]
impl crate::Writable for USB0CLKSEL {}
#[doc = "USB0 clock source select"]
pub mod usb0clksel;
#[doc = "USB1 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1clksel](usb1clksel) module"]
pub type USB1CLKSEL = crate::Reg<u32, _USB1CLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1CLKSEL;
#[doc = "`read()` method returns [usb1clksel::R](usb1clksel::R) reader structure"]
impl crate::Readable for USB1CLKSEL {}
#[doc = "`write(|w| ..)` method takes [usb1clksel::W](usb1clksel::W) writer structure"]
impl crate::Writable for USB1CLKSEL {}
#[doc = "USB1 clock source select"]
pub mod usb1clksel;
#[doc = "Flexcomm 0 clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fclksel](fclksel) module"]
pub type FCLKSEL = crate::Reg<u32, _FCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCLKSEL;
#[doc = "`read()` method returns [fclksel::R](fclksel::R) reader structure"]
impl crate::Readable for FCLKSEL {}
#[doc = "`write(|w| ..)` method takes [fclksel::W](fclksel::W) writer structure"]
impl crate::Writable for FCLKSEL {}
#[doc = "Flexcomm 0 clock source select"]
pub mod fclksel;
#[doc = "MCLK clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkclksel](mclkclksel) module"]
pub type MCLKCLKSEL = crate::Reg<u32, _MCLKCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCLKCLKSEL;
#[doc = "`read()` method returns [mclkclksel::R](mclkclksel::R) reader structure"]
impl crate::Readable for MCLKCLKSEL {}
#[doc = "`write(|w| ..)` method takes [mclkclksel::W](mclkclksel::W) writer structure"]
impl crate::Writable for MCLKCLKSEL {}
#[doc = "MCLK clock source select"]
pub mod mclkclksel;
#[doc = "Fractional Rate Generator clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frgclksel](frgclksel) module"]
pub type FRGCLKSEL = crate::Reg<u32, _FRGCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRGCLKSEL;
#[doc = "`read()` method returns [frgclksel::R](frgclksel::R) reader structure"]
impl crate::Readable for FRGCLKSEL {}
#[doc = "`write(|w| ..)` method takes [frgclksel::W](frgclksel::W) writer structure"]
impl crate::Writable for FRGCLKSEL {}
#[doc = "Fractional Rate Generator clock source select"]
pub mod frgclksel;
#[doc = "Digital microphone (DMIC) subsystem clock select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmicclksel](dmicclksel) module"]
pub type DMICCLKSEL = crate::Reg<u32, _DMICCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMICCLKSEL;
#[doc = "`read()` method returns [dmicclksel::R](dmicclksel::R) reader structure"]
impl crate::Readable for DMICCLKSEL {}
#[doc = "`write(|w| ..)` method takes [dmicclksel::W](dmicclksel::W) writer structure"]
impl crate::Writable for DMICCLKSEL {}
#[doc = "Digital microphone (DMIC) subsystem clock select"]
pub mod dmicclksel;
#[doc = "SCTimer/PWM clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctclksel](sctclksel) module"]
pub type SCTCLKSEL = crate::Reg<u32, _SCTCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCLKSEL;
#[doc = "`read()` method returns [sctclksel::R](sctclksel::R) reader structure"]
impl crate::Readable for SCTCLKSEL {}
#[doc = "`write(|w| ..)` method takes [sctclksel::W](sctclksel::W) writer structure"]
impl crate::Writable for SCTCLKSEL {}
#[doc = "SCTimer/PWM clock source select"]
pub mod sctclksel;
#[doc = "LCD clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdclksel](lcdclksel) module"]
pub type LCDCLKSEL = crate::Reg<u32, _LCDCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCLKSEL;
#[doc = "`read()` method returns [lcdclksel::R](lcdclksel::R) reader structure"]
impl crate::Readable for LCDCLKSEL {}
#[doc = "`write(|w| ..)` method takes [lcdclksel::W](lcdclksel::W) writer structure"]
impl crate::Writable for LCDCLKSEL {}
#[doc = "LCD clock source select"]
pub mod lcdclksel;
#[doc = "SDIO clock source select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdioclksel](sdioclksel) module"]
pub type SDIOCLKSEL = crate::Reg<u32, _SDIOCLKSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIOCLKSEL;
#[doc = "`read()` method returns [sdioclksel::R](sdioclksel::R) reader structure"]
impl crate::Readable for SDIOCLKSEL {}
#[doc = "`write(|w| ..)` method takes [sdioclksel::W](sdioclksel::W) writer structure"]
impl crate::Writable for SDIOCLKSEL {}
#[doc = "SDIO clock source select"]
pub mod sdioclksel;
#[doc = "SYSTICK clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systickclkdiv](systickclkdiv) module"]
pub type SYSTICKCLKDIV = crate::Reg<u32, _SYSTICKCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTICKCLKDIV;
#[doc = "`read()` method returns [systickclkdiv::R](systickclkdiv::R) reader structure"]
impl crate::Readable for SYSTICKCLKDIV {}
#[doc = "`write(|w| ..)` method takes [systickclkdiv::W](systickclkdiv::W) writer structure"]
impl crate::Writable for SYSTICKCLKDIV {}
#[doc = "SYSTICK clock divider"]
pub mod systickclkdiv;
#[doc = "ARM Trace clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [armtraceclkdiv](armtraceclkdiv) module"]
pub type ARMTRACECLKDIV = crate::Reg<u32, _ARMTRACECLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARMTRACECLKDIV;
#[doc = "`read()` method returns [armtraceclkdiv::R](armtraceclkdiv::R) reader structure"]
impl crate::Readable for ARMTRACECLKDIV {}
#[doc = "`write(|w| ..)` method takes [armtraceclkdiv::W](armtraceclkdiv::W) writer structure"]
impl crate::Writable for ARMTRACECLKDIV {}
#[doc = "ARM Trace clock divider"]
pub mod armtraceclkdiv;
#[doc = "MCAN0 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can0clkdiv](can0clkdiv) module"]
pub type CAN0CLKDIV = crate::Reg<u32, _CAN0CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN0CLKDIV;
#[doc = "`read()` method returns [can0clkdiv::R](can0clkdiv::R) reader structure"]
impl crate::Readable for CAN0CLKDIV {}
#[doc = "`write(|w| ..)` method takes [can0clkdiv::W](can0clkdiv::W) writer structure"]
impl crate::Writable for CAN0CLKDIV {}
#[doc = "MCAN0 clock divider"]
pub mod can0clkdiv;
#[doc = "MCAN1 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can1clkdiv](can1clkdiv) module"]
pub type CAN1CLKDIV = crate::Reg<u32, _CAN1CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN1CLKDIV;
#[doc = "`read()` method returns [can1clkdiv::R](can1clkdiv::R) reader structure"]
impl crate::Readable for CAN1CLKDIV {}
#[doc = "`write(|w| ..)` method takes [can1clkdiv::W](can1clkdiv::W) writer structure"]
impl crate::Writable for CAN1CLKDIV {}
#[doc = "MCAN1 clock divider"]
pub mod can1clkdiv;
#[doc = "Smartcard0 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc0clkdiv](sc0clkdiv) module"]
pub type SC0CLKDIV = crate::Reg<u32, _SC0CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC0CLKDIV;
#[doc = "`read()` method returns [sc0clkdiv::R](sc0clkdiv::R) reader structure"]
impl crate::Readable for SC0CLKDIV {}
#[doc = "`write(|w| ..)` method takes [sc0clkdiv::W](sc0clkdiv::W) writer structure"]
impl crate::Writable for SC0CLKDIV {}
#[doc = "Smartcard0 clock divider"]
pub mod sc0clkdiv;
#[doc = "Smartcard1 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc1clkdiv](sc1clkdiv) module"]
pub type SC1CLKDIV = crate::Reg<u32, _SC1CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC1CLKDIV;
#[doc = "`read()` method returns [sc1clkdiv::R](sc1clkdiv::R) reader structure"]
impl crate::Readable for SC1CLKDIV {}
#[doc = "`write(|w| ..)` method takes [sc1clkdiv::W](sc1clkdiv::W) writer structure"]
impl crate::Writable for SC1CLKDIV {}
#[doc = "Smartcard1 clock divider"]
pub mod sc1clkdiv;
#[doc = "AHB clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkdiv](ahbclkdiv) module"]
pub type AHBCLKDIV = crate::Reg<u32, _AHBCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBCLKDIV;
#[doc = "`read()` method returns [ahbclkdiv::R](ahbclkdiv::R) reader structure"]
impl crate::Readable for AHBCLKDIV {}
#[doc = "`write(|w| ..)` method takes [ahbclkdiv::W](ahbclkdiv::W) writer structure"]
impl crate::Writable for AHBCLKDIV {}
#[doc = "AHB clock divider"]
pub mod ahbclkdiv;
#[doc = "CLKOUT clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutdiv](clkoutdiv) module"]
pub type CLKOUTDIV = crate::Reg<u32, _CLKOUTDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTDIV;
#[doc = "`read()` method returns [clkoutdiv::R](clkoutdiv::R) reader structure"]
impl crate::Readable for CLKOUTDIV {}
#[doc = "`write(|w| ..)` method takes [clkoutdiv::W](clkoutdiv::W) writer structure"]
impl crate::Writable for CLKOUTDIV {}
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "FROHF clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frohfclkdiv](frohfclkdiv) module"]
pub type FROHFCLKDIV = crate::Reg<u32, _FROHFCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FROHFCLKDIV;
#[doc = "`read()` method returns [frohfclkdiv::R](frohfclkdiv::R) reader structure"]
impl crate::Readable for FROHFCLKDIV {}
#[doc = "`write(|w| ..)` method takes [frohfclkdiv::W](frohfclkdiv::W) writer structure"]
impl crate::Writable for FROHFCLKDIV {}
#[doc = "FROHF clock divider"]
pub mod frohfclkdiv;
#[doc = "SPIFI clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spificlkdiv](spificlkdiv) module"]
pub type SPIFICLKDIV = crate::Reg<u32, _SPIFICLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPIFICLKDIV;
#[doc = "`read()` method returns [spificlkdiv::R](spificlkdiv::R) reader structure"]
impl crate::Readable for SPIFICLKDIV {}
#[doc = "`write(|w| ..)` method takes [spificlkdiv::W](spificlkdiv::W) writer structure"]
impl crate::Writable for SPIFICLKDIV {}
#[doc = "SPIFI clock divider"]
pub mod spificlkdiv;
#[doc = "ADC clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclkdiv](adcclkdiv) module"]
pub type ADCCLKDIV = crate::Reg<u32, _ADCCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCLKDIV;
#[doc = "`read()` method returns [adcclkdiv::R](adcclkdiv::R) reader structure"]
impl crate::Readable for ADCCLKDIV {}
#[doc = "`write(|w| ..)` method takes [adcclkdiv::W](adcclkdiv::W) writer structure"]
impl crate::Writable for ADCCLKDIV {}
#[doc = "ADC clock divider"]
pub mod adcclkdiv;
#[doc = "USB0 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0clkdiv](usb0clkdiv) module"]
pub type USB0CLKDIV = crate::Reg<u32, _USB0CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB0CLKDIV;
#[doc = "`read()` method returns [usb0clkdiv::R](usb0clkdiv::R) reader structure"]
impl crate::Readable for USB0CLKDIV {}
#[doc = "`write(|w| ..)` method takes [usb0clkdiv::W](usb0clkdiv::W) writer structure"]
impl crate::Writable for USB0CLKDIV {}
#[doc = "USB0 clock divider"]
pub mod usb0clkdiv;
#[doc = "USB1 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1clkdiv](usb1clkdiv) module"]
pub type USB1CLKDIV = crate::Reg<u32, _USB1CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1CLKDIV;
#[doc = "`read()` method returns [usb1clkdiv::R](usb1clkdiv::R) reader structure"]
impl crate::Readable for USB1CLKDIV {}
#[doc = "`write(|w| ..)` method takes [usb1clkdiv::W](usb1clkdiv::W) writer structure"]
impl crate::Writable for USB1CLKDIV {}
#[doc = "USB1 clock divider"]
pub mod usb1clkdiv;
#[doc = "Fractional rate divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frgctrl](frgctrl) module"]
pub type FRGCTRL = crate::Reg<u32, _FRGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRGCTRL;
#[doc = "`read()` method returns [frgctrl::R](frgctrl::R) reader structure"]
impl crate::Readable for FRGCTRL {}
#[doc = "`write(|w| ..)` method takes [frgctrl::W](frgctrl::W) writer structure"]
impl crate::Writable for FRGCTRL {}
#[doc = "Fractional rate divider"]
pub mod frgctrl;
#[doc = "DMIC clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmicclkdiv](dmicclkdiv) module"]
pub type DMICCLKDIV = crate::Reg<u32, _DMICCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMICCLKDIV;
#[doc = "`read()` method returns [dmicclkdiv::R](dmicclkdiv::R) reader structure"]
impl crate::Readable for DMICCLKDIV {}
#[doc = "`write(|w| ..)` method takes [dmicclkdiv::W](dmicclkdiv::W) writer structure"]
impl crate::Writable for DMICCLKDIV {}
#[doc = "DMIC clock divider"]
pub mod dmicclkdiv;
#[doc = "I2S MCLK clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkdiv](mclkdiv) module"]
pub type MCLKDIV = crate::Reg<u32, _MCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCLKDIV;
#[doc = "`read()` method returns [mclkdiv::R](mclkdiv::R) reader structure"]
impl crate::Readable for MCLKDIV {}
#[doc = "`write(|w| ..)` method takes [mclkdiv::W](mclkdiv::W) writer structure"]
impl crate::Writable for MCLKDIV {}
#[doc = "I2S MCLK clock divider"]
pub mod mclkdiv;
#[doc = "LCD clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdclkdiv](lcdclkdiv) module"]
pub type LCDCLKDIV = crate::Reg<u32, _LCDCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCDCLKDIV;
#[doc = "`read()` method returns [lcdclkdiv::R](lcdclkdiv::R) reader structure"]
impl crate::Readable for LCDCLKDIV {}
#[doc = "`write(|w| ..)` method takes [lcdclkdiv::W](lcdclkdiv::W) writer structure"]
impl crate::Writable for LCDCLKDIV {}
#[doc = "LCD clock divider"]
pub mod lcdclkdiv;
#[doc = "SCT/PWM clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctclkdiv](sctclkdiv) module"]
pub type SCTCLKDIV = crate::Reg<u32, _SCTCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCLKDIV;
#[doc = "`read()` method returns [sctclkdiv::R](sctclkdiv::R) reader structure"]
impl crate::Readable for SCTCLKDIV {}
#[doc = "`write(|w| ..)` method takes [sctclkdiv::W](sctclkdiv::W) writer structure"]
impl crate::Writable for SCTCLKDIV {}
#[doc = "SCT/PWM clock divider"]
pub mod sctclkdiv;
#[doc = "EMC clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcclkdiv](emcclkdiv) module"]
pub type EMCCLKDIV = crate::Reg<u32, _EMCCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMCCLKDIV;
#[doc = "`read()` method returns [emcclkdiv::R](emcclkdiv::R) reader structure"]
impl crate::Readable for EMCCLKDIV {}
#[doc = "`write(|w| ..)` method takes [emcclkdiv::W](emcclkdiv::W) writer structure"]
impl crate::Writable for EMCCLKDIV {}
#[doc = "EMC clock divider"]
pub mod emcclkdiv;
#[doc = "SDIO clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdioclkdiv](sdioclkdiv) module"]
pub type SDIOCLKDIV = crate::Reg<u32, _SDIOCLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIOCLKDIV;
#[doc = "`read()` method returns [sdioclkdiv::R](sdioclkdiv::R) reader structure"]
impl crate::Readable for SDIOCLKDIV {}
#[doc = "`write(|w| ..)` method takes [sdioclkdiv::W](sdioclkdiv::W) writer structure"]
impl crate::Writable for SDIOCLKDIV {}
#[doc = "SDIO clock divider"]
pub mod sdioclkdiv;
#[doc = "Flash wait states configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](flashcfg) module"]
pub type FLASHCFG = crate::Reg<u32, _FLASHCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCFG;
#[doc = "`read()` method returns [flashcfg::R](flashcfg::R) reader structure"]
impl crate::Readable for FLASHCFG {}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](flashcfg::W) writer structure"]
impl crate::Writable for FLASHCFG {}
#[doc = "Flash wait states configuration"]
pub mod flashcfg;
#[doc = "USB0 clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0clkctrl](usb0clkctrl) module"]
pub type USB0CLKCTRL = crate::Reg<u32, _USB0CLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB0CLKCTRL;
#[doc = "`read()` method returns [usb0clkctrl::R](usb0clkctrl::R) reader structure"]
impl crate::Readable for USB0CLKCTRL {}
#[doc = "`write(|w| ..)` method takes [usb0clkctrl::W](usb0clkctrl::W) writer structure"]
impl crate::Writable for USB0CLKCTRL {}
#[doc = "USB0 clock control"]
pub mod usb0clkctrl;
#[doc = "USB0 clock status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0clkstat](usb0clkstat) module"]
pub type USB0CLKSTAT = crate::Reg<u32, _USB0CLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB0CLKSTAT;
#[doc = "`read()` method returns [usb0clkstat::R](usb0clkstat::R) reader structure"]
impl crate::Readable for USB0CLKSTAT {}
#[doc = "`write(|w| ..)` method takes [usb0clkstat::W](usb0clkstat::W) writer structure"]
impl crate::Writable for USB0CLKSTAT {}
#[doc = "USB0 clock status"]
pub mod usb0clkstat;
#[doc = "Frequency measure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqmectrl](freqmectrl) module"]
pub type FREQMECTRL = crate::Reg<u32, _FREQMECTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQMECTRL;
#[doc = "`read()` method returns [freqmectrl::R](freqmectrl::R) reader structure"]
impl crate::Readable for FREQMECTRL {}
#[doc = "`write(|w| ..)` method takes [freqmectrl::W](freqmectrl::W) writer structure"]
impl crate::Writable for FREQMECTRL {}
#[doc = "Frequency measure register"]
pub mod freqmectrl;
#[doc = "MCLK input/output control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkio](mclkio) module"]
pub type MCLKIO = crate::Reg<u32, _MCLKIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCLKIO;
#[doc = "`read()` method returns [mclkio::R](mclkio::R) reader structure"]
impl crate::Readable for MCLKIO {}
#[doc = "`write(|w| ..)` method takes [mclkio::W](mclkio::W) writer structure"]
impl crate::Writable for MCLKIO {}
#[doc = "MCLK input/output control"]
pub mod mclkio;
#[doc = "USB1 clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1clkctrl](usb1clkctrl) module"]
pub type USB1CLKCTRL = crate::Reg<u32, _USB1CLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1CLKCTRL;
#[doc = "`read()` method returns [usb1clkctrl::R](usb1clkctrl::R) reader structure"]
impl crate::Readable for USB1CLKCTRL {}
#[doc = "`write(|w| ..)` method takes [usb1clkctrl::W](usb1clkctrl::W) writer structure"]
impl crate::Writable for USB1CLKCTRL {}
#[doc = "USB1 clock control"]
pub mod usb1clkctrl;
#[doc = "USB1 clock status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1clkstat](usb1clkstat) module"]
pub type USB1CLKSTAT = crate::Reg<u32, _USB1CLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB1CLKSTAT;
#[doc = "`read()` method returns [usb1clkstat::R](usb1clkstat::R) reader structure"]
impl crate::Readable for USB1CLKSTAT {}
#[doc = "`write(|w| ..)` method takes [usb1clkstat::W](usb1clkstat::W) writer structure"]
impl crate::Writable for USB1CLKSTAT {}
#[doc = "USB1 clock status"]
pub mod usb1clkstat;
#[doc = "EMC system control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcsysctrl](emcsysctrl) module"]
pub type EMCSYSCTRL = crate::Reg<u32, _EMCSYSCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMCSYSCTRL;
#[doc = "`read()` method returns [emcsysctrl::R](emcsysctrl::R) reader structure"]
impl crate::Readable for EMCSYSCTRL {}
#[doc = "`write(|w| ..)` method takes [emcsysctrl::W](emcsysctrl::W) writer structure"]
impl crate::Writable for EMCSYSCTRL {}
#[doc = "EMC system control"]
pub mod emcsysctrl;
#[doc = "EMC clock delay control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcdlyctrl](emcdlyctrl) module"]
pub type EMCDLYCTRL = crate::Reg<u32, _EMCDLYCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMCDLYCTRL;
#[doc = "`read()` method returns [emcdlyctrl::R](emcdlyctrl::R) reader structure"]
impl crate::Readable for EMCDLYCTRL {}
#[doc = "`write(|w| ..)` method takes [emcdlyctrl::W](emcdlyctrl::W) writer structure"]
impl crate::Writable for EMCDLYCTRL {}
#[doc = "EMC clock delay control"]
pub mod emcdlyctrl;
#[doc = "EMC delay chain calibration control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emcdlycal](emcdlycal) module"]
pub type EMCDLYCAL = crate::Reg<u32, _EMCDLYCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMCDLYCAL;
#[doc = "`read()` method returns [emcdlycal::R](emcdlycal::R) reader structure"]
impl crate::Readable for EMCDLYCAL {}
#[doc = "`write(|w| ..)` method takes [emcdlycal::W](emcdlycal::W) writer structure"]
impl crate::Writable for EMCDLYCAL {}
#[doc = "EMC delay chain calibration control"]
pub mod emcdlycal;
#[doc = "Ethernet PHY Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ethphysel](ethphysel) module"]
pub type ETHPHYSEL = crate::Reg<u32, _ETHPHYSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETHPHYSEL;
#[doc = "`read()` method returns [ethphysel::R](ethphysel::R) reader structure"]
impl crate::Readable for ETHPHYSEL {}
#[doc = "`write(|w| ..)` method takes [ethphysel::W](ethphysel::W) writer structure"]
impl crate::Writable for ETHPHYSEL {}
#[doc = "Ethernet PHY Selection"]
pub mod ethphysel;
#[doc = "Ethernet SBD flow control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ethsbdctrl](ethsbdctrl) module"]
pub type ETHSBDCTRL = crate::Reg<u32, _ETHSBDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETHSBDCTRL;
#[doc = "`read()` method returns [ethsbdctrl::R](ethsbdctrl::R) reader structure"]
impl crate::Readable for ETHSBDCTRL {}
#[doc = "`write(|w| ..)` method takes [ethsbdctrl::W](ethsbdctrl::W) writer structure"]
impl crate::Writable for ETHSBDCTRL {}
#[doc = "Ethernet SBD flow control"]
pub mod ethsbdctrl;
#[doc = "SDIO CCLKIN phase and delay control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdioclkctrl](sdioclkctrl) module"]
pub type SDIOCLKCTRL = crate::Reg<u32, _SDIOCLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDIOCLKCTRL;
#[doc = "`read()` method returns [sdioclkctrl::R](sdioclkctrl::R) reader structure"]
impl crate::Readable for SDIOCLKCTRL {}
#[doc = "`write(|w| ..)` method takes [sdioclkctrl::W](sdioclkctrl::W) writer structure"]
impl crate::Writable for SDIOCLKCTRL {}
#[doc = "SDIO CCLKIN phase and delay control"]
pub mod sdioclkctrl;
#[doc = "FRO oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [froctrl](froctrl) module"]
pub type FROCTRL = crate::Reg<u32, _FROCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FROCTRL;
#[doc = "`read()` method returns [froctrl::R](froctrl::R) reader structure"]
impl crate::Readable for FROCTRL {}
#[doc = "`write(|w| ..)` method takes [froctrl::W](froctrl::W) writer structure"]
impl crate::Writable for FROCTRL {}
#[doc = "FRO oscillator control"]
pub mod froctrl;
#[doc = "System oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysoscctrl](sysoscctrl) module"]
pub type SYSOSCCTRL = crate::Reg<u32, _SYSOSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSOSCCTRL;
#[doc = "`read()` method returns [sysoscctrl::R](sysoscctrl::R) reader structure"]
impl crate::Readable for SYSOSCCTRL {}
#[doc = "`write(|w| ..)` method takes [sysoscctrl::W](sysoscctrl::W) writer structure"]
impl crate::Writable for SYSOSCCTRL {}
#[doc = "System oscillator control"]
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
#[doc = "RTC oscillator 32 kHz output control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcoscctrl](rtcoscctrl) module"]
pub type RTCOSCCTRL = crate::Reg<u32, _RTCOSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCOSCCTRL;
#[doc = "`read()` method returns [rtcoscctrl::R](rtcoscctrl::R) reader structure"]
impl crate::Readable for RTCOSCCTRL {}
#[doc = "`write(|w| ..)` method takes [rtcoscctrl::W](rtcoscctrl::W) writer structure"]
impl crate::Writable for RTCOSCCTRL {}
#[doc = "RTC oscillator 32 kHz output control"]
pub mod rtcoscctrl;
#[doc = "USB PLL control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllctrl](usbpllctrl) module"]
pub type USBPLLCTRL = crate::Reg<u32, _USBPLLCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPLLCTRL;
#[doc = "`read()` method returns [usbpllctrl::R](usbpllctrl::R) reader structure"]
impl crate::Readable for USBPLLCTRL {}
#[doc = "`write(|w| ..)` method takes [usbpllctrl::W](usbpllctrl::W) writer structure"]
impl crate::Writable for USBPLLCTRL {}
#[doc = "USB PLL control"]
pub mod usbpllctrl;
#[doc = "USB PLL status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllstat](usbpllstat) module"]
pub type USBPLLSTAT = crate::Reg<u32, _USBPLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPLLSTAT;
#[doc = "`read()` method returns [usbpllstat::R](usbpllstat::R) reader structure"]
impl crate::Readable for USBPLLSTAT {}
#[doc = "`write(|w| ..)` method takes [usbpllstat::W](usbpllstat::W) writer structure"]
impl crate::Writable for USBPLLSTAT {}
#[doc = "USB PLL status"]
pub mod usbpllstat;
#[doc = "System PLL control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllctrl](syspllctrl) module"]
pub type SYSPLLCTRL = crate::Reg<u32, _SYSPLLCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLCTRL;
#[doc = "`read()` method returns [syspllctrl::R](syspllctrl::R) reader structure"]
impl crate::Readable for SYSPLLCTRL {}
#[doc = "`write(|w| ..)` method takes [syspllctrl::W](syspllctrl::W) writer structure"]
impl crate::Writable for SYSPLLCTRL {}
#[doc = "System PLL control"]
pub mod syspllctrl;
#[doc = "PLL status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllstat](syspllstat) module"]
pub type SYSPLLSTAT = crate::Reg<u32, _SYSPLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLSTAT;
#[doc = "`read()` method returns [syspllstat::R](syspllstat::R) reader structure"]
impl crate::Readable for SYSPLLSTAT {}
#[doc = "`write(|w| ..)` method takes [syspllstat::W](syspllstat::W) writer structure"]
impl crate::Writable for SYSPLLSTAT {}
#[doc = "PLL status"]
pub mod syspllstat;
#[doc = "PLL N divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllndec](syspllndec) module"]
pub type SYSPLLNDEC = crate::Reg<u32, _SYSPLLNDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLNDEC;
#[doc = "`read()` method returns [syspllndec::R](syspllndec::R) reader structure"]
impl crate::Readable for SYSPLLNDEC {}
#[doc = "`write(|w| ..)` method takes [syspllndec::W](syspllndec::W) writer structure"]
impl crate::Writable for SYSPLLNDEC {}
#[doc = "PLL N divider"]
pub mod syspllndec;
#[doc = "PLL P divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllpdec](syspllpdec) module"]
pub type SYSPLLPDEC = crate::Reg<u32, _SYSPLLPDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLPDEC;
#[doc = "`read()` method returns [syspllpdec::R](syspllpdec::R) reader structure"]
impl crate::Readable for SYSPLLPDEC {}
#[doc = "`write(|w| ..)` method takes [syspllpdec::W](syspllpdec::W) writer structure"]
impl crate::Writable for SYSPLLPDEC {}
#[doc = "PLL P divider"]
pub mod syspllpdec;
#[doc = "System PLL M divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllmdec](syspllmdec) module"]
pub type SYSPLLMDEC = crate::Reg<u32, _SYSPLLMDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSPLLMDEC;
#[doc = "`read()` method returns [syspllmdec::R](syspllmdec::R) reader structure"]
impl crate::Readable for SYSPLLMDEC {}
#[doc = "`write(|w| ..)` method takes [syspllmdec::W](syspllmdec::W) writer structure"]
impl crate::Writable for SYSPLLMDEC {}
#[doc = "System PLL M divider"]
pub mod syspllmdec;
#[doc = "Audio PLL control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllctrl](audpllctrl) module"]
pub type AUDPLLCTRL = crate::Reg<u32, _AUDPLLCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDPLLCTRL;
#[doc = "`read()` method returns [audpllctrl::R](audpllctrl::R) reader structure"]
impl crate::Readable for AUDPLLCTRL {}
#[doc = "`write(|w| ..)` method takes [audpllctrl::W](audpllctrl::W) writer structure"]
impl crate::Writable for AUDPLLCTRL {}
#[doc = "Audio PLL control"]
pub mod audpllctrl;
#[doc = "Audio PLL status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllstat](audpllstat) module"]
pub type AUDPLLSTAT = crate::Reg<u32, _AUDPLLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDPLLSTAT;
#[doc = "`read()` method returns [audpllstat::R](audpllstat::R) reader structure"]
impl crate::Readable for AUDPLLSTAT {}
#[doc = "`write(|w| ..)` method takes [audpllstat::W](audpllstat::W) writer structure"]
impl crate::Writable for AUDPLLSTAT {}
#[doc = "Audio PLL status"]
pub mod audpllstat;
#[doc = "Audio PLL N divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllndec](audpllndec) module"]
pub type AUDPLLNDEC = crate::Reg<u32, _AUDPLLNDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDPLLNDEC;
#[doc = "`read()` method returns [audpllndec::R](audpllndec::R) reader structure"]
impl crate::Readable for AUDPLLNDEC {}
#[doc = "`write(|w| ..)` method takes [audpllndec::W](audpllndec::W) writer structure"]
impl crate::Writable for AUDPLLNDEC {}
#[doc = "Audio PLL N divider"]
pub mod audpllndec;
#[doc = "Audio PLL P divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllpdec](audpllpdec) module"]
pub type AUDPLLPDEC = crate::Reg<u32, _AUDPLLPDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDPLLPDEC;
#[doc = "`read()` method returns [audpllpdec::R](audpllpdec::R) reader structure"]
impl crate::Readable for AUDPLLPDEC {}
#[doc = "`write(|w| ..)` method takes [audpllpdec::W](audpllpdec::W) writer structure"]
impl crate::Writable for AUDPLLPDEC {}
#[doc = "Audio PLL P divider"]
pub mod audpllpdec;
#[doc = "Audio PLL M divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllmdec](audpllmdec) module"]
pub type AUDPLLMDEC = crate::Reg<u32, _AUDPLLMDEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDPLLMDEC;
#[doc = "`read()` method returns [audpllmdec::R](audpllmdec::R) reader structure"]
impl crate::Readable for AUDPLLMDEC {}
#[doc = "`write(|w| ..)` method takes [audpllmdec::W](audpllmdec::W) writer structure"]
impl crate::Writable for AUDPLLMDEC {}
#[doc = "Audio PLL M divider"]
pub mod audpllmdec;
#[doc = "Audio PLL fractional divider control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [audpllfrac](audpllfrac) module"]
pub type AUDPLLFRAC = crate::Reg<u32, _AUDPLLFRAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUDPLLFRAC;
#[doc = "`read()` method returns [audpllfrac::R](audpllfrac::R) reader structure"]
impl crate::Readable for AUDPLLFRAC {}
#[doc = "`write(|w| ..)` method takes [audpllfrac::W](audpllfrac::W) writer structure"]
impl crate::Writable for AUDPLLFRAC {}
#[doc = "Audio PLL fractional divider control"]
pub mod audpllfrac;
#[doc = "Sleep configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsleepcfg0](pdsleepcfg0) module"]
pub type PDSLEEPCFG0 = crate::Reg<u32, _PDSLEEPCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSLEEPCFG0;
#[doc = "`read()` method returns [pdsleepcfg0::R](pdsleepcfg0::R) reader structure"]
impl crate::Readable for PDSLEEPCFG0 {}
#[doc = "`write(|w| ..)` method takes [pdsleepcfg0::W](pdsleepcfg0::W) writer structure"]
impl crate::Writable for PDSLEEPCFG0 {}
#[doc = "Sleep configuration register"]
pub mod pdsleepcfg0;
#[doc = "Sleep configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsleepcfg1](pdsleepcfg1) module"]
pub type PDSLEEPCFG1 = crate::Reg<u32, _PDSLEEPCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSLEEPCFG1;
#[doc = "`read()` method returns [pdsleepcfg1::R](pdsleepcfg1::R) reader structure"]
impl crate::Readable for PDSLEEPCFG1 {}
#[doc = "`write(|w| ..)` method takes [pdsleepcfg1::W](pdsleepcfg1::W) writer structure"]
impl crate::Writable for PDSLEEPCFG1 {}
#[doc = "Sleep configuration register"]
pub mod pdsleepcfg1;
#[doc = "Power configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfg0](pdruncfg0) module"]
pub type PDRUNCFG0 = crate::Reg<u32, _PDRUNCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFG0;
#[doc = "`read()` method returns [pdruncfg0::R](pdruncfg0::R) reader structure"]
impl crate::Readable for PDRUNCFG0 {}
#[doc = "`write(|w| ..)` method takes [pdruncfg0::W](pdruncfg0::W) writer structure"]
impl crate::Writable for PDRUNCFG0 {}
#[doc = "Power configuration register"]
pub mod pdruncfg0;
#[doc = "Power configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfg1](pdruncfg1) module"]
pub type PDRUNCFG1 = crate::Reg<u32, _PDRUNCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFG1;
#[doc = "`read()` method returns [pdruncfg1::R](pdruncfg1::R) reader structure"]
impl crate::Readable for PDRUNCFG1 {}
#[doc = "`write(|w| ..)` method takes [pdruncfg1::W](pdruncfg1::W) writer structure"]
impl crate::Writable for PDRUNCFG1 {}
#[doc = "Power configuration register"]
pub mod pdruncfg1;
#[doc = "Power configuration set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgset0](pdruncfgset0) module"]
pub type PDRUNCFGSET0 = crate::Reg<u32, _PDRUNCFGSET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFGSET0;
#[doc = "`read()` method returns [pdruncfgset0::R](pdruncfgset0::R) reader structure"]
impl crate::Readable for PDRUNCFGSET0 {}
#[doc = "`write(|w| ..)` method takes [pdruncfgset0::W](pdruncfgset0::W) writer structure"]
impl crate::Writable for PDRUNCFGSET0 {}
#[doc = "Power configuration set register"]
pub mod pdruncfgset0;
#[doc = "Power configuration set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgset1](pdruncfgset1) module"]
pub type PDRUNCFGSET1 = crate::Reg<u32, _PDRUNCFGSET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFGSET1;
#[doc = "`read()` method returns [pdruncfgset1::R](pdruncfgset1::R) reader structure"]
impl crate::Readable for PDRUNCFGSET1 {}
#[doc = "`write(|w| ..)` method takes [pdruncfgset1::W](pdruncfgset1::W) writer structure"]
impl crate::Writable for PDRUNCFGSET1 {}
#[doc = "Power configuration set register"]
pub mod pdruncfgset1;
#[doc = "Power configuration clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgclr0](pdruncfgclr0) module"]
pub type PDRUNCFGCLR0 = crate::Reg<u32, _PDRUNCFGCLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFGCLR0;
#[doc = "`read()` method returns [pdruncfgclr0::R](pdruncfgclr0::R) reader structure"]
impl crate::Readable for PDRUNCFGCLR0 {}
#[doc = "`write(|w| ..)` method takes [pdruncfgclr0::W](pdruncfgclr0::W) writer structure"]
impl crate::Writable for PDRUNCFGCLR0 {}
#[doc = "Power configuration clear register"]
pub mod pdruncfgclr0;
#[doc = "Power configuration clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgclr1](pdruncfgclr1) module"]
pub type PDRUNCFGCLR1 = crate::Reg<u32, _PDRUNCFGCLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDRUNCFGCLR1;
#[doc = "`read()` method returns [pdruncfgclr1::R](pdruncfgclr1::R) reader structure"]
impl crate::Readable for PDRUNCFGCLR1 {}
#[doc = "`write(|w| ..)` method takes [pdruncfgclr1::W](pdruncfgclr1::W) writer structure"]
impl crate::Writable for PDRUNCFGCLR1 {}
#[doc = "Power configuration clear register"]
pub mod pdruncfgclr1;
#[doc = "Start logic 0 wake-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starter0](starter0) module"]
pub type STARTER0 = crate::Reg<u32, _STARTER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTER0;
#[doc = "`read()` method returns [starter0::R](starter0::R) reader structure"]
impl crate::Readable for STARTER0 {}
#[doc = "`write(|w| ..)` method takes [starter0::W](starter0::W) writer structure"]
impl crate::Writable for STARTER0 {}
#[doc = "Start logic 0 wake-up enable register"]
pub mod starter0;
#[doc = "Start logic 0 wake-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starter1](starter1) module"]
pub type STARTER1 = crate::Reg<u32, _STARTER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTER1;
#[doc = "`read()` method returns [starter1::R](starter1::R) reader structure"]
impl crate::Readable for STARTER1 {}
#[doc = "`write(|w| ..)` method takes [starter1::W](starter1::W) writer structure"]
impl crate::Writable for STARTER1 {}
#[doc = "Start logic 0 wake-up enable register"]
pub mod starter1;
#[doc = "Set bits in STARTER\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterset](starterset) module"]
pub type STARTERSET = crate::Reg<u32, _STARTERSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTERSET;
#[doc = "`write(|w| ..)` method takes [starterset::W](starterset::W) writer structure"]
impl crate::Writable for STARTERSET {}
#[doc = "Set bits in STARTER"]
pub mod starterset;
#[doc = "Clear bits in STARTER0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterclr](starterclr) module"]
pub type STARTERCLR = crate::Reg<u32, _STARTERCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STARTERCLR;
#[doc = "`write(|w| ..)` method takes [starterclr::W](starterclr::W) writer structure"]
impl crate::Writable for STARTERCLR {}
#[doc = "Clear bits in STARTER0"]
pub mod starterclr;
#[doc = "Configures special cases of hardware wake-up\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwwake](hwwake) module"]
pub type HWWAKE = crate::Reg<u32, _HWWAKE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWWAKE;
#[doc = "`read()` method returns [hwwake::R](hwwake::R) reader structure"]
impl crate::Readable for HWWAKE {}
#[doc = "`write(|w| ..)` method takes [hwwake::W](hwwake::W) writer structure"]
impl crate::Writable for HWWAKE {}
#[doc = "Configures special cases of hardware wake-up"]
pub mod hwwake;
#[doc = "Auto Clock-Gate Override Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autocgor](autocgor) module"]
pub type AUTOCGOR = crate::Reg<u32, _AUTOCGOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTOCGOR;
#[doc = "`read()` method returns [autocgor::R](autocgor::R) reader structure"]
impl crate::Readable for AUTOCGOR {}
#[doc = "`write(|w| ..)` method takes [autocgor::W](autocgor::W) writer structure"]
impl crate::Writable for AUTOCGOR {}
#[doc = "Auto Clock-Gate Override Register"]
pub mod autocgor;
#[doc = "JTAG ID code register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtagidcode](jtagidcode) module"]
pub type JTAGIDCODE = crate::Reg<u32, _JTAGIDCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JTAGIDCODE;
#[doc = "`read()` method returns [jtagidcode::R](jtagidcode::R) reader structure"]
impl crate::Readable for JTAGIDCODE {}
#[doc = "JTAG ID code register"]
pub mod jtagidcode;
#[doc = "Part ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_id0](device_id0) module"]
pub type DEVICE_ID0 = crate::Reg<u32, _DEVICE_ID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_ID0;
#[doc = "`read()` method returns [device_id0::R](device_id0::R) reader structure"]
impl crate::Readable for DEVICE_ID0 {}
#[doc = "Part ID register"]
pub mod device_id0;
#[doc = "Boot ROM and die revision register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_id1](device_id1) module"]
pub type DEVICE_ID1 = crate::Reg<u32, _DEVICE_ID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICE_ID1;
#[doc = "`read()` method returns [device_id1::R](device_id1::R) reader structure"]
impl crate::Readable for DEVICE_ID1 {}
#[doc = "Boot ROM and die revision register"]
pub mod device_id1;
#[doc = "Brown-Out Detect control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodctrl](bodctrl) module"]
pub type BODCTRL = crate::Reg<u32, _BODCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BODCTRL;
#[doc = "`read()` method returns [bodctrl::R](bodctrl::R) reader structure"]
impl crate::Readable for BODCTRL {}
#[doc = "`write(|w| ..)` method takes [bodctrl::W](bodctrl::W) writer structure"]
impl crate::Writable for BODCTRL {}
#[doc = "Brown-Out Detect control"]
pub mod bodctrl;
