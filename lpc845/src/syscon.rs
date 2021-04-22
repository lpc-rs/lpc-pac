#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Remap register"]
    pub sysmemremap: crate::Reg<sysmemremap::SYSMEMREMAP_SPEC>,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - PLL control"]
    pub syspllctrl: crate::Reg<syspllctrl::SYSPLLCTRL_SPEC>,
    #[doc = "0x0c - PLL status"]
    pub syspllstat: crate::Reg<syspllstat::SYSPLLSTAT_SPEC>,
    _reserved3: [u8; 16usize],
    #[doc = "0x20 - system oscillator control"]
    pub sysoscctrl: crate::Reg<sysoscctrl::SYSOSCCTRL_SPEC>,
    #[doc = "0x24 - Watchdog oscillator control"]
    pub wdtoscctrl: crate::Reg<wdtoscctrl::WDTOSCCTRL_SPEC>,
    #[doc = "0x28 - FRO oscillator control"]
    pub frooscctrl: crate::Reg<frooscctrl::FROOSCCTRL_SPEC>,
    _reserved6: [u8; 4usize],
    #[doc = "0x30 - FRO direct clock source update enable register"]
    pub frodirectclkuen: crate::Reg<frodirectclkuen::FRODIRECTCLKUEN_SPEC>,
    _reserved7: [u8; 4usize],
    #[doc = "0x38 - System reset status register"]
    pub sysrststat: crate::Reg<sysrststat::SYSRSTSTAT_SPEC>,
    _reserved8: [u8; 4usize],
    #[doc = "0x40 - System PLL clock source select register"]
    pub syspllclksel: crate::Reg<syspllclksel::SYSPLLCLKSEL_SPEC>,
    #[doc = "0x44 - System PLL clock source update enable register"]
    pub syspllclkuen: crate::Reg<syspllclkuen::SYSPLLCLKUEN_SPEC>,
    #[doc = "0x48 - Main clock source select register"]
    pub mainclkpllsel: crate::Reg<mainclkpllsel::MAINCLKPLLSEL_SPEC>,
    #[doc = "0x4c - Main clock source update enable register"]
    pub mainclkplluen: crate::Reg<mainclkplluen::MAINCLKPLLUEN_SPEC>,
    #[doc = "0x50 - Main clock source select register"]
    pub mainclksel: crate::Reg<mainclksel::MAINCLKSEL_SPEC>,
    #[doc = "0x54 - Main clock source update enable register"]
    pub mainclkuen: crate::Reg<mainclkuen::MAINCLKUEN_SPEC>,
    #[doc = "0x58 - System clock divider register"]
    pub sysahbclkdiv: crate::Reg<sysahbclkdiv::SYSAHBCLKDIV_SPEC>,
    _reserved15: [u8; 4usize],
    #[doc = "0x60 - CAPT clock source select register"]
    pub captclksel: crate::Reg<captclksel::CAPTCLKSEL_SPEC>,
    #[doc = "0x64 - ADC clock source select register"]
    pub adcclksel: crate::Reg<adcclksel::ADCCLKSEL_SPEC>,
    #[doc = "0x68 - ADC clock divider register"]
    pub adcclkdiv: crate::Reg<adcclkdiv::ADCCLKDIV_SPEC>,
    #[doc = "0x6c - SCT clock source select register"]
    pub sctclksel: crate::Reg<sctclksel::SCTCLKSEL_SPEC>,
    #[doc = "0x70 - SCT clock divider register"]
    pub sctclkdiv: crate::Reg<sctclkdiv::SCTCLKDIV_SPEC>,
    #[doc = "0x74 - external clock source select register"]
    pub extclksel: crate::Reg<extclksel::EXTCLKSEL_SPEC>,
    _reserved21: [u8; 8usize],
    #[doc = "0x80 - System clock group 0 control register"]
    pub sysahbclkctrl0: crate::Reg<sysahbclkctrl0::SYSAHBCLKCTRL0_SPEC>,
    #[doc = "0x84 - System clock group 1 control register"]
    pub sysahbclkctrl1: crate::Reg<sysahbclkctrl1::SYSAHBCLKCTRL1_SPEC>,
    #[doc = "0x88 - Peripheral reset group 0 control register"]
    pub presetctrl0: crate::Reg<presetctrl0::PRESETCTRL0_SPEC>,
    #[doc = "0x8c - Peripheral reset group 1 control register"]
    pub presetctrl1: crate::Reg<presetctrl1::PRESETCTRL1_SPEC>,
    #[doc = "0x90 - peripheral clock source select register. FCLK0SEL~FCLK4SEL are for UART0~UART4 clock source select register. FCLK5SEL~FCLK8SEL are for I2C0~I2C3 clock source select register. FCLK9SEL~FCLK10SEL are for SPI0~SPI1 clock source select register."]
    pub fclksel: [crate::Reg<fclksel::FCLKSEL_SPEC>; 11],
    _reserved26: [u8; 20usize],
    #[doc = "0xd0 - no description available"]
    pub frg0: FRG,
    _reserved27: [u8; 4usize],
    #[doc = "0xe0 - no description available"]
    pub frg1: FRG,
    _reserved28: [u8; 4usize],
    #[doc = "0xf0 - CLKOUT clock source select register"]
    pub clkoutsel: crate::Reg<clkoutsel::CLKOUTSEL_SPEC>,
    #[doc = "0xf4 - CLKOUT clock divider registers"]
    pub clkoutdiv: crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>,
    _reserved30: [u8; 4usize],
    #[doc = "0xfc - External trace buffer command register"]
    pub exttracecmd: crate::Reg<exttracecmd::EXTTRACECMD_SPEC>,
    #[doc = "0x100 - POR captured PIO N status register(PIO0 has 32 PIOs, PIO1 has 22 PIOs)"]
    pub pioporcap: [crate::Reg<pioporcap::PIOPORCAP_SPEC>; 2],
    _reserved32: [u8; 44usize],
    #[doc = "0x134 - Peripheral clock 6 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv6: crate::Reg<ioconclkdiv6::IOCONCLKDIV6_SPEC>,
    #[doc = "0x138 - Peripheral clock 6 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv5: crate::Reg<ioconclkdiv5::IOCONCLKDIV5_SPEC>,
    #[doc = "0x13c - Peripheral clock 4 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv4: crate::Reg<ioconclkdiv4::IOCONCLKDIV4_SPEC>,
    #[doc = "0x140 - Peripheral clock 3 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv3: crate::Reg<ioconclkdiv3::IOCONCLKDIV3_SPEC>,
    #[doc = "0x144 - Peripheral clock 2 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv2: crate::Reg<ioconclkdiv2::IOCONCLKDIV2_SPEC>,
    #[doc = "0x148 - Peripheral clock 1 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv1: crate::Reg<ioconclkdiv1::IOCONCLKDIV1_SPEC>,
    #[doc = "0x14c - Peripheral clock 0 to the IOCON block for programmable glitch filter"]
    pub ioconclkdiv0: crate::Reg<ioconclkdiv0::IOCONCLKDIV0_SPEC>,
    #[doc = "0x150 - BOD control register"]
    pub bodctrl: crate::Reg<bodctrl::BODCTRL_SPEC>,
    #[doc = "0x154 - System tick timer calibration register"]
    pub systckcal: crate::Reg<systckcal::SYSTCKCAL_SPEC>,
    _reserved41: [u8; 24usize],
    #[doc = "0x170 - IRQ latency register"]
    pub irqlatency: crate::Reg<irqlatency::IRQLATENCY_SPEC>,
    #[doc = "0x174 - NMI source selection register"]
    pub nmisrc: crate::Reg<nmisrc::NMISRC_SPEC>,
    #[doc = "0x178 - Pin interrupt select registers N"]
    pub pintsel: [crate::Reg<pintsel::PINTSEL_SPEC>; 8],
    _reserved44: [u8; 108usize],
    #[doc = "0x204 - Start logic 0 pin wake-up enable register 0"]
    pub starterp0: crate::Reg<starterp0::STARTERP0_SPEC>,
    _reserved45: [u8; 12usize],
    #[doc = "0x214 - Start logic 0 pin wake-up enable register 1"]
    pub starterp1: crate::Reg<starterp1::STARTERP1_SPEC>,
    _reserved46: [u8; 24usize],
    #[doc = "0x230 - Deep-sleep configuration register"]
    pub pdsleepcfg: crate::Reg<pdsleepcfg::PDSLEEPCFG_SPEC>,
    #[doc = "0x234 - Wake-up configuration register"]
    pub pdawakecfg: crate::Reg<pdawakecfg::PDAWAKECFG_SPEC>,
    #[doc = "0x238 - Power configuration register"]
    pub pdruncfg: crate::Reg<pdruncfg::PDRUNCFG_SPEC>,
    _reserved49: [u8; 444usize],
    #[doc = "0x3f8 - Part ID register"]
    pub device_id: crate::Reg<device_id::DEVICE_ID_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct FRG {
    #[doc = "0x00 - fractional generator N divider value register"]
    pub frgdiv: crate::Reg<self::frg::frgdiv::FRGDIV_SPEC>,
    #[doc = "0x04 - fractional generator N multiplier value register"]
    pub frgmult: crate::Reg<self::frg::frgmult::FRGMULT_SPEC>,
    #[doc = "0x08 - FRG N clock source select register"]
    pub frgclksel: crate::Reg<self::frg::frgclksel::FRGCLKSEL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod frg;
#[doc = "SYSMEMREMAP register accessor: an alias for `Reg<SYSMEMREMAP_SPEC>`"]
pub type SYSMEMREMAP = crate::Reg<sysmemremap::SYSMEMREMAP_SPEC>;
#[doc = "System Remap register"]
pub mod sysmemremap;
#[doc = "SYSPLLCTRL register accessor: an alias for `Reg<SYSPLLCTRL_SPEC>`"]
pub type SYSPLLCTRL = crate::Reg<syspllctrl::SYSPLLCTRL_SPEC>;
#[doc = "PLL control"]
pub mod syspllctrl;
#[doc = "SYSPLLSTAT register accessor: an alias for `Reg<SYSPLLSTAT_SPEC>`"]
pub type SYSPLLSTAT = crate::Reg<syspllstat::SYSPLLSTAT_SPEC>;
#[doc = "PLL status"]
pub mod syspllstat;
#[doc = "SYSOSCCTRL register accessor: an alias for `Reg<SYSOSCCTRL_SPEC>`"]
pub type SYSOSCCTRL = crate::Reg<sysoscctrl::SYSOSCCTRL_SPEC>;
#[doc = "system oscillator control"]
pub mod sysoscctrl;
#[doc = "WDTOSCCTRL register accessor: an alias for `Reg<WDTOSCCTRL_SPEC>`"]
pub type WDTOSCCTRL = crate::Reg<wdtoscctrl::WDTOSCCTRL_SPEC>;
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "FROOSCCTRL register accessor: an alias for `Reg<FROOSCCTRL_SPEC>`"]
pub type FROOSCCTRL = crate::Reg<frooscctrl::FROOSCCTRL_SPEC>;
#[doc = "FRO oscillator control"]
pub mod frooscctrl;
#[doc = "FRODIRECTCLKUEN register accessor: an alias for `Reg<FRODIRECTCLKUEN_SPEC>`"]
pub type FRODIRECTCLKUEN = crate::Reg<frodirectclkuen::FRODIRECTCLKUEN_SPEC>;
#[doc = "FRO direct clock source update enable register"]
pub mod frodirectclkuen;
#[doc = "SYSRSTSTAT register accessor: an alias for `Reg<SYSRSTSTAT_SPEC>`"]
pub type SYSRSTSTAT = crate::Reg<sysrststat::SYSRSTSTAT_SPEC>;
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "SYSPLLCLKSEL register accessor: an alias for `Reg<SYSPLLCLKSEL_SPEC>`"]
pub type SYSPLLCLKSEL = crate::Reg<syspllclksel::SYSPLLCLKSEL_SPEC>;
#[doc = "System PLL clock source select register"]
pub mod syspllclksel;
#[doc = "SYSPLLCLKUEN register accessor: an alias for `Reg<SYSPLLCLKUEN_SPEC>`"]
pub type SYSPLLCLKUEN = crate::Reg<syspllclkuen::SYSPLLCLKUEN_SPEC>;
#[doc = "System PLL clock source update enable register"]
pub mod syspllclkuen;
#[doc = "MAINCLKPLLSEL register accessor: an alias for `Reg<MAINCLKPLLSEL_SPEC>`"]
pub type MAINCLKPLLSEL = crate::Reg<mainclkpllsel::MAINCLKPLLSEL_SPEC>;
#[doc = "Main clock source select register"]
pub mod mainclkpllsel;
#[doc = "MAINCLKPLLUEN register accessor: an alias for `Reg<MAINCLKPLLUEN_SPEC>`"]
pub type MAINCLKPLLUEN = crate::Reg<mainclkplluen::MAINCLKPLLUEN_SPEC>;
#[doc = "Main clock source update enable register"]
pub mod mainclkplluen;
#[doc = "MAINCLKSEL register accessor: an alias for `Reg<MAINCLKSEL_SPEC>`"]
pub type MAINCLKSEL = crate::Reg<mainclksel::MAINCLKSEL_SPEC>;
#[doc = "Main clock source select register"]
pub mod mainclksel;
#[doc = "MAINCLKUEN register accessor: an alias for `Reg<MAINCLKUEN_SPEC>`"]
pub type MAINCLKUEN = crate::Reg<mainclkuen::MAINCLKUEN_SPEC>;
#[doc = "Main clock source update enable register"]
pub mod mainclkuen;
#[doc = "SYSAHBCLKDIV register accessor: an alias for `Reg<SYSAHBCLKDIV_SPEC>`"]
pub type SYSAHBCLKDIV = crate::Reg<sysahbclkdiv::SYSAHBCLKDIV_SPEC>;
#[doc = "System clock divider register"]
pub mod sysahbclkdiv;
#[doc = "CAPTCLKSEL register accessor: an alias for `Reg<CAPTCLKSEL_SPEC>`"]
pub type CAPTCLKSEL = crate::Reg<captclksel::CAPTCLKSEL_SPEC>;
#[doc = "CAPT clock source select register"]
pub mod captclksel;
#[doc = "ADCCLKSEL register accessor: an alias for `Reg<ADCCLKSEL_SPEC>`"]
pub type ADCCLKSEL = crate::Reg<adcclksel::ADCCLKSEL_SPEC>;
#[doc = "ADC clock source select register"]
pub mod adcclksel;
#[doc = "ADCCLKDIV register accessor: an alias for `Reg<ADCCLKDIV_SPEC>`"]
pub type ADCCLKDIV = crate::Reg<adcclkdiv::ADCCLKDIV_SPEC>;
#[doc = "ADC clock divider register"]
pub mod adcclkdiv;
#[doc = "SCTCLKSEL register accessor: an alias for `Reg<SCTCLKSEL_SPEC>`"]
pub type SCTCLKSEL = crate::Reg<sctclksel::SCTCLKSEL_SPEC>;
#[doc = "SCT clock source select register"]
pub mod sctclksel;
#[doc = "SCTCLKDIV register accessor: an alias for `Reg<SCTCLKDIV_SPEC>`"]
pub type SCTCLKDIV = crate::Reg<sctclkdiv::SCTCLKDIV_SPEC>;
#[doc = "SCT clock divider register"]
pub mod sctclkdiv;
#[doc = "EXTCLKSEL register accessor: an alias for `Reg<EXTCLKSEL_SPEC>`"]
pub type EXTCLKSEL = crate::Reg<extclksel::EXTCLKSEL_SPEC>;
#[doc = "external clock source select register"]
pub mod extclksel;
#[doc = "SYSAHBCLKCTRL0 register accessor: an alias for `Reg<SYSAHBCLKCTRL0_SPEC>`"]
pub type SYSAHBCLKCTRL0 = crate::Reg<sysahbclkctrl0::SYSAHBCLKCTRL0_SPEC>;
#[doc = "System clock group 0 control register"]
pub mod sysahbclkctrl0;
#[doc = "SYSAHBCLKCTRL1 register accessor: an alias for `Reg<SYSAHBCLKCTRL1_SPEC>`"]
pub type SYSAHBCLKCTRL1 = crate::Reg<sysahbclkctrl1::SYSAHBCLKCTRL1_SPEC>;
#[doc = "System clock group 1 control register"]
pub mod sysahbclkctrl1;
#[doc = "PRESETCTRL0 register accessor: an alias for `Reg<PRESETCTRL0_SPEC>`"]
pub type PRESETCTRL0 = crate::Reg<presetctrl0::PRESETCTRL0_SPEC>;
#[doc = "Peripheral reset group 0 control register"]
pub mod presetctrl0;
#[doc = "PRESETCTRL1 register accessor: an alias for `Reg<PRESETCTRL1_SPEC>`"]
pub type PRESETCTRL1 = crate::Reg<presetctrl1::PRESETCTRL1_SPEC>;
#[doc = "Peripheral reset group 1 control register"]
pub mod presetctrl1;
#[doc = "FCLKSEL register accessor: an alias for `Reg<FCLKSEL_SPEC>`"]
pub type FCLKSEL = crate::Reg<fclksel::FCLKSEL_SPEC>;
#[doc = "peripheral clock source select register. FCLK0SEL~FCLK4SEL are for UART0~UART4 clock source select register. FCLK5SEL~FCLK8SEL are for I2C0~I2C3 clock source select register. FCLK9SEL~FCLK10SEL are for SPI0~SPI1 clock source select register."]
pub mod fclksel;
#[doc = "CLKOUTSEL register accessor: an alias for `Reg<CLKOUTSEL_SPEC>`"]
pub type CLKOUTSEL = crate::Reg<clkoutsel::CLKOUTSEL_SPEC>;
#[doc = "CLKOUT clock source select register"]
pub mod clkoutsel;
#[doc = "CLKOUTDIV register accessor: an alias for `Reg<CLKOUTDIV_SPEC>`"]
pub type CLKOUTDIV = crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>;
#[doc = "CLKOUT clock divider registers"]
pub mod clkoutdiv;
#[doc = "EXTTRACECMD register accessor: an alias for `Reg<EXTTRACECMD_SPEC>`"]
pub type EXTTRACECMD = crate::Reg<exttracecmd::EXTTRACECMD_SPEC>;
#[doc = "External trace buffer command register"]
pub mod exttracecmd;
#[doc = "PIOPORCAP register accessor: an alias for `Reg<PIOPORCAP_SPEC>`"]
pub type PIOPORCAP = crate::Reg<pioporcap::PIOPORCAP_SPEC>;
#[doc = "POR captured PIO N status register(PIO0 has 32 PIOs, PIO1 has 22 PIOs)"]
pub mod pioporcap;
#[doc = "IOCONCLKDIV6 register accessor: an alias for `Reg<IOCONCLKDIV6_SPEC>`"]
pub type IOCONCLKDIV6 = crate::Reg<ioconclkdiv6::IOCONCLKDIV6_SPEC>;
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv6;
#[doc = "IOCONCLKDIV5 register accessor: an alias for `Reg<IOCONCLKDIV5_SPEC>`"]
pub type IOCONCLKDIV5 = crate::Reg<ioconclkdiv5::IOCONCLKDIV5_SPEC>;
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv5;
#[doc = "IOCONCLKDIV4 register accessor: an alias for `Reg<IOCONCLKDIV4_SPEC>`"]
pub type IOCONCLKDIV4 = crate::Reg<ioconclkdiv4::IOCONCLKDIV4_SPEC>;
#[doc = "Peripheral clock 4 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv4;
#[doc = "IOCONCLKDIV3 register accessor: an alias for `Reg<IOCONCLKDIV3_SPEC>`"]
pub type IOCONCLKDIV3 = crate::Reg<ioconclkdiv3::IOCONCLKDIV3_SPEC>;
#[doc = "Peripheral clock 3 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv3;
#[doc = "IOCONCLKDIV2 register accessor: an alias for `Reg<IOCONCLKDIV2_SPEC>`"]
pub type IOCONCLKDIV2 = crate::Reg<ioconclkdiv2::IOCONCLKDIV2_SPEC>;
#[doc = "Peripheral clock 2 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv2;
#[doc = "IOCONCLKDIV1 register accessor: an alias for `Reg<IOCONCLKDIV1_SPEC>`"]
pub type IOCONCLKDIV1 = crate::Reg<ioconclkdiv1::IOCONCLKDIV1_SPEC>;
#[doc = "Peripheral clock 1 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv1;
#[doc = "IOCONCLKDIV0 register accessor: an alias for `Reg<IOCONCLKDIV0_SPEC>`"]
pub type IOCONCLKDIV0 = crate::Reg<ioconclkdiv0::IOCONCLKDIV0_SPEC>;
#[doc = "Peripheral clock 0 to the IOCON block for programmable glitch filter"]
pub mod ioconclkdiv0;
#[doc = "BODCTRL register accessor: an alias for `Reg<BODCTRL_SPEC>`"]
pub type BODCTRL = crate::Reg<bodctrl::BODCTRL_SPEC>;
#[doc = "BOD control register"]
pub mod bodctrl;
#[doc = "SYSTCKCAL register accessor: an alias for `Reg<SYSTCKCAL_SPEC>`"]
pub type SYSTCKCAL = crate::Reg<systckcal::SYSTCKCAL_SPEC>;
#[doc = "System tick timer calibration register"]
pub mod systckcal;
#[doc = "IRQLATENCY register accessor: an alias for `Reg<IRQLATENCY_SPEC>`"]
pub type IRQLATENCY = crate::Reg<irqlatency::IRQLATENCY_SPEC>;
#[doc = "IRQ latency register"]
pub mod irqlatency;
#[doc = "NMISRC register accessor: an alias for `Reg<NMISRC_SPEC>`"]
pub type NMISRC = crate::Reg<nmisrc::NMISRC_SPEC>;
#[doc = "NMI source selection register"]
pub mod nmisrc;
#[doc = "PINTSEL register accessor: an alias for `Reg<PINTSEL_SPEC>`"]
pub type PINTSEL = crate::Reg<pintsel::PINTSEL_SPEC>;
#[doc = "Pin interrupt select registers N"]
pub mod pintsel;
#[doc = "STARTERP0 register accessor: an alias for `Reg<STARTERP0_SPEC>`"]
pub type STARTERP0 = crate::Reg<starterp0::STARTERP0_SPEC>;
#[doc = "Start logic 0 pin wake-up enable register 0"]
pub mod starterp0;
#[doc = "STARTERP1 register accessor: an alias for `Reg<STARTERP1_SPEC>`"]
pub type STARTERP1 = crate::Reg<starterp1::STARTERP1_SPEC>;
#[doc = "Start logic 0 pin wake-up enable register 1"]
pub mod starterp1;
#[doc = "PDSLEEPCFG register accessor: an alias for `Reg<PDSLEEPCFG_SPEC>`"]
pub type PDSLEEPCFG = crate::Reg<pdsleepcfg::PDSLEEPCFG_SPEC>;
#[doc = "Deep-sleep configuration register"]
pub mod pdsleepcfg;
#[doc = "PDAWAKECFG register accessor: an alias for `Reg<PDAWAKECFG_SPEC>`"]
pub type PDAWAKECFG = crate::Reg<pdawakecfg::PDAWAKECFG_SPEC>;
#[doc = "Wake-up configuration register"]
pub mod pdawakecfg;
#[doc = "PDRUNCFG register accessor: an alias for `Reg<PDRUNCFG_SPEC>`"]
pub type PDRUNCFG = crate::Reg<pdruncfg::PDRUNCFG_SPEC>;
#[doc = "Power configuration register"]
pub mod pdruncfg;
#[doc = "DEVICE_ID register accessor: an alias for `Reg<DEVICE_ID_SPEC>`"]
pub type DEVICE_ID = crate::Reg<device_id::DEVICE_ID_SPEC>;
#[doc = "Part ID register"]
pub mod device_id;
