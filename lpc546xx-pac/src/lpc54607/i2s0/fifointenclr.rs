#[doc = "Register `FIFOINTENCLR` reader"]
pub struct R(crate::R<FIFOINTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOINTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOINTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOINTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOINTENCLR` writer"]
pub struct W(crate::W<FIFOINTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOINTENCLR_SPEC>;
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
impl From<crate::W<FIFOINTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOINTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXERR` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub struct TXERR_R(crate::FieldReader<bool, bool>);
impl TXERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXERR` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub struct TXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERR_W<'a> {
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
#[doc = "Field `RXERR` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub struct RXERR_R(crate::FieldReader<bool, bool>);
impl RXERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXERR` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub struct RXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERR_W<'a> {
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
#[doc = "Field `TXLVL` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub struct TXLVL_R(crate::FieldReader<bool, bool>);
impl TXLVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLVL` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub struct TXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLVL_W<'a> {
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
#[doc = "Field `RXLVL` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub struct RXLVL_R(crate::FieldReader<bool, bool>);
impl RXLVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLVL` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub struct RXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W {
        TXERR_W { w: self }
    }
    #[doc = "Bit 1 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RXERR_W {
        RXERR_W { w: self }
    }
    #[doc = "Bit 2 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txlvl(&mut self) -> TXLVL_W {
        TXLVL_W { w: self }
    }
    #[doc = "Bit 3 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxlvl(&mut self) -> RXLVL_W {
        RXLVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO interrupt enable clear (disable) and read register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointenclr](index.html) module"]
pub struct FIFOINTENCLR_SPEC;
impl crate::RegisterSpec for FIFOINTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifointenclr::R](R) reader structure"]
impl crate::Readable for FIFOINTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifointenclr::W](W) writer structure"]
impl crate::Writable for FIFOINTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOINTENCLR to value 0"]
impl crate::Resettable for FIFOINTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
