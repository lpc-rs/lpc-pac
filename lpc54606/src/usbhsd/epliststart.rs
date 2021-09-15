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
#[doc = "Field `EP_LIST_PRG` reader - Programmable portion of the USB EP Command/Status List address."]
pub struct EP_LIST_PRG_R(crate::FieldReader<u16, u16>);
impl EP_LIST_PRG_R {
    pub(crate) fn new(bits: u16) -> Self {
        EP_LIST_PRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_LIST_PRG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP_LIST_PRG` writer - Programmable portion of the USB EP Command/Status List address."]
pub struct EP_LIST_PRG_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_LIST_PRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | ((value as u32 & 0x0fff) << 8);
        self.w
    }
}
#[doc = "Field `EP_LIST_FIXED` reader - Fixed portion of USB EP Command/Status List address."]
pub struct EP_LIST_FIXED_R(crate::FieldReader<u16, u16>);
impl EP_LIST_FIXED_R {
    pub(crate) fn new(bits: u16) -> Self {
        EP_LIST_FIXED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_LIST_FIXED_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:19 - Programmable portion of the USB EP Command/Status List address."]
    #[inline(always)]
    pub fn ep_list_prg(&self) -> EP_LIST_PRG_R {
        EP_LIST_PRG_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - Fixed portion of USB EP Command/Status List address."]
    #[inline(always)]
    pub fn ep_list_fixed(&self) -> EP_LIST_FIXED_R {
        EP_LIST_FIXED_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:19 - Programmable portion of the USB EP Command/Status List address."]
    #[inline(always)]
    pub fn ep_list_prg(&mut self) -> EP_LIST_PRG_W {
        EP_LIST_PRG_W { w: self }
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
