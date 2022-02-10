///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer.
    pub mod_: crate::Reg<mod_::MOD_SPEC>,
    ///0x04 - Watchdog timer constant register. This 24-bit register determines the time-out value.
    pub tc: crate::Reg<tc::TC_SPEC>,
    ///0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC.
    pub feed: crate::Reg<feed::FEED_SPEC>,
    ///0x0c - Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer.
    pub tv: crate::Reg<tv::TV_SPEC>,
    _reserved4: [u8; 0x04],
    ///0x14 - Watchdog Warning Interrupt compare value.
    pub warnint: crate::Reg<warnint::WARNINT_SPEC>,
    ///0x18 - Watchdog Window compare value.
    pub window: crate::Reg<window::WINDOW_SPEC>,
}
///MOD register accessor: an alias for `Reg<MOD_SPEC>`
pub type MOD = crate::Reg<mod_::MOD_SPEC>;
///Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer.
pub mod mod_;
///TC register accessor: an alias for `Reg<TC_SPEC>`
pub type TC = crate::Reg<tc::TC_SPEC>;
///Watchdog timer constant register. This 24-bit register determines the time-out value.
pub mod tc;
///FEED register accessor: an alias for `Reg<FEED_SPEC>`
pub type FEED = crate::Reg<feed::FEED_SPEC>;
///Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC.
pub mod feed;
///TV register accessor: an alias for `Reg<TV_SPEC>`
pub type TV = crate::Reg<tv::TV_SPEC>;
///Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer.
pub mod tv;
///WARNINT register accessor: an alias for `Reg<WARNINT_SPEC>`
pub type WARNINT = crate::Reg<warnint::WARNINT_SPEC>;
///Watchdog Warning Interrupt compare value.
pub mod warnint;
///WINDOW register accessor: an alias for `Reg<WINDOW_SPEC>`
pub type WINDOW = crate::Reg<window::WINDOW_SPEC>;
///Watchdog Window compare value.
pub mod window;
