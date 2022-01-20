#[doc = "INTVAL register accessor: an alias for `Reg<INTVAL_SPEC>`"]
pub type INTVAL = crate::Reg<intval::INTVAL_SPEC>;
#[doc = "MRT Time interval value register. This value is loaded into the TIMER register."]
pub mod intval;
#[doc = "TIMER register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "MRT Timer register. This register reads the value of the down-counter."]
pub mod timer;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "MRT Control register. This register controls the MRT modes."]
pub mod ctrl;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "MRT Status register."]
pub mod stat;
