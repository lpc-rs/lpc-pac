#![doc = "Peripheral access API for LPC845 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
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
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn SPI0();
    fn SPI1();
    fn DAC0();
    fn USART0();
    fn USART1();
    fn USART2();
    fn I2C1();
    fn I2C0();
    fn SCT0();
    fn CMP_CAPT();
    fn WKT();
    fn ADC0_SEQA();
    fn ADC0_SEQB();
    fn ADC0_THCMP();
    fn ADC0_OVR();
    fn DMA0();
    fn I2C2();
    fn I2C3();
    fn CTIMER0();
    fn PIN_INT0();
    fn PIN_INT1();
    fn PIN_INT2();
    fn PIN_INT3();
    fn PIN_INT4();
    fn PIN_INT5_DAC1();
    fn PIN_INT6_USART3();
    fn PIN_INT7_USART4();
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
    Vector { _handler: DAC0 },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C0 },
    Vector { _handler: SCT0 },
    Vector { _reserved: 0 },
    Vector { _handler: CMP_CAPT },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
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
    Vector { _handler: CTIMER0 },
    Vector { _handler: PIN_INT0 },
    Vector { _handler: PIN_INT1 },
    Vector { _handler: PIN_INT2 },
    Vector { _handler: PIN_INT3 },
    Vector { _handler: PIN_INT4 },
    Vector {
        _handler: PIN_INT5_DAC1,
    },
    Vector {
        _handler: PIN_INT6_USART3,
    },
    Vector {
        _handler: PIN_INT7_USART4,
    },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - SPI0"]
    SPI0,
    #[doc = "1 - SPI1"]
    SPI1,
    #[doc = "2 - DAC0"]
    DAC0,
    #[doc = "3 - USART0"]
    USART0,
    #[doc = "4 - USART1"]
    USART1,
    #[doc = "5 - USART2"]
    USART2,
    #[doc = "7 - I2C1"]
    I2C1,
    #[doc = "8 - I2C0"]
    I2C0,
    #[doc = "9 - SCT0"]
    SCT0,
    #[doc = "11 - CMP_CAPT"]
    CMP_CAPT,
    #[doc = "15 - WKT"]
    WKT,
    #[doc = "16 - ADC0_SEQA"]
    ADC0_SEQA,
    #[doc = "17 - ADC0_SEQB"]
    ADC0_SEQB,
    #[doc = "18 - ADC0_THCMP"]
    ADC0_THCMP,
    #[doc = "19 - ADC0_OVR"]
    ADC0_OVR,
    #[doc = "20 - DMA0"]
    DMA0,
    #[doc = "21 - I2C2"]
    I2C2,
    #[doc = "22 - I2C3"]
    I2C3,
    #[doc = "23 - CTIMER0"]
    CTIMER0,
    #[doc = "24 - PIN_INT0"]
    PIN_INT0,
    #[doc = "25 - PIN_INT1"]
    PIN_INT1,
    #[doc = "26 - PIN_INT2"]
    PIN_INT2,
    #[doc = "27 - PIN_INT3"]
    PIN_INT3,
    #[doc = "28 - PIN_INT4"]
    PIN_INT4,
    #[doc = "29 - PIN_INT5_DAC1"]
    PIN_INT5_DAC1,
    #[doc = "30 - PIN_INT6_USART3"]
    PIN_INT6_USART3,
    #[doc = "31 - PIN_INT7_USART4"]
    PIN_INT7_USART4,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SPI0 => 0,
            Interrupt::SPI1 => 1,
            Interrupt::DAC0 => 2,
            Interrupt::USART0 => 3,
            Interrupt::USART1 => 4,
            Interrupt::USART2 => 5,
            Interrupt::I2C1 => 7,
            Interrupt::I2C0 => 8,
            Interrupt::SCT0 => 9,
            Interrupt::CMP_CAPT => 11,
            Interrupt::WKT => 15,
            Interrupt::ADC0_SEQA => 16,
            Interrupt::ADC0_SEQB => 17,
            Interrupt::ADC0_THCMP => 18,
            Interrupt::ADC0_OVR => 19,
            Interrupt::DMA0 => 20,
            Interrupt::I2C2 => 21,
            Interrupt::I2C3 => 22,
            Interrupt::CTIMER0 => 23,
            Interrupt::PIN_INT0 => 24,
            Interrupt::PIN_INT1 => 25,
            Interrupt::PIN_INT2 => 26,
            Interrupt::PIN_INT3 => 27,
            Interrupt::PIN_INT4 => 28,
            Interrupt::PIN_INT5_DAC1 => 29,
            Interrupt::PIN_INT6_USART3 => 30,
            Interrupt::PIN_INT7_USART4 => 31,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "LPC84x Windowed Watchdog Timer (WWDT)"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdt::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    fn deref(&self) -> &wwdt::RegisterBlock {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "LPC84x Windowed Watchdog Timer (WWDT)"]
