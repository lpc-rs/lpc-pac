#[doc = "Reader of register DEVICE_ID0"]
pub type R = crate::R<u32, super::DEVICE_ID0>;
#[doc = "Reader of field `PARTID`"]
pub type PARTID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Part ID"]
    #[inline(always)]
    pub fn partid(&self) -> PARTID_R {
        PARTID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
