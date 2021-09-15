#[doc = "Register `EPLISTSTART` reader"]
pub struct R(crate::R<EPLISTSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPLISTSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPLISTSTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPLISTSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPLISTSTART` writer"]
pub struct W(crate::W<EPLISTSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPLISTSTART_SPEC>;
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
impl From<crate::W<EPLISTSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPLISTSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_LIST` reader - Start address of the USB EP Command/Status List."]
pub struct EP_LIST_R(crate::FieldReader<u32, u32>);
impl EP_LIST_R {
    pub(crate) fn new(bits: u32) -> Self {
        EP_LIST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_LIST_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_LIST` writer - Start address of the USB EP Command/Status List."]
pub struct EP_LIST_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_LIST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Start address of the USB EP Command/Status List."]
    #[inline(always)]
    pub fn ep_list(&self) -> EP_LIST_R {
        EP_LIST_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - Start address of the USB EP Command/Status List."]
    #[inline(always)]
    pub fn ep_list(&mut self) -> EP_LIST_W {
        EP_LIST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB EP Command/Status List start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epliststart](index.html) module"]
pub struct EPLISTSTART_SPEC;
impl crate::RegisterSpec for EPLISTSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epliststart::R](R) reader structure"]
impl crate::Readable for EPLISTSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epliststart::W](W) writer structure"]
impl crate::Writable for EPLISTSTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPLISTSTART to value 0"]
impl crate::Resettable for EPLISTSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
