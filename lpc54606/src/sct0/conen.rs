#[doc = "Register `CONEN` reader"]
pub struct R(crate::R<CONEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CONEN_SPEC>> for R {
    fn from(reader: crate::R<CONEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONEN` writer"]
pub struct W(crate::W<CONEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONEN_SPEC>;
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
impl core::convert::From<crate::W<CONEN_SPEC>> for W {
    fn from(writer: crate::W<CONEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCEN` reader - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub struct NCEN_R(crate::FieldReader<u16, u16>);
impl NCEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        NCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCEN` writer - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub struct NCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NCEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncen(&self) -> NCEN_R {
        NCEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncen(&mut self) -> NCEN_W {
        NCEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT conflict interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conen](index.html) module"]
pub struct CONEN_SPEC;
impl crate::RegisterSpec for CONEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conen::R](R) reader structure"]
impl crate::Readable for CONEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conen::W](W) writer structure"]
impl crate::Writable for CONEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONEN to value 0"]
impl crate::Resettable for CONEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
