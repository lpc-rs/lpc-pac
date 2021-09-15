#[doc = "Register `CLAIMSET` reader"]
pub struct R(crate::R<CLAIMSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIMSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIMSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIMSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLAIMSET` writer"]
pub struct W(crate::W<CLAIMSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIMSET_SPEC>;
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
impl From<crate::W<CLAIMSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIMSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLAIMSET` reader - A bit programmable register bank which sets the Claim Tag Value. Write 1 to set the bit in the claim tag. A read will return a logic 1 for all implemented locations."]
pub struct CLAIMSET_R(crate::FieldReader<u8, u8>);
impl CLAIMSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLAIMSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLAIMSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLAIMSET` writer - A bit programmable register bank which sets the Claim Tag Value. Write 1 to set the bit in the claim tag. A read will return a logic 1 for all implemented locations."]
pub struct CLAIMSET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - A bit programmable register bank which sets the Claim Tag Value. Write 1 to set the bit in the claim tag. A read will return a logic 1 for all implemented locations."]
    #[inline(always)]
    pub fn claimset(&self) -> CLAIMSET_R {
        CLAIMSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - A bit programmable register bank which sets the Claim Tag Value. Write 1 to set the bit in the claim tag. A read will return a logic 1 for all implemented locations."]
    #[inline(always)]
    pub fn claimset(&mut self) -> CLAIMSET_W {
        CLAIMSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Claim Tag Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimset](index.html) module"]
pub struct CLAIMSET_SPEC;
impl crate::RegisterSpec for CLAIMSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claimset::R](R) reader structure"]
impl crate::Readable for CLAIMSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claimset::W](W) writer structure"]
impl crate::Writable for CLAIMSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLAIMSET to value 0"]
impl crate::Resettable for CLAIMSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
