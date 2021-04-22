#[doc = "Register `MSTDAT` reader"]
pub struct R(crate::R<MSTDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MSTDAT_SPEC>> for R {
    fn from(reader: crate::R<MSTDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTDAT` writer"]
pub struct W(crate::W<MSTDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTDAT_SPEC>;
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
impl core::convert::From<crate::W<MSTDAT_SPEC>> for W {
    fn from(writer: crate::W<MSTDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
pub struct DATA_R(crate::FieldReader<u8, u8>);
impl DATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined Master receiver and transmitter data register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstdat](index.html) module"]
pub struct MSTDAT_SPEC;
impl crate::RegisterSpec for MSTDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstdat::R](R) reader structure"]
impl crate::Readable for MSTDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstdat::W](W) writer structure"]
impl crate::Writable for MSTDAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSTDAT to value 0"]
impl crate::Resettable for MSTDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
