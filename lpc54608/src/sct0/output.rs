#[doc = "Register `OUTPUT` reader"]
pub struct R(crate::R<OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OUTPUT_SPEC>> for R {
    fn from(reader: crate::R<OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTPUT` writer"]
pub struct W(crate::W<OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUT_SPEC>;
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
impl core::convert::From<crate::W<OUTPUT_SPEC>> for W {
    fn from(writer: crate::W<OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT` reader - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub struct OUT_R(crate::FieldReader<u16, u16>);
impl OUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT` writer - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub struct OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W {
        OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [output](index.html) module"]
pub struct OUTPUT_SPEC;
impl crate::RegisterSpec for OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [output::R](R) reader structure"]
impl crate::Readable for OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [output::W](W) writer structure"]
impl crate::Writable for OUTPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTPUT to value 0"]
impl crate::Resettable for OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
