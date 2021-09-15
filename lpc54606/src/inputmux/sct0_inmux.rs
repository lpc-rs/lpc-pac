#[doc = "Register `SCT0_INMUX[%s]` reader"]
pub struct R(crate::R<SCT0_INMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCT0_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCT0_INMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCT0_INMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCT0_INMUX[%s]` writer"]
pub struct W(crate::W<SCT0_INMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCT0_INMUX_SPEC>;
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
impl From<crate::W<SCT0_INMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCT0_INMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INP_N` reader - Input number to SCT0 inputs 0 to 6.."]
pub struct INP_N_R(crate::FieldReader<u8, u8>);
impl INP_N_R {
    pub(crate) fn new(bits: u8) -> Self {
        INP_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INP_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INP_N` writer - Input number to SCT0 inputs 0 to 6.."]
pub struct INP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6.."]
    #[inline(always)]
    pub fn inp_n(&self) -> INP_N_R {
        INP_N_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6.."]
    #[inline(always)]
    pub fn inp_n(&mut self) -> INP_N_W {
        INP_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger select register for DMA channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sct0_inmux](index.html) module"]
pub struct SCT0_INMUX_SPEC;
impl crate::RegisterSpec for SCT0_INMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sct0_inmux::R](R) reader structure"]
impl crate::Readable for SCT0_INMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sct0_inmux::W](W) writer structure"]
impl crate::Writable for SCT0_INMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCT0_INMUX[%s]
to value 0x1f"]
impl crate::Resettable for SCT0_INMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
