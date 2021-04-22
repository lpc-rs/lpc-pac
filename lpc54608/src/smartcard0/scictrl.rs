#[doc = "Register `SCICTRL` reader"]
pub struct R(crate::R<SCICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SCICTRL_SPEC>> for R {
    fn from(reader: crate::R<SCICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCICTRL` writer"]
pub struct W(crate::W<SCICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCICTRL_SPEC>;
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
impl core::convert::From<crate::W<SCICTRL_SPEC>> for W {
    fn from(writer: crate::W<SCICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCIEN` reader - Smart Card Interface Enable."]
pub struct SCIEN_R(crate::FieldReader<bool, bool>);
impl SCIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCIEN` writer - Smart Card Interface Enable."]
pub struct SCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `NACKDIS` reader - NACK response disable."]
pub struct NACKDIS_R(crate::FieldReader<bool, bool>);
impl NACKDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NACKDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACKDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACKDIS` writer - NACK response disable."]
pub struct NACKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PROTSEL` reader - Protocol selection as defined in the ISO7816-3 standard."]
pub struct PROTSEL_R(crate::FieldReader<bool, bool>);
impl PROTSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROTSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROTSEL` writer - Protocol selection as defined in the ISO7816-3 standard."]
pub struct PROTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TXRETRY` reader - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
pub struct TXRETRY_R(crate::FieldReader<u8, u8>);
impl TXRETRY_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXRETRY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRETRY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRETRY` writer - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
pub struct TXRETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRETRY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `GUARDTIME` reader - Extra guard time."]
pub struct GUARDTIME_R(crate::FieldReader<u8, u8>);
impl GUARDTIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        GUARDTIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GUARDTIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GUARDTIME` writer - Extra guard time."]
pub struct GUARDTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> GUARDTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&self) -> SCIEN_R {
        SCIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NACK response disable."]
    #[inline(always)]
    pub fn nackdis(&self) -> NACKDIS_R {
        NACKDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&self) -> PROTSEL_R {
        PROTSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
    #[inline(always)]
    pub fn txretry(&self) -> TXRETRY_R {
        TXRETRY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Extra guard time."]
    #[inline(always)]
    pub fn guardtime(&self) -> GUARDTIME_R {
        GUARDTIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&mut self) -> SCIEN_W {
        SCIEN_W { w: self }
    }
    #[doc = "Bit 1 - NACK response disable."]
    #[inline(always)]
    pub fn nackdis(&mut self) -> NACKDIS_W {
        NACKDIS_W { w: self }
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&mut self) -> PROTSEL_W {
        PROTSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
    #[inline(always)]
    pub fn txretry(&mut self) -> TXRETRY_W {
        TXRETRY_W { w: self }
    }
    #[doc = "Bits 8:15 - Extra guard time."]
    #[inline(always)]
    pub fn guardtime(&mut self) -> GUARDTIME_W {
        GUARDTIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Smart Card Interface control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scictrl](index.html) module"]
pub struct SCICTRL_SPEC;
impl crate::RegisterSpec for SCICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scictrl::R](R) reader structure"]
impl crate::Readable for SCICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scictrl::W](W) writer structure"]
impl crate::Writable for SCICTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCICTRL to value 0"]
impl crate::Resettable for SCICTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
