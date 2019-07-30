#![doc = "Peripheral access API for LPC11U6X microcontrollers (generated using svd2rust v0.15.2)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.15.2/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
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
    fn PIN_INT0();
    fn PIN_INT1();
    fn PIN_INT2();
    fn PIN_INT3();
    fn PIN_INT4();
    fn PIN_INT5();
    fn PIN_INT6();
    fn PIN_INT7();
    fn GINT0();
    fn GINT1();
    fn I2C1();
    fn USART1_4();
    fn USART2_3();
    fn SCT0_1();
    fn SSP1();
    fn I2C0();
    fn CT16B0();
    fn CT16B1();
    fn CT32B0();
    fn CT32B1();
    fn SSP0();
    fn USART();
    fn USB();
    fn USB_FIQ();
    fn ADC_A();
    fn RTC();
    fn BOD_WDT();
    fn FLASH();
    fn DMA();
    fn USBWAKEUP();
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
pub static __INTERRUPTS: [Vector; 31] = [
    Vector { _handler: PIN_INT0 },
    Vector { _handler: PIN_INT1 },
    Vector { _handler: PIN_INT2 },
    Vector { _handler: PIN_INT3 },
    Vector { _handler: PIN_INT4 },
    Vector { _handler: PIN_INT5 },
    Vector { _handler: PIN_INT6 },
    Vector { _handler: PIN_INT7 },
    Vector { _handler: GINT0 },
    Vector { _handler: GINT1 },
    Vector { _handler: I2C1 },
    Vector { _handler: USART1_4 },
    Vector { _handler: USART2_3 },
    Vector { _handler: SCT0_1 },
    Vector { _handler: SSP1 },
    Vector { _handler: I2C0 },
    Vector { _handler: CT16B0 },
    Vector { _handler: CT16B1 },
    Vector { _handler: CT32B0 },
    Vector { _handler: CT32B1 },
    Vector { _handler: SSP0 },
    Vector { _handler: USART },
    Vector { _handler: USB },
    Vector { _handler: USB_FIQ },
    Vector { _handler: ADC_A },
    Vector { _handler: RTC },
    Vector { _handler: BOD_WDT },
    Vector { _handler: FLASH },
    Vector { _handler: DMA },
    Vector { _reserved: 0 },
    Vector {
        _handler: USBWAKEUP,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - PIN_INT0"]
    PIN_INT0,
    #[doc = "1 - PIN_INT1"]
    PIN_INT1,
    #[doc = "2 - PIN_INT2"]
    PIN_INT2,
    #[doc = "3 - PIN_INT3"]
    PIN_INT3,
    #[doc = "4 - PIN_INT4"]
    PIN_INT4,
    #[doc = "5 - PIN_INT5"]
    PIN_INT5,
    #[doc = "6 - PIN_INT6"]
    PIN_INT6,
    #[doc = "7 - PIN_INT7"]
    PIN_INT7,
    #[doc = "8 - GINT0"]
    GINT0,
    #[doc = "9 - GINT1"]
    GINT1,
    #[doc = "10 - I2C1"]
    I2C1,
    #[doc = "11 - USART1_4"]
    USART1_4,
    #[doc = "12 - USART2_3"]
    USART2_3,
    #[doc = "13 - SCT0_1"]
    SCT0_1,
    #[doc = "14 - SSP1"]
    SSP1,
    #[doc = "15 - I2C0"]
    I2C0,
    #[doc = "16 - CT16B0"]
    CT16B0,
    #[doc = "17 - CT16B1"]
    CT16B1,
    #[doc = "18 - CT32B0"]
    CT32B0,
    #[doc = "19 - CT32B1"]
    CT32B1,
    #[doc = "20 - SSP0"]
    SSP0,
    #[doc = "21 - USART"]
    USART,
    #[doc = "22 - USB"]
    USB,
    #[doc = "23 - USB_FIQ"]
    USB_FIQ,
    #[doc = "24 - ADC_A"]
    ADC_A,
    #[doc = "25 - RTC"]
    RTC,
    #[doc = "26 - BOD_WDT"]
    BOD_WDT,
    #[doc = "27 - FLASH"]
    FLASH,
    #[doc = "28 - DMA"]
    DMA,
    #[doc = "30 - USBWAKEUP"]
    USBWAKEUP,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PIN_INT0 => 0,
            Interrupt::PIN_INT1 => 1,
            Interrupt::PIN_INT2 => 2,
            Interrupt::PIN_INT3 => 3,
            Interrupt::PIN_INT4 => 4,
            Interrupt::PIN_INT5 => 5,
            Interrupt::PIN_INT6 => 6,
            Interrupt::PIN_INT7 => 7,
            Interrupt::GINT0 => 8,
            Interrupt::GINT1 => 9,
            Interrupt::I2C1 => 10,
            Interrupt::USART1_4 => 11,
            Interrupt::USART2_3 => 12,
            Interrupt::SCT0_1 => 13,
            Interrupt::SSP1 => 14,
            Interrupt::I2C0 => 15,
            Interrupt::CT16B0 => 16,
            Interrupt::CT16B1 => 17,
            Interrupt::CT32B0 => 18,
            Interrupt::CT32B1 => 19,
            Interrupt::SSP0 => 20,
            Interrupt::USART => 21,
            Interrupt::USB => 22,
            Interrupt::USB_FIQ => 23,
            Interrupt::ADC_A => 24,
            Interrupt::RTC => 25,
            Interrupt::BOD_WDT => 26,
            Interrupt::FLASH => 27,
            Interrupt::DMA => 28,
            Interrupt::USBWAKEUP => 30,
        }
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
#[doc = r" Common register and bit access and modify traits"]
pub mod generic;
#[doc = "I2C-bus controller"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C-bus controller"]
pub mod i2c0;
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdt::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub mod wwdt;
#[doc = "USART0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "USART0"]
pub mod usart0;
#[doc = "16-bit counter/timers CT16B0"]
pub struct CT16B0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT16B0 {}
impl CT16B0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ct16b0::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for CT16B0 {
    type Target = ct16b0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CT16B0::ptr() }
    }
}
#[doc = "16-bit counter/timers CT16B0"]
pub mod ct16b0;
#[doc = "16-bit counter/timers CT16B1"]
pub struct CT16B1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT16B1 {}
impl CT16B1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ct16b0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for CT16B1 {
    type Target = ct16b0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CT16B1::ptr() }
    }
}
#[doc = "32-bit counter/timers CT32B0"]
pub struct CT32B0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT32B0 {}
impl CT32B0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ct32b0::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for CT32B0 {
    type Target = ct32b0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CT32B0::ptr() }
    }
}
#[doc = "32-bit counter/timers CT32B0"]
pub mod ct32b0;
#[doc = "32-bit counter/timers CT32B1"]
pub struct CT32B1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CT32B1 {}
impl CT32B1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ct32b0::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for CT32B1 {
    type Target = ct32b0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CT32B1::ptr() }
    }
}
#[doc = "12-bit Analog-to-Digital Converter (ADC)"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "12-bit Analog-to-Digital Converter (ADC)"]
pub mod adc;
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Real-Time Clock (RTC)"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-Time Clock (RTC)"]
pub mod rtc;
#[doc = "DMA controller"]
pub struct DMATRIGMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMATRIGMUX {}
impl DMATRIGMUX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmatrigmux::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for DMATRIGMUX {
    type Target = dmatrigmux::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMATRIGMUX::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dmatrigmux;
#[doc = "Power Management Unit (PMU)"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmu::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMU::ptr() }
    }
}
#[doc = "Power Management Unit (PMU)"]
pub mod pmu;
#[doc = "Flash controller"]
pub struct FLASHCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASHCTRL {}
impl FLASHCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flashctrl::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for FLASHCTRL {
    type Target = flashctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASHCTRL::ptr() }
    }
}
#[doc = "Flash controller"]
pub mod flashctrl;
#[doc = "SSP/SPI"]
pub struct SSP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP0 {}
impl SSP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssp0::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for SSP0 {
    type Target = ssp0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSP0::ptr() }
    }
}
#[doc = "SSP/SPI"]
pub mod ssp0;
#[doc = "I/O control (IOCON)"]
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
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOCON::ptr() }
    }
}
#[doc = "I/O control (IOCON)"]
pub mod iocon;
#[doc = "System configuration (SYSCON)"]
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
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "System configuration (SYSCON)"]
pub mod syscon;
#[doc = "USART4"]
pub struct USART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART4 {}
impl USART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart4::RegisterBlock {
        0x4004_c000 as *const _
    }
}
impl Deref for USART4 {
    type Target = usart4::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART4::ptr() }
    }
}
#[doc = "USART4"]
pub mod usart4;
#[doc = "SSP1"]
pub struct SSP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP1 {}
impl SSP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssp0::RegisterBlock {
        0x4005_8000 as *const _
    }
}
impl Deref for SSP1 {
    type Target = ssp0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSP1::ptr() }
    }
}
#[doc = "GPIO group interrupt 0"]
pub struct GINT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GINT0 {}
impl GINT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gint0::RegisterBlock {
        0x4005_c000 as *const _
    }
}
impl Deref for GINT0 {
    type Target = gint0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GINT0::ptr() }
    }
}
#[doc = "GPIO group interrupt 0"]
pub mod gint0;
#[doc = "GINT1"]
pub struct GINT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GINT1 {}
impl GINT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gint0::RegisterBlock {
        0x4006_0000 as *const _
    }
}
impl Deref for GINT1 {
    type Target = gint0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GINT1::ptr() }
    }
}
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart4::RegisterBlock {
        0x4006_c000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart4::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart4::RegisterBlock {
        0x4007_0000 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart4::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart4::RegisterBlock {
        0x4007_4000 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart4::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "USB device controller"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB device controller"]
pub mod usb;
#[doc = "Cyclic Redundancy Check (CRC) engine"]
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
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check (CRC) engine"]
pub mod crc;
#[doc = "DMA controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma;
#[doc = "State Configurable Timers (SCTimer/PWM)"]
pub struct SCT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCT0 {}
impl SCT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sct0::RegisterBlock {
        0x5000_c000 as *const _
    }
}
impl Deref for SCT0 {
    type Target = sct0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCT0::ptr() }
    }
}
#[doc = "State Configurable Timers (SCTimer/PWM)"]
pub mod sct0;
#[doc = "SCT1"]
pub struct SCT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCT1 {}
impl SCT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sct0::RegisterBlock {
        0x5000_e000 as *const _
    }
}
impl Deref for SCT1 {
    type Target = sct0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCT1::ptr() }
    }
}
#[doc = "General Purpose I/O (GPIO)"]
pub struct GPIO_PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORT {}
impl GPIO_PORT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_port::RegisterBlock {
        0xa000_0000 as *const _
    }
}
impl Deref for GPIO_PORT {
    type Target = gpio_port::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORT::ptr() }
    }
}
#[doc = "General Purpose I/O (GPIO)"]
pub mod gpio_port;
#[doc = "Pin interrupt and pattern match (PINT)"]
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
    fn deref(&self) -> &Self::Target {
        unsafe { &*PINT::ptr() }
    }
}
#[doc = "Pin interrupt and pattern match (PINT)"]
pub mod pint;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "CT16B0"]
    pub CT16B0: CT16B0,
    #[doc = "CT16B1"]
    pub CT16B1: CT16B1,
    #[doc = "CT32B0"]
    pub CT32B0: CT32B0,
    #[doc = "CT32B1"]
    pub CT32B1: CT32B1,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "DMATRIGMUX"]
    pub DMATRIGMUX: DMATRIGMUX,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "FLASHCTRL"]
    pub FLASHCTRL: FLASHCTRL,
    #[doc = "SSP0"]
    pub SSP0: SSP0,
    #[doc = "IOCON"]
    pub IOCON: IOCON,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "USART4"]
    pub USART4: USART4,
    #[doc = "SSP1"]
    pub SSP1: SSP1,
    #[doc = "GINT0"]
    pub GINT0: GINT0,
    #[doc = "GINT1"]
    pub GINT1: GINT1,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "SCT0"]
    pub SCT0: SCT0,
    #[doc = "SCT1"]
    pub SCT1: SCT1,
    #[doc = "GPIO_PORT"]
    pub GPIO_PORT: GPIO_PORT,
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
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            CT16B0: CT16B0 {
                _marker: PhantomData,
            },
            CT16B1: CT16B1 {
                _marker: PhantomData,
            },
            CT32B0: CT32B0 {
                _marker: PhantomData,
            },
            CT32B1: CT32B1 {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            DMATRIGMUX: DMATRIGMUX {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            FLASHCTRL: FLASHCTRL {
                _marker: PhantomData,
            },
            SSP0: SSP0 {
                _marker: PhantomData,
            },
            IOCON: IOCON {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            USART4: USART4 {
                _marker: PhantomData,
            },
            SSP1: SSP1 {
                _marker: PhantomData,
            },
            GINT0: GINT0 {
                _marker: PhantomData,
            },
            GINT1: GINT1 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            SCT0: SCT0 {
                _marker: PhantomData,
            },
            SCT1: SCT1 {
                _marker: PhantomData,
            },
            GPIO_PORT: GPIO_PORT {
                _marker: PhantomData,
            },
            PINT: PINT {
                _marker: PhantomData,
            },
        }
    }
}
