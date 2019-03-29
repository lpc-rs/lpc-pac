#![doc = "Peripheral access API for LPC176X5X microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
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
pub const NVIC_PRIO_BITS: u8 = 5;
#[cfg(feature = "rt")]
extern "C" {
    fn WDT();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn TIMER3();
    fn UART0();
    fn UART1();
    fn UART2();
    fn UART3();
    fn PWM1();
    fn I2C0();
    fn I2C1();
    fn I2C2();
    fn SPI();
    fn SSP0();
    fn SSP1();
    fn PLL0();
    fn RTC();
    fn EINT0();
    fn EINT1();
    fn EINT2();
    fn EINT3();
    fn ADC();
    fn BOD();
    fn USB();
    fn CAN();
    fn DMA();
    fn I2S();
    fn ENET();
    fn RIT();
    fn MCPWM();
    fn QEI();
    fn PLL1();
    fn USBACTIVITY();
    fn CANACTIVITY();
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
pub static __INTERRUPTS: [Vector; 35] = [
    Vector { _handler: WDT },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: TIMER3 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: UART2 },
    Vector { _handler: UART3 },
    Vector { _handler: PWM1 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _handler: SPI },
    Vector { _handler: SSP0 },
    Vector { _handler: SSP1 },
    Vector { _handler: PLL0 },
    Vector { _handler: RTC },
    Vector { _handler: EINT0 },
    Vector { _handler: EINT1 },
    Vector { _handler: EINT2 },
    Vector { _handler: EINT3 },
    Vector { _handler: ADC },
    Vector { _handler: BOD },
    Vector { _handler: USB },
    Vector { _handler: CAN },
    Vector { _handler: DMA },
    Vector { _handler: I2S },
    Vector { _handler: ENET },
    Vector { _handler: RIT },
    Vector { _handler: MCPWM },
    Vector { _handler: QEI },
    Vector { _handler: PLL1 },
    Vector {
        _handler: USBACTIVITY,
    },
    Vector {
        _handler: CANACTIVITY,
    },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - WDT"]
    WDT,
    #[doc = "1 - TIMER0"]
    TIMER0,
    #[doc = "2 - TIMER1"]
    TIMER1,
    #[doc = "3 - TIMER2"]
    TIMER2,
    #[doc = "4 - TIMER3"]
    TIMER3,
    #[doc = "5 - UART0"]
    UART0,
    #[doc = "6 - UART1"]
    UART1,
    #[doc = "7 - UART2"]
    UART2,
    #[doc = "8 - UART3"]
    UART3,
    #[doc = "9 - PWM1"]
    PWM1,
    #[doc = "10 - I2C0"]
    I2C0,
    #[doc = "11 - I2C1"]
    I2C1,
    #[doc = "12 - I2C2"]
    I2C2,
    #[doc = "13 - SPI"]
    SPI,
    #[doc = "14 - SSP0"]
    SSP0,
    #[doc = "15 - SSP1"]
    SSP1,
    #[doc = "16 - PLL0"]
    PLL0,
    #[doc = "17 - RTC"]
    RTC,
    #[doc = "18 - EINT0"]
    EINT0,
    #[doc = "19 - EINT1"]
    EINT1,
    #[doc = "20 - EINT2"]
    EINT2,
    #[doc = "21 - EINT3"]
    EINT3,
    #[doc = "22 - ADC"]
    ADC,
    #[doc = "23 - BOD"]
    BOD,
    #[doc = "24 - USB"]
    USB,
    #[doc = "25 - CAN"]
    CAN,
    #[doc = "26 - DMA"]
    DMA,
    #[doc = "27 - I2S"]
    I2S,
    #[doc = "28 - ENET"]
    ENET,
    #[doc = "29 - RIT"]
    RIT,
    #[doc = "30 - MCPWM"]
    MCPWM,
    #[doc = "31 - QEI"]
    QEI,
    #[doc = "32 - PLL1"]
    PLL1,
    #[doc = "33 - USBActivity"]
    USBACTIVITY,
    #[doc = "34 - CANActivity"]
    CANACTIVITY,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WDT => 0,
            Interrupt::TIMER0 => 1,
            Interrupt::TIMER1 => 2,
            Interrupt::TIMER2 => 3,
            Interrupt::TIMER3 => 4,
            Interrupt::UART0 => 5,
            Interrupt::UART1 => 6,
            Interrupt::UART2 => 7,
            Interrupt::UART3 => 8,
            Interrupt::PWM1 => 9,
            Interrupt::I2C0 => 10,
            Interrupt::I2C1 => 11,
            Interrupt::I2C2 => 12,
            Interrupt::SPI => 13,
            Interrupt::SSP0 => 14,
            Interrupt::SSP1 => 15,
            Interrupt::PLL0 => 16,
            Interrupt::RTC => 17,
            Interrupt::EINT0 => 18,
            Interrupt::EINT1 => 19,
            Interrupt::EINT2 => 20,
            Interrupt::EINT3 => 21,
            Interrupt::ADC => 22,
            Interrupt::BOD => 23,
            Interrupt::USB => 24,
            Interrupt::CAN => 25,
            Interrupt::DMA => 26,
            Interrupt::I2S => 27,
            Interrupt::ENET => 28,
            Interrupt::RIT => 29,
            Interrupt::MCPWM => 30,
            Interrupt::QEI => 31,
            Interrupt::PLL1 => 32,
            Interrupt::USBACTIVITY => 33,
            Interrupt::CANACTIVITY => 34,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Watchdog Timer (WDT)"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer (WDT)"]
