#[doc = "Register `FMSSTART` reader"]
pub struct R(crate::R<FMSSTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSSTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSSTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSSTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMSSTART` writer"]
pub struct W(crate::W<FMSSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMSSTART_SPEC>;
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
impl From<crate::W<FMSSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMSSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
pub struct START_R(crate::FieldReader<u32, u32>);
impl START_R {
    pub(crate) fn new(bits: u32) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Signature generation start address (corresponds to AHB byte address bits\\[20:4\\])."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signature start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstart](index.html) module"]
pub struct FMSSTART_SPEC;
impl crate::RegisterSpec for FMSSTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmsstart::R](R) reader structure"]
impl crate::Readable for FMSSTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmsstart::W](W) writer structure"]
impl crate::Writable for FMSSTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMSSTART to value 0"]
impl crate::Resettable for FMSSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
