#[doc = "Reader of register MAC_SYS_TIME_NSCND"]
pub type R = crate::R<u32, super::MAC_SYS_TIME_NSCND>;
#[doc = "Reader of field `TSSS`"]
pub type TSSS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:30 - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0."]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
