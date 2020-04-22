#[doc = "Reader of register HWVADGAIN"]
pub type R = crate::R<u32, super::HWVADGAIN>;
#[doc = "Writer for register HWVADGAIN"]
pub type W = crate::W<u32, super::HWVADGAIN>;
#[doc = "Register HWVADGAIN `reset()`'s with value 0x05"]
impl crate::ResetValue for super::HWVADGAIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x05
    }
}
#[doc = "Reader of field `INPUTGAIN`"]
pub type INPUTGAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INPUTGAIN`"]
pub struct INPUTGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shift value for input bits 0x00 -10 bits 0x01 -8 bits 0x02 -6 bits 0x03 -4 bits 0x04 -2 bits 0x05 0 bits (default) 0x06 +2 bits 0x07 +4 bits 0x08 +6 bits 0x09 +8 bits 0x0A +10 bits 0x0B +12 bits 0x0C +14 bits 0x0D to 0x0F Reserved."]
    #[inline(always)]
    pub fn inputgain(&self) -> INPUTGAIN_R {
        INPUTGAIN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shift value for input bits 0x00 -10 bits 0x01 -8 bits 0x02 -6 bits 0x03 -4 bits 0x04 -2 bits 0x05 0 bits (default) 0x06 +2 bits 0x07 +4 bits 0x08 +6 bits 0x09 +8 bits 0x0A +10 bits 0x0B +12 bits 0x0C +14 bits 0x0D to 0x0F Reserved."]
    #[inline(always)]
    pub fn inputgain(&mut self) -> INPUTGAIN_W {
        INPUTGAIN_W { w: self }
    }
}
