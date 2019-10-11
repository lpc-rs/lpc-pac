#[doc = "Reader of register PIOPORCAP0"]
pub type R = crate::R<u32, super::PIOPORCAP0>;
#[doc = "Writer for register PIOPORCAP0"]
pub type W = crate::W<u32, super::PIOPORCAP0>;
#[doc = "Register PIOPORCAP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PIOPORCAP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIOSTAT`"]
pub type PIOSTAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - State of PIO0_17 through PIO0_0 at power-on reset"]
    #[inline(always)]
    pub fn piostat(&self) -> PIOSTAT_R {
        PIOSTAT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {}
