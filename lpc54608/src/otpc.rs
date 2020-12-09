#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Register for reading the AES key."]
    pub aeskey: [AESKEY; 8],
    #[doc = "0x30 - ECRP options."]
    pub ecrp: ECRP,
    _reserved2: [u8; 4usize],
    #[doc = "0x38 - User application specific options."]
    pub user0: USER0,
    #[doc = "0x3c - User application specific options."]
    pub user1: USER1,
}
#[doc = "Register for reading the AES key.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeskey](aeskey) module"]
pub type AESKEY = crate::Reg<u32, _AESKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESKEY;
#[doc = "`read()` method returns [aeskey::R](aeskey::R) reader structure"]
impl crate::Readable for AESKEY {}
#[doc = "Register for reading the AES key."]
pub mod aeskey;
#[doc = "ECRP options.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecrp](ecrp) module"]
pub type ECRP = crate::Reg<u32, _ECRP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECRP;
#[doc = "`read()` method returns [ecrp::R](ecrp::R) reader structure"]
impl crate::Readable for ECRP {}
#[doc = "ECRP options."]
pub mod ecrp;
#[doc = "User application specific options.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user0](user0) module"]
pub type USER0 = crate::Reg<u32, _USER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER0;
#[doc = "`read()` method returns [user0::R](user0::R) reader structure"]
impl crate::Readable for USER0 {}
#[doc = "User application specific options."]
pub mod user0;
#[doc = "User application specific options.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user1](user1) module"]
pub type USER1 = crate::Reg<u32, _USER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER1;
#[doc = "`read()` method returns [user1::R](user1::R) reader structure"]
impl crate::Readable for USER1 {}
#[doc = "User application specific options."]
pub mod user1;
