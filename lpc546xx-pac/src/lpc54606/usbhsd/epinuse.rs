#[doc = "Register `EPINUSE` reader"]
pub struct R(crate::R<EPINUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPINUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPINUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPINUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPINUSE` writer"]
pub struct W(crate::W<EPINUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPINUSE_SPEC>;
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
impl From<crate::W<EPINUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPINUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF` reader - Buffer in use: This register has one bit per physical endpoint."]
pub struct BUF_R(crate::FieldReader<u16, u16>);
impl BUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF` writer - Buffer in use: This register has one bit per physical endpoint."]
pub struct BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | ((value as u32 & 0x03ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:11 - Buffer in use: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub fn buf(&self) -> BUF_R {
        BUF_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:11 - Buffer in use: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub fn buf(&mut self) -> BUF_W {
        BUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Buffer in use\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epinuse](index.html) module"]
pub struct EPINUSE_SPEC;
impl crate::RegisterSpec for EPINUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epinuse::R](R) reader structure"]
impl crate::Readable for EPINUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epinuse::W](W) writer structure"]
impl crate::Writable for EPINUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPINUSE to value 0"]
impl crate::Resettable for EPINUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