pub mod wwdt;
#[doc = "LPC84x Multi-Rate Timer (MRT)"]
pub struct MRT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MRT0 {}
impl MRT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mrt0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for MRT0 {
    type Target = mrt0::RegisterBlock;
    fn deref(&self) -> &mrt0::RegisterBlock {
        unsafe { &*MRT0::ptr() }
    }
}
#[doc = "LPC84x Multi-Rate Timer (MRT)"]
pub mod mrt0;
#[doc = "LPC84x Wake Up Timer(WKT)"]
pub struct WKT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WKT {}
impl WKT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wkt::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for WKT {
    type Target = wkt::RegisterBlock;
    fn deref(&self) -> &wkt::RegisterBlock {
        unsafe { &*WKT::ptr() }
    }
}
#[doc = "LPC84x Wake Up Timer(WKT)"]
pub mod wkt;
#[doc = "LPC84x SWM"]
pub struct SWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWM0 {}
impl SWM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swm0::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for SWM0 {
    type Target = swm0::RegisterBlock;
    fn deref(&self) -> &swm0::RegisterBlock {
        unsafe { &*SWM0::ptr() }
    }
}
#[doc = "LPC84x SWM"]
pub mod swm0;
#[doc = "LPC84x 10-bit DAC controller (DAC)"]
pub struct DAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC0 {}
impl DAC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for DAC0 {
    type Target = dac0::RegisterBlock;
    fn deref(&self) -> &dac0::RegisterBlock {
        unsafe { &*DAC0::ptr() }
    }
}
#[doc = "LPC84x 10-bit DAC controller (DAC)"]
pub mod dac0;
#[doc = "LPC84x 10-bit DAC controller (DAC)"]
pub struct DAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC1 {}
impl DAC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for DAC1 {
    type Target = dac0::RegisterBlock;
    fn deref(&self) -> &dac0::RegisterBlock {
        unsafe { &*DAC1::ptr() }
    }
}
#[doc = "LPC84x 12-bit ADC controller (ADC)"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "LPC84x 12-bit ADC controller (ADC)"]
pub mod adc0;
#[doc = "LPC84x PMU"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for PMU {
    type Target = pmu::RegisterBlock;
    fn deref(&self) -> &pmu::RegisterBlock {
        unsafe { &*PMU::ptr() }
    }
}
#[doc = "LPC84x PMU"]
pub mod pmu;
#[doc = "LPC84x analog comparator"]
pub struct ACOMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACOMP {}
impl ACOMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acomp::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for ACOMP {
    type Target = acomp::RegisterBlock;
    fn deref(&self) -> &acomp::RegisterBlock {
        unsafe { &*ACOMP::ptr() }
    }
}
#[doc = "LPC84x analog comparator"]
pub mod acomp;
#[doc = "LPC84x Input multiplexing (INPUT MUX)"]
pub struct INPUTMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INPUTMUX {}
impl INPUTMUX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const inputmux::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for INPUTMUX {
    type Target = inputmux::RegisterBlock;
    fn deref(&self) -> &inputmux::RegisterBlock {
        unsafe { &*INPUTMUX::ptr() }
    }
}
#[doc = "LPC84x Input multiplexing (INPUT MUX)"]
pub mod inputmux;
#[doc = "LPC184 Standard counter/timer"]
pub struct CTIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER0 {}
impl CTIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ctimer0::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for CTIMER0 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &ctimer0::RegisterBlock {
        unsafe { &*CTIMER0::ptr() }
    }
}
#[doc = "LPC184 Standard counter/timer"]
pub mod ctimer0;
#[doc = "LPC84x NVMC flash controller"]
pub struct FLASH_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CTRL {}
impl FLASH_CTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash_ctrl::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for FLASH_CTRL {
    type Target = flash_ctrl::RegisterBlock;
    fn deref(&self) -> &flash_ctrl::RegisterBlock {
        unsafe { &*FLASH_CTRL::ptr() }
    }
}
#[doc = "LPC84x NVMC flash controller"]
pub mod flash_ctrl;
#[doc = "LPC84x I/O pin configuration (IOCON)"]
pub struct IOCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOCON {}
impl IOCON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iocon::RegisterBlock {
        1074020352 as *const _
    }
}
impl Deref for IOCON {
    type Target = iocon::RegisterBlock;
    fn deref(&self) -> &iocon::RegisterBlock {
        unsafe { &*IOCON::ptr() }
    }
}
#[doc = "LPC84x I/O pin configuration (IOCON)"]
pub mod iocon;
#[doc = "LPC84x System configuration (SYSCON)"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscon::RegisterBlock {
        1074036736 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    fn deref(&self) -> &syscon::RegisterBlock {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "LPC84x System configuration (SYSCON)"]
