#[doc = "Register `FMSSTOP` reader"]
pub struct R(crate::R<FMSSTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSSTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMSSTOP_SPEC>> for R {
    fn from(reader: crate::R<FMSSTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMSSTOP` writer"]
pub struct W(crate::W<FMSSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMSSTOP_SPEC>;
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
impl core::convert::From<crate::W<FMSSTOP_SPEC>> for W {
    fn from(writer: crate::W<FMSSTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP` reader - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
pub struct STOP_R(crate::FieldReader<u32, u32>);
impl STOP_R {
    pub(crate) fn new(bits: u32) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
#[doc = "Start control bit for signature generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIG_START_A {
    #[doc = "0: Signature generation is stopped"]
    SIGNATURE_GENERATION = 0,
    #[doc = "1: Initiate signature generation"]
    INITIATE_SIGNATURE_G = 1,
}
impl From<SIG_START_A> for bool {
    #[inline(always)]
    fn from(variant: SIG_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIG_START` reader - Start control bit for signature generation."]
pub struct SIG_START_R(crate::FieldReader<bool, SIG_START_A>);
impl SIG_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIG_START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIG_START_A {
        match self.bits {
            false => SIG_START_A::SIGNATURE_GENERATION,
            true => SIG_START_A::INITIATE_SIGNATURE_G,
        }
    }
    #[doc = "Checks if the value of the field is `SIGNATURE_GENERATION`"]
    #[inline(always)]
    pub fn is_signature_generation(&self) -> bool {
        **self == SIG_START_A::SIGNATURE_GENERATION
    }
    #[doc = "Checks if the value of the field is `INITIATE_SIGNATURE_G`"]
    #[inline(always)]
    pub fn is_initiate_signature_g(&self) -> bool {
        **self == SIG_START_A::INITIATE_SIGNATURE_G
    }
}
impl core::ops::Deref for SIG_START_R {
    type Target = crate::FieldReader<bool, SIG_START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIG_START` writer - Start control bit for signature generation."]
pub struct SIG_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIG_START_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Signature generation is stopped"]
    #[inline(always)]
    pub fn signature_generation(self) -> &'a mut W {
        self.variant(SIG_START_A::SIGNATURE_GENERATION)
    }
    #[doc = "Initiate signature generation"]
    #[inline(always)]
    pub fn initiate_signature_g(self) -> &'a mut W {
        self.variant(SIG_START_A::INITIATE_SIGNATURE_G)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - Start control bit for signature generation."]
    #[inline(always)]
    pub fn sig_start(&self) -> SIG_START_R {
        SIG_START_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - BIST stop address divided by 16 (corresponds to AHB byte address \\[20:4\\])."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 17 - Start control bit for signature generation."]
    #[inline(always)]
    pub fn sig_start(&mut self) -> SIG_START_W {
        SIG_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signature stop-address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstop](index.html) module"]
pub struct FMSSTOP_SPEC;
impl crate::RegisterSpec for FMSSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmsstop::R](R) reader structure"]
impl crate::Readable for FMSSTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmsstop::W](W) writer structure"]
impl crate::Writable for FMSSTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMSSTOP to value 0"]
impl crate::Resettable for FMSSTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
