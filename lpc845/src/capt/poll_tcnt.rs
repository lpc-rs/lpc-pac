#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POLL_TCNT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct TCNTR {
    bits: u16,
}
impl TCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TOUTR {
    bits: u8,
}
impl TOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct POLLR {
    bits: u8,
}
impl POLLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MDELAYR {
    bits: u8,
}
impl MDELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RDELAYR {
    bits: u8,
}
impl RDELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TCHLOW_ERR {
    bits: bool,
}
impl TCHLOW_ERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _TCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _TCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TOUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POLLW<'a> {
    w: &'a mut W,
}
impl<'a> _POLLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _MDELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _RDELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCHLOW_ERW<'a> {
    w: &'a mut W,
}
impl<'a> _TCHLOW_ERW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:11 - Sets the threshold between touch and no-touch count. If not used, then the block will treat all events as touch or no-touch, depending whether at max or min. This is in terms of divided FCLK. If the comparator triggers it is no-touch; if bigger than TCNT counts, it is a touch event."]
    #[inline]
    pub fn tcnt(&self) -> TCNTR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TCNTR { bits }
    }
    #[doc = "Bits 12:15 - Time-out count expressed as 1 is smaller than TOUT, allowing for up to 12 bits. Must be less than 13. So, for example, 1 is smaller than 12=4096 counts; if TOUT=12, then if 4096 counts occur without a trigger, it is a time-out. This should be set to be large enough above TCNT to prevent timeout invalidly."]
    #[inline]
    pub fn tout(&self) -> TOUTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TOUTR { bits }
    }
    #[doc = "Bits 16:23 - Poll counter in (internal) 12-bit counter wraparounds (loosely 1msec), so related to divided FCLK. This expresses time delay between measurement cycles (ie. after one set of X measurements, time before starting next). This count is used to delay before the next set of measurements. Measuring too often wastes power and does not add value since movement of fingers is relatively slow. For low power mode, this must allow for the clock being used (e.g. a 1MHz osc) so 12 bit count will be potentially much longer. That means, lowering the count to get the reasonable delay period."]
    #[inline]
    pub fn poll(&self) -> POLLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        POLLR { bits }
    }
    #[doc = "Bits 24:25 - If not 0, this selects the number of divided FCLKs to wait after entry of measurement mode before deciding if has triggered. This gives the ACMP time to react to the transferred charge. It is used as 1+(1 smaller than MDELAY), , so between 2 and 8 ticks of the divided FCLK added during the measurement."]
    #[inline]
    pub fn mdelay(&self) -> MDELAYR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MDELAYR { bits }
    }
    #[doc = "Bits 26:27 - If not 0, this is the number of divided FCLKs to hold in Step 0 'Reset' state (draining capacitance). It is used as (1 is smaller than RDELAY), so between 2 and 8 ticks of the divided FCLK added to the 'Reset' state."]
    #[inline]
    pub fn rdelay(&self) -> RDELAYR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDELAYR { bits }
    }
    #[doc = "Bit 31 - If 1, then the touch/no-touch boundary of TCNT is reversed. In a floating system (most common), the no-touch case triggers at a lower count vs. touch; this is due to touch drawing off charge. In a grounded system, the reverse is true and the touch adds to the charge and so touch is a lower count. In a system which can switch between grounded and non-grounded, the SW will check for all of the Xs looking like they have been touched and reverse the setting of this bit. This should only be changed between polls."]
    #[inline]
    pub fn tchlow_er(&self) -> TCHLOW_ERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCHLOW_ERR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - Sets the threshold between touch and no-touch count. If not used, then the block will treat all events as touch or no-touch, depending whether at max or min. This is in terms of divided FCLK. If the comparator triggers it is no-touch; if bigger than TCNT counts, it is a touch event."]
    #[inline]
    pub fn tcnt(&mut self) -> _TCNTW {
        _TCNTW { w: self }
    }
    #[doc = "Bits 12:15 - Time-out count expressed as 1 is smaller than TOUT, allowing for up to 12 bits. Must be less than 13. So, for example, 1 is smaller than 12=4096 counts; if TOUT=12, then if 4096 counts occur without a trigger, it is a time-out. This should be set to be large enough above TCNT to prevent timeout invalidly."]
    #[inline]
    pub fn tout(&mut self) -> _TOUTW {
        _TOUTW { w: self }
    }
    #[doc = "Bits 16:23 - Poll counter in (internal) 12-bit counter wraparounds (loosely 1msec), so related to divided FCLK. This expresses time delay between measurement cycles (ie. after one set of X measurements, time before starting next). This count is used to delay before the next set of measurements. Measuring too often wastes power and does not add value since movement of fingers is relatively slow. For low power mode, this must allow for the clock being used (e.g. a 1MHz osc) so 12 bit count will be potentially much longer. That means, lowering the count to get the reasonable delay period."]
    #[inline]
    pub fn poll(&mut self) -> _POLLW {
        _POLLW { w: self }
    }
    #[doc = "Bits 24:25 - If not 0, this selects the number of divided FCLKs to wait after entry of measurement mode before deciding if has triggered. This gives the ACMP time to react to the transferred charge. It is used as 1+(1 smaller than MDELAY), , so between 2 and 8 ticks of the divided FCLK added during the measurement."]
    #[inline]
    pub fn mdelay(&mut self) -> _MDELAYW {
        _MDELAYW { w: self }
    }
    #[doc = "Bits 26:27 - If not 0, this is the number of divided FCLKs to hold in Step 0 'Reset' state (draining capacitance). It is used as (1 is smaller than RDELAY), so between 2 and 8 ticks of the divided FCLK added to the 'Reset' state."]
    #[inline]
    pub fn rdelay(&mut self) -> _RDELAYW {
        _RDELAYW { w: self }
    }
    #[doc = "Bit 31 - If 1, then the touch/no-touch boundary of TCNT is reversed. In a floating system (most common), the no-touch case triggers at a lower count vs. touch; this is due to touch drawing off charge. In a grounded system, the reverse is true and the touch adds to the charge and so touch is a lower count. In a system which can switch between grounded and non-grounded, the SW will check for all of the Xs looking like they have been touched and reverse the setting of this bit. This should only be changed between polls."]
    #[inline]
    pub fn tchlow_er(&mut self) -> _TCHLOW_ERW {
        _TCHLOW_ERW { w: self }
    }
}
