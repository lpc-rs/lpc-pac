#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `YESTOUCH` reader - Is 1 if a touch has been detected, including a wakeup from low-power mode."]
pub struct YESTOUCH_R(crate::FieldReader<bool, bool>);
impl YESTOUCH_R {
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
#[doc = "Field `YESTOUCH` writer - Is 1 if a touch has been detected, including a wakeup from low-power mode."]
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
#[doc = "Field `NOTOUCH` reader - Is 1 if a no-touch has been detected (ie. completed an integration cycle and found no-touch). This is not set when in low-power mode."]
pub struct NOTOUCH_R(crate::FieldReader<bool, bool>);
impl NOTOUCH_R {
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
#[doc = "Field `NOTOUCH` writer - Is 1 if a no-touch has been detected (ie. completed an integration cycle and found no-touch). This is not set when in low-power mode."]
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
#[doc = "Field `POLLDONE` reader - Is 1 if a poll or POLLNOW is complete."]
pub struct POLLDONE_R(crate::FieldReader<bool, bool>);
impl POLLDONE_R {
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
#[doc = "Field `POLLDONE` writer - Is 1 if a poll or POLLNOW is complete."]
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
#[doc = "Field `TIMEOUT` reader - Is 1 if an integration cycle ended with a timeout (should not happen)."]
pub struct TIMEOUT_R(crate::FieldReader<bool, bool>);
impl TIMEOUT_R {
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
#[doc = "Field `TIMEOUT` writer - Is 1 if an integration cycle ended with a timeout (should not happen)."]
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
#[doc = "Field `OVERUN` reader - Is 1 if new data was collected before application read out previous ISTOUCH. No-touch (ISTOUCH==0) data will be silently overrun. Is not possible if WAIT=1."]
pub struct OVERUN_R(crate::FieldReader<bool, bool>);
impl OVERUN_R {
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
#[doc = "Field `OVERUN` writer - Is 1 if new data was collected before application read out previous ISTOUCH. No-touch (ISTOUCH==0) data will be silently overrun. Is not possible if WAIT=1."]
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
#[doc = "Field `BUSY` reader - In a poll now."]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XMAX` reader - Indicates the maximum number of X pins allowed 0-relative. So, 15 means there are pins 0 to 15, or 16 total X pins. INTERNAL note: this may be setup to be written by ROM boot."]
pub struct XMAX_R(crate::FieldReader<u8, u8>);
impl XMAX_R {
    pub(crate) fn new(bits: u8) -> Self {
        XMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XMAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Is 1 if a touch has been detected, including a wakeup from low-power mode."]
    #[inline(always)]
    pub fn yestouch(&self) -> YESTOUCH_R {
        YESTOUCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Is 1 if a no-touch has been detected (ie. completed an integration cycle and found no-touch). This is not set when in low-power mode."]
    #[inline(always)]
    pub fn notouch(&self) -> NOTOUCH_R {
        NOTOUCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Is 1 if a poll or POLLNOW is complete."]
    #[inline(always)]
    pub fn polldone(&self) -> POLLDONE_R {
        POLLDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Is 1 if an integration cycle ended with a timeout (should not happen)."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Is 1 if new data was collected before application read out previous ISTOUCH. No-touch (ISTOUCH==0) data will be silently overrun. Is not possible if WAIT=1."]
    #[inline(always)]
    pub fn overun(&self) -> OVERUN_R {
        OVERUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - In a poll now."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Indicates the maximum number of X pins allowed 0-relative. So, 15 means there are pins 0 to 15, or 16 total X pins. INTERNAL note: this may be setup to be written by ROM boot."]
    #[inline(always)]
    pub fn xmax(&self) -> XMAX_R {
        XMAX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Is 1 if a touch has been detected, including a wakeup from low-power mode."]
    #[inline(always)]
    pub fn yestouch(&mut self) -> YESTOUCH_W {
        YESTOUCH_W { w: self }
    }
    #[doc = "Bit 1 - Is 1 if a no-touch has been detected (ie. completed an integration cycle and found no-touch). This is not set when in low-power mode."]
    #[inline(always)]
    pub fn notouch(&mut self) -> NOTOUCH_W {
        NOTOUCH_W { w: self }
    }
    #[doc = "Bit 2 - Is 1 if a poll or POLLNOW is complete."]
    #[inline(always)]
    pub fn polldone(&mut self) -> POLLDONE_W {
        POLLDONE_W { w: self }
    }
    #[doc = "Bit 3 - Is 1 if an integration cycle ended with a timeout (should not happen)."]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 4 - Is 1 if new data was collected before application read out previous ISTOUCH. No-touch (ISTOUCH==0) data will be silently overrun. Is not possible if WAIT=1."]
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
#[doc = "Status from triggers and time-outs including if in a poll now. Some are used for interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0x0008_0000"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0000
    }
}
