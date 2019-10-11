#[doc = "Reader of register SCTMATCHREL0"]
pub type R = crate::R<u32, super::SCTMATCHREL0>;
#[doc = "Writer for register SCTMATCHREL0"]
pub type W = crate::W<u32, super::SCTMATCHREL0>;
#[doc = "Register SCTMATCHREL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCTMATCHREL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RELOADn_L`"]
pub type RELOADN_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RELOADn_L`"]
pub struct RELOADN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOADN_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `RELOADn_H`"]
pub type RELOADN_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RELOADn_H`"]
pub struct RELOADN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOADN_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_l(&self) -> RELOADN_L_R {
        RELOADN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_h(&self) -> RELOADN_H_R {
        RELOADN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_l(&mut self) -> RELOADN_L_W {
        RELOADN_L_W { w: self }
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_h(&mut self) -> RELOADN_H_W {
        RELOADN_H_W { w: self }
    }
}
