#[doc = "Register `TESSEICR` reader"]
pub struct R(crate::R<TESSEICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TESSEICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TESSEICR_SPEC>> for R {
    fn from(reader: crate::R<TESSEICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TESSEICR` writer"]
pub struct W(crate::W<TESSEICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TESSEICR_SPEC>;
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
impl core::convert::From<crate::W<TESSEICR_SPEC>> for W {
    fn from(writer: crate::W<TESSEICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `StartResourceSelection` reader - Start resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable start resource. Bit \\[0\\]
corresponds to input 1, bit \\[1\\]
corresponds to input 2, bit \\[2\\]
corresponds to input 3, and bit \\[3\\]
corresponds to input 4."]
pub struct STARTRESOURCESELECTION_R(crate::FieldReader<u8, u8>);
impl STARTRESOURCESELECTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        STARTRESOURCESELECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTRESOURCESELECTION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `StartResourceSelection` writer - Start resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable start resource. Bit \\[0\\]
corresponds to input 1, bit \\[1\\]
corresponds to input 2, bit \\[2\\]
corresponds to input 3, and bit \\[3\\]
corresponds to input 4."]
pub struct STARTRESOURCESELECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTRESOURCESELECTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `StopResourceSelection` reader - Stop resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable stop resource. Bit \\[16\\]
corresponds to input 1, bit \\[17\\]
corresponds to input 2, bit \\[18\\]
corresponds to input 3, and bit \\[19\\]
corresponds to input 4."]
pub struct STOPRESOURCESELECTION_R(crate::FieldReader<u8, u8>);
impl STOPRESOURCESELECTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        STOPRESOURCESELECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOPRESOURCESELECTION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `StopResourceSelection` writer - Stop resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable stop resource. Bit \\[16\\]
corresponds to input 1, bit \\[17\\]
corresponds to input 2, bit \\[18\\]
corresponds to input 3, and bit \\[19\\]
corresponds to input 4."]
pub struct STOPRESOURCESELECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPRESOURCESELECTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Start resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable start resource. Bit \\[0\\]
corresponds to input 1, bit \\[1\\]
corresponds to input 2, bit \\[2\\]
corresponds to input 3, and bit \\[3\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn start_resource_selection(&self) -> STARTRESOURCESELECTION_R {
        STARTRESOURCESELECTION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Stop resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable stop resource. Bit \\[16\\]
corresponds to input 1, bit \\[17\\]
corresponds to input 2, bit \\[18\\]
corresponds to input 3, and bit \\[19\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn stop_resource_selection(&self) -> STOPRESOURCESELECTION_R {
        STOPRESOURCESELECTION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Start resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable start resource. Bit \\[0\\]
corresponds to input 1, bit \\[1\\]
corresponds to input 2, bit \\[2\\]
corresponds to input 3, and bit \\[3\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn start_resource_selection(&mut self) -> STARTRESOURCESELECTION_W {
        STARTRESOURCESELECTION_W { w: self }
    }
    #[doc = "Bits 16:19 - Stop resource selection. Setting any of these bits to 1 selects the corresponding EmbeddedICE watchpoint input as a TraceEnable stop resource. Bit \\[16\\]
corresponds to input 1, bit \\[17\\]
corresponds to input 2, bit \\[18\\]
corresponds to input 3, and bit \\[19\\]
corresponds to input 4."]
    #[inline(always)]
    pub fn stop_resource_selection(&mut self) -> STOPRESOURCESELECTION_W {
        STOPRESOURCESELECTION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tesseicr](index.html) module"]
pub struct TESSEICR_SPEC;
impl crate::RegisterSpec for TESSEICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tesseicr::R](R) reader structure"]
impl crate::Readable for TESSEICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tesseicr::W](W) writer structure"]
impl crate::Writable for TESSEICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TESSEICR to value 0"]
impl crate::Resettable for TESSEICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
