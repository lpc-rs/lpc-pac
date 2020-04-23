#[doc = "Reader of register DEVICE_ID1"]
pub type R = crate::R<u32, super::DEVICE_ID1>;
#[doc = "Reader of field `REVID`"]
pub type REVID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Revision."]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
