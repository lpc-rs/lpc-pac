#[doc = "Register `TCBCNT` reader"]
pub struct R(crate::R<TCBCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TCBCNT_SPEC>> for R {
    fn from(reader: crate::R<TCBCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCBCNT` writer"]
pub struct W(crate::W<TCBCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCBCNT_SPEC>;
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
impl core::convert::From<crate::W<TCBCNT_SPEC>> for W {
    fn from(writer: crate::W<TCBCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRANS_CARD_BYTE_COUNT` reader - Number of bytes transferred by CIU unit to card."]
pub struct TRANS_CARD_BYTE_COUNT_R(crate::FieldReader<u32, u32>);
impl TRANS_CARD_BYTE_COUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        TRANS_CARD_BYTE_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_CARD_BYTE_COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_CARD_BYTE_COUNT` writer - Number of bytes transferred by CIU unit to card."]
pub struct TRANS_CARD_BYTE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_CARD_BYTE_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card."]
    #[inline(always)]
    pub fn trans_card_byte_count(&self) -> TRANS_CARD_BYTE_COUNT_R {
        TRANS_CARD_BYTE_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card."]
    #[inline(always)]
    pub fn trans_card_byte_count(&mut self) -> TRANS_CARD_BYTE_COUNT_W {
        TRANS_CARD_BYTE_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transferred CIU Card Byte Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcbcnt](index.html) module"]
pub struct TCBCNT_SPEC;
impl crate::RegisterSpec for TCBCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcbcnt::R](R) reader structure"]
impl crate::Readable for TCBCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcbcnt::W](W) writer structure"]
impl crate::Writable for TCBCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCBCNT to value 0"]
impl crate::Resettable for TCBCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
