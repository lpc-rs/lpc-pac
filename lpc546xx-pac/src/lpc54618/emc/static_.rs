///STATICCONFIG register accessor: an alias for `Reg<STATICCONFIG_SPEC>`
pub type STATICCONFIG = crate::Reg<staticconfig::STATICCONFIG_SPEC>;
///Configuration for EMC_CSx
pub mod staticconfig;
///STATICWAITWEN register accessor: an alias for `Reg<STATICWAITWEN_SPEC>`
pub type STATICWAITWEN = crate::Reg<staticwaitwen::STATICWAITWEN_SPEC>;
///Delay from EMC_CSx to write enable
pub mod staticwaitwen;
///STATICWAITOEN register accessor: an alias for `Reg<STATICWAITOEN_SPEC>`
pub type STATICWAITOEN = crate::Reg<staticwaitoen::STATICWAITOEN_SPEC>;
///Delay from EMC_CSx or address change, whichever is later, to output enable
pub mod staticwaitoen;
///STATICWAITRD register accessor: an alias for `Reg<STATICWAITRD_SPEC>`
pub type STATICWAITRD = crate::Reg<staticwaitrd::STATICWAITRD_SPEC>;
///Delay from EMC_CSx to a read access
pub mod staticwaitrd;
///STATICWAITPAGE register accessor: an alias for `Reg<STATICWAITPAGE_SPEC>`
pub type STATICWAITPAGE = crate::Reg<staticwaitpage::STATICWAITPAGE_SPEC>;
///Delay for asynchronous page mode sequential accesses for EMC_CSx
pub mod staticwaitpage;
///STATICWAITWR register accessor: an alias for `Reg<STATICWAITWR_SPEC>`
pub type STATICWAITWR = crate::Reg<staticwaitwr::STATICWAITWR_SPEC>;
///Delay from EMC_CSx to a write access
pub mod staticwaitwr;
///STATICWAITTURN register accessor: an alias for `Reg<STATICWAITTURN_SPEC>`
pub type STATICWAITTURN = crate::Reg<staticwaitturn::STATICWAITTURN_SPEC>;
///Number of bus turnaround cycles EMC_CSx
pub mod staticwaitturn;
