#![doc = "Peripheral access API for LPC11UXX microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(untagged_unions)]
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
#[doc = r" Enumeration of all the interrupts"]
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
    #[doc = "14 - SSP1"]
    SSP1,
    #[doc = "15 - I2C"]
    I2C,
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
    #[doc = "22 - USB_IRQ"]
    USB_IRQ,
    #[doc = "23 - USB_FIQ"]
    USB_FIQ,
    #[doc = "24 - ADC"]
    ADC,
    #[doc = "25 - WDT"]
    WDT,
    #[doc = "26 - BOD_IRQ"]
    BOD_IRQ,
    #[doc = "27 - FLASH_IRQ"]
    FLASH_IRQ,
    #[doc = "30 - USBWAKEUP"]
    USBWAKEUP,
}
unsafe impl ::bare_metal::Nr for Interrupt {
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
            Interrupt::SSP1 => 14,
            Interrupt::I2C => 15,
            Interrupt::CT16B0 => 16,
            Interrupt::CT16B1 => 17,
            Interrupt::CT32B0 => 18,
            Interrupt::CT32B1 => 19,
            Interrupt::SSP0 => 20,
            Interrupt::USART => 21,
            Interrupt::USB_IRQ => 22,
            Interrupt::USB_FIQ => 23,
            Interrupt::ADC => 24,
            Interrupt::WDT => 25,
            Interrupt::BOD_IRQ => 26,
            Interrupt::FLASH_IRQ => 27,
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
#[doc = "I2C-bus controller"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    fn deref(&self) -> &i2c::RegisterBlock {
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
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdt::RegisterBlock {
        1073758208 as *const _
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
#[doc = "USART"]
pub struct USART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART {}
impl USART {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for USART {
    type Target = usart::RegisterBlock;
    fn deref(&self) -> &usart::RegisterBlock {
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
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ct16b0::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for CT16B0 {
    type Target = ct16b0::RegisterBlock;
    fn deref(&self) -> &ct16b0::RegisterBlock {
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
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ct16b1::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for CT16B1 {
    type Target = ct16b1::RegisterBlock;
    fn deref(&self) -> &ct16b1::RegisterBlock {
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
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ct32b0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for CT32B0 {
    type Target = ct32b0::RegisterBlock;
    fn deref(&self) -> &ct32b0::RegisterBlock {
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
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ct32b1::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for CT32B1 {
    type Target = ct32b1::RegisterBlock;
    fn deref(&self) -> &ct32b1::RegisterBlock {
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
#[doc = "ADC"]
pub mod adc;
#[doc = "Power Management Unit (PMU)"]
pub struct PMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU {}
impl PMU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu::RegisterBlock {
        1073971200 as *const _
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
#[doc = "Flash controller"]
pub struct FLASHCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASHCTRL {}
impl FLASHCTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flashctrl::RegisterBlock {
        1073987584 as *const _
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
#[doc = "SSP/SPI"]
pub struct SSP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP0 {}
impl SSP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssp0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for SSP0 {
    type Target = ssp0::RegisterBlock;
    fn deref(&self) -> &ssp0::RegisterBlock {
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
#[doc = "I/O configuration Modification"]
pub mod iocon;
#[doc = "System control block"]
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
#[doc = "System control block"]
pub mod syscon;
#[doc = "GPIO pin interrupt"]
pub struct GPIO_PIN_INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PIN_INT {}
impl GPIO_PIN_INT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio_pin_int::RegisterBlock {
        1074053120 as *const _
    }
}
impl Deref for GPIO_PIN_INT {
    type Target = gpio_pin_int::RegisterBlock;
    fn deref(&self) -> &gpio_pin_int::RegisterBlock {
        unsafe { &*GPIO_PIN_INT::ptr() }
    }
}
#[doc = "GPIO pin interrupt"]
pub mod gpio_pin_int;
#[doc = "SSP1"]
pub struct SSP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP1 {}
impl SSP1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssp0::RegisterBlock {
        1074102272 as *const _
    }
}
impl Deref for SSP1 {
    type Target = ssp0::RegisterBlock;
    fn deref(&self) -> &ssp0::RegisterBlock {
        unsafe { &*SSP1::ptr() }
    }
}
#[doc = "GPIO group interrupt"]
pub struct GPIO_GROUP_INT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_GROUP_INT0 {}
impl GPIO_GROUP_INT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio_group_int0::RegisterBlock {
        1074118656 as *const _
    }
}
impl Deref for GPIO_GROUP_INT0 {
    type Target = gpio_group_int0::RegisterBlock;
    fn deref(&self) -> &gpio_group_int0::RegisterBlock {
        unsafe { &*GPIO_GROUP_INT0::ptr() }
    }
}
#[doc = "GPIO group interrupt"]
pub mod gpio_group_int0;
#[doc = "GPIO_GROUP_INT1"]
pub struct GPIO_GROUP_INT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_GROUP_INT1 {}
impl GPIO_GROUP_INT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio_group_int0::RegisterBlock {
        1074135040 as *const _
    }
}
impl Deref for GPIO_GROUP_INT1 {
    type Target = gpio_group_int0::RegisterBlock;
    fn deref(&self) -> &gpio_group_int0::RegisterBlock {
        unsafe { &*GPIO_GROUP_INT1::ptr() }
    }
}
#[doc = "USB2.0 device controller"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        1074266112 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
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
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio_port::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for GPIO_PORT {
    type Target = gpio_port::RegisterBlock;
    fn deref(&self) -> &gpio_port::RegisterBlock {
        unsafe { &*GPIO_PORT::ptr() }
    }
}
#[doc = "GPIO port"]
pub mod gpio_port;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
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
