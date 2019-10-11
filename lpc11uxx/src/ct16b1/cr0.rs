#[doc = "Reader of register CR0"]
pub type R = crate::R<u32, super::CR0>;
#[doc = "Reader of field `CAP`"]
pub type CAP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer counter capture value."]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new((self.bits & 0xffff) as u16)
    }
}