pub mod wdt;
#[doc = "Timer0/1/2/3"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer0/1/2/3"]
pub mod timer0;
#[doc = "TIMER1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "UART0/2/3"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART0/2/3"]
pub mod uart0;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart1::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &uart1::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART1"]
pub mod uart1;
#[doc = "Pulse Width Modulators (PWM1)"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm1::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm1::RegisterBlock;
    fn deref(&self) -> &pwm1::RegisterBlock {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "Pulse Width Modulators (PWM1)"]
pub mod pwm1;
#[doc = "I2C bus interface"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C bus interface"]
pub mod i2c0;
#[doc = "SPI"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    fn deref(&self) -> &spi::RegisterBlock {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "SPI"]
pub mod spi;
#[doc = "Real Time Clock (RTC)"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock (RTC)"]
pub mod rtc;
#[doc = "GPIO"]
pub struct GPIOINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOINT {}
impl GPIOINT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioint::RegisterBlock {
        1073905792 as *const _
    }
}
impl Deref for GPIOINT {
    type Target = gpioint::RegisterBlock;
    fn deref(&self) -> &gpioint::RegisterBlock {
        unsafe { &*GPIOINT::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpioint;
#[doc = "Pin connect block"]
pub struct PINCONNECT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PINCONNECT {}
impl PINCONNECT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pinconnect::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for PINCONNECT {
    type Target = pinconnect::RegisterBlock;
    fn deref(&self) -> &pinconnect::RegisterBlock {
        unsafe { &*PINCONNECT::ptr() }
    }
}
#[doc = "Pin connect block"]
pub mod pinconnect;
#[doc = "SSP1 controller"]
pub struct SSP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP1 {}
impl SSP1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssp1::RegisterBlock {
        1073938432 as *const _
    }
}
impl Deref for SSP1 {
    type Target = ssp1::RegisterBlock;
    fn deref(&self) -> &ssp1::RegisterBlock {
        unsafe { &*SSP1::ptr() }
    }
}
#[doc = "SSP1 controller"]
pub mod ssp1;
#[doc = "Analog-to-Digital Converter (ADC)"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1073954816 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter (ADC)"]
pub mod adc;
#[doc = "CAN acceptance filter RAM"]
pub struct CANAFRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CANAFRAM {}
impl CANAFRAM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const canafram::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for CANAFRAM {
    type Target = canafram::RegisterBlock;
    fn deref(&self) -> &canafram::RegisterBlock {
        unsafe { &*CANAFRAM::ptr() }
    }
}
#[doc = "CAN acceptance filter RAM"]
pub mod canafram;
#[doc = "CAN controller acceptance filter"]
pub struct CANAF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CANAF {}
impl CANAF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const canaf::RegisterBlock {
        1073987584 as *const _
    }
}
impl Deref for CANAF {
    type Target = canaf::RegisterBlock;
    fn deref(&self) -> &canaf::RegisterBlock {
        unsafe { &*CANAF::ptr() }
    }
}
#[doc = "CAN controller acceptance filter"]
pub mod canaf;
#[doc = "Central CAN controller"]
pub struct CCAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCAN {}
impl CCAN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccan::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for CCAN {
    type Target = ccan::RegisterBlock;
    fn deref(&self) -> &ccan::RegisterBlock {
        unsafe { &*CCAN::ptr() }
    }
}
#[doc = "Central CAN controller"]
pub mod ccan;
#[doc = "CAN1 controller"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1074020352 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "CAN1 controller"]
pub mod can1;
#[doc = "CAN2"]
pub struct CAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN2 {}
impl CAN2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1074036736 as *const _
    }
}
impl Deref for CAN2 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN2::ptr() }
    }
}
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1074118656 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "SSP controller"]
pub struct SSP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP0 {}
impl SSP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssp1::RegisterBlock {
        1074298880 as *const _
    }
}
impl Deref for SSP0 {
    type Target = ssp1::RegisterBlock;
    fn deref(&self) -> &ssp1::RegisterBlock {
        unsafe { &*SSP0::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter (DAC)"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac::RegisterBlock {
        1074315264 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter (DAC)"]
pub mod dac;
#[doc = "TIMER2"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1074331648 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "TIMER3"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1074348032 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "UART2"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1074364416 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "UART3"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1074380800 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART3::ptr() }
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
        1074397184 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2S interface"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s::RegisterBlock {
        1074429952 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    fn deref(&self) -> &i2s::RegisterBlock {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "I2S interface"]
pub mod i2s;
#[doc = "Repetitive Interrupt Timer (RIT)"]
pub struct RITIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RITIMER {}
impl RITIMER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ritimer::RegisterBlock {
        1074462720 as *const _
    }
}
impl Deref for RITIMER {
    type Target = ritimer::RegisterBlock;
    fn deref(&self) -> &ritimer::RegisterBlock {
        unsafe { &*RITIMER::ptr() }
    }
}
#[doc = "Repetitive Interrupt Timer (RIT)"]
pub mod ritimer;
#[doc = "Motor Control PWM"]
pub struct MCPWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCPWM {}
impl MCPWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcpwm::RegisterBlock {
        1074495488 as *const _
    }
}
impl Deref for MCPWM {
    type Target = mcpwm::RegisterBlock;
    fn deref(&self) -> &mcpwm::RegisterBlock {
        unsafe { &*MCPWM::ptr() }
    }
}
#[doc = "Motor Control PWM"]
pub mod mcpwm;
#[doc = "Quadrature Encoder Interface (QEI)"]
pub struct QEI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QEI {}
impl QEI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const qei::RegisterBlock {
        1074511872 as *const _
    }
}
impl Deref for QEI {
    type Target = qei::RegisterBlock;
    fn deref(&self) -> &qei::RegisterBlock {
        unsafe { &*QEI::ptr() }
    }
}
#[doc = "Quadrature Encoder Interface (QEI)"]
pub mod qei;
#[doc = "System and clock control"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscon::RegisterBlock {
        1074774016 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    fn deref(&self) -> &syscon::RegisterBlock {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "System and clock control"]
