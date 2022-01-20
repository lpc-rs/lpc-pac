#[doc = "Register `HCLSTHRESHOLD` reader"]
pub struct R(crate::R<HCLSTHRESHOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCLSTHRESHOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCLSTHRESHOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCLSTHRESHOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCLSTHRESHOLD` writer"]
pub struct W(crate::W<HCLSTHRESHOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCLSTHRESHOLD_SPEC>;
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
impl From<crate::W<HCLSTHRESHOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCLSTHRESHOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LST` reader - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
pub struct LST_R(crate::FieldReader<u16, u16>);
impl LST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LST_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LST` writer - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
pub struct LST_W<'a> {
    w: &'a mut W,
}
impl<'a> LST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[inline(always)]
    pub fn lst(&mut self) -> LST_W {
        LST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hclsthreshold](index.html) module"]
pub struct HCLSTHRESHOLD_SPEC;
impl crate::RegisterSpec for HCLSTHRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hclsthreshold::R](R) reader structure"]
impl crate::Readable for HCLSTHRESHOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hclsthreshold::W](W) writer structure"]
impl crate::Writable for HCLSTHRESHOLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCLSTHRESHOLD to value 0x0628"]
impl crate::Resettable for HCLSTHRESHOLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0628
    }
}
