#[doc = "Reader of register HALT"]
pub type R = crate::R<u32, super::HALT>;
#[doc = "Writer for register HALT"]
pub type W = crate::W<u32, super::HALT>;
#[doc = "Register HALT `reset()`'s with value 0"]
impl crate::ResetValue for super::HALT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HALTMSK_L`"]
pub type HALTMSK_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HALTMSK_L`"]
pub struct HALTMSK_L_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTMSK_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `HALTMSK_H`"]
pub type HALTMSK_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HALTMSK_H`"]
pub struct HALTMSK_H_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTMSK_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_l(&self) -> HALTMSK_L_R {
        HALTMSK_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_h(&self) -> HALTMSK_H_R {
        HALTMSK_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_l(&mut self) -> HALTMSK_L_W {
        HALTMSK_L_W { w: self }
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_h(&mut self) -> HALTMSK_H_W {
        HALTMSK_H_W { w: self }
    }
}
