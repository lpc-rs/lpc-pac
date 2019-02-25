#![doc = "Peripheral access API for LPC82X microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
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
    fn UART0();
    fn UART1();
    fn UART2();
    fn I2C1();
    fn I2C0();
    fn SCT();
    fn MRT();
    fn CMP();
    fn WDT();
    fn BOD();
    fn FLASH();
    fn WKT();
    fn ADC_SEQA();
    fn ADC_SEQB();
    fn ADC_THCMP();
    fn ADC_OVR();
    fn DMA();
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
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: UART2 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C0 },
    Vector { _handler: SCT },
    Vector { _handler: MRT },
    Vector { _handler: CMP },
    Vector { _handler: WDT },
    Vector { _handler: BOD },
    Vector { _handler: FLASH },
    Vector { _handler: WKT },
    Vector { _handler: ADC_SEQA },
    Vector { _handler: ADC_SEQB },
    Vector {
        _handler: ADC_THCMP,
    },
    Vector { _handler: ADC_OVR },
    Vector { _handler: DMA },
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
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - SPI0"]
    SPI0,
    #[doc = "1 - SPI1"]
    SPI1,
    #[doc = "3 - UART0"]
    UART0,
    #[doc = "4 - UART1"]
    UART1,
    #[doc = "5 - UART2"]
    UART2,
    #[doc = "7 - I2C1"]
    I2C1,
    #[doc = "8 - I2C0"]
    I2C0,
    #[doc = "9 - SCT"]
    SCT,
    #[doc = "10 - MRT"]
    MRT,
    #[doc = "11 - CMP"]
    CMP,
    #[doc = "12 - WDT"]
    WDT,
    #[doc = "13 - BOD"]
    BOD,
    #[doc = "14 - FLASH"]
    FLASH,
    #[doc = "15 - WKT"]
    WKT,
    #[doc = "16 - ADC_SEQA"]
    ADC_SEQA,
    #[doc = "17 - ADC_SEQB"]
    ADC_SEQB,
    #[doc = "18 - ADC_THCMP"]
    ADC_THCMP,
    #[doc = "19 - ADC_OVR"]
    ADC_OVR,
    #[doc = "20 - DMA"]
    DMA,
    #[doc = "21 - I2C2"]
    I2C2,
    #[doc = "22 - I2C3"]
    I2C3,
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
    #[doc = "29 - PIN_INT5"]
    PIN_INT5,
    #[doc = "30 - PIN_INT6"]
    PIN_INT6,
    #[doc = "31 - PIN_INT7"]
    PIN_INT7,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SPI0 => 0,
            Interrupt::SPI1 => 1,
            Interrupt::UART0 => 3,
            Interrupt::UART1 => 4,
            Interrupt::UART2 => 5,
            Interrupt::I2C1 => 7,
            Interrupt::I2C0 => 8,
            Interrupt::SCT => 9,
            Interrupt::MRT => 10,
            Interrupt::CMP => 11,
            Interrupt::WDT => 12,
            Interrupt::BOD => 13,
            Interrupt::FLASH => 14,
            Interrupt::WKT => 15,
            Interrupt::ADC_SEQA => 16,
            Interrupt::ADC_SEQB => 17,
            Interrupt::ADC_THCMP => 18,
            Interrupt::ADC_OVR => 19,
            Interrupt::DMA => 20,
            Interrupt::I2C2 => 21,
            Interrupt::I2C3 => 22,
            Interrupt::PIN_INT0 => 24,
            Interrupt::PIN_INT1 => 25,
            Interrupt::PIN_INT2 => 26,
            Interrupt::PIN_INT3 => 27,
            Interrupt::PIN_INT4 => 28,
            Interrupt::PIN_INT5 => 29,
            Interrupt::PIN_INT6 => 30,
            Interrupt::PIN_INT7 => 31,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Windowed Watchdog Timer (WWDT)"]
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
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub mod wwdt;
#[doc = "Multi-Rate Timer (MRT)"]
pub struct MRT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MRT {}
impl MRT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mrt::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for MRT {
    type Target = mrt::RegisterBlock;
    fn deref(&self) -> &mrt::RegisterBlock {
        unsafe { &*MRT::ptr() }
    }
}
#[doc = "Multi-Rate Timer (MRT)"]
pub mod mrt;
#[doc = "Self wake-up timer (WKT)"]
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
#[doc = "Self wake-up timer (WKT)"]
pub mod wkt;
#[doc = "Switch matrix (SWM)"]
pub struct SWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWM {}
impl SWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swm::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for SWM {
    type Target = swm::RegisterBlock;
    fn deref(&self) -> &swm::RegisterBlock {
        unsafe { &*SWM::ptr() }
    }
}
#[doc = "Switch matrix (SWM)"]
pub mod swm;
#[doc = "12-bit Analog-to-Digital Converter (ADC)"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "12-bit Analog-to-Digital Converter (ADC)"]
pub mod adc;
#[doc = "Power Management Unit (PMU)"]
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
#[doc = "Power Management Unit (PMU)"]
pub mod pmu;
#[doc = "Analog comparator"]
pub struct CMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP {}
impl CMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmp::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for CMP {
    type Target = cmp::RegisterBlock;
    fn deref(&self) -> &cmp::RegisterBlock {
        unsafe { &*CMP::ptr() }
    }
}
#[doc = "Analog comparator"]
pub mod cmp;
#[doc = "DMA trigger mux"]
pub struct DMATRIGMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMATRIGMUX {}
impl DMATRIGMUX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmatrigmux::RegisterBlock {
        1073905664 as *const _
    }
}
impl Deref for DMATRIGMUX {
    type Target = dmatrigmux::RegisterBlock;
    fn deref(&self) -> &dmatrigmux::RegisterBlock {
        unsafe { &*DMATRIGMUX::ptr() }
    }
}
#[doc = "DMA trigger mux"]
pub mod dmatrigmux;
#[doc = "Input multiplexing"]
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
#[doc = "Input multiplexing"]
pub mod inputmux;
#[doc = "Flash controller"]
pub struct FLASHCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASHCTRL {}
impl FLASHCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flashctrl::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for FLASHCTRL {
    type Target = flashctrl::RegisterBlock;
    fn deref(&self) -> &flashctrl::RegisterBlock {
        unsafe { &*FLASHCTRL::ptr() }
    }
}
#[doc = "Flash controller"]
pub mod flashctrl;
#[doc = "I/O configuration (IOCON)"]
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
#[doc = "I/O configuration (IOCON)"]
pub mod iocon;
#[doc = "System configuration (SYSCON)"]
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
#[doc = "System configuration (SYSCON)"]
pub mod syscon;
#[doc = "I2C0-bus interface"]
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
#[doc = "I2C0-bus interface"]
pub mod i2c0;
#[doc = "I2C1"]
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
#[doc = "SPI0"]
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
#[doc = "SPI0"]
pub mod spi0;
#[doc = "SPI1"]
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
#[doc = "USART0"]
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
#[doc = "USART0"]
pub mod usart0;
#[doc = "USART1"]
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
#[doc = "USART2"]
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
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074200576 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C3"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074216960 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check (CRC) engine"]
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
#[doc = "Cyclic Redundancy Check (CRC) engine"]
pub mod crc;
#[doc = "State Configurable Timer (SCT)"]
pub struct SCT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCT {}
impl SCT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sct::RegisterBlock {
        1342193664 as *const _
    }
}
impl Deref for SCT {
    type Target = sct::RegisterBlock;
    fn deref(&self) -> &sct::RegisterBlock {
        unsafe { &*SCT::ptr() }
    }
}
#[doc = "State Configurable Timer (SCT)"]
pub mod sct;
#[doc = "DMA controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma::RegisterBlock {
        1342210048 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma;
