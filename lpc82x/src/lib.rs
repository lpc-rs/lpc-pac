#![doc = "Peripheral access API for LPC82X microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn SPI0();
    fn SPI1();
    fn USART0();
    fn USART1();
    fn USART2();
    fn I2C1();
    fn I2C0();
    fn SCT0();
    fn MRT0();
    fn CMP();
    fn WDT();
    fn BOD();
    fn FLASH();
    fn WKT();
    fn ADC0_SEQA();
    fn ADC0_SEQB();
    fn ADC0_THCMP();
    fn ADC0_OVR();
    fn DMA0();
    fn I2C2();
    fn I2C3();
    fn PIN_INT0();
    fn PIN_INT1();
    fn PIN_INT2();
    fn PIN_INT3();
    fn PIN_INT4();
    fn PIN_INT5();
    fn PIN_INT6();
    fn PIN_INT7();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: SPI0 },
    Vector { _handler: SPI1 },
    Vector { _reserved: 0 },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C0 },
    Vector { _handler: SCT0 },
    Vector { _handler: MRT0 },
    Vector { _handler: CMP },
    Vector { _handler: WDT },
    Vector { _handler: BOD },
    Vector { _handler: FLASH },
    Vector { _handler: WKT },
    Vector {
        _handler: ADC0_SEQA,
    },
    Vector {
        _handler: ADC0_SEQB,
    },
    Vector {
        _handler: ADC0_THCMP,
    },
    Vector { _handler: ADC0_OVR },
    Vector { _handler: DMA0 },
    Vector { _handler: I2C2 },
    Vector { _handler: I2C3 },
    Vector { _reserved: 0 },
    Vector { _handler: PIN_INT0 },
    Vector { _handler: PIN_INT1 },
    Vector { _handler: PIN_INT2 },
    Vector { _handler: PIN_INT3 },
    Vector { _handler: PIN_INT4 },
    Vector { _handler: PIN_INT5 },
    Vector { _handler: PIN_INT6 },
    Vector { _handler: PIN_INT7 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - SPI0"]
    SPI0 = 0,
    #[doc = "1 - SPI1"]
    SPI1 = 1,
    #[doc = "3 - USART0"]
    USART0 = 3,
    #[doc = "4 - USART1"]
    USART1 = 4,
    #[doc = "5 - USART2"]
    USART2 = 5,
    #[doc = "7 - I2C1"]
    I2C1 = 7,
    #[doc = "8 - I2C0"]
    I2C0 = 8,
    #[doc = "9 - SCT0"]
    SCT0 = 9,
    #[doc = "10 - MRT0"]
    MRT0 = 10,
    #[doc = "11 - CMP"]
    CMP = 11,
    #[doc = "12 - WDT"]
    WDT = 12,
    #[doc = "13 - BOD"]
    BOD = 13,
    #[doc = "14 - FLASH"]
    FLASH = 14,
    #[doc = "15 - WKT"]
    WKT = 15,
    #[doc = "16 - ADC0_SEQA"]
    ADC0_SEQA = 16,
    #[doc = "17 - ADC0_SEQB"]
    ADC0_SEQB = 17,
    #[doc = "18 - ADC0_THCMP"]
    ADC0_THCMP = 18,
    #[doc = "19 - ADC0_OVR"]
    ADC0_OVR = 19,
    #[doc = "20 - DMA0"]
    DMA0 = 20,
    #[doc = "21 - I2C2"]
    I2C2 = 21,
    #[doc = "22 - I2C3"]
    I2C3 = 22,
    #[doc = "24 - PIN_INT0"]
    PIN_INT0 = 24,
    #[doc = "25 - PIN_INT1"]
    PIN_INT1 = 25,
    #[doc = "26 - PIN_INT2"]
    PIN_INT2 = 26,
    #[doc = "27 - PIN_INT3"]
    PIN_INT3 = 27,
    #[doc = "28 - PIN_INT4"]
    PIN_INT4 = 28,
    #[doc = "29 - PIN_INT5"]
    PIN_INT5 = 29,
    #[doc = "30 - PIN_INT6"]
    PIN_INT6 = 30,
    #[doc = "31 - PIN_INT7"]
    PIN_INT7 = 31,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "LPC82x Micro Trace Buffer"]
