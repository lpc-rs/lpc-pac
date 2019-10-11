#[doc = "Reader of register POLL_TCNT"]
pub type R = crate::R<u32, super::POLL_TCNT>;
#[doc = "Writer for register POLL_TCNT"]
pub type W = crate::W<u32, super::POLL_TCNT>;
#[doc = "Register POLL_TCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::POLL_TCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCNT`"]
pub type TCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TCNT`"]
pub struct TCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `TOUT`"]
pub type TOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOUT`"]
pub struct TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `POLL`"]
pub type POLL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POLL`"]
pub struct POLL_W<'a> {
    w: &'a mut W,
}
impl<'a> POLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `MDELAY`"]
pub type MDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MDELAY`"]
pub struct MDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> MDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `RDELAY`"]
pub type RDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDELAY`"]
pub struct RDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `TCHLOW_ER`"]
pub type TCHLOW_ER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCHLOW_ER`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
