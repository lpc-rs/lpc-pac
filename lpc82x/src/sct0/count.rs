#[doc = "Reader of register COUNT"]
pub type R = crate::R<u32, super::COUNT>;
#[doc = "Writer for register COUNT"]
pub type W = crate::W<u32, super::COUNT>;
#[doc = "Register COUNT `reset()`'s with value 0"]
impl crate::ResetValue for super::COUNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTR_L`"]
pub type CTR_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTR_L`"]
pub struct CTR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `CTR_H`"]
pub type CTR_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTR_H`"]
pub struct CTR_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_l(&self) -> CTR_L_R {
        CTR_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_h(&self) -> CTR_H_R {
        CTR_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_l(&mut self) -> CTR_L_W {
        CTR_L_W { w: self }
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_h(&mut self) -> CTR_H_W {
        CTR_H_W { w: self }
    }
}
