///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - This register contains the offset value towards the start of the operational register space and the version number of the IP block
    pub caplength_chipid: crate::Reg<caplength_chipid::CAPLENGTH_CHIPID_SPEC>,
    ///0x04 - Host Controller Structural Parameters
    pub hcsparams: crate::Reg<hcsparams::HCSPARAMS_SPEC>,
    ///0x08 - Host Controller Capability Parameters
    pub hccparams: crate::Reg<hccparams::HCCPARAMS_SPEC>,
    ///0x0c - Frame Length Adjustment
    pub fladj_frindex: crate::Reg<fladj_frindex::FLADJ_FRINDEX_SPEC>,
    ///0x10 - Memory base address where ATL PTD0 is stored
    pub atl_ptd_base_addr: crate::Reg<atl_ptd_base_addr::ATL_PTD_BASE_ADDR_SPEC>,
    ///0x14 - Memory base address where ISO PTD0 is stored
    pub iso_ptd_base_addr: crate::Reg<iso_ptd_base_addr::ISO_PTD_BASE_ADDR_SPEC>,
    ///0x18 - Memory base address where INT PTD0 is stored
    pub int_ptd_base_addr: crate::Reg<int_ptd_base_addr::INT_PTD_BASE_ADDR_SPEC>,
    ///0x1c - Memory base address that indicates the start of the data payload buffers
    pub data_payload_base_addr: crate::Reg<data_payload_base_addr::DATA_PAYLOAD_BASE_ADDR_SPEC>,
    ///0x20 - USB Command register
    pub usbcmd: crate::Reg<usbcmd::USBCMD_SPEC>,
    ///0x24 - USB Interrupt Status register
    pub usbsts: crate::Reg<usbsts::USBSTS_SPEC>,
    ///0x28 - USB Interrupt Enable register
    pub usbintr: crate::Reg<usbintr::USBINTR_SPEC>,
    ///0x2c - Port Status and Control register
    pub portsc1: crate::Reg<portsc1::PORTSC1_SPEC>,
    ///0x30 - Done map for each ATL PTD
    pub atl_ptd_done_map: crate::Reg<atl_ptd_done_map::ATL_PTD_DONE_MAP_SPEC>,
    ///0x34 - Skip map for each ATL PTD
    pub atl_ptd_skip_map: crate::Reg<atl_ptd_skip_map::ATL_PTD_SKIP_MAP_SPEC>,
    ///0x38 - Done map for each ISO PTD
    pub iso_ptd_done_map: crate::Reg<iso_ptd_done_map::ISO_PTD_DONE_MAP_SPEC>,
    ///0x3c - Skip map for each ISO PTD
    pub iso_ptd_skip_map: crate::Reg<iso_ptd_skip_map::ISO_PTD_SKIP_MAP_SPEC>,
    ///0x40 - Done map for each INT PTD
    pub int_ptd_done_map: crate::Reg<int_ptd_done_map::INT_PTD_DONE_MAP_SPEC>,
    ///0x44 - Skip map for each INT PTD
    pub int_ptd_skip_map: crate::Reg<int_ptd_skip_map::INT_PTD_SKIP_MAP_SPEC>,
    ///0x48 - Marks the last PTD in the list for ISO, INT and ATL
    pub last_ptd_inuse: crate::Reg<last_ptd_inuse::LAST_PTD_INUSE_SPEC>,
    ///0x4c - Register to read/write registers in the attached USB PHY
    pub utmiplus_ulpi_debug: crate::Reg<utmiplus_ulpi_debug::UTMIPLUS_ULPI_DEBUG_SPEC>,
    ///0x50 - Controls the port if it is attached to the host block or the device block
    pub portmode: crate::Reg<portmode::PORTMODE_SPEC>,
}
///CAPLENGTH_CHIPID register accessor: an alias for `Reg<CAPLENGTH_CHIPID_SPEC>`
pub type CAPLENGTH_CHIPID = crate::Reg<caplength_chipid::CAPLENGTH_CHIPID_SPEC>;
///This register contains the offset value towards the start of the operational register space and the version number of the IP block
pub mod caplength_chipid;
///HCSPARAMS register accessor: an alias for `Reg<HCSPARAMS_SPEC>`
pub type HCSPARAMS = crate::Reg<hcsparams::HCSPARAMS_SPEC>;
///Host Controller Structural Parameters
pub mod hcsparams;
///HCCPARAMS register accessor: an alias for `Reg<HCCPARAMS_SPEC>`
pub type HCCPARAMS = crate::Reg<hccparams::HCCPARAMS_SPEC>;
///Host Controller Capability Parameters
pub mod hccparams;
///FLADJ_FRINDEX register accessor: an alias for `Reg<FLADJ_FRINDEX_SPEC>`
pub type FLADJ_FRINDEX = crate::Reg<fladj_frindex::FLADJ_FRINDEX_SPEC>;
///Frame Length Adjustment
pub mod fladj_frindex;
///ATL_PTD_BASE_ADDR register accessor: an alias for `Reg<ATL_PTD_BASE_ADDR_SPEC>`
pub type ATL_PTD_BASE_ADDR = crate::Reg<atl_ptd_base_addr::ATL_PTD_BASE_ADDR_SPEC>;
///Memory base address where ATL PTD0 is stored
pub mod atl_ptd_base_addr;
///ISO_PTD_BASE_ADDR register accessor: an alias for `Reg<ISO_PTD_BASE_ADDR_SPEC>`
pub type ISO_PTD_BASE_ADDR = crate::Reg<iso_ptd_base_addr::ISO_PTD_BASE_ADDR_SPEC>;
///Memory base address where ISO PTD0 is stored
pub mod iso_ptd_base_addr;
///INT_PTD_BASE_ADDR register accessor: an alias for `Reg<INT_PTD_BASE_ADDR_SPEC>`
pub type INT_PTD_BASE_ADDR = crate::Reg<int_ptd_base_addr::INT_PTD_BASE_ADDR_SPEC>;
///Memory base address where INT PTD0 is stored
pub mod int_ptd_base_addr;
///DATA_PAYLOAD_BASE_ADDR register accessor: an alias for `Reg<DATA_PAYLOAD_BASE_ADDR_SPEC>`
pub type DATA_PAYLOAD_BASE_ADDR = crate::Reg<data_payload_base_addr::DATA_PAYLOAD_BASE_ADDR_SPEC>;
///Memory base address that indicates the start of the data payload buffers
pub mod data_payload_base_addr;
///USBCMD register accessor: an alias for `Reg<USBCMD_SPEC>`
pub type USBCMD = crate::Reg<usbcmd::USBCMD_SPEC>;
///USB Command register
pub mod usbcmd;
///USBSTS register accessor: an alias for `Reg<USBSTS_SPEC>`
pub type USBSTS = crate::Reg<usbsts::USBSTS_SPEC>;
///USB Interrupt Status register
pub mod usbsts;
///USBINTR register accessor: an alias for `Reg<USBINTR_SPEC>`
pub type USBINTR = crate::Reg<usbintr::USBINTR_SPEC>;
///USB Interrupt Enable register
pub mod usbintr;
///PORTSC1 register accessor: an alias for `Reg<PORTSC1_SPEC>`
pub type PORTSC1 = crate::Reg<portsc1::PORTSC1_SPEC>;
///Port Status and Control register
pub mod portsc1;
///ATL_PTD_DONE_MAP register accessor: an alias for `Reg<ATL_PTD_DONE_MAP_SPEC>`
pub type ATL_PTD_DONE_MAP = crate::Reg<atl_ptd_done_map::ATL_PTD_DONE_MAP_SPEC>;
///Done map for each ATL PTD
pub mod atl_ptd_done_map;
///ATL_PTD_SKIP_MAP register accessor: an alias for `Reg<ATL_PTD_SKIP_MAP_SPEC>`
pub type ATL_PTD_SKIP_MAP = crate::Reg<atl_ptd_skip_map::ATL_PTD_SKIP_MAP_SPEC>;
///Skip map for each ATL PTD
pub mod atl_ptd_skip_map;
///ISO_PTD_DONE_MAP register accessor: an alias for `Reg<ISO_PTD_DONE_MAP_SPEC>`
pub type ISO_PTD_DONE_MAP = crate::Reg<iso_ptd_done_map::ISO_PTD_DONE_MAP_SPEC>;
///Done map for each ISO PTD
pub mod iso_ptd_done_map;
///ISO_PTD_SKIP_MAP register accessor: an alias for `Reg<ISO_PTD_SKIP_MAP_SPEC>`
pub type ISO_PTD_SKIP_MAP = crate::Reg<iso_ptd_skip_map::ISO_PTD_SKIP_MAP_SPEC>;
///Skip map for each ISO PTD
pub mod iso_ptd_skip_map;
///INT_PTD_DONE_MAP register accessor: an alias for `Reg<INT_PTD_DONE_MAP_SPEC>`
pub type INT_PTD_DONE_MAP = crate::Reg<int_ptd_done_map::INT_PTD_DONE_MAP_SPEC>;
///Done map for each INT PTD
pub mod int_ptd_done_map;
///INT_PTD_SKIP_MAP register accessor: an alias for `Reg<INT_PTD_SKIP_MAP_SPEC>`
pub type INT_PTD_SKIP_MAP = crate::Reg<int_ptd_skip_map::INT_PTD_SKIP_MAP_SPEC>;
///Skip map for each INT PTD
pub mod int_ptd_skip_map;
///LAST_PTD_INUSE register accessor: an alias for `Reg<LAST_PTD_INUSE_SPEC>`
pub type LAST_PTD_INUSE = crate::Reg<last_ptd_inuse::LAST_PTD_INUSE_SPEC>;
///Marks the last PTD in the list for ISO, INT and ATL
pub mod last_ptd_inuse;
///UTMIPLUS_ULPI_DEBUG register accessor: an alias for `Reg<UTMIPLUS_ULPI_DEBUG_SPEC>`
pub type UTMIPLUS_ULPI_DEBUG = crate::Reg<utmiplus_ulpi_debug::UTMIPLUS_ULPI_DEBUG_SPEC>;
///Register to read/write registers in the attached USB PHY
pub mod utmiplus_ulpi_debug;
///PORTMODE register accessor: an alias for `Reg<PORTMODE_SPEC>`
pub type PORTMODE = crate::Reg<portmode::PORTMODE_SPEC>;
///Controls the port if it is attached to the host block or the device block
pub mod portmode;
