#[doc = "Reader of register EEMSSTART"]
pub type R = crate::R<u32, super::EEMSSTART>;
#[doc = "Writer for register EEMSSTART"]
pub type W = crate::W<u32, super::EEMSSTART>;
#[doc = "Register EEMSSTART `reset()`'s with value 0"]
impl crate::ResetValue for super::EEMSSTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STARTA`"]
pub type STARTA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STARTA`"]
pub struct STARTA_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - BIST start address: Bit 0 is fixed zero since only even addresses are allowed."]
    #[inline(always)]
    pub fn starta(&self) -> STARTA_R {
        STARTA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - BIST start address: Bit 0 is fixed zero since only even addresses are allowed."]
    #[inline(always)]
    pub fn starta(&mut self) -> STARTA_W {
        STARTA_W { w: self }
    }
}
