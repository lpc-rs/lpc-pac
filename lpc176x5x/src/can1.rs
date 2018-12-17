#[doc = r" Register block"]
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
#[doc = "Controls the operating mode of the CAN Controller."]
pub struct MOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the operating mode of the CAN Controller."]
pub mod mod_;
#[doc = "Command bits that affect the state of the CAN Controller"]
pub struct CMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command bits that affect the state of the CAN Controller"]
pub mod cmr;
#[doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1."]
pub struct GSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1."]
pub mod gsr;
#[doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture"]
pub mod icr;
#[doc = "Interrupt Enable"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "Bus Timing. Can only be written when RM in CANMOD is 1."]
pub struct BTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Timing. Can only be written when RM in CANMOD is 1."]
pub mod btr;
#[doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1."]
pub struct EWL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1."]
pub mod ewl;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Receive frame status. Can only be written when RM in CANMOD is 1."]
pub struct RFS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive frame status. Can only be written when RM in CANMOD is 1."]
pub mod rfs;
#[doc = "Received Identifier. Can only be written when RM in CANMOD is 1."]
pub struct RID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received Identifier. Can only be written when RM in CANMOD is 1."]
pub mod rid;
#[doc = "Received data bytes 1-4. Can only be written when RM in CANMOD is 1."]
pub struct RDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received data bytes 1-4. Can only be written when RM in CANMOD is 1."]
pub mod rda;
#[doc = "Received data bytes 5-8. Can only be written when RM in CANMOD is 1."]
pub struct RDB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received data bytes 5-8. Can only be written when RM in CANMOD is 1."]
pub mod rdb;
#[doc = "Transmit frame info (Tx Buffer )"]
pub struct TFI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit frame info (Tx Buffer )"]
pub mod tfi;
#[doc = "Transmit Identifier (Tx Buffer)"]
pub struct TID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Identifier (Tx Buffer)"]
pub mod tid;
#[doc = "Transmit data bytes 1-4 (Tx Buffer)"]
pub struct TDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit data bytes 1-4 (Tx Buffer)"]
pub mod tda;
#[doc = "Transmit data bytes 5-8 (Tx Buffer )"]
pub struct TDB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit data bytes 5-8 (Tx Buffer )"]
pub mod tdb;
