#[doc = "Reader of register DEBNCE"]
pub type R = crate::R<u32, super::DEBNCE>;
#[doc = "Writer for register DEBNCE"]
pub type W = crate::W<u32, super::DEBNCE>;
#[doc = "Register DEBNCE `reset()`'s with value 0x00ff_ffff"]
impl crate::ResetValue for super::DEBNCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_ffff
    }
}
#[doc = "Reader of field `DEBOUNCE_COUNT`"]
pub type DEBOUNCE_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DEBOUNCE_COUNT`"]
pub struct DEBOUNCE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBOUNCE_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Number of host clocks (SD_CLK) used by debounce filter logic for card detect; typical debounce time is 5-25 ms."]
    #[inline(always)]
    pub fn debounce_count(&self) -> DEBOUNCE_COUNT_R {
        DEBOUNCE_COUNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Number of host clocks (SD_CLK) used by debounce filter logic for card detect; typical debounce time is 5-25 ms."]
    #[inline(always)]
    pub fn debounce_count(&mut self) -> DEBOUNCE_COUNT_W {
        DEBOUNCE_COUNT_W { w: self }
    }
}
