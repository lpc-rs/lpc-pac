#[doc = "Register `TMOUT` reader"]
pub struct R(crate::R<TMOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMOUT` writer"]
pub struct W(crate::W<TMOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMOUT_SPEC>;
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
impl From<crate::W<TMOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESPONSE_TIMEOUT` reader - Response time-out value."]
pub struct RESPONSE_TIMEOUT_R(crate::FieldReader<u8, u8>);
impl RESPONSE_TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RESPONSE_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE_TIMEOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESPONSE_TIMEOUT` writer - Response time-out value."]
pub struct RESPONSE_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DATA_TIMEOUT` reader - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
pub struct DATA_TIMEOUT_R(crate::FieldReader<u32, u32>);
impl DATA_TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_TIMEOUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_TIMEOUT` writer - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
pub struct DATA_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Response time-out value."]
    #[inline(always)]
    pub fn response_timeout(&self) -> RESPONSE_TIMEOUT_R {
        RESPONSE_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
    #[inline(always)]
    pub fn data_timeout(&self) -> DATA_TIMEOUT_R {
        DATA_TIMEOUT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:7 - Response time-out value."]
    #[inline(always)]
    pub fn response_timeout(&mut self) -> RESPONSE_TIMEOUT_W {
        RESPONSE_TIMEOUT_W { w: self }
    }
    #[doc = "Bits 8:31 - Value for card Data Read time-out; same value also used for Data Starvation by Host time-out."]
    #[inline(always)]
    pub fn data_timeout(&mut self) -> DATA_TIMEOUT_W {
        DATA_TIMEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time-out register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmout](index.html) module"]
pub struct TMOUT_SPEC;
impl crate::RegisterSpec for TMOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmout::R](R) reader structure"]
impl crate::Readable for TMOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmout::W](W) writer structure"]
impl crate::Writable for TMOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMOUT to value 0xffff_ff40"]
impl crate::Resettable for TMOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ff40
    }
}
