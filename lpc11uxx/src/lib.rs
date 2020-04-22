#![doc = "Peripheral access API for LPC11UXX microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
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
    fn SSP1();
    fn I2C();
    fn CT16B0();
    fn CT16B1();
    fn CT32B0();
    fn CT32B1();
    fn SSP0();
    fn USART();
    fn USB_IRQ();
    fn USB_FIQ();
    fn ADC();
    fn WDT();
    fn BOD_IRQ();
    fn FLASH_IRQ();
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
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SSP1 },
    Vector { _handler: I2C },
    Vector { _handler: CT16B0 },
    Vector { _handler: CT16B1 },
    Vector { _handler: CT32B0 },
    Vector { _handler: CT32B1 },
    Vector { _handler: SSP0 },
    Vector { _handler: USART },
    Vector { _handler: USB_IRQ },
    Vector { _handler: USB_FIQ },
    Vector { _handler: ADC },
    Vector { _handler: WDT },
    Vector { _handler: BOD_IRQ },
    Vector {
        _handler: FLASH_IRQ,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: USBWAKEUP,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - PIN_INT0"]
    PIN_INT0 = 0,
    #[doc = "1 - PIN_INT1"]
    PIN_INT1 = 1,
    #[doc = "2 - PIN_INT2"]
    PIN_INT2 = 2,
    #[doc = "3 - PIN_INT3"]
    PIN_INT3 = 3,
    #[doc = "4 - PIN_INT4"]
    PIN_INT4 = 4,
    #[doc = "5 - PIN_INT5"]
    PIN_INT5 = 5,
    #[doc = "6 - PIN_INT6"]
    PIN_INT6 = 6,
    #[doc = "7 - PIN_INT7"]
    PIN_INT7 = 7,
    #[doc = "8 - GINT0"]
    GINT0 = 8,
    #[doc = "9 - GINT1"]
    GINT1 = 9,
    #[doc = "14 - SSP1"]
    SSP1 = 14,
    #[doc = "15 - I2C"]
    I2C = 15,
    #[doc = "16 - CT16B0"]
    CT16B0 = 16,
    #[doc = "17 - CT16B1"]
    CT16B1 = 17,
    #[doc = "18 - CT32B0"]
    CT32B0 = 18,
    #[doc = "19 - CT32B1"]
    CT32B1 = 19,
    #[doc = "20 - SSP0"]
    SSP0 = 20,
    #[doc = "21 - USART"]
    USART = 21,
    #[doc = "22 - USB_IRQ"]
    USB_IRQ = 22,
    #[doc = "23 - USB_FIQ"]
    USB_FIQ = 23,
    #[doc = "24 - ADC"]
    ADC = 24,
    #[doc = "25 - WDT"]
    WDT = 25,
    #[doc = "26 - BOD_IRQ"]
    BOD_IRQ = 26,
    #[doc = "27 - FLASH_IRQ"]
    FLASH_IRQ = 27,
    #[doc = "30 - USBWAKEUP"]
    USBWAKEUP = 30,
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
#[doc = "I2C-bus controller"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C::ptr() }
    }
}
#[doc = "I2C-bus controller"]
pub mod i2c;
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
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "Windowed Watchdog Timer (WWDT)"]
pub mod wwdt;
#[doc = "USART"]
pub struct USART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART {}
impl USART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for USART {
    type Target = usart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART::ptr() }
    }
}
#[doc = "USART"]
pub mod usart;
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
    #[inline(always)]
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
    pub const fn ptr() -> *const ct16b1::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for CT16B1 {
    type Target = ct16b1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CT16B1::ptr() }
    }
}
#[doc = "16-bit counter/timers CT16B1"]
pub mod ct16b1;
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
    #[inline(always)]
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
    pub const fn ptr() -> *const ct32b1::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for CT32B1 {
    type Target = ct32b1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CT32B1::ptr() }
    }
}
#[doc = "32-bit counter/timers CT32B1"]
pub mod ct32b1;
#[doc = "ADC"]
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
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "ADC"]
pub mod adc;
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSP0::ptr() }
    }
}
#[doc = "SSP/SPI"]
pub mod ssp0;
#[doc = "I/O configuration Modification"]
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
#[doc = "I/O configuration Modification"]
pub mod iocon;
#[doc = "System control block"]
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
#[doc = "System control block"]
pub mod syscon;
#[doc = "GPIO pin interrupt"]
pub struct GPIO_PIN_INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PIN_INT {}
impl GPIO_PIN_INT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_pin_int::RegisterBlock {
        0x4004_c000 as *const _
    }
}
impl Deref for GPIO_PIN_INT {
    type Target = gpio_pin_int::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PIN_INT::ptr() }
    }
}
#[doc = "GPIO pin interrupt"]
pub mod gpio_pin_int;
#[doc = "SSP/SPI"]
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
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSP1::ptr() }
    }
}
#[doc = "GPIO group interrupt"]
pub struct GPIO_GROUP_INT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_GROUP_INT0 {}
impl GPIO_GROUP_INT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_group_int0::RegisterBlock {
        0x4005_c000 as *const _
    }
}
impl Deref for GPIO_GROUP_INT0 {
    type Target = gpio_group_int0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_GROUP_INT0::ptr() }
    }
}
#[doc = "GPIO group interrupt"]
pub mod gpio_group_int0;
#[doc = "GPIO group interrupt"]
pub struct GPIO_GROUP_INT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_GROUP_INT1 {}
impl GPIO_GROUP_INT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_group_int0::RegisterBlock {
        0x4006_0000 as *const _
    }
}
impl Deref for GPIO_GROUP_INT1 {
    type Target = gpio_group_int0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_GROUP_INT1::ptr() }
    }
}
#[doc = "USB2.0 device controller"]
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
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB2.0 device controller"]
pub mod usb;
#[doc = "GPIO port"]
pub struct GPIO_PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORT {}
impl GPIO_PORT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_port::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for GPIO_PORT {
    type Target = gpio_port::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORT::ptr() }
    }
}
#[doc = "GPIO port"]
pub mod gpio_port;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "USART"]
    pub USART: USART,
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
    #[doc = "GPIO_PIN_INT"]
    pub GPIO_PIN_INT: GPIO_PIN_INT,
    #[doc = "SSP1"]
    pub SSP1: SSP1,
    #[doc = "GPIO_GROUP_INT0"]
    pub GPIO_GROUP_INT0: GPIO_GROUP_INT0,
    #[doc = "GPIO_GROUP_INT1"]
    pub GPIO_GROUP_INT1: GPIO_GROUP_INT1,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "GPIO_PORT"]
    pub GPIO_PORT: GPIO_PORT,
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
            I2C: I2C {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            USART: USART {
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
            GPIO_PIN_INT: GPIO_PIN_INT {
                _marker: PhantomData,
            },
            SSP1: SSP1 {
                _marker: PhantomData,
            },
            GPIO_GROUP_INT0: GPIO_GROUP_INT0 {
                _marker: PhantomData,
            },
            GPIO_GROUP_INT1: GPIO_GROUP_INT1 {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            GPIO_PORT: GPIO_PORT {
                _marker: PhantomData,
            },
        }
    }
}
