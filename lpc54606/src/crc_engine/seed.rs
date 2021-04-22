#[doc = "Register `SEED` reader"]
pub struct R(crate::R<SEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SEED_SPEC>> for R {
    fn from(reader: crate::R<SEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEED` writer"]
pub struct W(crate::W<SEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEED_SPEC>;
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
impl core::convert::From<crate::W<SEED_SPEC>> for W {
    fn from(writer: crate::W<SEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_SEED` reader - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
pub struct CRC_SEED_R(crate::FieldReader<u32, u32>);
impl CRC_SEED_R {
    pub(crate) fn new(bits: u32) -> Self {
        CRC_SEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_SEED_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_SEED` writer - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
pub struct CRC_SEED_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_SEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[inline(always)]
    pub fn crc_seed(&self) -> CRC_SEED_R {
        CRC_SEED_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[inline(always)]
    pub fn crc_seed(&mut self) -> CRC_SEED_W {
        CRC_SEED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC seed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seed](index.html) module"]
pub struct SEED_SPEC;
impl crate::RegisterSpec for SEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seed::R](R) reader structure"]
impl crate::Readable for SEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seed::W](W) writer structure"]
impl crate::Writable for SEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEED to value 0xffff"]
impl crate::Resettable for SEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