#[doc = "General Purpose I/O port (GPIO)"]
pub struct GPIO_PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORT {}
impl GPIO_PORT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio_port::RegisterBlock {
        2684354560 as *const _
    }
}
impl Deref for GPIO_PORT {
    type Target = gpio_port::RegisterBlock;
    fn deref(&self) -> &gpio_port::RegisterBlock {
        unsafe { &*GPIO_PORT::ptr() }
    }
}
#[doc = "General Purpose I/O port (GPIO)"]
pub mod gpio_port;
#[doc = "Pin interrupt and pattern match engine"]
pub struct PIN_INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIN_INT {}
impl PIN_INT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pin_int::RegisterBlock {
        2684370944 as *const _
    }
}
impl Deref for PIN_INT {
    type Target = pin_int::RegisterBlock;
    fn deref(&self) -> &pin_int::RegisterBlock {
        unsafe { &*PIN_INT::ptr() }
    }
}
#[doc = "Pin interrupt and pattern match engine"]
pub mod pin_int;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "MRT"]
    pub MRT: MRT,
    #[doc = "WKT"]
    pub WKT: WKT,
    #[doc = "SWM"]
    pub SWM: SWM,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "PMU"]
    pub PMU: PMU,
    #[doc = "CMP"]
    pub CMP: CMP,
    #[doc = "DMATRIGMUX"]
    pub DMATRIGMUX: DMATRIGMUX,
    #[doc = "INPUTMUX"]
    pub INPUTMUX: INPUTMUX,
    #[doc = "FLASHCTRL"]
    pub FLASHCTRL: FLASHCTRL,
    #[doc = "IOCON"]
    pub IOCON: IOCON,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
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
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "SCT"]
    pub SCT: SCT,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "GPIO_PORT"]
    pub GPIO_PORT: GPIO_PORT,
    #[doc = "PIN_INT"]
    pub PIN_INT: PIN_INT,
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
            MRT: MRT {
                _marker: PhantomData,
            },
            WKT: WKT {
                _marker: PhantomData,
            },
            SWM: SWM {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            PMU: PMU {
                _marker: PhantomData,
            },
            CMP: CMP {
                _marker: PhantomData,
            },
            DMATRIGMUX: DMATRIGMUX {
                _marker: PhantomData,
            },
            INPUTMUX: INPUTMUX {
                _marker: PhantomData,
            },
            FLASHCTRL: FLASHCTRL {
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
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            SCT: SCT {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            GPIO_PORT: GPIO_PORT {
                _marker: PhantomData,
            },
            PIN_INT: PIN_INT {
                _marker: PhantomData,
            },
        }
    }
}
