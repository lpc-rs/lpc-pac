#[doc = "Reader of register POS"]
pub type R = crate::R<u32, super::POS>;
#[doc = "Reader of field `POS`"]
pub type POS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current position value."]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
