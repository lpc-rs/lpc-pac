///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    ///0x10 - AHB multilayer matrix priority control
    pub ahbmatprio: crate::Reg<ahbmatprio::AHBMATPRIO_SPEC>,
    _reserved1: [u8; 0x2c],
    ///0x40 - System tick counter calibration
    pub systckcal: crate::Reg<systckcal::SYSTCKCAL_SPEC>,
    _reserved2: [u8; 0x04],
    ///0x48 - NMI Source Select
    pub nmisrc: crate::Reg<nmisrc::NMISRC_SPEC>,
    ///0x4c - Asynchronous APB Control
    pub asyncapbctrl: crate::Reg<asyncapbctrl::ASYNCAPBCTRL_SPEC>,
    _reserved4: [u8; 0x70],
    ///0xc0..0xc8 - POR captured value of port n
    pub pioporcap: [crate::Reg<pioporcap::PIOPORCAP_SPEC>; 2],
    _reserved5: [u8; 0x08],
    ///0xd0..0xd8 - Reset captured value of port n
    pub piorescap: [crate::Reg<piorescap::PIORESCAP_SPEC>; 2],
    _reserved6: [u8; 0x28],
    ///0x100 - Peripheral reset control n
    pub presetctrl0: crate::Reg<presetctrl0::PRESETCTRL0_SPEC>,
    ///0x104 - Peripheral reset control n
    pub presetctrl1: crate::Reg<presetctrl1::PRESETCTRL1_SPEC>,
    ///0x108 - Peripheral reset control n
    pub presetctrl2: crate::Reg<presetctrl2::PRESETCTRL2_SPEC>,
    _reserved9: [u8; 0x14],
    ///0x120..0x12c - Set bits in PRESETCTRLn
    pub presetctrlset: [crate::Reg<presetctrlset::PRESETCTRLSET_SPEC>; 3],
    _reserved10: [u8; 0x14],
    ///0x140..0x14c - Clear bits in PRESETCTRLn
    pub presetctrlclr: [crate::Reg<presetctrlclr::PRESETCTRLCLR_SPEC>; 3],
    _reserved11: [u8; 0xa4],
    ///0x1f0 - System reset status register
    pub sysrststat: crate::Reg<sysrststat::SYSRSTSTAT_SPEC>,
    _reserved12: [u8; 0x0c],
    ///0x200 - AHB Clock control n
    pub ahbclkctrl0: crate::Reg<ahbclkctrl0::AHBCLKCTRL0_SPEC>,
    ///0x204 - AHB Clock control n
    pub ahbclkctrl1: crate::Reg<ahbclkctrl1::AHBCLKCTRL1_SPEC>,
    ///0x208 - AHB Clock control n
    pub ahbclkctrl2: crate::Reg<ahbclkctrl2::AHBCLKCTRL2_SPEC>,
    _reserved15: [u8; 0x14],
    ///0x220..0x22c - Set bits in AHBCLKCTRLn
    pub ahbclkctrlset: [crate::Reg<ahbclkctrlset::AHBCLKCTRLSET_SPEC>; 3],
    _reserved16: [u8; 0x14],
    ///0x240..0x24c - Clear bits in AHBCLKCTRLn
    pub ahbclkctrlclr: [crate::Reg<ahbclkctrlclr::AHBCLKCTRLCLR_SPEC>; 3],
    _reserved17: [u8; 0x34],
    ///0x280 - Main clock source select A
    pub mainclksela: crate::Reg<mainclksela::MAINCLKSELA_SPEC>,
    ///0x284 - Main clock source select B
    pub mainclkselb: crate::Reg<mainclkselb::MAINCLKSELB_SPEC>,
    ///0x288 - CLKOUT clock source select A
    pub clkoutsela: crate::Reg<clkoutsela::CLKOUTSELA_SPEC>,
    _reserved20: [u8; 0x04],
    ///0x290 - PLL clock source select
    pub syspllclksel: crate::Reg<syspllclksel::SYSPLLCLKSEL_SPEC>,
    _reserved21: [u8; 0x04],
    ///0x298 - Audio PLL clock source select
    pub audpllclksel: crate::Reg<audpllclksel::AUDPLLCLKSEL_SPEC>,
    _reserved22: [u8; 0x04],
    ///0x2a0 - SPIFI clock source select
    pub spificlksel: crate::Reg<spificlksel::SPIFICLKSEL_SPEC>,
    ///0x2a4 - ADC clock source select
    pub adcclksel: crate::Reg<adcclksel::ADCCLKSEL_SPEC>,
    ///0x2a8 - USB0 clock source select
    pub usb0clksel: crate::Reg<usb0clksel::USB0CLKSEL_SPEC>,
    ///0x2ac - USB1 clock source select
    pub usb1clksel: crate::Reg<usb1clksel::USB1CLKSEL_SPEC>,
    ///0x2b0..0x2d8 - Flexcomm 0 clock source select
    pub fclksel: [crate::Reg<fclksel::FCLKSEL_SPEC>; 10],
    _reserved27: [u8; 0x08],
    ///0x2e0 - MCLK clock source select
    pub mclkclksel: crate::Reg<mclkclksel::MCLKCLKSEL_SPEC>,
    _reserved28: [u8; 0x04],
    ///0x2e8 - Fractional Rate Generator clock source select
    pub frgclksel: crate::Reg<frgclksel::FRGCLKSEL_SPEC>,
    ///0x2ec - Digital microphone (DMIC) subsystem clock select
    pub dmicclksel: crate::Reg<dmicclksel::DMICCLKSEL_SPEC>,
    ///0x2f0 - SCTimer/PWM clock source select
    pub sctclksel: crate::Reg<sctclksel::SCTCLKSEL_SPEC>,
    ///0x2f4 - LCD clock source select
    pub lcdclksel: crate::Reg<lcdclksel::LCDCLKSEL_SPEC>,
    ///0x2f8 - SDIO clock source select
    pub sdioclksel: crate::Reg<sdioclksel::SDIOCLKSEL_SPEC>,
    _reserved33: [u8; 0x04],
    ///0x300 - SYSTICK clock divider
    pub systickclkdiv: crate::Reg<systickclkdiv::SYSTICKCLKDIV_SPEC>,
    ///0x304 - ARM Trace clock divider
    pub armtraceclkdiv: crate::Reg<armtraceclkdiv::ARMTRACECLKDIV_SPEC>,
    ///0x308 - MCAN0 clock divider
    pub can0clkdiv: crate::Reg<can0clkdiv::CAN0CLKDIV_SPEC>,
    ///0x30c - MCAN1 clock divider
    pub can1clkdiv: crate::Reg<can1clkdiv::CAN1CLKDIV_SPEC>,
    ///0x310 - Smartcard0 clock divider
    pub sc0clkdiv: crate::Reg<sc0clkdiv::SC0CLKDIV_SPEC>,
    ///0x314 - Smartcard1 clock divider
    pub sc1clkdiv: crate::Reg<sc1clkdiv::SC1CLKDIV_SPEC>,
    _reserved39: [u8; 0x68],
    ///0x380 - AHB clock divider
    pub ahbclkdiv: crate::Reg<ahbclkdiv::AHBCLKDIV_SPEC>,
    ///0x384 - CLKOUT clock divider
    pub clkoutdiv: crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>,
    ///0x388 - FROHF clock divider
    pub frohfclkdiv: crate::Reg<frohfclkdiv::FROHFCLKDIV_SPEC>,
    _reserved42: [u8; 0x04],
    ///0x390 - SPIFI clock divider
    pub spificlkdiv: crate::Reg<spificlkdiv::SPIFICLKDIV_SPEC>,
    ///0x394 - ADC clock divider
    pub adcclkdiv: crate::Reg<adcclkdiv::ADCCLKDIV_SPEC>,
    ///0x398 - USB0 clock divider
    pub usb0clkdiv: crate::Reg<usb0clkdiv::USB0CLKDIV_SPEC>,
    ///0x39c - USB1 clock divider
    pub usb1clkdiv: crate::Reg<usb1clkdiv::USB1CLKDIV_SPEC>,
    ///0x3a0 - Fractional rate divider
    pub frgctrl: crate::Reg<frgctrl::FRGCTRL_SPEC>,
    _reserved47: [u8; 0x04],
    ///0x3a8 - DMIC clock divider
    pub dmicclkdiv: crate::Reg<dmicclkdiv::DMICCLKDIV_SPEC>,
    ///0x3ac - I2S MCLK clock divider
    pub mclkdiv: crate::Reg<mclkdiv::MCLKDIV_SPEC>,
    ///0x3b0 - LCD clock divider
    pub lcdclkdiv: crate::Reg<lcdclkdiv::LCDCLKDIV_SPEC>,
    ///0x3b4 - SCT/PWM clock divider
    pub sctclkdiv: crate::Reg<sctclkdiv::SCTCLKDIV_SPEC>,
    ///0x3b8 - EMC clock divider
    pub emcclkdiv: crate::Reg<emcclkdiv::EMCCLKDIV_SPEC>,
    ///0x3bc - SDIO clock divider
    pub sdioclkdiv: crate::Reg<sdioclkdiv::SDIOCLKDIV_SPEC>,
    _reserved53: [u8; 0x40],
    ///0x400 - Flash wait states configuration
    pub flashcfg: crate::Reg<flashcfg::FLASHCFG_SPEC>,
    _reserved54: [u8; 0x08],
    ///0x40c - USB0 clock control
    pub usb0clkctrl: crate::Reg<usb0clkctrl::USB0CLKCTRL_SPEC>,
    ///0x410 - USB0 clock status
    pub usb0clkstat: crate::Reg<usb0clkstat::USB0CLKSTAT_SPEC>,
    _reserved56: [u8; 0x04],
    ///0x418 - Frequency measure register
    pub freqmectrl: crate::Reg<freqmectrl::FREQMECTRL_SPEC>,
    _reserved57: [u8; 0x04],
    ///0x420 - MCLK input/output control
    pub mclkio: crate::Reg<mclkio::MCLKIO_SPEC>,
    ///0x424 - USB1 clock control
    pub usb1clkctrl: crate::Reg<usb1clkctrl::USB1CLKCTRL_SPEC>,
    ///0x428 - USB1 clock status
    pub usb1clkstat: crate::Reg<usb1clkstat::USB1CLKSTAT_SPEC>,
    _reserved60: [u8; 0x18],
    ///0x444 - EMC system control
    pub emcsysctrl: crate::Reg<emcsysctrl::EMCSYSCTRL_SPEC>,
    ///0x448 - EMC clock delay control
    pub emcdlyctrl: crate::Reg<emcdlyctrl::EMCDLYCTRL_SPEC>,
    ///0x44c - EMC delay chain calibration control
    pub emcdlycal: crate::Reg<emcdlycal::EMCDLYCAL_SPEC>,
    ///0x450 - Ethernet PHY Selection
    pub ethphysel: crate::Reg<ethphysel::ETHPHYSEL_SPEC>,
    ///0x454 - Ethernet SBD flow control
    pub ethsbdctrl: crate::Reg<ethsbdctrl::ETHSBDCTRL_SPEC>,
    _reserved65: [u8; 0x08],
    ///0x460 - SDIO CCLKIN phase and delay control
    pub sdioclkctrl: crate::Reg<sdioclkctrl::SDIOCLKCTRL_SPEC>,
    _reserved66: [u8; 0x9c],
    ///0x500 - FRO oscillator control
    pub froctrl: crate::Reg<froctrl::FROCTRL_SPEC>,
    ///0x504 - System oscillator control
    pub sysoscctrl: crate::Reg<sysoscctrl::SYSOSCCTRL_SPEC>,
    ///0x508 - Watchdog oscillator control
    pub wdtoscctrl: crate::Reg<wdtoscctrl::WDTOSCCTRL_SPEC>,
    ///0x50c - RTC oscillator 32 kHz output control
    pub rtcoscctrl: crate::Reg<rtcoscctrl::RTCOSCCTRL_SPEC>,
    _reserved70: [u8; 0x0c],
    ///0x51c - USB PLL control
    pub usbpllctrl: crate::Reg<usbpllctrl::USBPLLCTRL_SPEC>,
    ///0x520 - USB PLL status
    pub usbpllstat: crate::Reg<usbpllstat::USBPLLSTAT_SPEC>,
    _reserved72: [u8; 0x5c],
    ///0x580 - System PLL control
    pub syspllctrl: crate::Reg<syspllctrl::SYSPLLCTRL_SPEC>,
    ///0x584 - PLL status
    pub syspllstat: crate::Reg<syspllstat::SYSPLLSTAT_SPEC>,
    ///0x588 - PLL N divider
    pub syspllndec: crate::Reg<syspllndec::SYSPLLNDEC_SPEC>,
    ///0x58c - PLL P divider
    pub syspllpdec: crate::Reg<syspllpdec::SYSPLLPDEC_SPEC>,
    ///0x590 - System PLL M divider
    pub syspllmdec: crate::Reg<syspllmdec::SYSPLLMDEC_SPEC>,
    _reserved77: [u8; 0x0c],
    ///0x5a0 - Audio PLL control
    pub audpllctrl: crate::Reg<audpllctrl::AUDPLLCTRL_SPEC>,
    ///0x5a4 - Audio PLL status
    pub audpllstat: crate::Reg<audpllstat::AUDPLLSTAT_SPEC>,
    ///0x5a8 - Audio PLL N divider
    pub audpllndec: crate::Reg<audpllndec::AUDPLLNDEC_SPEC>,
    ///0x5ac - Audio PLL P divider
    pub audpllpdec: crate::Reg<audpllpdec::AUDPLLPDEC_SPEC>,
    ///0x5b0 - Audio PLL M divider
    pub audpllmdec: crate::Reg<audpllmdec::AUDPLLMDEC_SPEC>,
    ///0x5b4 - Audio PLL fractional divider control
    pub audpllfrac: crate::Reg<audpllfrac::AUDPLLFRAC_SPEC>,
    _reserved83: [u8; 0x48],
    ///0x600 - Sleep configuration register
    pub pdsleepcfg0: crate::Reg<pdsleepcfg0::PDSLEEPCFG0_SPEC>,
    ///0x604 - Sleep configuration register
    pub pdsleepcfg1: crate::Reg<pdsleepcfg1::PDSLEEPCFG1_SPEC>,
    _reserved85: [u8; 0x08],
    ///0x610 - Power configuration register
    pub pdruncfg0: crate::Reg<pdruncfg0::PDRUNCFG0_SPEC>,
    ///0x614 - Power configuration register
    pub pdruncfg1: crate::Reg<pdruncfg1::PDRUNCFG1_SPEC>,
    _reserved87: [u8; 0x08],
    ///0x620 - Power configuration set register
    pub pdruncfgset0: crate::Reg<pdruncfgset0::PDRUNCFGSET0_SPEC>,
    ///0x624 - Power configuration set register
    pub pdruncfgset1: crate::Reg<pdruncfgset1::PDRUNCFGSET1_SPEC>,
    _reserved89: [u8; 0x08],
    ///0x630 - Power configuration clear register
    pub pdruncfgclr0: crate::Reg<pdruncfgclr0::PDRUNCFGCLR0_SPEC>,
    ///0x634 - Power configuration clear register
    pub pdruncfgclr1: crate::Reg<pdruncfgclr1::PDRUNCFGCLR1_SPEC>,
    _reserved91: [u8; 0x48],
    ///0x680 - Start logic 0 wake-up enable register
    pub starter0: crate::Reg<starter0::STARTER0_SPEC>,
    ///0x684 - Start logic 0 wake-up enable register
    pub starter1: crate::Reg<starter1::STARTER1_SPEC>,
    _reserved93: [u8; 0x18],
    ///0x6a0..0x6a8 - Set bits in STARTER
    pub starterset: [crate::Reg<starterset::STARTERSET_SPEC>; 2],
    _reserved94: [u8; 0x18],
    ///0x6c0..0x6c8 - Clear bits in STARTER0
    pub starterclr: [crate::Reg<starterclr::STARTERCLR_SPEC>; 2],
    _reserved95: [u8; 0xb8],
    ///0x780 - Configures special cases of hardware wake-up
    pub hwwake: crate::Reg<hwwake::HWWAKE_SPEC>,
    _reserved96: [u8; 0x0680],
    ///0xe04 - Auto Clock-Gate Override Register
    pub autocgor: crate::Reg<autocgor::AUTOCGOR_SPEC>,
    _reserved97: [u8; 0x01ec],
    ///0xff4 - JTAG ID code register
    pub jtagidcode: crate::Reg<jtagidcode::JTAGIDCODE_SPEC>,
    ///0xff8 - Part ID register
    pub device_id0: crate::Reg<device_id0::DEVICE_ID0_SPEC>,
    ///0xffc - Boot ROM and die revision register
    pub device_id1: crate::Reg<device_id1::DEVICE_ID1_SPEC>,
    _reserved100: [u8; 0x0001_f044],
    ///0x20044 - Brown-Out Detect control
    pub bodctrl: crate::Reg<bodctrl::BODCTRL_SPEC>,
}
///AHBMATPRIO register accessor: an alias for `Reg<AHBMATPRIO_SPEC>`
pub type AHBMATPRIO = crate::Reg<ahbmatprio::AHBMATPRIO_SPEC>;
///AHB multilayer matrix priority control
pub mod ahbmatprio;
///SYSTCKCAL register accessor: an alias for `Reg<SYSTCKCAL_SPEC>`
pub type SYSTCKCAL = crate::Reg<systckcal::SYSTCKCAL_SPEC>;
///System tick counter calibration
pub mod systckcal;
///NMISRC register accessor: an alias for `Reg<NMISRC_SPEC>`
pub type NMISRC = crate::Reg<nmisrc::NMISRC_SPEC>;
///NMI Source Select
pub mod nmisrc;
///ASYNCAPBCTRL register accessor: an alias for `Reg<ASYNCAPBCTRL_SPEC>`
pub type ASYNCAPBCTRL = crate::Reg<asyncapbctrl::ASYNCAPBCTRL_SPEC>;
///Asynchronous APB Control
pub mod asyncapbctrl;
///PIOPORCAP register accessor: an alias for `Reg<PIOPORCAP_SPEC>`
pub type PIOPORCAP = crate::Reg<pioporcap::PIOPORCAP_SPEC>;
///POR captured value of port n
pub mod pioporcap;
///PIORESCAP register accessor: an alias for `Reg<PIORESCAP_SPEC>`
pub type PIORESCAP = crate::Reg<piorescap::PIORESCAP_SPEC>;
///Reset captured value of port n
pub mod piorescap;
///PRESETCTRL0 register accessor: an alias for `Reg<PRESETCTRL0_SPEC>`
pub type PRESETCTRL0 = crate::Reg<presetctrl0::PRESETCTRL0_SPEC>;
///Peripheral reset control n
pub mod presetctrl0;
///PRESETCTRL1 register accessor: an alias for `Reg<PRESETCTRL1_SPEC>`
pub type PRESETCTRL1 = crate::Reg<presetctrl1::PRESETCTRL1_SPEC>;
///Peripheral reset control n
pub mod presetctrl1;
///PRESETCTRL2 register accessor: an alias for `Reg<PRESETCTRL2_SPEC>`
pub type PRESETCTRL2 = crate::Reg<presetctrl2::PRESETCTRL2_SPEC>;
///Peripheral reset control n
pub mod presetctrl2;
///PRESETCTRLSET register accessor: an alias for `Reg<PRESETCTRLSET_SPEC>`
pub type PRESETCTRLSET = crate::Reg<presetctrlset::PRESETCTRLSET_SPEC>;
///Set bits in PRESETCTRLn
pub mod presetctrlset;
///PRESETCTRLCLR register accessor: an alias for `Reg<PRESETCTRLCLR_SPEC>`
pub type PRESETCTRLCLR = crate::Reg<presetctrlclr::PRESETCTRLCLR_SPEC>;
///Clear bits in PRESETCTRLn
pub mod presetctrlclr;
///SYSRSTSTAT register accessor: an alias for `Reg<SYSRSTSTAT_SPEC>`
pub type SYSRSTSTAT = crate::Reg<sysrststat::SYSRSTSTAT_SPEC>;
///System reset status register
pub mod sysrststat;
///AHBCLKCTRL0 register accessor: an alias for `Reg<AHBCLKCTRL0_SPEC>`
pub type AHBCLKCTRL0 = crate::Reg<ahbclkctrl0::AHBCLKCTRL0_SPEC>;
///AHB Clock control n
pub mod ahbclkctrl0;
///AHBCLKCTRL1 register accessor: an alias for `Reg<AHBCLKCTRL1_SPEC>`
pub type AHBCLKCTRL1 = crate::Reg<ahbclkctrl1::AHBCLKCTRL1_SPEC>;
///AHB Clock control n
pub mod ahbclkctrl1;
///AHBCLKCTRL2 register accessor: an alias for `Reg<AHBCLKCTRL2_SPEC>`
pub type AHBCLKCTRL2 = crate::Reg<ahbclkctrl2::AHBCLKCTRL2_SPEC>;
///AHB Clock control n
pub mod ahbclkctrl2;
///AHBCLKCTRLSET register accessor: an alias for `Reg<AHBCLKCTRLSET_SPEC>`
pub type AHBCLKCTRLSET = crate::Reg<ahbclkctrlset::AHBCLKCTRLSET_SPEC>;
///Set bits in AHBCLKCTRLn
pub mod ahbclkctrlset;
///AHBCLKCTRLCLR register accessor: an alias for `Reg<AHBCLKCTRLCLR_SPEC>`
pub type AHBCLKCTRLCLR = crate::Reg<ahbclkctrlclr::AHBCLKCTRLCLR_SPEC>;
///Clear bits in AHBCLKCTRLn
pub mod ahbclkctrlclr;
///MAINCLKSELA register accessor: an alias for `Reg<MAINCLKSELA_SPEC>`
pub type MAINCLKSELA = crate::Reg<mainclksela::MAINCLKSELA_SPEC>;
///Main clock source select A
pub mod mainclksela;
///MAINCLKSELB register accessor: an alias for `Reg<MAINCLKSELB_SPEC>`
pub type MAINCLKSELB = crate::Reg<mainclkselb::MAINCLKSELB_SPEC>;
///Main clock source select B
pub mod mainclkselb;
///CLKOUTSELA register accessor: an alias for `Reg<CLKOUTSELA_SPEC>`
pub type CLKOUTSELA = crate::Reg<clkoutsela::CLKOUTSELA_SPEC>;
///CLKOUT clock source select A
pub mod clkoutsela;
///SYSPLLCLKSEL register accessor: an alias for `Reg<SYSPLLCLKSEL_SPEC>`
pub type SYSPLLCLKSEL = crate::Reg<syspllclksel::SYSPLLCLKSEL_SPEC>;
///PLL clock source select
pub mod syspllclksel;
///AUDPLLCLKSEL register accessor: an alias for `Reg<AUDPLLCLKSEL_SPEC>`
pub type AUDPLLCLKSEL = crate::Reg<audpllclksel::AUDPLLCLKSEL_SPEC>;
///Audio PLL clock source select
pub mod audpllclksel;
///SPIFICLKSEL register accessor: an alias for `Reg<SPIFICLKSEL_SPEC>`
pub type SPIFICLKSEL = crate::Reg<spificlksel::SPIFICLKSEL_SPEC>;
///SPIFI clock source select
pub mod spificlksel;
///ADCCLKSEL register accessor: an alias for `Reg<ADCCLKSEL_SPEC>`
pub type ADCCLKSEL = crate::Reg<adcclksel::ADCCLKSEL_SPEC>;
///ADC clock source select
pub mod adcclksel;
///USB0CLKSEL register accessor: an alias for `Reg<USB0CLKSEL_SPEC>`
pub type USB0CLKSEL = crate::Reg<usb0clksel::USB0CLKSEL_SPEC>;
///USB0 clock source select
pub mod usb0clksel;
///USB1CLKSEL register accessor: an alias for `Reg<USB1CLKSEL_SPEC>`
pub type USB1CLKSEL = crate::Reg<usb1clksel::USB1CLKSEL_SPEC>;
///USB1 clock source select
pub mod usb1clksel;
///FCLKSEL register accessor: an alias for `Reg<FCLKSEL_SPEC>`
pub type FCLKSEL = crate::Reg<fclksel::FCLKSEL_SPEC>;
///Flexcomm 0 clock source select
pub mod fclksel;
///MCLKCLKSEL register accessor: an alias for `Reg<MCLKCLKSEL_SPEC>`
pub type MCLKCLKSEL = crate::Reg<mclkclksel::MCLKCLKSEL_SPEC>;
///MCLK clock source select
pub mod mclkclksel;
///FRGCLKSEL register accessor: an alias for `Reg<FRGCLKSEL_SPEC>`
pub type FRGCLKSEL = crate::Reg<frgclksel::FRGCLKSEL_SPEC>;
///Fractional Rate Generator clock source select
pub mod frgclksel;
///DMICCLKSEL register accessor: an alias for `Reg<DMICCLKSEL_SPEC>`
pub type DMICCLKSEL = crate::Reg<dmicclksel::DMICCLKSEL_SPEC>;
///Digital microphone (DMIC) subsystem clock select
pub mod dmicclksel;
///SCTCLKSEL register accessor: an alias for `Reg<SCTCLKSEL_SPEC>`
pub type SCTCLKSEL = crate::Reg<sctclksel::SCTCLKSEL_SPEC>;
///SCTimer/PWM clock source select
pub mod sctclksel;
///LCDCLKSEL register accessor: an alias for `Reg<LCDCLKSEL_SPEC>`
pub type LCDCLKSEL = crate::Reg<lcdclksel::LCDCLKSEL_SPEC>;
///LCD clock source select
pub mod lcdclksel;
///SDIOCLKSEL register accessor: an alias for `Reg<SDIOCLKSEL_SPEC>`
pub type SDIOCLKSEL = crate::Reg<sdioclksel::SDIOCLKSEL_SPEC>;
///SDIO clock source select
pub mod sdioclksel;
///SYSTICKCLKDIV register accessor: an alias for `Reg<SYSTICKCLKDIV_SPEC>`
pub type SYSTICKCLKDIV = crate::Reg<systickclkdiv::SYSTICKCLKDIV_SPEC>;
///SYSTICK clock divider
pub mod systickclkdiv;
///ARMTRACECLKDIV register accessor: an alias for `Reg<ARMTRACECLKDIV_SPEC>`
pub type ARMTRACECLKDIV = crate::Reg<armtraceclkdiv::ARMTRACECLKDIV_SPEC>;
///ARM Trace clock divider
pub mod armtraceclkdiv;
///CAN0CLKDIV register accessor: an alias for `Reg<CAN0CLKDIV_SPEC>`
pub type CAN0CLKDIV = crate::Reg<can0clkdiv::CAN0CLKDIV_SPEC>;
///MCAN0 clock divider
pub mod can0clkdiv;
///CAN1CLKDIV register accessor: an alias for `Reg<CAN1CLKDIV_SPEC>`
pub type CAN1CLKDIV = crate::Reg<can1clkdiv::CAN1CLKDIV_SPEC>;
///MCAN1 clock divider
pub mod can1clkdiv;
///SC0CLKDIV register accessor: an alias for `Reg<SC0CLKDIV_SPEC>`
pub type SC0CLKDIV = crate::Reg<sc0clkdiv::SC0CLKDIV_SPEC>;
///Smartcard0 clock divider
pub mod sc0clkdiv;
///SC1CLKDIV register accessor: an alias for `Reg<SC1CLKDIV_SPEC>`
pub type SC1CLKDIV = crate::Reg<sc1clkdiv::SC1CLKDIV_SPEC>;
///Smartcard1 clock divider
pub mod sc1clkdiv;
///AHBCLKDIV register accessor: an alias for `Reg<AHBCLKDIV_SPEC>`
pub type AHBCLKDIV = crate::Reg<ahbclkdiv::AHBCLKDIV_SPEC>;
///AHB clock divider
pub mod ahbclkdiv;
///CLKOUTDIV register accessor: an alias for `Reg<CLKOUTDIV_SPEC>`
pub type CLKOUTDIV = crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>;
///CLKOUT clock divider
pub mod clkoutdiv;
///FROHFCLKDIV register accessor: an alias for `Reg<FROHFCLKDIV_SPEC>`
pub type FROHFCLKDIV = crate::Reg<frohfclkdiv::FROHFCLKDIV_SPEC>;
///FROHF clock divider
pub mod frohfclkdiv;
///SPIFICLKDIV register accessor: an alias for `Reg<SPIFICLKDIV_SPEC>`
pub type SPIFICLKDIV = crate::Reg<spificlkdiv::SPIFICLKDIV_SPEC>;
///SPIFI clock divider
pub mod spificlkdiv;
///ADCCLKDIV register accessor: an alias for `Reg<ADCCLKDIV_SPEC>`
pub type ADCCLKDIV = crate::Reg<adcclkdiv::ADCCLKDIV_SPEC>;
///ADC clock divider
pub mod adcclkdiv;
///USB0CLKDIV register accessor: an alias for `Reg<USB0CLKDIV_SPEC>`
pub type USB0CLKDIV = crate::Reg<usb0clkdiv::USB0CLKDIV_SPEC>;
///USB0 clock divider
pub mod usb0clkdiv;
///USB1CLKDIV register accessor: an alias for `Reg<USB1CLKDIV_SPEC>`
pub type USB1CLKDIV = crate::Reg<usb1clkdiv::USB1CLKDIV_SPEC>;
///USB1 clock divider
pub mod usb1clkdiv;
///FRGCTRL register accessor: an alias for `Reg<FRGCTRL_SPEC>`
pub type FRGCTRL = crate::Reg<frgctrl::FRGCTRL_SPEC>;
///Fractional rate divider
pub mod frgctrl;
///DMICCLKDIV register accessor: an alias for `Reg<DMICCLKDIV_SPEC>`
pub type DMICCLKDIV = crate::Reg<dmicclkdiv::DMICCLKDIV_SPEC>;
///DMIC clock divider
pub mod dmicclkdiv;
///MCLKDIV register accessor: an alias for `Reg<MCLKDIV_SPEC>`
pub type MCLKDIV = crate::Reg<mclkdiv::MCLKDIV_SPEC>;
///I2S MCLK clock divider
pub mod mclkdiv;
///LCDCLKDIV register accessor: an alias for `Reg<LCDCLKDIV_SPEC>`
pub type LCDCLKDIV = crate::Reg<lcdclkdiv::LCDCLKDIV_SPEC>;
///LCD clock divider
pub mod lcdclkdiv;
///SCTCLKDIV register accessor: an alias for `Reg<SCTCLKDIV_SPEC>`
pub type SCTCLKDIV = crate::Reg<sctclkdiv::SCTCLKDIV_SPEC>;
///SCT/PWM clock divider
pub mod sctclkdiv;
///EMCCLKDIV register accessor: an alias for `Reg<EMCCLKDIV_SPEC>`
pub type EMCCLKDIV = crate::Reg<emcclkdiv::EMCCLKDIV_SPEC>;
///EMC clock divider
pub mod emcclkdiv;
///SDIOCLKDIV register accessor: an alias for `Reg<SDIOCLKDIV_SPEC>`
pub type SDIOCLKDIV = crate::Reg<sdioclkdiv::SDIOCLKDIV_SPEC>;
///SDIO clock divider
pub mod sdioclkdiv;
///FLASHCFG register accessor: an alias for `Reg<FLASHCFG_SPEC>`
pub type FLASHCFG = crate::Reg<flashcfg::FLASHCFG_SPEC>;
///Flash wait states configuration
pub mod flashcfg;
///USB0CLKCTRL register accessor: an alias for `Reg<USB0CLKCTRL_SPEC>`
pub type USB0CLKCTRL = crate::Reg<usb0clkctrl::USB0CLKCTRL_SPEC>;
///USB0 clock control
pub mod usb0clkctrl;
///USB0CLKSTAT register accessor: an alias for `Reg<USB0CLKSTAT_SPEC>`
pub type USB0CLKSTAT = crate::Reg<usb0clkstat::USB0CLKSTAT_SPEC>;
///USB0 clock status
pub mod usb0clkstat;
///FREQMECTRL register accessor: an alias for `Reg<FREQMECTRL_SPEC>`
pub type FREQMECTRL = crate::Reg<freqmectrl::FREQMECTRL_SPEC>;
///Frequency measure register
pub mod freqmectrl;
///MCLKIO register accessor: an alias for `Reg<MCLKIO_SPEC>`
pub type MCLKIO = crate::Reg<mclkio::MCLKIO_SPEC>;
///MCLK input/output control
pub mod mclkio;
///USB1CLKCTRL register accessor: an alias for `Reg<USB1CLKCTRL_SPEC>`
pub type USB1CLKCTRL = crate::Reg<usb1clkctrl::USB1CLKCTRL_SPEC>;
///USB1 clock control
pub mod usb1clkctrl;
///USB1CLKSTAT register accessor: an alias for `Reg<USB1CLKSTAT_SPEC>`
pub type USB1CLKSTAT = crate::Reg<usb1clkstat::USB1CLKSTAT_SPEC>;
///USB1 clock status
pub mod usb1clkstat;
///EMCSYSCTRL register accessor: an alias for `Reg<EMCSYSCTRL_SPEC>`
pub type EMCSYSCTRL = crate::Reg<emcsysctrl::EMCSYSCTRL_SPEC>;
///EMC system control
pub mod emcsysctrl;
///EMCDLYCTRL register accessor: an alias for `Reg<EMCDLYCTRL_SPEC>`
pub type EMCDLYCTRL = crate::Reg<emcdlyctrl::EMCDLYCTRL_SPEC>;
///EMC clock delay control
pub mod emcdlyctrl;
///EMCDLYCAL register accessor: an alias for `Reg<EMCDLYCAL_SPEC>`
pub type EMCDLYCAL = crate::Reg<emcdlycal::EMCDLYCAL_SPEC>;
///EMC delay chain calibration control
pub mod emcdlycal;
///ETHPHYSEL register accessor: an alias for `Reg<ETHPHYSEL_SPEC>`
pub type ETHPHYSEL = crate::Reg<ethphysel::ETHPHYSEL_SPEC>;
///Ethernet PHY Selection
pub mod ethphysel;
///ETHSBDCTRL register accessor: an alias for `Reg<ETHSBDCTRL_SPEC>`
pub type ETHSBDCTRL = crate::Reg<ethsbdctrl::ETHSBDCTRL_SPEC>;
///Ethernet SBD flow control
pub mod ethsbdctrl;
///SDIOCLKCTRL register accessor: an alias for `Reg<SDIOCLKCTRL_SPEC>`
pub type SDIOCLKCTRL = crate::Reg<sdioclkctrl::SDIOCLKCTRL_SPEC>;
///SDIO CCLKIN phase and delay control
pub mod sdioclkctrl;
///FROCTRL register accessor: an alias for `Reg<FROCTRL_SPEC>`
pub type FROCTRL = crate::Reg<froctrl::FROCTRL_SPEC>;
///FRO oscillator control
pub mod froctrl;
///SYSOSCCTRL register accessor: an alias for `Reg<SYSOSCCTRL_SPEC>`
pub type SYSOSCCTRL = crate::Reg<sysoscctrl::SYSOSCCTRL_SPEC>;
///System oscillator control
pub mod sysoscctrl;
///WDTOSCCTRL register accessor: an alias for `Reg<WDTOSCCTRL_SPEC>`
pub type WDTOSCCTRL = crate::Reg<wdtoscctrl::WDTOSCCTRL_SPEC>;
///Watchdog oscillator control
pub mod wdtoscctrl;
///RTCOSCCTRL register accessor: an alias for `Reg<RTCOSCCTRL_SPEC>`
pub type RTCOSCCTRL = crate::Reg<rtcoscctrl::RTCOSCCTRL_SPEC>;
///RTC oscillator 32 kHz output control
pub mod rtcoscctrl;
///USBPLLCTRL register accessor: an alias for `Reg<USBPLLCTRL_SPEC>`
pub type USBPLLCTRL = crate::Reg<usbpllctrl::USBPLLCTRL_SPEC>;
///USB PLL control
pub mod usbpllctrl;
///USBPLLSTAT register accessor: an alias for `Reg<USBPLLSTAT_SPEC>`
pub type USBPLLSTAT = crate::Reg<usbpllstat::USBPLLSTAT_SPEC>;
///USB PLL status
pub mod usbpllstat;
///SYSPLLCTRL register accessor: an alias for `Reg<SYSPLLCTRL_SPEC>`
pub type SYSPLLCTRL = crate::Reg<syspllctrl::SYSPLLCTRL_SPEC>;
///System PLL control
pub mod syspllctrl;
///SYSPLLSTAT register accessor: an alias for `Reg<SYSPLLSTAT_SPEC>`
pub type SYSPLLSTAT = crate::Reg<syspllstat::SYSPLLSTAT_SPEC>;
///PLL status
pub mod syspllstat;
///SYSPLLNDEC register accessor: an alias for `Reg<SYSPLLNDEC_SPEC>`
pub type SYSPLLNDEC = crate::Reg<syspllndec::SYSPLLNDEC_SPEC>;
///PLL N divider
pub mod syspllndec;
///SYSPLLPDEC register accessor: an alias for `Reg<SYSPLLPDEC_SPEC>`
pub type SYSPLLPDEC = crate::Reg<syspllpdec::SYSPLLPDEC_SPEC>;
///PLL P divider
pub mod syspllpdec;
///SYSPLLMDEC register accessor: an alias for `Reg<SYSPLLMDEC_SPEC>`
pub type SYSPLLMDEC = crate::Reg<syspllmdec::SYSPLLMDEC_SPEC>;
///System PLL M divider
pub mod syspllmdec;
///AUDPLLCTRL register accessor: an alias for `Reg<AUDPLLCTRL_SPEC>`
pub type AUDPLLCTRL = crate::Reg<audpllctrl::AUDPLLCTRL_SPEC>;
///Audio PLL control
pub mod audpllctrl;
///AUDPLLSTAT register accessor: an alias for `Reg<AUDPLLSTAT_SPEC>`
pub type AUDPLLSTAT = crate::Reg<audpllstat::AUDPLLSTAT_SPEC>;
///Audio PLL status
pub mod audpllstat;
///AUDPLLNDEC register accessor: an alias for `Reg<AUDPLLNDEC_SPEC>`
pub type AUDPLLNDEC = crate::Reg<audpllndec::AUDPLLNDEC_SPEC>;
///Audio PLL N divider
pub mod audpllndec;
///AUDPLLPDEC register accessor: an alias for `Reg<AUDPLLPDEC_SPEC>`
pub type AUDPLLPDEC = crate::Reg<audpllpdec::AUDPLLPDEC_SPEC>;
///Audio PLL P divider
pub mod audpllpdec;
///AUDPLLMDEC register accessor: an alias for `Reg<AUDPLLMDEC_SPEC>`
pub type AUDPLLMDEC = crate::Reg<audpllmdec::AUDPLLMDEC_SPEC>;
///Audio PLL M divider
pub mod audpllmdec;
///AUDPLLFRAC register accessor: an alias for `Reg<AUDPLLFRAC_SPEC>`
pub type AUDPLLFRAC = crate::Reg<audpllfrac::AUDPLLFRAC_SPEC>;
///Audio PLL fractional divider control
pub mod audpllfrac;
///PDSLEEPCFG0 register accessor: an alias for `Reg<PDSLEEPCFG0_SPEC>`
pub type PDSLEEPCFG0 = crate::Reg<pdsleepcfg0::PDSLEEPCFG0_SPEC>;
///Sleep configuration register
pub mod pdsleepcfg0;
///PDSLEEPCFG1 register accessor: an alias for `Reg<PDSLEEPCFG1_SPEC>`
pub type PDSLEEPCFG1 = crate::Reg<pdsleepcfg1::PDSLEEPCFG1_SPEC>;
///Sleep configuration register
pub mod pdsleepcfg1;
///PDRUNCFG0 register accessor: an alias for `Reg<PDRUNCFG0_SPEC>`
pub type PDRUNCFG0 = crate::Reg<pdruncfg0::PDRUNCFG0_SPEC>;
///Power configuration register
pub mod pdruncfg0;
///PDRUNCFG1 register accessor: an alias for `Reg<PDRUNCFG1_SPEC>`
pub type PDRUNCFG1 = crate::Reg<pdruncfg1::PDRUNCFG1_SPEC>;
///Power configuration register
pub mod pdruncfg1;
///PDRUNCFGSET0 register accessor: an alias for `Reg<PDRUNCFGSET0_SPEC>`
pub type PDRUNCFGSET0 = crate::Reg<pdruncfgset0::PDRUNCFGSET0_SPEC>;
///Power configuration set register
pub mod pdruncfgset0;
///PDRUNCFGSET1 register accessor: an alias for `Reg<PDRUNCFGSET1_SPEC>`
pub type PDRUNCFGSET1 = crate::Reg<pdruncfgset1::PDRUNCFGSET1_SPEC>;
///Power configuration set register
pub mod pdruncfgset1;
///PDRUNCFGCLR0 register accessor: an alias for `Reg<PDRUNCFGCLR0_SPEC>`
pub type PDRUNCFGCLR0 = crate::Reg<pdruncfgclr0::PDRUNCFGCLR0_SPEC>;
///Power configuration clear register
pub mod pdruncfgclr0;
///PDRUNCFGCLR1 register accessor: an alias for `Reg<PDRUNCFGCLR1_SPEC>`
pub type PDRUNCFGCLR1 = crate::Reg<pdruncfgclr1::PDRUNCFGCLR1_SPEC>;
///Power configuration clear register
pub mod pdruncfgclr1;
///STARTER0 register accessor: an alias for `Reg<STARTER0_SPEC>`
pub type STARTER0 = crate::Reg<starter0::STARTER0_SPEC>;
///Start logic 0 wake-up enable register
pub mod starter0;
///STARTER1 register accessor: an alias for `Reg<STARTER1_SPEC>`
pub type STARTER1 = crate::Reg<starter1::STARTER1_SPEC>;
///Start logic 0 wake-up enable register
pub mod starter1;
///STARTERSET register accessor: an alias for `Reg<STARTERSET_SPEC>`
pub type STARTERSET = crate::Reg<starterset::STARTERSET_SPEC>;
///Set bits in STARTER
pub mod starterset;
///STARTERCLR register accessor: an alias for `Reg<STARTERCLR_SPEC>`
pub type STARTERCLR = crate::Reg<starterclr::STARTERCLR_SPEC>;
///Clear bits in STARTER0
pub mod starterclr;
///HWWAKE register accessor: an alias for `Reg<HWWAKE_SPEC>`
pub type HWWAKE = crate::Reg<hwwake::HWWAKE_SPEC>;
///Configures special cases of hardware wake-up
pub mod hwwake;
///AUTOCGOR register accessor: an alias for `Reg<AUTOCGOR_SPEC>`
pub type AUTOCGOR = crate::Reg<autocgor::AUTOCGOR_SPEC>;
///Auto Clock-Gate Override Register
pub mod autocgor;
///JTAGIDCODE register accessor: an alias for `Reg<JTAGIDCODE_SPEC>`
pub type JTAGIDCODE = crate::Reg<jtagidcode::JTAGIDCODE_SPEC>;
///JTAG ID code register
pub mod jtagidcode;
///DEVICE_ID0 register accessor: an alias for `Reg<DEVICE_ID0_SPEC>`
pub type DEVICE_ID0 = crate::Reg<device_id0::DEVICE_ID0_SPEC>;
///Part ID register
pub mod device_id0;
///DEVICE_ID1 register accessor: an alias for `Reg<DEVICE_ID1_SPEC>`
pub type DEVICE_ID1 = crate::Reg<device_id1::DEVICE_ID1_SPEC>;
///Boot ROM and die revision register
pub mod device_id1;
///BODCTRL register accessor: an alias for `Reg<BODCTRL_SPEC>`
pub type BODCTRL = crate::Reg<bodctrl::BODCTRL_SPEC>;
///Brown-Out Detect control
pub mod bodctrl;
