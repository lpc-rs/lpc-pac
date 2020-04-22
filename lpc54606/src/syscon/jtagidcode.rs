#[doc = "Reader of register JTAGIDCODE"]
pub type R = crate::R<u32, super::JTAGIDCODE>;
#[doc = "Reader of field `JTAGID`"]
pub type JTAGID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - JTAG ID code."]
    #[inline(always)]
    pub fn jtagid(&self) -> JTAGID_R {
        JTAGID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
