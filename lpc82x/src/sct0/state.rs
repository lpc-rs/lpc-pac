#[doc = "Reader of register STATE"]
pub type R = crate::R<u32, super::STATE>;
#[doc = "Writer for register STATE"]
pub type W = crate::W<u32, super::STATE>;
#[doc = "Register STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATE_L`"]
pub type STATE_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE_L`"]
pub struct STATE_L_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `STATE_H`"]
pub type STATE_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE_H`"]
pub struct STATE_H_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - State variable."]
    #[inline(always)]
    pub fn state_l(&self) -> STATE_L_R {
        STATE_L_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - State variable."]
    #[inline(always)]
    pub fn state_h(&self) -> STATE_H_R {
        STATE_H_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - State variable."]
    #[inline(always)]
    pub fn state_l(&mut self) -> STATE_L_W {
        STATE_L_W { w: self }
    }
    #[doc = "Bits 16:20 - State variable."]
    #[inline(always)]
    pub fn state_h(&mut self) -> STATE_H_W {
        STATE_H_W { w: self }
    }
}
