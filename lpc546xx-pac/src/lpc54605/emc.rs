///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Controls operation of the memory controller
    pub control: crate::Reg<control::CONTROL_SPEC>,
    ///0x04 - Provides EMC status information
    pub status: crate::Reg<status::STATUS_SPEC>,
    ///0x08 - Configures operation of the memory controller
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved3: [u8; 0x14],
    ///0x20 - Controls dynamic memory operation
    pub dynamiccontrol: crate::Reg<dynamiccontrol::DYNAMICCONTROL_SPEC>,
    ///0x24 - Configures dynamic memory refresh
    pub dynamicrefresh: crate::Reg<dynamicrefresh::DYNAMICREFRESH_SPEC>,
    ///0x28 - Configures dynamic memory read strategy
    pub dynamicreadconfig: crate::Reg<dynamicreadconfig::DYNAMICREADCONFIG_SPEC>,
    _reserved6: [u8; 0x04],
    ///0x30 - Precharge command period
    pub dynamicrp: crate::Reg<dynamicrp::DYNAMICRP_SPEC>,
    ///0x34 - Active to precharge command period
    pub dynamicras: crate::Reg<dynamicras::DYNAMICRAS_SPEC>,
    ///0x38 - Self-refresh exit time
    pub dynamicsrex: crate::Reg<dynamicsrex::DYNAMICSREX_SPEC>,
    ///0x3c - Last-data-out to active command time
    pub dynamicapr: crate::Reg<dynamicapr::DYNAMICAPR_SPEC>,
    ///0x40 - Data-in to active command time
    pub dynamicdal: crate::Reg<dynamicdal::DYNAMICDAL_SPEC>,
    ///0x44 - Write recovery time
    pub dynamicwr: crate::Reg<dynamicwr::DYNAMICWR_SPEC>,
    ///0x48 - Selects the active to active command period
    pub dynamicrc: crate::Reg<dynamicrc::DYNAMICRC_SPEC>,
    ///0x4c - Selects the auto-refresh period
    pub dynamicrfc: crate::Reg<dynamicrfc::DYNAMICRFC_SPEC>,
    ///0x50 - Time for exit self-refresh to active command
    pub dynamicxsr: crate::Reg<dynamicxsr::DYNAMICXSR_SPEC>,
    ///0x54 - Latency for active bank A to active bank B
    pub dynamicrrd: crate::Reg<dynamicrrd::DYNAMICRRD_SPEC>,
    ///0x58 - Time for load mode register to active command
    pub dynamicmrd: crate::Reg<dynamicmrd::DYNAMICMRD_SPEC>,
    _reserved17: [u8; 0x24],
    ///0x80 - Time for long static memory read and write transfers
    pub staticextendedwait: crate::Reg<staticextendedwait::STATICEXTENDEDWAIT_SPEC>,
    _reserved18: [u8; 0x7c],
    ///0x100..0x108 - no description available
    pub dynamic0: DYNAMIC,
    _reserved19: [u8; 0x18],
    ///0x120..0x128 - no description available
    pub dynamic1: DYNAMIC,
    _reserved20: [u8; 0x18],
    ///0x140..0x148 - no description available
    pub dynamic2: DYNAMIC,
    _reserved21: [u8; 0x18],
    ///0x160..0x168 - no description available
    pub dynamic3: DYNAMIC,
    _reserved22: [u8; 0x98],
    ///0x200..0x21c - no description available
    pub static0: STATIC,
    _reserved23: [u8; 0x04],
    ///0x220..0x23c - no description available
    pub static1: STATIC,
    _reserved24: [u8; 0x04],
    ///0x240..0x25c - no description available
    pub static2: STATIC,
    _reserved25: [u8; 0x04],
    ///0x260..0x27c - no description available
    pub static3: STATIC,
}
///Register block
#[repr(C)]
pub struct DYNAMIC {
    ///0x00 - Configuration information for EMC_DYCSx
    pub dynamicconfig: crate::Reg<self::dynamic::dynamicconfig::DYNAMICCONFIG_SPEC>,
    ///0x04 - RAS and CAS latencies for EMC_DYCSx
    pub dynamicrascas: crate::Reg<self::dynamic::dynamicrascas::DYNAMICRASCAS_SPEC>,
}
///Register block
///no description available
pub mod dynamic;
///Register block
#[repr(C)]
pub struct STATIC {
    ///0x00 - Configuration for EMC_CSx
    pub staticconfig: crate::Reg<self::static_::staticconfig::STATICCONFIG_SPEC>,
    ///0x04 - Delay from EMC_CSx to write enable
    pub staticwaitwen: crate::Reg<self::static_::staticwaitwen::STATICWAITWEN_SPEC>,
    ///0x08 - Delay from EMC_CSx or address change, whichever is later, to output enable
    pub staticwaitoen: crate::Reg<self::static_::staticwaitoen::STATICWAITOEN_SPEC>,
    ///0x0c - Delay from EMC_CSx to a read access
    pub staticwaitrd: crate::Reg<self::static_::staticwaitrd::STATICWAITRD_SPEC>,
    ///0x10 - Delay for asynchronous page mode sequential accesses for EMC_CSx
    pub staticwaitpage: crate::Reg<self::static_::staticwaitpage::STATICWAITPAGE_SPEC>,
    ///0x14 - Delay from EMC_CSx to a write access
    pub staticwaitwr: crate::Reg<self::static_::staticwaitwr::STATICWAITWR_SPEC>,
    ///0x18 - Number of bus turnaround cycles EMC_CSx
    pub staticwaitturn: crate::Reg<self::static_::staticwaitturn::STATICWAITTURN_SPEC>,
}
///Register block
///no description available
pub mod static_;
///CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
///Controls operation of the memory controller
pub mod control;
///STATUS register accessor: an alias for `Reg<STATUS_SPEC>`
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///Provides EMC status information
pub mod status;
///CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
///Configures operation of the memory controller
pub mod config;
///DYNAMICCONTROL register accessor: an alias for `Reg<DYNAMICCONTROL_SPEC>`
pub type DYNAMICCONTROL = crate::Reg<dynamiccontrol::DYNAMICCONTROL_SPEC>;
///Controls dynamic memory operation
pub mod dynamiccontrol;
///DYNAMICREFRESH register accessor: an alias for `Reg<DYNAMICREFRESH_SPEC>`
pub type DYNAMICREFRESH = crate::Reg<dynamicrefresh::DYNAMICREFRESH_SPEC>;
///Configures dynamic memory refresh
pub mod dynamicrefresh;
///DYNAMICREADCONFIG register accessor: an alias for `Reg<DYNAMICREADCONFIG_SPEC>`
pub type DYNAMICREADCONFIG = crate::Reg<dynamicreadconfig::DYNAMICREADCONFIG_SPEC>;
///Configures dynamic memory read strategy
pub mod dynamicreadconfig;
///DYNAMICRP register accessor: an alias for `Reg<DYNAMICRP_SPEC>`
pub type DYNAMICRP = crate::Reg<dynamicrp::DYNAMICRP_SPEC>;
///Precharge command period
pub mod dynamicrp;
///DYNAMICRAS register accessor: an alias for `Reg<DYNAMICRAS_SPEC>`
pub type DYNAMICRAS = crate::Reg<dynamicras::DYNAMICRAS_SPEC>;
///Active to precharge command period
pub mod dynamicras;
///DYNAMICSREX register accessor: an alias for `Reg<DYNAMICSREX_SPEC>`
pub type DYNAMICSREX = crate::Reg<dynamicsrex::DYNAMICSREX_SPEC>;
///Self-refresh exit time
pub mod dynamicsrex;
///DYNAMICAPR register accessor: an alias for `Reg<DYNAMICAPR_SPEC>`
pub type DYNAMICAPR = crate::Reg<dynamicapr::DYNAMICAPR_SPEC>;
///Last-data-out to active command time
pub mod dynamicapr;
///DYNAMICDAL register accessor: an alias for `Reg<DYNAMICDAL_SPEC>`
pub type DYNAMICDAL = crate::Reg<dynamicdal::DYNAMICDAL_SPEC>;
///Data-in to active command time
pub mod dynamicdal;
///DYNAMICWR register accessor: an alias for `Reg<DYNAMICWR_SPEC>`
pub type DYNAMICWR = crate::Reg<dynamicwr::DYNAMICWR_SPEC>;
///Write recovery time
pub mod dynamicwr;
///DYNAMICRC register accessor: an alias for `Reg<DYNAMICRC_SPEC>`
pub type DYNAMICRC = crate::Reg<dynamicrc::DYNAMICRC_SPEC>;
///Selects the active to active command period
pub mod dynamicrc;
///DYNAMICRFC register accessor: an alias for `Reg<DYNAMICRFC_SPEC>`
pub type DYNAMICRFC = crate::Reg<dynamicrfc::DYNAMICRFC_SPEC>;
///Selects the auto-refresh period
pub mod dynamicrfc;
///DYNAMICXSR register accessor: an alias for `Reg<DYNAMICXSR_SPEC>`
pub type DYNAMICXSR = crate::Reg<dynamicxsr::DYNAMICXSR_SPEC>;
///Time for exit self-refresh to active command
pub mod dynamicxsr;
///DYNAMICRRD register accessor: an alias for `Reg<DYNAMICRRD_SPEC>`
pub type DYNAMICRRD = crate::Reg<dynamicrrd::DYNAMICRRD_SPEC>;
///Latency for active bank A to active bank B
pub mod dynamicrrd;
///DYNAMICMRD register accessor: an alias for `Reg<DYNAMICMRD_SPEC>`
pub type DYNAMICMRD = crate::Reg<dynamicmrd::DYNAMICMRD_SPEC>;
///Time for load mode register to active command
pub mod dynamicmrd;
///STATICEXTENDEDWAIT register accessor: an alias for `Reg<STATICEXTENDEDWAIT_SPEC>`
pub type STATICEXTENDEDWAIT = crate::Reg<staticextendedwait::STATICEXTENDEDWAIT_SPEC>;
///Time for long static memory read and write transfers
pub mod staticextendedwait;
