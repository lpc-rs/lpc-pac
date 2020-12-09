#[doc = "Reader of register LPCURR"]
pub type R = crate::R<u32, super::LPCURR>;
#[doc = "Reader of field `LCDLPCURR`"]
pub type LCDLPCURR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - LCD Lower Panel Current Address."]
    #[inline(always)]
    pub fn lcdlpcurr(&self) -> LCDLPCURR_R {
        LCDLPCURR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
