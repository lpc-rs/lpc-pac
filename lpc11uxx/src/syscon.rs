#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System memory remap"]
    pub sysmemremap: crate::Reg<sysmemremap::SYSMEMREMAP_SPEC>,
    #[doc = "0x04 - Peripheral reset control"]
    pub presetctrl: crate::Reg<presetctrl::PRESETCTRL_SPEC>,
    #[doc = "0x08 - System PLL control"]
    pub syspllctrl: crate::Reg<syspllctrl::SYSPLLCTRL_SPEC>,
    #[doc = "0x0c - System PLL status"]
    pub syspllstat: crate::Reg<syspllstat::SYSPLLSTAT_SPEC>,
    #[doc = "0x10 - USB PLL control"]
    pub usbpllctrl: crate::Reg<usbpllctrl::USBPLLCTRL_SPEC>,
    #[doc = "0x14 - USB PLL status"]
    pub usbpllstat: crate::Reg<usbpllstat::USBPLLSTAT_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - System oscillator control"]
    pub sysoscctrl: crate::Reg<sysoscctrl::SYSOSCCTRL_SPEC>,
    #[doc = "0x24 - Watchdog oscillator control"]
    pub wdtoscctrl: crate::Reg<wdtoscctrl::WDTOSCCTRL_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - System reset status register"]
    pub sysrststat: crate::Reg<sysrststat::SYSRSTSTAT_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x40 - System PLL clock source select"]
    pub syspllclksel: crate::Reg<syspllclksel::SYSPLLCLKSEL_SPEC>,
    #[doc = "0x44 - System PLL clock source update enable"]
    pub syspllclkuen: crate::Reg<syspllclkuen::SYSPLLCLKUEN_SPEC>,
    #[doc = "0x48 - USB PLL clock source select"]
    pub usbpllclksel: crate::Reg<usbpllclksel::USBPLLCLKSEL_SPEC>,
    #[doc = "0x4c - USB PLL clock source update enable"]
    pub usbpllclkuen: crate::Reg<usbpllclkuen::USBPLLCLKUEN_SPEC>,
    _reserved13: [u8; 0x20],
    #[doc = "0x70 - Main clock source select"]
    pub mainclksel: crate::Reg<mainclksel::MAINCLKSEL_SPEC>,
    #[doc = "0x74 - Main clock source update enable"]
    pub mainclkuen: crate::Reg<mainclkuen::MAINCLKUEN_SPEC>,
    #[doc = "0x78 - System clock divider"]
    pub sysahbclkdiv: crate::Reg<sysahbclkdiv::SYSAHBCLKDIV_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x80 - System clock control"]
    pub sysahbclkctrl: crate::Reg<sysahbclkctrl::SYSAHBCLKCTRL_SPEC>,
    _reserved17: [u8; 0x10],
    #[doc = "0x94 - SSP0 clock divider"]
    pub ssp0clkdiv: crate::Reg<ssp0clkdiv::SSP0CLKDIV_SPEC>,
    #[doc = "0x98 - UART clock divider"]
    pub uartclkdiv: crate::Reg<uartclkdiv::UARTCLKDIV_SPEC>,
    #[doc = "0x9c - SSP1 clock divider"]
    pub ssp1clkdiv: crate::Reg<ssp1clkdiv::SSP1CLKDIV_SPEC>,
    _reserved20: [u8; 0x20],
    #[doc = "0xc0 - USB clock source select"]
    pub usbclksel: crate::Reg<usbclksel::USBCLKSEL_SPEC>,
    #[doc = "0xc4 - USB clock source update enable"]
    pub usbclkuen: crate::Reg<usbclkuen::USBCLKUEN_SPEC>,
    #[doc = "0xc8 - USB clock source divider"]
    pub usbclkdiv: crate::Reg<usbclkdiv::USBCLKDIV_SPEC>,
    _reserved23: [u8; 0x14],
    #[doc = "0xe0 - CLKOUT clock source select"]
    pub clkoutsel: crate::Reg<clkoutsel::CLKOUTSEL_SPEC>,
    #[doc = "0xe4 - CLKOUT clock source update enable"]
    pub clkoutuen: crate::Reg<clkoutuen::CLKOUTUEN_SPEC>,
    #[doc = "0xe8 - CLKOUT clock divider"]
    pub clkoutdiv: crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>,
    _reserved26: [u8; 0x14],
    #[doc = "0x100 - POR captured PIO status 0"]
    pub pioporcap0: crate::Reg<pioporcap0::PIOPORCAP0_SPEC>,
    #[doc = "0x104 - POR captured PIO status 1"]
    pub pioporcap1: crate::Reg<pioporcap1::PIOPORCAP1_SPEC>,
    _reserved28: [u8; 0x48],
    #[doc = "0x150 - Brown-Out Detect"]
    pub bodctrl: crate::Reg<bodctrl::BODCTRL_SPEC>,
    #[doc = "0x154 - System tick counter calibration"]
    pub systckcal: crate::Reg<systckcal::SYSTCKCAL_SPEC>,
    _reserved30: [u8; 0x18],
    #[doc = "0x170 - IQR delay. Allows trade-off between interrupt latency and determinism."]
    pub irqlatency: crate::Reg<irqlatency::IRQLATENCY_SPEC>,
    #[doc = "0x174 - NMI Source Control"]
    pub nmisrc: crate::Reg<nmisrc::NMISRC_SPEC>,
    #[doc = "0x178..0x198 - GPIO Pin Interrupt Select register 0"]
    pub pintsel: [crate::Reg<pintsel::PINTSEL_SPEC>; 8],
    #[doc = "0x198 - USB clock control"]
    pub usbclkctrl: crate::Reg<usbclkctrl::USBCLKCTRL_SPEC>,
    #[doc = "0x19c - USB clock status"]
    pub usbclkst: crate::Reg<usbclkst::USBCLKST_SPEC>,
    _reserved35: [u8; 0x64],
    #[doc = "0x204 - Start logic 0 interrupt wake-up enable register 0"]
    pub starterp0: crate::Reg<starterp0::STARTERP0_SPEC>,
    _reserved36: [u8; 0x0c],
    #[doc = "0x214 - Start logic 1 interrupt wake-up enable register 1"]
    pub starterp1: crate::Reg<starterp1::STARTERP1_SPEC>,
    _reserved37: [u8; 0x18],
    #[doc = "0x230 - Power-down states in deep-sleep mode"]
    pub pdsleepcfg: crate::Reg<pdsleepcfg::PDSLEEPCFG_SPEC>,
    #[doc = "0x234 - Power-down states for wake-up from deep-sleep"]
    pub pdawakecfg: crate::Reg<pdawakecfg::PDAWAKECFG_SPEC>,
    #[doc = "0x238 - Power configuration register"]
    pub pdruncfg: crate::Reg<pdruncfg::PDRUNCFG_SPEC>,
    _reserved40: [u8; 0x01b8],
    #[doc = "0x3f4 - Device ID"]
    pub device_id: crate::Reg<device_id::DEVICE_ID_SPEC>,
}
#[doc = "SYSMEMREMAP register accessor: an alias for `Reg<SYSMEMREMAP_SPEC>`"]
pub type SYSMEMREMAP = crate::Reg<sysmemremap::SYSMEMREMAP_SPEC>;
#[doc = "System memory remap"]
pub mod sysmemremap;
#[doc = "PRESETCTRL register accessor: an alias for `Reg<PRESETCTRL_SPEC>`"]
pub type PRESETCTRL = crate::Reg<presetctrl::PRESETCTRL_SPEC>;
#[doc = "Peripheral reset control"]
pub mod presetctrl;
#[doc = "SYSPLLCTRL register accessor: an alias for `Reg<SYSPLLCTRL_SPEC>`"]
pub type SYSPLLCTRL = crate::Reg<syspllctrl::SYSPLLCTRL_SPEC>;
#[doc = "System PLL control"]
pub mod syspllctrl;
#[doc = "SYSPLLSTAT register accessor: an alias for `Reg<SYSPLLSTAT_SPEC>`"]
pub type SYSPLLSTAT = crate::Reg<syspllstat::SYSPLLSTAT_SPEC>;
#[doc = "System PLL status"]
pub mod syspllstat;
#[doc = "USBPLLCTRL register accessor: an alias for `Reg<USBPLLCTRL_SPEC>`"]
pub type USBPLLCTRL = crate::Reg<usbpllctrl::USBPLLCTRL_SPEC>;
#[doc = "USB PLL control"]
pub mod usbpllctrl;
#[doc = "USBPLLSTAT register accessor: an alias for `Reg<USBPLLSTAT_SPEC>`"]
pub type USBPLLSTAT = crate::Reg<usbpllstat::USBPLLSTAT_SPEC>;
#[doc = "USB PLL status"]
pub mod usbpllstat;
#[doc = "SYSOSCCTRL register accessor: an alias for `Reg<SYSOSCCTRL_SPEC>`"]
pub type SYSOSCCTRL = crate::Reg<sysoscctrl::SYSOSCCTRL_SPEC>;
#[doc = "System oscillator control"]
pub mod sysoscctrl;
#[doc = "WDTOSCCTRL register accessor: an alias for `Reg<WDTOSCCTRL_SPEC>`"]
pub type WDTOSCCTRL = crate::Reg<wdtoscctrl::WDTOSCCTRL_SPEC>;
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "SYSRSTSTAT register accessor: an alias for `Reg<SYSRSTSTAT_SPEC>`"]
pub type SYSRSTSTAT = crate::Reg<sysrststat::SYSRSTSTAT_SPEC>;
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "SYSPLLCLKSEL register accessor: an alias for `Reg<SYSPLLCLKSEL_SPEC>`"]
pub type SYSPLLCLKSEL = crate::Reg<syspllclksel::SYSPLLCLKSEL_SPEC>;
#[doc = "System PLL clock source select"]
pub mod syspllclksel;
#[doc = "SYSPLLCLKUEN register accessor: an alias for `Reg<SYSPLLCLKUEN_SPEC>`"]
pub type SYSPLLCLKUEN = crate::Reg<syspllclkuen::SYSPLLCLKUEN_SPEC>;
#[doc = "System PLL clock source update enable"]
pub mod syspllclkuen;
#[doc = "USBPLLCLKSEL register accessor: an alias for `Reg<USBPLLCLKSEL_SPEC>`"]
pub type USBPLLCLKSEL = crate::Reg<usbpllclksel::USBPLLCLKSEL_SPEC>;
#[doc = "USB PLL clock source select"]
pub mod usbpllclksel;
#[doc = "USBPLLCLKUEN register accessor: an alias for `Reg<USBPLLCLKUEN_SPEC>`"]
pub type USBPLLCLKUEN = crate::Reg<usbpllclkuen::USBPLLCLKUEN_SPEC>;
#[doc = "USB PLL clock source update enable"]
pub mod usbpllclkuen;
#[doc = "MAINCLKSEL register accessor: an alias for `Reg<MAINCLKSEL_SPEC>`"]
pub type MAINCLKSEL = crate::Reg<mainclksel::MAINCLKSEL_SPEC>;
#[doc = "Main clock source select"]
pub mod mainclksel;
#[doc = "MAINCLKUEN register accessor: an alias for `Reg<MAINCLKUEN_SPEC>`"]
pub type MAINCLKUEN = crate::Reg<mainclkuen::MAINCLKUEN_SPEC>;
#[doc = "Main clock source update enable"]
pub mod mainclkuen;
#[doc = "SYSAHBCLKDIV register accessor: an alias for `Reg<SYSAHBCLKDIV_SPEC>`"]
pub type SYSAHBCLKDIV = crate::Reg<sysahbclkdiv::SYSAHBCLKDIV_SPEC>;
#[doc = "System clock divider"]
pub mod sysahbclkdiv;
#[doc = "SYSAHBCLKCTRL register accessor: an alias for `Reg<SYSAHBCLKCTRL_SPEC>`"]
pub type SYSAHBCLKCTRL = crate::Reg<sysahbclkctrl::SYSAHBCLKCTRL_SPEC>;
#[doc = "System clock control"]
pub mod sysahbclkctrl;
#[doc = "SSP0CLKDIV register accessor: an alias for `Reg<SSP0CLKDIV_SPEC>`"]
pub type SSP0CLKDIV = crate::Reg<ssp0clkdiv::SSP0CLKDIV_SPEC>;
#[doc = "SSP0 clock divider"]
pub mod ssp0clkdiv;
#[doc = "UARTCLKDIV register accessor: an alias for `Reg<UARTCLKDIV_SPEC>`"]
pub type UARTCLKDIV = crate::Reg<uartclkdiv::UARTCLKDIV_SPEC>;
#[doc = "UART clock divider"]
pub mod uartclkdiv;
#[doc = "SSP1CLKDIV register accessor: an alias for `Reg<SSP1CLKDIV_SPEC>`"]
pub type SSP1CLKDIV = crate::Reg<ssp1clkdiv::SSP1CLKDIV_SPEC>;
#[doc = "SSP1 clock divider"]
pub mod ssp1clkdiv;
#[doc = "USBCLKSEL register accessor: an alias for `Reg<USBCLKSEL_SPEC>`"]
pub type USBCLKSEL = crate::Reg<usbclksel::USBCLKSEL_SPEC>;
#[doc = "USB clock source select"]
pub mod usbclksel;
#[doc = "USBCLKUEN register accessor: an alias for `Reg<USBCLKUEN_SPEC>`"]
pub type USBCLKUEN = crate::Reg<usbclkuen::USBCLKUEN_SPEC>;
#[doc = "USB clock source update enable"]
pub mod usbclkuen;
#[doc = "USBCLKDIV register accessor: an alias for `Reg<USBCLKDIV_SPEC>`"]
pub type USBCLKDIV = crate::Reg<usbclkdiv::USBCLKDIV_SPEC>;
#[doc = "USB clock source divider"]
pub mod usbclkdiv;
#[doc = "CLKOUTSEL register accessor: an alias for `Reg<CLKOUTSEL_SPEC>`"]
pub type CLKOUTSEL = crate::Reg<clkoutsel::CLKOUTSEL_SPEC>;
#[doc = "CLKOUT clock source select"]
pub mod clkoutsel;
#[doc = "CLKOUTUEN register accessor: an alias for `Reg<CLKOUTUEN_SPEC>`"]
pub type CLKOUTUEN = crate::Reg<clkoutuen::CLKOUTUEN_SPEC>;
#[doc = "CLKOUT clock source update enable"]
pub mod clkoutuen;
#[doc = "CLKOUTDIV register accessor: an alias for `Reg<CLKOUTDIV_SPEC>`"]
pub type CLKOUTDIV = crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>;
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "PIOPORCAP0 register accessor: an alias for `Reg<PIOPORCAP0_SPEC>`"]
pub type PIOPORCAP0 = crate::Reg<pioporcap0::PIOPORCAP0_SPEC>;
#[doc = "POR captured PIO status 0"]
pub mod pioporcap0;
#[doc = "PIOPORCAP1 register accessor: an alias for `Reg<PIOPORCAP1_SPEC>`"]
pub type PIOPORCAP1 = crate::Reg<pioporcap1::PIOPORCAP1_SPEC>;
#[doc = "POR captured PIO status 1"]
pub mod pioporcap1;
#[doc = "BODCTRL register accessor: an alias for `Reg<BODCTRL_SPEC>`"]
pub type BODCTRL = crate::Reg<bodctrl::BODCTRL_SPEC>;
#[doc = "Brown-Out Detect"]
pub mod bodctrl;
#[doc = "SYSTCKCAL register accessor: an alias for `Reg<SYSTCKCAL_SPEC>`"]
pub type SYSTCKCAL = crate::Reg<systckcal::SYSTCKCAL_SPEC>;
#[doc = "System tick counter calibration"]
pub mod systckcal;
#[doc = "IRQLATENCY register accessor: an alias for `Reg<IRQLATENCY_SPEC>`"]
pub type IRQLATENCY = crate::Reg<irqlatency::IRQLATENCY_SPEC>;
#[doc = "IQR delay. Allows trade-off between interrupt latency and determinism."]
pub mod irqlatency;
#[doc = "NMISRC register accessor: an alias for `Reg<NMISRC_SPEC>`"]
pub type NMISRC = crate::Reg<nmisrc::NMISRC_SPEC>;
#[doc = "NMI Source Control"]
pub mod nmisrc;
#[doc = "PINTSEL register accessor: an alias for `Reg<PINTSEL_SPEC>`"]
pub type PINTSEL = crate::Reg<pintsel::PINTSEL_SPEC>;
#[doc = "GPIO Pin Interrupt Select register 0"]
pub mod pintsel;
#[doc = "USBCLKCTRL register accessor: an alias for `Reg<USBCLKCTRL_SPEC>`"]
pub type USBCLKCTRL = crate::Reg<usbclkctrl::USBCLKCTRL_SPEC>;
#[doc = "USB clock control"]
pub mod usbclkctrl;
#[doc = "USBCLKST register accessor: an alias for `Reg<USBCLKST_SPEC>`"]
pub type USBCLKST = crate::Reg<usbclkst::USBCLKST_SPEC>;
#[doc = "USB clock status"]
pub mod usbclkst;
#[doc = "STARTERP0 register accessor: an alias for `Reg<STARTERP0_SPEC>`"]
pub type STARTERP0 = crate::Reg<starterp0::STARTERP0_SPEC>;
#[doc = "Start logic 0 interrupt wake-up enable register 0"]
pub mod starterp0;
#[doc = "STARTERP1 register accessor: an alias for `Reg<STARTERP1_SPEC>`"]
pub type STARTERP1 = crate::Reg<starterp1::STARTERP1_SPEC>;
#[doc = "Start logic 1 interrupt wake-up enable register 1"]
pub mod starterp1;
#[doc = "PDSLEEPCFG register accessor: an alias for `Reg<PDSLEEPCFG_SPEC>`"]
pub type PDSLEEPCFG = crate::Reg<pdsleepcfg::PDSLEEPCFG_SPEC>;
#[doc = "Power-down states in deep-sleep mode"]
pub mod pdsleepcfg;
#[doc = "PDAWAKECFG register accessor: an alias for `Reg<PDAWAKECFG_SPEC>`"]
pub type PDAWAKECFG = crate::Reg<pdawakecfg::PDAWAKECFG_SPEC>;
#[doc = "Power-down states for wake-up from deep-sleep"]
pub mod pdawakecfg;
#[doc = "PDRUNCFG register accessor: an alias for `Reg<PDRUNCFG_SPEC>`"]
pub type PDRUNCFG = crate::Reg<pdruncfg::PDRUNCFG_SPEC>;
#[doc = "Power configuration register"]
pub mod pdruncfg;
#[doc = "DEVICE_ID register accessor: an alias for `Reg<DEVICE_ID_SPEC>`"]
pub type DEVICE_ID = crate::Reg<device_id::DEVICE_ID_SPEC>;
#[doc = "Device ID"]
pub mod device_id;
