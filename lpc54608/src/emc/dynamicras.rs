#[doc = "Reader of register DYNAMICRAS"]
pub type R = crate::R<u32, super::DYNAMICRAS>;
#[doc = "Writer for register DYNAMICRAS"]
pub type W = crate::W<u32, super::DYNAMICRAS>;
#[doc = "Register DYNAMICRAS `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DYNAMICRAS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `TRAS`"]
pub type TRAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRAS`"]
pub struct TRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Active to precharge command period."]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active to precharge command period."]
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W {
        TRAS_W { w: self }
    }
}
