#[doc = "Register `DYNAMICRRD` reader"]
pub struct R(crate::R<DYNAMICRRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICRRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICRRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICRRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICRRD` writer"]
pub struct W(crate::W<DYNAMICRRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICRRD_SPEC>;
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
impl From<crate::W<DYNAMICRRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICRRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRRD` reader - Active bank A to active bank B latency 0x0 - 0xE = n + 1 clock cycles."]
pub struct TRRD_R(crate::FieldReader<u8, u8>);
impl TRRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRRD` writer - Active bank A to active bank B latency 0x0 - 0xE = n + 1 clock cycles."]
pub struct TRRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Active bank A to active bank B latency 0x0 - 0xE = n + 1 clock cycles."]
    #[inline(always)]
    pub fn trrd(&self) -> TRRD_R {
        TRRD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active bank A to active bank B latency 0x0 - 0xE = n + 1 clock cycles."]
    #[inline(always)]
    pub fn trrd(&mut self) -> TRRD_W {
        TRRD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Latency for active bank A to active bank B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrrd](index.html) module"]
pub struct DYNAMICRRD_SPEC;
impl crate::RegisterSpec for DYNAMICRRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamicrrd::R](R) reader structure"]
impl crate::Readable for DYNAMICRRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamicrrd::W](W) writer structure"]
impl crate::Writable for DYNAMICRRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICRRD to value 0x0f"]
impl crate::Resettable for DYNAMICRRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
