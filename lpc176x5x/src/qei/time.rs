#[doc = "Reader of register TIME"]
pub type R = crate::R<u32, super::TIME>;
#[doc = "Reader of field `VELVAL`"]
pub type VELVAL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current velocity timer value."]
    #[inline(always)]
    pub fn velval(&self) -> VELVAL_R {
        VELVAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
