#[doc = "Reader of register DYNAMICRFC"]
pub type R = crate::R<u32, super::DYNAMICRFC>;
#[doc = "Writer for register DYNAMICRFC"]
pub type W = crate::W<u32, super::DYNAMICRFC>;
#[doc = "Register DYNAMICRFC `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::DYNAMICRFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `TRFC`"]
pub type TRFC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRFC`"]
pub struct TRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Auto-refresh period and auto-refresh to active command period."]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Auto-refresh period and auto-refresh to active command period."]
    #[inline(always)]
    pub fn trfc(&mut self) -> TRFC_W {
        TRFC_W { w: self }
    }
}
