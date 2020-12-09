#[doc = "Reader of register PCFG2"]
pub type R = crate::R<u32, super::PCFG2>;
#[doc = "Writer for register PCFG2"]
pub type W = crate::W<u32, super::PCFG2>;
#[doc = "Register PCFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POSITION`"]
pub type POSITION_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `POSITION`"]
pub struct POSITION_W<'a> {
    w: &'a mut W,
}
impl<'a> POSITION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:24 - Data Position."]
    #[inline(always)]
    pub fn position(&self) -> POSITION_R {
        POSITION_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:24 - Data Position."]
    #[inline(always)]
    pub fn position(&mut self) -> POSITION_W {
        POSITION_W { w: self }
    }
}
