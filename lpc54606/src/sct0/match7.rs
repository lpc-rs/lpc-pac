#[doc = "Reader of register MATCH7"]
pub type R = crate::R<u32, super::MATCH7>;
#[doc = "Writer for register MATCH7"]
pub type W = crate::W<u32, super::MATCH7>;
#[doc = "Register MATCH7 `reset()`'s with value 0"]
impl crate::ResetValue for super::MATCH7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCHn_L`"]
pub type MATCHN_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MATCHn_L`"]
pub struct MATCHN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHN_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `MATCHn_H`"]
pub type MATCHN_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MATCHn_H`"]
pub struct MATCHN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHN_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_l(&self) -> MATCHN_L_R {
        MATCHN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_h(&self) -> MATCHN_H_R {
        MATCHN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_l(&mut self) -> MATCHN_L_W {
        MATCHN_L_W { w: self }
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_h(&mut self) -> MATCHN_H_W {
        MATCHN_H_W { w: self }
    }
}
