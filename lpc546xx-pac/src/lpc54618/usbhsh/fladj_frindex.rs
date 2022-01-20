#[doc = "Register `FLADJ_FRINDEX` reader"]
pub struct R(crate::R<FLADJ_FRINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLADJ_FRINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLADJ_FRINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLADJ_FRINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLADJ_FRINDEX` writer"]
pub struct W(crate::W<FLADJ_FRINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLADJ_FRINDEX_SPEC>;
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
impl From<crate::W<FLADJ_FRINDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLADJ_FRINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLADJ` reader - Frame Length Timing Value."]
pub struct FLADJ_R(crate::FieldReader<u8, u8>);
impl FLADJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLADJ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLADJ` writer - Frame Length Timing Value."]
pub struct FLADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `FRINDEX` reader - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
pub struct FRINDEX_R(crate::FieldReader<u16, u16>);
impl FRINDEX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FRINDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRINDEX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRINDEX` writer - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
pub struct FRINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Frame Length Timing Value."]
    #[inline(always)]
    pub fn fladj(&self) -> FLADJ_R {
        FLADJ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:29 - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub fn frindex(&self) -> FRINDEX_R {
        FRINDEX_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frame Length Timing Value."]
    #[inline(always)]
    pub fn fladj(&mut self) -> FLADJ_W {
        FLADJ_W { w: self }
    }
    #[doc = "Bits 16:29 - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub fn frindex(&mut self) -> FRINDEX_W {
        FRINDEX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Length Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fladj_frindex](index.html) module"]
pub struct FLADJ_FRINDEX_SPEC;
impl crate::RegisterSpec for FLADJ_FRINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fladj_frindex::R](R) reader structure"]
impl crate::Readable for FLADJ_FRINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fladj_frindex::W](W) writer structure"]
impl crate::Writable for FLADJ_FRINDEX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLADJ_FRINDEX to value 0x20"]
impl crate::Resettable for FLADJ_FRINDEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
