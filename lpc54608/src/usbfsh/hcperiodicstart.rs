#[doc = "Register `HCPERIODICSTART` reader"]
pub struct R(crate::R<HCPERIODICSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCPERIODICSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCPERIODICSTART_SPEC>> for R {
    fn from(reader: crate::R<HCPERIODICSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCPERIODICSTART` writer"]
pub struct W(crate::W<HCPERIODICSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCPERIODICSTART_SPEC>;
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
impl core::convert::From<crate::W<HCPERIODICSTART_SPEC>> for W {
    fn from(writer: crate::W<HCPERIODICSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PS` reader - PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
pub struct PS_R(crate::FieldReader<u16, u16>);
impl PS_R {
    pub(crate) fn new(bits: u16) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcperiodicstart](index.html) module"]
pub struct HCPERIODICSTART_SPEC;
impl crate::RegisterSpec for HCPERIODICSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcperiodicstart::R](R) reader structure"]
impl crate::Readable for HCPERIODICSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcperiodicstart::W](W) writer structure"]
impl crate::Writable for HCPERIODICSTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCPERIODICSTART to value 0"]
impl crate::Resettable for HCPERIODICSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
