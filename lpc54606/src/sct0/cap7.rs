#[doc = "Reader of register CAP7"]
pub type R = crate::R<u32, super::CAP7>;
#[doc = "Writer for register CAP7"]
pub type W = crate::W<u32, super::CAP7>;
#[doc = "Register CAP7 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAP7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPn_L`"]
pub type CAPN_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAPn_L`"]
pub struct CAPN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPN_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `CAPn_H`"]
pub type CAPN_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAPn_H`"]
pub struct CAPN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPN_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_l(&self) -> CAPN_L_R {
        CAPN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_h(&self) -> CAPN_H_R {
        CAPN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_l(&mut self) -> CAPN_L_W {
        CAPN_L_W { w: self }
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_h(&mut self) -> CAPN_H_W {
        CAPN_H_W { w: self }
    }
}
