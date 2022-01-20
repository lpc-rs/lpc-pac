#[doc = "STATICCONFIG register accessor: an alias for `Reg<STATICCONFIG_SPEC>`"]
pub type STATICCONFIG = crate::Reg<staticconfig::STATICCONFIG_SPEC>;
#[doc = "Configuration for EMC_CSx"]
pub mod staticconfig;
#[doc = "STATICWAITWEN register accessor: an alias for `Reg<STATICWAITWEN_SPEC>`"]
pub type STATICWAITWEN = crate::Reg<staticwaitwen::STATICWAITWEN_SPEC>;
#[doc = "Delay from EMC_CSx to write enable"]
pub mod staticwaitwen;
#[doc = "STATICWAITOEN register accessor: an alias for `Reg<STATICWAITOEN_SPEC>`"]
pub type STATICWAITOEN = crate::Reg<staticwaitoen::STATICWAITOEN_SPEC>;
#[doc = "Delay from EMC_CSx or address change, whichever is later, to output enable"]
pub mod staticwaitoen;
#[doc = "STATICWAITRD register accessor: an alias for `Reg<STATICWAITRD_SPEC>`"]
pub type STATICWAITRD = crate::Reg<staticwaitrd::STATICWAITRD_SPEC>;
#[doc = "Delay from EMC_CSx to a read access"]
pub mod staticwaitrd;
#[doc = "STATICWAITPAGE register accessor: an alias for `Reg<STATICWAITPAGE_SPEC>`"]
pub type STATICWAITPAGE = crate::Reg<staticwaitpage::STATICWAITPAGE_SPEC>;
#[doc = "Delay for asynchronous page mode sequential accesses for EMC_CSx"]
pub mod staticwaitpage;
#[doc = "STATICWAITWR register accessor: an alias for `Reg<STATICWAITWR_SPEC>`"]
pub type STATICWAITWR = crate::Reg<staticwaitwr::STATICWAITWR_SPEC>;
#[doc = "Delay from EMC_CSx to a write access"]
pub mod staticwaitwr;
#[doc = "STATICWAITTURN register accessor: an alias for `Reg<STATICWAITTURN_SPEC>`"]
pub type STATICWAITTURN = crate::Reg<staticwaitturn::STATICWAITTURN_SPEC>;
#[doc = "Number of bus turnaround cycles EMC_CSx"]
pub mod staticwaitturn;
