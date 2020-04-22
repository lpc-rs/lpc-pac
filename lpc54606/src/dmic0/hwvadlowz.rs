#[doc = "Reader of register HWVADLOWZ"]
pub type R = crate::R<u32, super::HWVADLOWZ>;
#[doc = "Reader of field `LOWZ`"]
pub type LOWZ_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Noise envelope estimator value."]
    #[inline(always)]
    pub fn lowz(&self) -> LOWZ_R {
        LOWZ_R::new((self.bits & 0xffff) as u16)
    }
}