pub mod syscon;
#[doc = "Ethernet"]
pub struct EMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMAC {}
impl EMAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emac::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for EMAC {
    type Target = emac::RegisterBlock;
    fn deref(&self) -> &emac::RegisterBlock {
        unsafe { &*EMAC::ptr() }
    }
}
#[doc = "Ethernet"]
pub mod emac;
#[doc = "General purpose DMA controller"]
pub struct GPDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA {}
impl GPDMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpdma::RegisterBlock {
        1342193664 as *const _
    }
}
impl Deref for GPDMA {
    type Target = gpdma::RegisterBlock;
    fn deref(&self) -> &gpdma::RegisterBlock {
        unsafe { &*GPDMA::ptr() }
    }
}
#[doc = "General purpose DMA controller"]
pub mod gpdma;
#[doc = "USB device/host/OTG controller"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        1342226432 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB device/host/OTG controller"]
pub mod usb;
#[doc = "General Purpose I/O"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        537509888 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub mod gpio;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "GPIOINT"]
    pub GPIOINT: GPIOINT,
    #[doc = "PINCONNECT"]
    pub PINCONNECT: PINCONNECT,
    #[doc = "SSP1"]
    pub SSP1: SSP1,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "CANAFRAM"]
    pub CANAFRAM: CANAFRAM,
    #[doc = "CANAF"]
    pub CANAF: CANAF,
    #[doc = "CCAN"]
    pub CCAN: CCAN,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CAN2"]
    pub CAN2: CAN2,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "SSP0"]
    pub SSP0: SSP0,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "RITIMER"]
    pub RITIMER: RITIMER,
    #[doc = "MCPWM"]
    pub MCPWM: MCPWM,
    #[doc = "QEI"]
    pub QEI: QEI,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "EMAC"]
    pub EMAC: EMAC,
    #[doc = "GPDMA"]
    pub GPDMA: GPDMA,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
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
            WDT: WDT {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            GPIOINT: GPIOINT {
                _marker: PhantomData,
            },
            PINCONNECT: PINCONNECT {
                _marker: PhantomData,
            },
            SSP1: SSP1 {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            CANAFRAM: CANAFRAM {
                _marker: PhantomData,
            },
            CANAF: CANAF {
                _marker: PhantomData,
            },
            CCAN: CCAN {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            SSP0: SSP0 {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            RITIMER: RITIMER {
                _marker: PhantomData,
            },
            MCPWM: MCPWM {
                _marker: PhantomData,
            },
            QEI: QEI {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            EMAC: EMAC {
                _marker: PhantomData,
            },
            GPDMA: GPDMA {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
        }
    }
}
