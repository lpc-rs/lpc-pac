///Register `INTENSET` reader
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
///Register `INTENSET` writer
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WAITING` reader - This field indicates if interrupt should be enabled when waiting for input data.
pub struct WAITING_R(crate::FieldReader<bool, bool>);
impl WAITING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAITING` writer - This field indicates if interrupt should be enabled when waiting for input data.
pub struct WAITING_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITING_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Field `DIGEST` reader - This field indicates if interrupt is generated when Digest is ready (completed a Hash or completed a full sequence).
pub struct DIGEST_R(crate::FieldReader<bool, bool>);
impl DIGEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIGEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIGEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DIGEST` writer - This field indicates if interrupt is generated when Digest is ready (completed a Hash or completed a full sequence).
pub struct DIGEST_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGEST_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Field `ERROR` reader - This field indicates if interrupt is generated on an ERROR (as defined in STAT register).
pub struct ERROR_R(crate::FieldReader<bool, bool>);
impl ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ERROR` writer - This field indicates if interrupt is generated on an ERROR (as defined in STAT register).
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    ///Bit 0 - This field indicates if interrupt should be enabled when waiting for input data.
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - This field indicates if interrupt is generated when Digest is ready (completed a Hash or completed a full sequence).
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - This field indicates if interrupt is generated on an ERROR (as defined in STAT register).
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - This field indicates if interrupt should be enabled when waiting for input data.
    #[inline(always)]
    pub fn waiting(&mut self) -> WAITING_W {
        WAITING_W { w: self }
    }
    ///Bit 1 - This field indicates if interrupt is generated when Digest is ready (completed a Hash or completed a full sequence).
    #[inline(always)]
    pub fn digest(&mut self) -> DIGEST_W {
        DIGEST_W { w: self }
    }
    ///Bit 2 - This field indicates if interrupt is generated on an ERROR (as defined in STAT register).
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt Enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [intenset](index.html) module
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
///`read()` method returns [intenset::R](R) reader structure
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [intenset::W](W) writer structure
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
///`reset()` method sets INTENSET to value 0
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
