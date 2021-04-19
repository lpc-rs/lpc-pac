#[doc = "Register `POLL_TCNT` reader"]
pub struct R(crate::R<POLL_TCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POLL_TCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<POLL_TCNT_SPEC>> for R {
    fn from(reader: crate::R<POLL_TCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POLL_TCNT` writer"]
pub struct W(crate::W<POLL_TCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POLL_TCNT_SPEC>;
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
impl core::convert::From<crate::W<POLL_TCNT_SPEC>> for W {
    fn from(writer: crate::W<POLL_TCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCNT` reader - Sets the threshold between touch and no-touch count. If not used, then the block will treat all events as touch or no-touch, depending whether at max or min. This is in terms of divided FCLK. If the comparator triggers it is no-touch; if bigger than TCNT counts, it is a touch event."]
pub struct TCNT_R(crate::FieldReader<u16, u16>);
impl TCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCNT` writer - Sets the threshold between touch and no-touch count. If not used, then the block will treat all events as touch or no-touch, depending whether at max or min. This is in terms of divided FCLK. If the comparator triggers it is no-touch; if bigger than TCNT counts, it is a touch event."]
pub struct TCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `TOUT` reader - Time-out count expressed as 1 is smaller than TOUT, allowing for up to 12 bits. Must be less than 13. So, for example, 1 is smaller than 12=4096 counts; if TOUT=12, then if 4096 counts occur without a trigger, it is a time-out. This should be set to be large enough above TCNT to prevent timeout invalidly."]
pub struct TOUT_R(crate::FieldReader<u8, u8>);
impl TOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUT` writer - Time-out count expressed as 1 is smaller than TOUT, allowing for up to 12 bits. Must be less than 13. So, for example, 1 is smaller than 12=4096 counts; if TOUT=12, then if 4096 counts occur without a trigger, it is a time-out. This should be set to be large enough above TCNT to prevent timeout invalidly."]
pub struct TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `POLL` reader - Poll counter in (internal) 12-bit counter wraparounds (loosely 1msec), so related to divided FCLK. This expresses time delay between measurement cycles (ie. after one set of X measurements, time before starting next). This count is used to delay before the next set of measurements. Measuring too often wastes power and does not add value since movement of fingers is relatively slow. For low power mode, this must allow for the clock being used (e.g. a 1MHz osc) so 12 bit count will be potentially much longer. That means, lowering the count to get the reasonable delay period."]
pub struct POLL_R(crate::FieldReader<u8, u8>);
impl POLL_R {
    pub(crate) fn new(bits: u8) -> Self {
        POLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLL` writer - Poll counter in (internal) 12-bit counter wraparounds (loosely 1msec), so related to divided FCLK. This expresses time delay between measurement cycles (ie. after one set of X measurements, time before starting next). This count is used to delay before the next set of measurements. Measuring too often wastes power and does not add value since movement of fingers is relatively slow. For low power mode, this must allow for the clock being used (e.g. a 1MHz osc) so 12 bit count will be potentially much longer. That means, lowering the count to get the reasonable delay period."]
pub struct POLL_W<'a> {
    w: &'a mut W,
}
impl<'a> POLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `MDELAY` reader - If not 0, this selects the number of divided FCLKs to wait after entry of measurement mode before deciding if has triggered. This gives the ACMP time to react to the transferred charge. It is used as 1+(1 smaller than MDELAY), , so between 2 and 8 ticks of the divided FCLK added during the measurement."]
pub struct MDELAY_R(crate::FieldReader<u8, u8>);
impl MDELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        MDELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDELAY` writer - If not 0, this selects the number of divided FCLKs to wait after entry of measurement mode before deciding if has triggered. This gives the ACMP time to react to the transferred charge. It is used as 1+(1 smaller than MDELAY), , so between 2 and 8 ticks of the divided FCLK added during the measurement."]
pub struct MDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> MDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `RDELAY` reader - If not 0, this is the number of divided FCLKs to hold in Step 0 'Reset' state (draining capacitance). It is used as (1 is smaller than RDELAY), so between 2 and 8 ticks of the divided FCLK added to the 'Reset' state."]
pub struct RDELAY_R(crate::FieldReader<u8, u8>);
impl RDELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDELAY` writer - If not 0, this is the number of divided FCLKs to hold in Step 0 'Reset' state (draining capacitance). It is used as (1 is smaller than RDELAY), so between 2 and 8 ticks of the divided FCLK added to the 'Reset' state."]
pub struct RDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `TCHLOW_ER` reader - If 1, then the touch/no-touch boundary of TCNT is reversed. In a floating system (most common), the no-touch case triggers at a lower count vs. touch; this is due to touch drawing off charge. In a grounded system, the reverse is true and the touch adds to the charge and so touch is a lower count. In a system which can switch between grounded and non-grounded, the SW will check for all of the Xs looking like they have been touched and reverse the setting of this bit. This should only be changed between polls."]
pub struct TCHLOW_ER_R(crate::FieldReader<bool, bool>);
impl TCHLOW_ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCHLOW_ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCHLOW_ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCHLOW_ER` writer - If 1, then the touch/no-touch boundary of TCNT is reversed. In a floating system (most common), the no-touch case triggers at a lower count vs. touch; this is due to touch drawing off charge. In a grounded system, the reverse is true and the touch adds to the charge and so touch is a lower count. In a system which can switch between grounded and non-grounded, the SW will check for all of the Xs looking like they have been touched and reverse the setting of this bit. This should only be changed between polls."]
pub struct TCHLOW_ER_W<'a> {
    w: &'a mut W,
}
impl<'a> TCHLOW_ER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Sets the threshold between touch and no-touch count. If not used, then the block will treat all events as touch or no-touch, depending whether at max or min. This is in terms of divided FCLK. If the comparator triggers it is no-touch; if bigger than TCNT counts, it is a touch event."]
    #[inline(always)]
    pub fn tcnt(&self) -> TCNT_R {
        TCNT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Time-out count expressed as 1 is smaller than TOUT, allowing for up to 12 bits. Must be less than 13. So, for example, 1 is smaller than 12=4096 counts; if TOUT=12, then if 4096 counts occur without a trigger, it is a time-out. This should be set to be large enough above TCNT to prevent timeout invalidly."]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Poll counter in (internal) 12-bit counter wraparounds (loosely 1msec), so related to divided FCLK. This expresses time delay between measurement cycles (ie. after one set of X measurements, time before starting next). This count is used to delay before the next set of measurements. Measuring too often wastes power and does not add value since movement of fingers is relatively slow. For low power mode, this must allow for the clock being used (e.g. a 1MHz osc) so 12 bit count will be potentially much longer. That means, lowering the count to get the reasonable delay period."]
    #[inline(always)]
    pub fn poll(&self) -> POLL_R {
        POLL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - If not 0, this selects the number of divided FCLKs to wait after entry of measurement mode before deciding if has triggered. This gives the ACMP time to react to the transferred charge. It is used as 1+(1 smaller than MDELAY), , so between 2 and 8 ticks of the divided FCLK added during the measurement."]
    #[inline(always)]
    pub fn mdelay(&self) -> MDELAY_R {
        MDELAY_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - If not 0, this is the number of divided FCLKs to hold in Step 0 'Reset' state (draining capacitance). It is used as (1 is smaller than RDELAY), so between 2 and 8 ticks of the divided FCLK added to the 'Reset' state."]
    #[inline(always)]
    pub fn rdelay(&self) -> RDELAY_R {
        RDELAY_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 31 - If 1, then the touch/no-touch boundary of TCNT is reversed. In a floating system (most common), the no-touch case triggers at a lower count vs. touch; this is due to touch drawing off charge. In a grounded system, the reverse is true and the touch adds to the charge and so touch is a lower count. In a system which can switch between grounded and non-grounded, the SW will check for all of the Xs looking like they have been touched and reverse the setting of this bit. This should only be changed between polls."]
    #[inline(always)]
    pub fn tchlow_er(&self) -> TCHLOW_ER_R {
        TCHLOW_ER_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Sets the threshold between touch and no-touch count. If not used, then the block will treat all events as touch or no-touch, depending whether at max or min. This is in terms of divided FCLK. If the comparator triggers it is no-touch; if bigger than TCNT counts, it is a touch event."]
    #[inline(always)]
    pub fn tcnt(&mut self) -> TCNT_W {
        TCNT_W { w: self }
    }
    #[doc = "Bits 12:15 - Time-out count expressed as 1 is smaller than TOUT, allowing for up to 12 bits. Must be less than 13. So, for example, 1 is smaller than 12=4096 counts; if TOUT=12, then if 4096 counts occur without a trigger, it is a time-out. This should be set to be large enough above TCNT to prevent timeout invalidly."]
    #[inline(always)]
    pub fn tout(&mut self) -> TOUT_W {
        TOUT_W { w: self }
    }
    #[doc = "Bits 16:23 - Poll counter in (internal) 12-bit counter wraparounds (loosely 1msec), so related to divided FCLK. This expresses time delay between measurement cycles (ie. after one set of X measurements, time before starting next). This count is used to delay before the next set of measurements. Measuring too often wastes power and does not add value since movement of fingers is relatively slow. For low power mode, this must allow for the clock being used (e.g. a 1MHz osc) so 12 bit count will be potentially much longer. That means, lowering the count to get the reasonable delay period."]
    #[inline(always)]
    pub fn poll(&mut self) -> POLL_W {
        POLL_W { w: self }
    }
    #[doc = "Bits 24:25 - If not 0, this selects the number of divided FCLKs to wait after entry of measurement mode before deciding if has triggered. This gives the ACMP time to react to the transferred charge. It is used as 1+(1 smaller than MDELAY), , so between 2 and 8 ticks of the divided FCLK added during the measurement."]
    #[inline(always)]
    pub fn mdelay(&mut self) -> MDELAY_W {
        MDELAY_W { w: self }
    }
    #[doc = "Bits 26:27 - If not 0, this is the number of divided FCLKs to hold in Step 0 'Reset' state (draining capacitance). It is used as (1 is smaller than RDELAY), so between 2 and 8 ticks of the divided FCLK added to the 'Reset' state."]
    #[inline(always)]
    pub fn rdelay(&mut self) -> RDELAY_W {
        RDELAY_W { w: self }
    }
    #[doc = "Bit 31 - If 1, then the touch/no-touch boundary of TCNT is reversed. In a floating system (most common), the no-touch case triggers at a lower count vs. touch; this is due to touch drawing off charge. In a grounded system, the reverse is true and the touch adds to the charge and so touch is a lower count. In a system which can switch between grounded and non-grounded, the SW will check for all of the Xs looking like they have been touched and reverse the setting of this bit. This should only be changed between polls."]
    #[inline(always)]
    pub fn tchlow_er(&mut self) -> TCHLOW_ER_W {
        TCHLOW_ER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This sets up the polling counter and measurement counter rules.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poll_tcnt](index.html) module"]
pub struct POLL_TCNT_SPEC;
impl crate::RegisterSpec for POLL_TCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poll_tcnt::R](R) reader structure"]
impl crate::Readable for POLL_TCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poll_tcnt::W](W) writer structure"]
impl crate::Writable for POLL_TCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POLL_TCNT to value 0"]
impl crate::Resettable for POLL_TCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
