#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls the operating mode of the CAN Controller."]
    pub mod_: MOD,
    #[doc = "0x04 - Command bits that affect the state of the CAN Controller"]
    pub cmr: CMR,
    #[doc = "0x08 - Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1."]
    pub gsr: GSR,
    #[doc = "0x0c - Interrupt status, Arbitration Lost Capture, Error Code Capture"]
    pub icr: ICR,
    #[doc = "0x10 - Interrupt Enable"]
    pub ier: IER,
    #[doc = "0x14 - Bus Timing. Can only be written when RM in CANMOD is 1."]
    pub btr: BTR,
    #[doc = "0x18 - Error Warning Limit. Can only be written when RM in CANMOD is 1."]
    pub ewl: EWL,
    #[doc = "0x1c - Status Register"]
    pub sr: SR,
    #[doc = "0x20 - Receive frame status. Can only be written when RM in CANMOD is 1."]
    pub rfs: RFS,
    #[doc = "0x24 - Received Identifier. Can only be written when RM in CANMOD is 1."]
    pub rid: RID,
    #[doc = "0x28 - Received data bytes 1-4. Can only be written when RM in CANMOD is 1."]
    pub rda: RDA,
    #[doc = "0x2c - Received data bytes 5-8. Can only be written when RM in CANMOD is 1."]
    pub rdb: RDB,
    #[doc = "0x30 - Transmit frame info (Tx Buffer )"]
    pub tfi1: TFI,
    #[doc = "0x34 - Transmit Identifier (Tx Buffer)"]
    pub tid1: TID,
    #[doc = "0x38 - Transmit data bytes 1-4 (Tx Buffer)"]
    pub tda1: TDA,
    #[doc = "0x3c - Transmit data bytes 5-8 (Tx Buffer )"]
    pub tdb1: TDB,
    #[doc = "0x40 - Transmit frame info (Tx Buffer )"]
    pub tfi2: TFI,
    #[doc = "0x44 - Transmit Identifier (Tx Buffer)"]
    pub tid2: TID,
    #[doc = "0x48 - Transmit data bytes 1-4 (Tx Buffer)"]
    pub tda2: TDA,
    #[doc = "0x4c - Transmit data bytes 5-8 (Tx Buffer )"]
    pub tdb2: TDB,
    #[doc = "0x50 - Transmit frame info (Tx Buffer )"]
    pub tfi3: TFI,
    #[doc = "0x54 - Transmit Identifier (Tx Buffer)"]
    pub tid3: TID,
    #[doc = "0x58 - Transmit data bytes 1-4 (Tx Buffer)"]
    pub tda3: TDA,
    #[doc = "0x5c - Transmit data bytes 5-8 (Tx Buffer )"]
    pub tdb3: TDB,
}
#[doc = "Controls the operating mode of the CAN Controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_](mod_) module"]
pub type MOD = crate::Reg<u32, _MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOD;
#[doc = "`read()` method returns [mod_::R](mod_::R) reader structure"]
impl crate::Readable for MOD {}
#[doc = "`write(|w| ..)` method takes [mod_::W](mod_::W) writer structure"]
impl crate::Writable for MOD {}
#[doc = "Controls the operating mode of the CAN Controller."]
pub mod mod_;
#[doc = "Command bits that affect the state of the CAN Controller\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](cmr) module"]
pub type CMR = crate::Reg<u32, _CMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR;
#[doc = "`write(|w| ..)` method takes [cmr::W](cmr::W) writer structure"]
impl crate::Writable for CMR {}
#[doc = "Command bits that affect the state of the CAN Controller"]
pub mod cmr;
#[doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gsr](gsr) module"]
pub type GSR = crate::Reg<u32, _GSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSR;
#[doc = "`read()` method returns [gsr::R](gsr::R) reader structure"]
impl crate::Readable for GSR {}
#[doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1."]
pub mod gsr;
#[doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture"]
pub mod icr;
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "Bus Timing. Can only be written when RM in CANMOD is 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr](btr) module"]
pub type BTR = crate::Reg<u32, _BTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR;
#[doc = "`read()` method returns [btr::R](btr::R) reader structure"]
impl crate::Readable for BTR {}
#[doc = "`write(|w| ..)` method takes [btr::W](btr::W) writer structure"]
impl crate::Writable for BTR {}
#[doc = "Bus Timing. Can only be written when RM in CANMOD is 1."]
pub mod btr;
#[doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ewl](ewl) module"]
pub type EWL = crate::Reg<u32, _EWL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EWL;
#[doc = "`read()` method returns [ewl::R](ewl::R) reader structure"]
impl crate::Readable for EWL {}
#[doc = "`write(|w| ..)` method takes [ewl::W](ewl::W) writer structure"]
impl crate::Writable for EWL {}
#[doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1."]
pub mod ewl;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Receive frame status. Can only be written when RM in CANMOD is 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfs](rfs) module"]
pub type RFS = crate::Reg<u32, _RFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFS;
#[doc = "`read()` method returns [rfs::R](rfs::R) reader structure"]
impl crate::Readable for RFS {}
#[doc = "`write(|w| ..)` method takes [rfs::W](rfs::W) writer structure"]
impl crate::Writable for RFS {}
#[doc = "Receive frame status. Can only be written when RM in CANMOD is 1."]
pub mod rfs;
#[doc = "Received Identifier. Can only be written when RM in CANMOD is 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rid](rid) module"]
pub type RID = crate::Reg<u32, _RID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RID;
#[doc = "`read()` method returns [rid::R](rid::R) reader structure"]
impl crate::Readable for RID {}
#[doc = "`write(|w| ..)` method takes [rid::W](rid::W) writer structure"]
impl crate::Writable for RID {}
#[doc = "Received Identifier. Can only be written when RM in CANMOD is 1."]
pub mod rid;
#[doc = "Received data bytes 1-4. Can only be written when RM in CANMOD is 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rda](rda) module"]
pub type RDA = crate::Reg<u32, _RDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDA;
#[doc = "`read()` method returns [rda::R](rda::R) reader structure"]
impl crate::Readable for RDA {}
#[doc = "`write(|w| ..)` method takes [rda::W](rda::W) writer structure"]
impl crate::Writable for RDA {}
#[doc = "Received data bytes 1-4. Can only be written when RM in CANMOD is 1."]
pub mod rda;
#[doc = "Received data bytes 5-8. Can only be written when RM in CANMOD is 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdb](rdb) module"]
pub type RDB = crate::Reg<u32, _RDB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDB;
#[doc = "`read()` method returns [rdb::R](rdb::R) reader structure"]
impl crate::Readable for RDB {}
#[doc = "`write(|w| ..)` method takes [rdb::W](rdb::W) writer structure"]
impl crate::Writable for RDB {}
#[doc = "Received data bytes 5-8. Can only be written when RM in CANMOD is 1."]
pub mod rdb;
#[doc = "Transmit frame info (Tx Buffer )\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfi](tfi) module"]
pub type TFI = crate::Reg<u32, _TFI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFI;
#[doc = "`read()` method returns [tfi::R](tfi::R) reader structure"]
impl crate::Readable for TFI {}
#[doc = "`write(|w| ..)` method takes [tfi::W](tfi::W) writer structure"]
impl crate::Writable for TFI {}
#[doc = "Transmit frame info (Tx Buffer )"]
pub mod tfi;
#[doc = "Transmit Identifier (Tx Buffer)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tid](tid) module"]
pub type TID = crate::Reg<u32, _TID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TID;
#[doc = "`read()` method returns [tid::R](tid::R) reader structure"]
impl crate::Readable for TID {}
#[doc = "`write(|w| ..)` method takes [tid::W](tid::W) writer structure"]
impl crate::Writable for TID {}
#[doc = "Transmit Identifier (Tx Buffer)"]
pub mod tid;
#[doc = "Transmit data bytes 1-4 (Tx Buffer)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tda](tda) module"]
pub type TDA = crate::Reg<u32, _TDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDA;
#[doc = "`read()` method returns [tda::R](tda::R) reader structure"]
impl crate::Readable for TDA {}
#[doc = "`write(|w| ..)` method takes [tda::W](tda::W) writer structure"]
impl crate::Writable for TDA {}
#[doc = "Transmit data bytes 1-4 (Tx Buffer)"]
pub mod tda;
#[doc = "Transmit data bytes 5-8 (Tx Buffer )\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdb](tdb) module"]
pub type TDB = crate::Reg<u32, _TDB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDB;
#[doc = "`read()` method returns [tdb::R](tdb::R) reader structure"]
impl crate::Readable for TDB {}
#[doc = "`write(|w| ..)` method takes [tdb::W](tdb::W) writer structure"]
impl crate::Writable for TDB {}
#[doc = "Transmit data bytes 5-8 (Tx Buffer )"]
pub mod tdb;
