#[doc = "Register `SYSPLLMDEC` reader"]
pub struct R(crate::R<SYSPLLMDEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLMDEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLMDEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLMDEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSPLLMDEC` writer"]
pub struct W(crate::W<SYSPLLMDEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSPLLMDEC_SPEC>;
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
impl From<crate::W<SYSPLLMDEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSPLLMDEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDEC` reader - Decoded M-divider coefficient value."]
pub struct MDEC_R(crate::FieldReader<u32, u32>);
impl MDEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MDEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDEC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDEC` writer - Decoded M-divider coefficient value."]
pub struct MDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MDEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
#[doc = "Field `MREQ` reader - MDEC reload request."]
pub struct MREQ_R(crate::FieldReader<bool, bool>);
impl MREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MREQ` writer - MDEC reload request."]
pub struct MREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MREQ_W<'a> {
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
    #[doc = "Bits 0:16 - Decoded M-divider coefficient value."]
    #[inline(always)]
    pub fn mdec(&self) -> MDEC_R {
        MDEC_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 17 - MDEC reload request."]
    #[inline(always)]
    pub fn mreq(&self) -> MREQ_R {
        MREQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Decoded M-divider coefficient value."]
    #[inline(always)]
    pub fn mdec(&mut self) -> MDEC_W {
        MDEC_W { w: self }
    }
    #[doc = "Bit 17 - MDEC reload request."]
    #[inline(always)]
    pub fn mreq(&mut self) -> MREQ_W {
        MREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System PLL M divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllmdec](index.html) module"]
pub struct SYSPLLMDEC_SPEC;
impl crate::RegisterSpec for SYSPLLMDEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspllmdec::R](R) reader structure"]
impl crate::Readable for SYSPLLMDEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syspllmdec::W](W) writer structure"]
impl crate::Writable for SYSPLLMDEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSPLLMDEC to value 0"]
impl crate::Resettable for SYSPLLMDEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
