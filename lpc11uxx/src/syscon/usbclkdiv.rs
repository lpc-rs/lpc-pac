#[doc = "Register `USBCLKDIV` reader"]
pub struct R(crate::R<USBCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCLKDIV` writer"]
pub struct W(crate::W<USBCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCLKDIV_SPEC>;
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
impl From<crate::W<USBCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - USB clock divider values 0: Disable USB clock. 1: Divide by 1. to 255: Divide by 255."]
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - USB clock divider values 0: Disable USB clock. 1: Divide by 1. to 255: Divide by 255."]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USB clock divider values 0: Disable USB clock. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USB clock divider values 0: Disable USB clock. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB clock source divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclkdiv](index.html) module"]
pub struct USBCLKDIV_SPEC;
impl crate::RegisterSpec for USBCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbclkdiv::R](R) reader structure"]
impl crate::Readable for USBCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbclkdiv::W](W) writer structure"]
impl crate::Writable for USBCLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCLKDIV to value 0x01"]
impl crate::Resettable for USBCLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
