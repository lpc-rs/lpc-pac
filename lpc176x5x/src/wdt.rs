#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer."]
    pub mod_: MOD,
    #[doc = "0x04 - Watchdog timer constant register. The value in this register determines the time-out value."]
    pub tc: TC,
    #[doc = "0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
    pub feed: FEED,
    #[doc = "0x0c - Watchdog timer value register. This register reads out the current value of the Watchdog timer."]
    pub tv: TV,
    #[doc = "0x10 - Watchdog clock select register."]
    pub clksel: CLKSEL,
}
#[doc = "Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer."]
pub struct MOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer."]
pub mod mod_;
#[doc = "Watchdog timer constant register. The value in this register determines the time-out value."]
pub struct TC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog timer constant register. The value in this register determines the time-out value."]
pub mod tc;
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
pub struct FEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC."]
pub mod feed;
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer."]
pub struct TV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog timer value register. This register reads out the current value of the Watchdog timer."]
pub mod tv;
#[doc = "Watchdog clock select register."]
pub struct CLKSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog clock select register."]
pub mod clksel;
