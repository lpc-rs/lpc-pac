#[doc = "Reader of register LIMIT"]
pub type R = crate::R<u32, super::LIMIT>;
#[doc = "Writer for register LIMIT"]
pub type W = crate::W<u32, super::LIMIT>;
#[doc = "Register LIMIT `reset()`'s with value 0"]
impl crate::ResetValue for super::LIMIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LIMMSK_L`"]
pub type LIMMSK_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LIMMSK_L`"]
pub struct LIMMSK_L_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMMSK_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `LIMMSK_H`"]
pub type LIMMSK_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LIMMSK_H`"]
pub struct LIMMSK_H_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMMSK_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn limmsk_l(&self) -> LIMMSK_L_R {
        LIMMSK_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn limmsk_h(&self) -> LIMMSK_H_R {
        LIMMSK_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn limmsk_l(&mut self) -> LIMMSK_L_W {
        LIMMSK_L_W { w: self }
    }
    #[doc = "Bits 16:31 - If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn limmsk_h(&mut self) -> LIMMSK_H_W {
        LIMMSK_H_W { w: self }
    }
}
