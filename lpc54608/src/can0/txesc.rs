#[doc = "Register `TXESC` reader"]
pub struct R(crate::R<TXESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXESC` writer"]
pub struct W(crate::W<TXESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXESC_SPEC>;
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
impl From<crate::W<TXESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBDS` reader - Tx buffer data field size."]
pub struct TBDS_R(crate::FieldReader<u8, u8>);
impl TBDS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TBDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBDS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBDS` writer - Tx buffer data field size."]
pub struct TBDS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx buffer data field size."]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx buffer data field size."]
    #[inline(always)]
    pub fn tbds(&mut self) -> TBDS_W {
        TBDS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Element Size Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txesc](index.html) module"]
pub struct TXESC_SPEC;
impl crate::RegisterSpec for TXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txesc::R](R) reader structure"]
impl crate::Readable for TXESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txesc::W](W) writer structure"]
impl crate::Writable for TXESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXESC to value 0"]
impl crate::Resettable for TXESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