pub mod syscon;
#[doc = "LPC84x I2C-bus interfaces"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074069504 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "LPC84x I2C-bus interfaces"]
pub mod i2c0;
#[doc = "LPC84x I2C-bus interfaces"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1073938432 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "LPC84x I2C-bus interfaces"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1073954816 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "LPC84x I2C-bus interfaces"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074085888 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "LPC84x Serial Peripheral Interfaces (SPI)"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1074102272 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "LPC84x Serial Peripheral Interfaces (SPI)"]
pub mod spi0;
#[doc = "LPC84x Serial Peripheral Interfaces (SPI)"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1074118656 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "LPC84x Capacitive Touch"]
pub struct CAPT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAPT {}
impl CAPT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const capt::RegisterBlock {
        1074135040 as *const _
    }
}
impl Deref for CAPT {
    type Target = capt::RegisterBlock;
    fn deref(&self) -> &capt::RegisterBlock {
        unsafe { &*CAPT::ptr() }
    }
}
#[doc = "LPC84x Capacitive Touch"]
pub mod capt;
#[doc = "LPC84x USARTs"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074151424 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "LPC84x USARTs"]
pub mod usart0;
#[doc = "LPC84x USARTs"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074167808 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "LPC84x USARTs"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074184192 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "LPC84x USARTs"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074200576 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "LPC84x USARTs"]
pub struct USART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART4 {}
impl USART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074216960 as *const _
    }
}
impl Deref for USART4 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART4::ptr() }
    }
}
#[doc = "LPC5411x CRC engine"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "LPC5411x CRC engine"]
pub mod crc;
#[doc = "LPC84x SCTimer/PWM (SCT)"]
pub struct SCT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCT0 {}
impl SCT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sct0::RegisterBlock {
        1342193664 as *const _
    }
}
impl Deref for SCT0 {
    type Target = sct0::RegisterBlock;
    fn deref(&self) -> &sct0::RegisterBlock {
        unsafe { &*SCT0::ptr() }
    }
}
#[doc = "LPC84x SCTimer/PWM (SCT)"]
pub mod sct0;
#[doc = "LPC84x DMA controller"]
pub struct DMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA0 {}
impl DMA0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma0::RegisterBlock {
        1342210048 as *const _
    }
}
impl Deref for DMA0 {
    type Target = dma0::RegisterBlock;
    fn deref(&self) -> &dma0::RegisterBlock {
        unsafe { &*DMA0::ptr() }
    }
}
#[doc = "LPC84x DMA controller"]
pub mod dma0;
#[doc = "LPC84x Micro Trace Buffer"]
pub struct MTB_SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB_SFR {}
impl MTB_SFR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mtb_sfr::RegisterBlock {
        1342226432 as *const _
    }
}
impl Deref for MTB_SFR {
    type Target = mtb_sfr::RegisterBlock;
    fn deref(&self) -> &mtb_sfr::RegisterBlock {
        unsafe { &*MTB_SFR::ptr() }
    }
}
#[doc = "LPC84x Micro Trace Buffer"]
pub mod mtb_sfr;
#[doc = "LPC84x General Purpose I/O (GPIO)"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        2684354560 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "LPC84x General Purpose I/O (GPIO)"]
pub mod gpio;
#[doc = "LPC84x Pin interrupt and pattern match (PINT)"]
pub struct PINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PINT {}
impl PINT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pint::RegisterBlock {
        2684370944 as *const _
    }
}
impl Deref for PINT {
    type Target = pint::RegisterBlock;
    fn deref(&self) -> &pint::RegisterBlock {
        unsafe { &*PINT::ptr() }
    }
}
#[doc = "LPC84x Pin interrupt and pattern match (PINT)"]
pub mod pint;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "MRT0"]
    pub MRT0: MRT0,
    #[doc = "WKT"]
    pub WKT: WKT,
    #[doc = "SWM0"]
    pub SWM0: SWM0,
    #[doc = "DAC0"]
    pub DAC0: DAC0,
    #[doc = "DAC1"]
    pub DAC1: DAC1,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "ACOMP"]
    pub ACOMP: ACOMP,
    #[doc = "INPUTMUX"]
    pub INPUTMUX: INPUTMUX,
    #[doc = "CTIMER0"]
    pub CTIMER0: CTIMER0,
    #[doc = "FLASH_CTRL"]
    pub FLASH_CTRL: FLASH_CTRL,
    #[doc = "IOCON"]
    pub IOCON: IOCON,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "CAPT"]
    pub CAPT: CAPT,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "USART4"]
    pub USART4: USART4,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "SCT0"]
    pub SCT0: SCT0,
    #[doc = "DMA0"]
    pub DMA0: DMA0,
    #[doc = "MTB_SFR"]
    pub MTB_SFR: MTB_SFR,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "PINT"]
    pub PINT: PINT,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
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
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
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
            DAC0: DAC0 {
                _marker: PhantomData,
            },
            DAC1: DAC1 {
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
            CTIMER0: CTIMER0 {
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
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            CAPT: CAPT {
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
            USART3: USART3 {
                _marker: PhantomData,
            },
            USART4: USART4 {
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
            MTB_SFR: MTB_SFR {
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
