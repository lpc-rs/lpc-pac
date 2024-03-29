#[doc = "Register `GFC` reader"]
pub struct R(crate::R<GFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GFC` writer"]
pub struct W(crate::W<GFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRFE` reader - Reject remote frames extended."]
pub struct RRFE_R(crate::FieldReader<bool, bool>);
impl RRFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRFE` writer - Reject remote frames extended."]
pub struct RRFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RRFS` reader - Reject remote frames standard."]
pub struct RRFS_R(crate::FieldReader<bool, bool>);
impl RRFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRFS` writer - Reject remote frames standard."]
pub struct RRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ANFE` reader - Accept non-matching frames extended."]
pub struct ANFE_R(crate::FieldReader<u8, u8>);
impl ANFE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ANFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANFE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANFE` writer - Accept non-matching frames extended."]
pub struct ANFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `ANFS` reader - Accept non-matching frames standard."]
pub struct ANFS_R(crate::FieldReader<u8, u8>);
impl ANFS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ANFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANFS` writer - Accept non-matching frames standard."]
pub struct ANFS_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reject remote frames extended."]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reject remote frames standard."]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Accept non-matching frames extended."]
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Accept non-matching frames standard."]
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject remote frames extended."]
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W {
        RRFE_W { w: self }
    }
    #[doc = "Bit 1 - Reject remote frames standard."]
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W {
        RRFS_W { w: self }
    }
    #[doc = "Bits 2:3 - Accept non-matching frames extended."]
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W {
        ANFE_W { w: self }
    }
    #[doc = "Bits 4:5 - Accept non-matching frames standard."]
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W {
        ANFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Filter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfc](index.html) module"]
pub struct GFC_SPEC;
impl crate::RegisterSpec for GFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gfc::R](R) reader structure"]
impl crate::Readable for GFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gfc::W](W) writer structure"]
impl crate::Writable for GFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GFC to value 0"]
impl crate::Resettable for GFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
