#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
    pub caplength_chipid: CAPLENGTH_CHIPID,
    #[doc = "0x04 - Host Controller Structural Parameters"]
    pub hcsparams: HCSPARAMS,
    #[doc = "0x08 - Host Controller Capability Parameters"]
    pub hccparams: HCCPARAMS,
    #[doc = "0x0c - Frame Length Adjustment"]
    pub fladj_frindex: FLADJ_FRINDEX,
    #[doc = "0x10 - Memory base address where ATL PTD0 is stored"]
    pub atl_ptd_base_addr: ATL_PTD_BASE_ADDR,
    #[doc = "0x14 - Memory base address where ISO PTD0 is stored"]
    pub iso_ptd_base_addr: ISO_PTD_BASE_ADDR,
    #[doc = "0x18 - Memory base address where INT PTD0 is stored"]
    pub int_ptd_base_addr: INT_PTD_BASE_ADDR,
    #[doc = "0x1c - Memory base address that indicates the start of the data payload buffers"]
    pub data_payload_base_addr: DATA_PAYLOAD_BASE_ADDR,
    #[doc = "0x20 - USB Command register"]
    pub usbcmd: USBCMD,
    #[doc = "0x24 - USB Interrupt Status register"]
    pub usbsts: USBSTS,
    #[doc = "0x28 - USB Interrupt Enable register"]
    pub usbintr: USBINTR,
    #[doc = "0x2c - Port Status and Control register"]
    pub portsc1: PORTSC1,
    #[doc = "0x30 - Done map for each ATL PTD"]
    pub atl_ptd_done_map: ATL_PTD_DONE_MAP,
    #[doc = "0x34 - Skip map for each ATL PTD"]
    pub atl_ptd_skip_map: ATL_PTD_SKIP_MAP,
    #[doc = "0x38 - Done map for each ISO PTD"]
    pub iso_ptd_done_map: ISO_PTD_DONE_MAP,
    #[doc = "0x3c - Skip map for each ISO PTD"]
    pub iso_ptd_skip_map: ISO_PTD_SKIP_MAP,
    #[doc = "0x40 - Done map for each INT PTD"]
    pub int_ptd_done_map: INT_PTD_DONE_MAP,
    #[doc = "0x44 - Skip map for each INT PTD"]
    pub int_ptd_skip_map: INT_PTD_SKIP_MAP,
    #[doc = "0x48 - Marks the last PTD in the list for ISO, INT and ATL"]
    pub last_ptd_inuse: LAST_PTD_INUSE,
    #[doc = "0x4c - Register to read/write registers in the attached USB PHY"]
    pub utmiplus_ulpi_debug: UTMIPLUS_ULPI_DEBUG,
    #[doc = "0x50 - Controls the port if it is attached to the host block or the device block"]
    pub portmode: PORTMODE,
}
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caplength_chipid](caplength_chipid) module"]
pub type CAPLENGTH_CHIPID = crate::Reg<u32, _CAPLENGTH_CHIPID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPLENGTH_CHIPID;
#[doc = "`read()` method returns [caplength_chipid::R](caplength_chipid::R) reader structure"]
impl crate::Readable for CAPLENGTH_CHIPID {}
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
pub mod caplength_chipid;
#[doc = "Host Controller Structural Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsparams](hcsparams) module"]
pub type HCSPARAMS = crate::Reg<u32, _HCSPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPARAMS;
#[doc = "`read()` method returns [hcsparams::R](hcsparams::R) reader structure"]
impl crate::Readable for HCSPARAMS {}
#[doc = "Host Controller Structural Parameters"]
pub mod hcsparams;
#[doc = "Host Controller Capability Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccparams](hccparams) module"]
pub type HCCPARAMS = crate::Reg<u32, _HCCPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCPARAMS;
#[doc = "`read()` method returns [hccparams::R](hccparams::R) reader structure"]
impl crate::Readable for HCCPARAMS {}
#[doc = "Host Controller Capability Parameters"]
pub mod hccparams;
#[doc = "Frame Length Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fladj_frindex](fladj_frindex) module"]
pub type FLADJ_FRINDEX = crate::Reg<u32, _FLADJ_FRINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLADJ_FRINDEX;
#[doc = "`read()` method returns [fladj_frindex::R](fladj_frindex::R) reader structure"]
impl crate::Readable for FLADJ_FRINDEX {}
#[doc = "`write(|w| ..)` method takes [fladj_frindex::W](fladj_frindex::W) writer structure"]
impl crate::Writable for FLADJ_FRINDEX {}
#[doc = "Frame Length Adjustment"]
pub mod fladj_frindex;
#[doc = "Memory base address where ATL PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atl_ptd_base_addr](atl_ptd_base_addr) module"]
pub type ATL_PTD_BASE_ADDR = crate::Reg<u32, _ATL_PTD_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATL_PTD_BASE_ADDR;
#[doc = "`read()` method returns [atl_ptd_base_addr::R](atl_ptd_base_addr::R) reader structure"]
impl crate::Readable for ATL_PTD_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [atl_ptd_base_addr::W](atl_ptd_base_addr::W) writer structure"]
impl crate::Writable for ATL_PTD_BASE_ADDR {}
#[doc = "Memory base address where ATL PTD0 is stored"]
pub mod atl_ptd_base_addr;
#[doc = "Memory base address where ISO PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iso_ptd_base_addr](iso_ptd_base_addr) module"]
pub type ISO_PTD_BASE_ADDR = crate::Reg<u32, _ISO_PTD_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISO_PTD_BASE_ADDR;
#[doc = "`read()` method returns [iso_ptd_base_addr::R](iso_ptd_base_addr::R) reader structure"]
impl crate::Readable for ISO_PTD_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [iso_ptd_base_addr::W](iso_ptd_base_addr::W) writer structure"]
impl crate::Writable for ISO_PTD_BASE_ADDR {}
#[doc = "Memory base address where ISO PTD0 is stored"]
pub mod iso_ptd_base_addr;
#[doc = "Memory base address where INT PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ptd_base_addr](int_ptd_base_addr) module"]
pub type INT_PTD_BASE_ADDR = crate::Reg<u32, _INT_PTD_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_PTD_BASE_ADDR;
#[doc = "`read()` method returns [int_ptd_base_addr::R](int_ptd_base_addr::R) reader structure"]
impl crate::Readable for INT_PTD_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [int_ptd_base_addr::W](int_ptd_base_addr::W) writer structure"]
impl crate::Writable for INT_PTD_BASE_ADDR {}
#[doc = "Memory base address where INT PTD0 is stored"]
pub mod int_ptd_base_addr;
#[doc = "Memory base address that indicates the start of the data payload buffers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_payload_base_addr](data_payload_base_addr) module"]
pub type DATA_PAYLOAD_BASE_ADDR = crate::Reg<u32, _DATA_PAYLOAD_BASE_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_PAYLOAD_BASE_ADDR;
#[doc = "`read()` method returns [data_payload_base_addr::R](data_payload_base_addr::R) reader structure"]
impl crate::Readable for DATA_PAYLOAD_BASE_ADDR {}
#[doc = "`write(|w| ..)` method takes [data_payload_base_addr::W](data_payload_base_addr::W) writer structure"]
impl crate::Writable for DATA_PAYLOAD_BASE_ADDR {}
#[doc = "Memory base address that indicates the start of the data payload buffers"]
pub mod data_payload_base_addr;
#[doc = "USB Command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcmd](usbcmd) module"]
pub type USBCMD = crate::Reg<u32, _USBCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCMD;
#[doc = "`read()` method returns [usbcmd::R](usbcmd::R) reader structure"]
impl crate::Readable for USBCMD {}
#[doc = "`write(|w| ..)` method takes [usbcmd::W](usbcmd::W) writer structure"]
impl crate::Writable for USBCMD {}
#[doc = "USB Command register"]
pub mod usbcmd;
#[doc = "USB Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsts](usbsts) module"]
pub type USBSTS = crate::Reg<u32, _USBSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBSTS;
#[doc = "`read()` method returns [usbsts::R](usbsts::R) reader structure"]
impl crate::Readable for USBSTS {}
#[doc = "`write(|w| ..)` method takes [usbsts::W](usbsts::W) writer structure"]
impl crate::Writable for USBSTS {}
#[doc = "USB Interrupt Status register"]
pub mod usbsts;
#[doc = "USB Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbintr](usbintr) module"]
pub type USBINTR = crate::Reg<u32, _USBINTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBINTR;
#[doc = "`read()` method returns [usbintr::R](usbintr::R) reader structure"]
impl crate::Readable for USBINTR {}
#[doc = "`write(|w| ..)` method takes [usbintr::W](usbintr::W) writer structure"]
impl crate::Writable for USBINTR {}
#[doc = "USB Interrupt Enable register"]
pub mod usbintr;
#[doc = "Port Status and Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portsc1](portsc1) module"]
pub type PORTSC1 = crate::Reg<u32, _PORTSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTSC1;
#[doc = "`read()` method returns [portsc1::R](portsc1::R) reader structure"]
impl crate::Readable for PORTSC1 {}
#[doc = "`write(|w| ..)` method takes [portsc1::W](portsc1::W) writer structure"]
impl crate::Writable for PORTSC1 {}
#[doc = "Port Status and Control register"]
pub mod portsc1;
#[doc = "Done map for each ATL PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atl_ptd_done_map](atl_ptd_done_map) module"]
pub type ATL_PTD_DONE_MAP = crate::Reg<u32, _ATL_PTD_DONE_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATL_PTD_DONE_MAP;
#[doc = "`read()` method returns [atl_ptd_done_map::R](atl_ptd_done_map::R) reader structure"]
impl crate::Readable for ATL_PTD_DONE_MAP {}
#[doc = "`write(|w| ..)` method takes [atl_ptd_done_map::W](atl_ptd_done_map::W) writer structure"]
impl crate::Writable for ATL_PTD_DONE_MAP {}
#[doc = "Done map for each ATL PTD"]
pub mod atl_ptd_done_map;
#[doc = "Skip map for each ATL PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atl_ptd_skip_map](atl_ptd_skip_map) module"]
pub type ATL_PTD_SKIP_MAP = crate::Reg<u32, _ATL_PTD_SKIP_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATL_PTD_SKIP_MAP;
#[doc = "`read()` method returns [atl_ptd_skip_map::R](atl_ptd_skip_map::R) reader structure"]
impl crate::Readable for ATL_PTD_SKIP_MAP {}
#[doc = "`write(|w| ..)` method takes [atl_ptd_skip_map::W](atl_ptd_skip_map::W) writer structure"]
impl crate::Writable for ATL_PTD_SKIP_MAP {}
#[doc = "Skip map for each ATL PTD"]
pub mod atl_ptd_skip_map;
#[doc = "Done map for each ISO PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iso_ptd_done_map](iso_ptd_done_map) module"]
pub type ISO_PTD_DONE_MAP = crate::Reg<u32, _ISO_PTD_DONE_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISO_PTD_DONE_MAP;
#[doc = "`read()` method returns [iso_ptd_done_map::R](iso_ptd_done_map::R) reader structure"]
impl crate::Readable for ISO_PTD_DONE_MAP {}
#[doc = "`write(|w| ..)` method takes [iso_ptd_done_map::W](iso_ptd_done_map::W) writer structure"]
impl crate::Writable for ISO_PTD_DONE_MAP {}
#[doc = "Done map for each ISO PTD"]
pub mod iso_ptd_done_map;
#[doc = "Skip map for each ISO PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iso_ptd_skip_map](iso_ptd_skip_map) module"]
pub type ISO_PTD_SKIP_MAP = crate::Reg<u32, _ISO_PTD_SKIP_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISO_PTD_SKIP_MAP;
#[doc = "`read()` method returns [iso_ptd_skip_map::R](iso_ptd_skip_map::R) reader structure"]
impl crate::Readable for ISO_PTD_SKIP_MAP {}
#[doc = "`write(|w| ..)` method takes [iso_ptd_skip_map::W](iso_ptd_skip_map::W) writer structure"]
impl crate::Writable for ISO_PTD_SKIP_MAP {}
#[doc = "Skip map for each ISO PTD"]
pub mod iso_ptd_skip_map;
#[doc = "Done map for each INT PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ptd_done_map](int_ptd_done_map) module"]
pub type INT_PTD_DONE_MAP = crate::Reg<u32, _INT_PTD_DONE_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_PTD_DONE_MAP;
#[doc = "`read()` method returns [int_ptd_done_map::R](int_ptd_done_map::R) reader structure"]
impl crate::Readable for INT_PTD_DONE_MAP {}
#[doc = "`write(|w| ..)` method takes [int_ptd_done_map::W](int_ptd_done_map::W) writer structure"]
impl crate::Writable for INT_PTD_DONE_MAP {}
#[doc = "Done map for each INT PTD"]
pub mod int_ptd_done_map;
#[doc = "Skip map for each INT PTD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ptd_skip_map](int_ptd_skip_map) module"]
pub type INT_PTD_SKIP_MAP = crate::Reg<u32, _INT_PTD_SKIP_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_PTD_SKIP_MAP;
#[doc = "`read()` method returns [int_ptd_skip_map::R](int_ptd_skip_map::R) reader structure"]
impl crate::Readable for INT_PTD_SKIP_MAP {}
#[doc = "`write(|w| ..)` method takes [int_ptd_skip_map::W](int_ptd_skip_map::W) writer structure"]
impl crate::Writable for INT_PTD_SKIP_MAP {}
#[doc = "Skip map for each INT PTD"]
pub mod int_ptd_skip_map;
#[doc = "Marks the last PTD in the list for ISO, INT and ATL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [last_ptd_inuse](last_ptd_inuse) module"]
pub type LAST_PTD_INUSE = crate::Reg<u32, _LAST_PTD_INUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LAST_PTD_INUSE;
#[doc = "`read()` method returns [last_ptd_inuse::R](last_ptd_inuse::R) reader structure"]
impl crate::Readable for LAST_PTD_INUSE {}
#[doc = "`write(|w| ..)` method takes [last_ptd_inuse::W](last_ptd_inuse::W) writer structure"]
impl crate::Writable for LAST_PTD_INUSE {}
#[doc = "Marks the last PTD in the list for ISO, INT and ATL"]
pub mod last_ptd_inuse;
#[doc = "Register to read/write registers in the attached USB PHY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utmiplus_ulpi_debug](utmiplus_ulpi_debug) module"]
pub type UTMIPLUS_ULPI_DEBUG = crate::Reg<u32, _UTMIPLUS_ULPI_DEBUG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UTMIPLUS_ULPI_DEBUG;
#[doc = "`read()` method returns [utmiplus_ulpi_debug::R](utmiplus_ulpi_debug::R) reader structure"]
impl crate::Readable for UTMIPLUS_ULPI_DEBUG {}
#[doc = "`write(|w| ..)` method takes [utmiplus_ulpi_debug::W](utmiplus_ulpi_debug::W) writer structure"]
impl crate::Writable for UTMIPLUS_ULPI_DEBUG {}
#[doc = "Register to read/write registers in the attached USB PHY"]
pub mod utmiplus_ulpi_debug;
#[doc = "Controls the port if it is attached to the host block or the device block\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portmode](portmode) module"]
pub type PORTMODE = crate::Reg<u32, _PORTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTMODE;
#[doc = "`read()` method returns [portmode::R](portmode::R) reader structure"]
impl crate::Readable for PORTMODE {}
#[doc = "`write(|w| ..)` method takes [portmode::W](portmode::W) writer structure"]
impl crate::Writable for PORTMODE {}
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub mod portmode;
