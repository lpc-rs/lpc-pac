#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YESTOUCH` reader - clear the touch interrupt"]
pub struct YESTOUCH_R(crate::FieldReader<bool, bool>);
impl YESTOUCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        YESTOUCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YESTOUCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YESTOUCH` writer - clear the touch interrupt"]
pub struct YESTOUCH_W<'a> {
    w: &'a mut W,
}
impl<'a> YESTOUCH_W<'a> {
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
#[doc = "Field `NOTOUCH` reader - clear the no-touch interrupt"]
pub struct NOTOUCH_R(crate::FieldReader<bool, bool>);
impl NOTOUCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOTOUCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOTOUCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOTOUCH` writer - clear the no-touch interrupt"]
pub struct NOTOUCH_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTOUCH_W<'a> {
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
#[doc = "Field `POLLDONE` reader - clear the poll or POLLNOW completing interrupt"]
pub struct POLLDONE_R(crate::FieldReader<bool, bool>);
impl POLLDONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POLLDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLLDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLLDONE` writer - clear the poll or POLLNOW completing interrupt"]
pub struct POLLDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLDONE_W<'a> {
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
#[doc = "Field `TIMEOUT` reader - clear the timeout interrupt"]
pub struct TIMEOUT_R(crate::FieldReader<bool, bool>);
impl TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT` writer - clear the timeout interrupt"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
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
#[doc = "Field `OVERUN` reader - clear the overrun interrupt"]
pub struct OVERUN_R(crate::FieldReader<bool, bool>);
impl OVERUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERUN` writer - clear the overrun interrupt"]
pub struct OVERUN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - clear the touch interrupt"]
    #[inline(always)]
    pub fn yestouch(&self) -> YESTOUCH_R {
        YESTOUCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - clear the no-touch interrupt"]
    #[inline(always)]
    pub fn notouch(&self) -> NOTOUCH_R {
        NOTOUCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - clear the poll or POLLNOW completing interrupt"]
    #[inline(always)]
    pub fn polldone(&self) -> POLLDONE_R {
        POLLDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - clear the timeout interrupt"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - clear the overrun interrupt"]
    #[inline(always)]
    pub fn overun(&self) -> OVERUN_R {
        OVERUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clear the touch interrupt"]
    #[inline(always)]
    pub fn yestouch(&mut self) -> YESTOUCH_W {
        YESTOUCH_W { w: self }
    }
    #[doc = "Bit 1 - clear the no-touch interrupt"]
    #[inline(always)]
    pub fn notouch(&mut self) -> NOTOUCH_W {
        NOTOUCH_W { w: self }
    }
    #[doc = "Bit 2 - clear the poll or POLLNOW completing interrupt"]
    #[inline(always)]
    pub fn polldone(&mut self) -> POLLDONE_W {
        POLLDONE_W { w: self }
    }
    #[doc = "Bit 3 - clear the timeout interrupt"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 4 - clear the overrun interrupt"]
    #[inline(always)]
    pub fn overun(&mut self) -> OVERUN_W {
        OVERUN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
