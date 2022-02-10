///EV_STATE register accessor: an alias for `Reg<EV_STATE_SPEC>`
pub type EV_STATE = crate::Reg<ev_state::EV_STATE_SPEC>;
///SCT event state register 0
pub mod ev_state;
///EV_CTRL register accessor: an alias for `Reg<EV_CTRL_SPEC>`
pub type EV_CTRL = crate::Reg<ev_ctrl::EV_CTRL_SPEC>;
///SCT event control register 0
pub mod ev_ctrl;
