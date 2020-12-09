#[doc = "Reader of register STOP"]
pub type R = crate::R<u32, super::STOP>;
#[doc = "Writer for register STOP"]
pub type W = crate::W<u32, super::STOP>;
#[doc = "Register STOP `reset()`'s with value 0"]
impl crate::ResetValue for super::STOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STOPMSK_L`"]
pub type STOPMSK_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STOPMSK_L`"]
pub struct STOPMSK_L_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPMSK_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `STOPMSK_H`"]
pub type STOPMSK_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STOPMSK_H`"]
pub struct STOPMSK_H_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPMSK_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_l(&self) -> STOPMSK_L_R {
        STOPMSK_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_h(&self) -> STOPMSK_H_R {
        STOPMSK_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_l(&mut self) -> STOPMSK_L_W {
        STOPMSK_L_W { w: self }
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn stopmsk_h(&mut self) -> STOPMSK_H_W {
        STOPMSK_H_W { w: self }
    }
}
