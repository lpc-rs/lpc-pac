#[doc = "Reader of register DYNAMICRC"]
pub type R = crate::R<u32, super::DYNAMICRC>;
#[doc = "Writer for register DYNAMICRC"]
pub type W = crate::W<u32, super::DYNAMICRC>;
#[doc = "Register DYNAMICRC `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::DYNAMICRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `TRC`"]
pub type TRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRC`"]
pub struct TRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Active to active command period."]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Active to active command period."]
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W {
        TRC_W { w: self }
    }
}
