#[doc = "Reader of register START"]
pub type R = crate::R<u32, super::START>;
#[doc = "Writer for register START"]
pub type W = crate::W<u32, super::START>;
#[doc = "Register START `reset()`'s with value 0"]
impl crate::ResetValue for super::START {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STARTMSK_L`"]
pub type STARTMSK_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STARTMSK_L`"]
pub struct STARTMSK_L_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTMSK_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `STARTMSK_H`"]
pub type STARTMSK_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STARTMSK_H`"]
pub struct STARTMSK_H_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTMSK_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn startmsk_l(&self) -> STARTMSK_L_R {
        STARTMSK_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn startmsk_h(&self) -> STARTMSK_H_R {
        STARTMSK_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn startmsk_l(&mut self) -> STARTMSK_L_W {
        STARTMSK_L_W { w: self }
    }
    #[doc = "Bits 16:31 - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn startmsk_h(&mut self) -> STARTMSK_H_W {
        STARTMSK_H_W { w: self }
    }
}