pub struct MTB_SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB_SFR {}
impl MTB_SFR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mtb_sfr::RegisterBlock {
        0x1400_0000 as *const _
    }
}
impl Deref for MTB_SFR {
    type Target = mtb_sfr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MTB_SFR::ptr() }
    }
}
#[doc = "LPC82x Micro Trace Buffer"]
pub mod mtb_sfr;
#[doc = "LPC82x Windowed Watchdog Timer (WWDT)"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdt::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "LPC82x Windowed Watchdog Timer (WWDT)"]
pub mod wwdt;
#[doc = "LPC82x Multi-Rate Timer (MRT)"]
pub struct MRT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MRT0 {}
impl MRT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mrt0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for MRT0 {
    type Target = mrt0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MRT0::ptr() }
    }
}
#[doc = "LPC82x Multi-Rate Timer (MRT)"]
pub mod mrt0;
#[doc = "LPC82x Wake Up Timer(WKT)"]
pub struct WKT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WKT {}
impl WKT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wkt::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for WKT {
    type Target = wkt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WKT::ptr() }
    }
}
#[doc = "LPC82x Wake Up Timer(WKT)"]
pub mod wkt;
#[doc = "LPC82x SWM"]
pub struct SWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWM0 {}
impl SWM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swm0::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for SWM0 {
    type Target = swm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWM0::ptr() }
    }
}
#[doc = "LPC82x SWM"]
pub mod swm0;
#[doc = "LPC82x 12-bit ADC controller (ADC)"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "LPC82x 12-bit ADC controller (ADC)"]
pub mod adc0;
#[doc = "LPC82x PMU"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmu::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMU::ptr() }
    }
}
#[doc = "LPC82x PMU"]
pub mod pmu;
#[doc = "LPC82x analog comparator"]
pub struct ACOMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACOMP {}
impl ACOMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acomp::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for ACOMP {
    type Target = acomp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACOMP::ptr() }
    }
}
#[doc = "LPC82x analog comparator"]
pub mod acomp;
#[doc = "LPC82x Input multiplexing (INPUT MUX)"]
pub struct INPUTMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INPUTMUX {}
impl INPUTMUX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const inputmux::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for INPUTMUX {
    type Target = inputmux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*INPUTMUX::ptr() }
    }
}
#[doc = "LPC82x Input multiplexing (INPUT MUX)"]
pub mod inputmux;
#[doc = "LPC82x NVMC flash controller"]
pub struct FLASH_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CTRL {}
impl FLASH_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_ctrl::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for FLASH_CTRL {
    type Target = flash_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CTRL::ptr() }
    }
}
#[doc = "LPC82x NVMC flash controller"]
pub mod flash_ctrl;
#[doc = "LPC82x I/O pin configuration (IOCON)"]
pub struct IOCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOCON {}
impl IOCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iocon::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for IOCON {
    type Target = iocon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOCON::ptr() }
    }
}
#[doc = "LPC82x I/O pin configuration (IOCON)"]
pub mod iocon;
#[doc = "LPC82x System configuration (SYSCON)"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscon::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "LPC82x System configuration (SYSCON)"]
pub mod syscon;
#[doc = "LPC82x I2C-bus interfaces"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4005_0000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "LPC82x I2C-bus interfaces"]
pub mod i2c0;
#[doc = "LPC82x I2C-bus interfaces"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4005_4000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "LPC82x I2C-bus interfaces"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4007_0000 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "LPC82x I2C-bus interfaces"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4007_4000 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "LPC82x Serial Peripheral Interfaces (SPI)"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4005_8000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "LPC82x Serial Peripheral Interfaces (SPI)"]
pub mod spi0;
#[doc = "LPC82x Serial Peripheral Interfaces (SPI)"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4005_c000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "LPC82x USARTs"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4006_4000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "LPC82x USARTs"]
pub mod usart0;
#[doc = "LPC82x USARTs"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4006_8000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "LPC82x USARTs"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4006_c000 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "LPC5411x CRC engine"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "LPC5411x CRC engine"]
pub mod crc;
#[doc = "LPC82x SCTimer/PWM (SCT)"]
pub struct SCT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCT0 {}
impl SCT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sct0::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for SCT0 {
    type Target = sct0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCT0::ptr() }
    }
}
#[doc = "LPC82x SCTimer/PWM (SCT)"]
pub mod sct0;
#[doc = "LPC82x DMA controller"]
pub struct DMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA0 {}
impl DMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma0::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for DMA0 {
    type Target = dma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA0::ptr() }
    }
}
#[doc = "LPC82x DMA controller"]
pub mod dma0;
#[doc = "LPC82x General Purpose I/O (GPIO)"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0xa000_0000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "LPC82x General Purpose I/O (GPIO)"]
pub mod gpio;
#[doc = "LPC82x Pin interrupt and pattern match (PINT)"]
pub struct PINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PINT {}
impl PINT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pint::RegisterBlock {
        0xa000_4000 as *const _
    }
}
impl Deref for PINT {
    type Target = pint::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PINT::ptr() }
    }
}
#[doc = "LPC82x Pin interrupt and pattern match (PINT)"]
pub mod pint;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "MTB_SFR"]
    pub MTB_SFR: MTB_SFR,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "MRT0"]
    pub MRT0: MRT0,
    #[doc = "WKT"]
    pub WKT: WKT,
    #[doc = "SWM0"]
    pub SWM0: SWM0,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "ACOMP"]
    pub ACOMP: ACOMP,
    #[doc = "INPUTMUX"]
    pub INPUTMUX: INPUTMUX,
    #[doc = "FLASH_CTRL"]
    pub FLASH_CTRL: FLASH_CTRL,
    #[doc = "IOCON"]
    pub IOCON: IOCON,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "SCT0"]
    pub SCT0: SCT0,
    #[doc = "DMA0"]
    pub DMA0: DMA0,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "PINT"]
    pub PINT: PINT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            MTB_SFR: MTB_SFR {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            MRT0: MRT0 {
                _marker: PhantomData,
            },
            WKT: WKT {
                _marker: PhantomData,
            },
            SWM0: SWM0 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            ACOMP: ACOMP {
                _marker: PhantomData,
            },
            INPUTMUX: INPUTMUX {
                _marker: PhantomData,
            },
            FLASH_CTRL: FLASH_CTRL {
                _marker: PhantomData,
            },
            IOCON: IOCON {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            SCT0: SCT0 {
                _marker: PhantomData,
            },
            DMA0: DMA0 {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            PINT: PINT {
                _marker: PhantomData,
            },
        }
    }
}
