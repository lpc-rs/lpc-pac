#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Compare value LSB register"]
    pub compval: crate::Reg<compval::COMPVAL_SPEC>,
    #[doc = "0x04 - Mask LSB register"]
    pub mask: crate::Reg<mask::MASK_SPEC>,
    #[doc = "0x08 - Control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - Counter LSB register"]
    pub counter: crate::Reg<counter::COUNTER_SPEC>,
    #[doc = "0x10 - Compare value MSB register"]
    pub compval_h: crate::Reg<compval_h::COMPVAL_H_SPEC>,
    #[doc = "0x14 - Mask MSB register"]
    pub mask_h: crate::Reg<mask_h::MASK_H_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Counter MSB register"]
    pub counter_h: crate::Reg<counter_h::COUNTER_H_SPEC>,
}
#[doc = "COMPVAL register accessor: an alias for `Reg<COMPVAL_SPEC>`"]
pub type COMPVAL = crate::Reg<compval::COMPVAL_SPEC>;
#[doc = "Compare value LSB register"]
pub mod compval;
#[doc = "MASK register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask LSB register"]
pub mod mask;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "COUNTER register accessor: an alias for `Reg<COUNTER_SPEC>`"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Counter LSB register"]
pub mod counter;
#[doc = "COMPVAL_H register accessor: an alias for `Reg<COMPVAL_H_SPEC>`"]
pub type COMPVAL_H = crate::Reg<compval_h::COMPVAL_H_SPEC>;
#[doc = "Compare value MSB register"]
pub mod compval_h;
#[doc = "MASK_H register accessor: an alias for `Reg<MASK_H_SPEC>`"]
pub type MASK_H = crate::Reg<mask_h::MASK_H_SPEC>;
#[doc = "Mask MSB register"]
pub mod mask_h;
#[doc = "COUNTER_H register accessor: an alias for `Reg<COUNTER_H_SPEC>`"]
pub type COUNTER_H = crate::Reg<counter_h::COUNTER_H_SPEC>;
#[doc = "Counter MSB register"]
pub mod counter_h;
