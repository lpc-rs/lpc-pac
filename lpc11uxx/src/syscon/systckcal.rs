#[doc = "Reader of register SYSTCKCAL"]
pub type R = crate::R<u32, super::SYSTCKCAL>;
#[doc = "Writer for register SYSTCKCAL"]
pub type W = crate::W<u32, super::SYSTCKCAL>;
#[doc = "Register SYSTCKCAL `reset()`'s with value 0x04"]
impl crate::ResetValue for super::SYSTCKCAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `CAL`"]
pub type CAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CAL`"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - System tick timer calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - System tick timer calibration value"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
}
