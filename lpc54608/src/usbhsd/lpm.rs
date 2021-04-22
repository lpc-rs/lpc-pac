#[doc = "Register `LPM` reader"]
pub struct R(crate::R<LPM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LPM_SPEC>> for R {
    fn from(reader: crate::R<LPM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPM` writer"]
pub struct W(crate::W<LPM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPM_SPEC>;
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
impl core::convert::From<crate::W<LPM_SPEC>> for W {
    fn from(writer: crate::W<LPM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIRD_HW` reader - Host Initiated Resume Duration - HW."]
pub struct HIRD_HW_R(crate::FieldReader<u8, u8>);
impl HIRD_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        HIRD_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIRD_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIRD_SW` reader - Host Initiated Resume Duration - SW."]
pub struct HIRD_SW_R(crate::FieldReader<u8, u8>);
impl HIRD_SW_R {
    pub(crate) fn new(bits: u8) -> Self {
        HIRD_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIRD_SW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIRD_SW` writer - Host Initiated Resume Duration - SW."]
pub struct HIRD_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRD_SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DATA_PENDING` reader - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
pub struct DATA_PENDING_R(crate::FieldReader<bool, bool>);
impl DATA_PENDING_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_PENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_PENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_PENDING` writer - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
pub struct DATA_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_PENDING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Host Initiated Resume Duration - HW."]
    #[inline(always)]
    pub fn hird_hw(&self) -> HIRD_HW_R {
        HIRD_HW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW."]
    #[inline(always)]
    pub fn hird_sw(&self) -> HIRD_SW_R {
        HIRD_SW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[inline(always)]
    pub fn data_pending(&self) -> DATA_PENDING_R {
        DATA_PENDING_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW."]
    #[inline(always)]
    pub fn hird_sw(&mut self) -> HIRD_SW_W {
        HIRD_SW_W { w: self }
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[inline(always)]
    pub fn data_pending(&mut self) -> DATA_PENDING_W {
        DATA_PENDING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Link Power Management register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpm](index.html) module"]
pub struct LPM_SPEC;
impl crate::RegisterSpec for LPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpm::R](R) reader structure"]
impl crate::Readable for LPM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpm::W](W) writer structure"]
impl crate::Writable for LPM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPM to value 0"]
impl crate::Resettable for LPM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
