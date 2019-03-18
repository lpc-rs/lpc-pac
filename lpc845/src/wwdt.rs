#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
    pub mod_: MOD,
    #[doc = "0x04 - Watchdog timer constant register. This 24-bit register determines the time-out value."]
    pub tc: TC,
    #[doc = "0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
    pub feed: FEED,
    #[doc = "0x0c - Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
    pub tv: TV,
    _reserved0: [u8; 4usize],
    #[doc = "0x14 - Watchdog Warning Interrupt compare value."]
    pub warnint: WARNINT,
    #[doc = "0x18 - Watchdog Window compare value."]
    pub window: WINDOW,
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
pub struct MOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
pub mod mod_;
#[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value."]
pub struct TC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value."]
pub mod tc;
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
pub struct FEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
pub mod feed;
#[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
pub struct TV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
pub mod tv;
#[doc = "Watchdog Warning Interrupt compare value."]
pub struct WARNINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Warning Interrupt compare value."]
pub mod warnint;
#[doc = "Watchdog Window compare value."]
pub struct WINDOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Watchdog Window compare value."]
pub mod window;
