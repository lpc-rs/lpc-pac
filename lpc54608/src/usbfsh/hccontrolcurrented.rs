#[doc = "Register `HCCONTROLCURRENTED` reader"]
pub struct R(crate::R<HCCONTROLCURRENTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCONTROLCURRENTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCCONTROLCURRENTED_SPEC>> for R {
    fn from(reader: crate::R<HCCONTROLCURRENTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCONTROLCURRENTED` writer"]
pub struct W(crate::W<HCCONTROLCURRENTED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCONTROLCURRENTED_SPEC>;
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
impl core::convert::From<crate::W<HCCONTROLCURRENTED_SPEC>> for W {
    fn from(writer: crate::W<HCCONTROLCURRENTED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCED` reader - ControlCurrentED."]
pub struct CCED_R(crate::FieldReader<u32, u32>);
impl CCED_R {
    pub(crate) fn new(bits: u32) -> Self {
        CCED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCED_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCED` writer - ControlCurrentED."]
pub struct CCED_W<'a> {
    w: &'a mut W,
}
impl<'a> CCED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | ((value as u32 & 0x0fff_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - ControlCurrentED."]
    #[inline(always)]
    pub fn cced(&self) -> CCED_R {
        CCED_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:31 - ControlCurrentED."]
    #[inline(always)]
    pub fn cced(&mut self) -> CCED_W {
        CCED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the physical address of the current endpoint descriptor of the control list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccontrolcurrented](index.html) module"]
pub struct HCCONTROLCURRENTED_SPEC;
impl crate::RegisterSpec for HCCONTROLCURRENTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccontrolcurrented::R](R) reader structure"]
impl crate::Readable for HCCONTROLCURRENTED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hccontrolcurrented::W](W) writer structure"]
impl crate::Writable for HCCONTROLCURRENTED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCCONTROLCURRENTED to value 0"]
impl crate::Resettable for HCCONTROLCURRENTED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
