#[doc = "Register `TOUCH` reader"]
pub struct R(crate::R<TOUCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH` writer"]
pub struct W(crate::W<TOUCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_SPEC>;
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
impl From<crate::W<TOUCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Count value reached at trigger. If timeout, will be (1 bigger than TOUT)-1; e.g. if TOUT=12, then 0xFFF."]
pub struct COUNT_R(crate::FieldReader<u16, u16>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XVAL` reader - Is the X that triggered this, or lowest X if more than one."]
pub struct XVAL_R(crate::FieldReader<u8, u8>);
impl XVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISTOUCH` reader - 1 if is Touch (by count) or 0 if is no-touch."]
pub struct ISTOUCH_R(crate::FieldReader<bool, bool>);
impl ISTOUCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISTOUCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISTOUCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISTO` reader - 1 if is Timeout."]
pub struct ISTO_R(crate::FieldReader<bool, bool>);
impl ISTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQ` reader - Sequence number - rolling counter of polls. Changes after all selected Xs per poll (so, 0 for 1st set of Xs, then 1 for next set, etc)."]
pub struct SEQ_R(crate::FieldReader<u8, u8>);
impl SEQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANGE` reader - If 1, the rest of the register is 0 because the data is changing. This will only happen for 1 cycle and would never happen if using interrupts to read, unless took so long as to overrun."]
pub struct CHANGE_R(crate::FieldReader<bool, bool>);
impl CHANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHANGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Count value reached at trigger. If timeout, will be (1 bigger than TOUT)-1; e.g. if TOUT=12, then 0xFFF."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Is the X that triggered this, or lowest X if more than one."]
    #[inline(always)]
    pub fn xval(&self) -> XVAL_R {
        XVAL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 1 if is Touch (by count) or 0 if is no-touch."]
    #[inline(always)]
    pub fn istouch(&self) -> ISTOUCH_R {
        ISTOUCH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 1 if is Timeout."]
    #[inline(always)]
    pub fn isto(&self) -> ISTO_R {
        ISTO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Sequence number - rolling counter of polls. Changes after all selected Xs per poll (so, 0 for 1st set of Xs, then 1 for next set, etc)."]
    #[inline(always)]
    pub fn seq(&self) -> SEQ_R {
        SEQ_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - If 1, the rest of the register is 0 because the data is changing. This will only happen for 1 cycle and would never happen if using interrupts to read, unless took so long as to overrun."]
    #[inline(always)]
    pub fn change(&self) -> CHANGE_R {
        CHANGE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Last touch event (touch or no-touch) in context.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch](index.html) module"]
pub struct TOUCH_SPEC;
impl crate::RegisterSpec for TOUCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch::R](R) reader structure"]
impl crate::Readable for TOUCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch::W](W) writer structure"]
impl crate::Writable for TOUCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOUCH to value 0"]
impl crate::Resettable for TOUCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
