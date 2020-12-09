#[doc = "Reader of register CAPCTRL2"]
pub type R = crate::R<u32, super::CAPCTRL2>;
#[doc = "Writer for register CAPCTRL2"]
pub type W = crate::W<u32, super::CAPCTRL2>;
#[doc = "Register CAPCTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAPCTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPCONn_L`"]
pub type CAPCONN_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAPCONn_L`"]
pub struct CAPCONN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCONN_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `CAPCONn_H`"]
pub type CAPCONN_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAPCONn_H`"]
pub struct CAPCONN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCONN_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_l(&self) -> CAPCONN_L_R {
        CAPCONN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_h(&self) -> CAPCONN_H_R {
        CAPCONN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_l(&mut self) -> CAPCONN_L_W {
        CAPCONN_L_W { w: self }
    }
    #[doc = "Bits 16:31 - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_h(&mut self) -> CAPCONN_H_W {
        CAPCONN_H_W { w: self }
    }
}
