#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Register for reading the AES key."]
    pub aeskey: [crate::Reg<aeskey::AESKEY_SPEC>; 8],
    #[doc = "0x30 - ECRP options."]
    pub ecrp: crate::Reg<ecrp::ECRP_SPEC>,
    _reserved2: [u8; 4usize],
    #[doc = "0x38 - User application specific options."]
    pub user0: crate::Reg<user0::USER0_SPEC>,
    #[doc = "0x3c - User application specific options."]
    pub user1: crate::Reg<user1::USER1_SPEC>,
}
#[doc = "AESKEY register accessor: an alias for `Reg<AESKEY_SPEC>`"]
pub type AESKEY = crate::Reg<aeskey::AESKEY_SPEC>;
#[doc = "Register for reading the AES key."]
pub mod aeskey;
#[doc = "ECRP register accessor: an alias for `Reg<ECRP_SPEC>`"]
pub type ECRP = crate::Reg<ecrp::ECRP_SPEC>;
#[doc = "ECRP options."]
pub mod ecrp;
#[doc = "USER0 register accessor: an alias for `Reg<USER0_SPEC>`"]
pub type USER0 = crate::Reg<user0::USER0_SPEC>;
#[doc = "User application specific options."]
pub mod user0;
#[doc = "USER1 register accessor: an alias for `Reg<USER1_SPEC>`"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "User application specific options."]
pub mod user1;
