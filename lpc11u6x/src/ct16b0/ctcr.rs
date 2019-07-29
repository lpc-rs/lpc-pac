#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTCR {
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
#[doc = "Possible values of the field `CTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTMR {
    #[doc = "Timer Mode.  Increments every rising PCLK edge"]
    TIMER_MODE,
    #[doc = "Counter Moderising edge. . TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    RISING,
    #[doc = "Counter Mode falling edge: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    FALLING,
    #[doc = "Counter Mode dual edge: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    DUAL,
}
impl CTMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTMR::TIMER_MODE => 0,
            CTMR::RISING => 1,
            CTMR::FALLING => 2,
            CTMR::DUAL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTMR {
        match value {
            0 => CTMR::TIMER_MODE,
            1 => CTMR::RISING,
            2 => CTMR::FALLING,
            3 => CTMR::DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE`"]
    #[inline]
    pub fn is_timer_mode(&self) -> bool {
        *self == CTMR::TIMER_MODE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == CTMR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == CTMR::FALLING
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline]
    pub fn is_dual(&self) -> bool {
        *self == CTMR::DUAL
    }
}
#[doc = "Possible values of the field `CIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CISR {
    #[doc = "Capture channel 0."]
    CAPTURE_CHANNEL_0,
    #[doc = "Capture channel 1."]
    CAPTURE_CHANNEL_1,
    #[doc = "Capture channel 2."]
    CAPTURE_CHANNEL_2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CISR::CAPTURE_CHANNEL_0 => 0,
            CISR::CAPTURE_CHANNEL_1 => 1,
            CISR::CAPTURE_CHANNEL_2 => 2,
            CISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CISR {
        match value {
            0 => CISR::CAPTURE_CHANNEL_0,
            1 => CISR::CAPTURE_CHANNEL_1,
            2 => CISR::CAPTURE_CHANNEL_2,
            i => CISR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTURE_CHANNEL_0`"]
    #[inline]
    pub fn is_capture_channel_0(&self) -> bool {
        *self == CISR::CAPTURE_CHANNEL_0
    }
    #[doc = "Checks if the value of the field is `CAPTURE_CHANNEL_1`"]
    #[inline]
    pub fn is_capture_channel_1(&self) -> bool {
        *self == CISR::CAPTURE_CHANNEL_1
    }
    #[doc = "Checks if the value of the field is `CAPTURE_CHANNEL_2`"]
    #[inline]
    pub fn is_capture_channel_2(&self) -> bool {
        *self == CISR::CAPTURE_CHANNEL_2
    }
}
#[doc = r" Value of the field"]
pub struct ENCCR {
    bits: bool,
}
impl ENCCR {
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
#[doc = "Possible values of the field `SELCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELCCR {
    #[doc = "Rising Edge of thesignal on capture channel 0 clears the timer (if bit 4 is set)."]
    CAP0RISING,
    #[doc = "Falling Edge of thesignal on capture channel 0 clears the timer (if bit 4 is set)."]
    CAP0FALLING,
    #[doc = "Rising Edge of thesignal on capture channel 1 clears the timer (if bit 4 is set)."]
    CAP1RISING,
    #[doc = "Falling Edge of thesignal on capture channel 1 clears the timer (if bit 4 is set)."]
    CAP1FALLING,
    #[doc = "Rising Edge of thesignal on capture channel 2 clears the timer (if bit 4 is set)."]
    CAP2RISING,
    #[doc = "Falling Edge of thesignal on capture channel 2 clears the timer (if bit 4 is set)."]
    CAP1FALLING,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SELCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELCCR::CAP0RISING => 0,
            SELCCR::CAP0FALLING => 1,
            SELCCR::CAP1RISING => 2,
            SELCCR::CAP1FALLING => 3,
            SELCCR::CAP2RISING => 4,
            SELCCR::CAP1FALLING => 5,
            SELCCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELCCR {
        match value {
            0 => SELCCR::CAP0RISING,
            1 => SELCCR::CAP0FALLING,
            2 => SELCCR::CAP1RISING,
            3 => SELCCR::CAP1FALLING,
            4 => SELCCR::CAP2RISING,
            5 => SELCCR::CAP1FALLING,
            i => SELCCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAP0RISING`"]
    #[inline]
    pub fn is_cap0rising(&self) -> bool {
        *self == SELCCR::CAP0RISING
    }
    #[doc = "Checks if the value of the field is `CAP0FALLING`"]
    #[inline]
    pub fn is_cap0falling(&self) -> bool {
        *self == SELCCR::CAP0FALLING
    }
    #[doc = "Checks if the value of the field is `CAP1RISING`"]
    #[inline]
    pub fn is_cap1rising(&self) -> bool {
        *self == SELCCR::CAP1RISING
    }
    #[doc = "Checks if the value of the field is `CAP1FALLING`"]
    #[inline]
    pub fn is_cap1falling(&self) -> bool {
        *self == SELCCR::CAP1FALLING
    }
    #[doc = "Checks if the value of the field is `CAP2RISING`"]
    #[inline]
    pub fn is_cap2rising(&self) -> bool {
        *self == SELCCR::CAP2RISING
    }
    #[doc = "Checks if the value of the field is `CAP1FALLING`"]
    #[inline]
    pub fn is_cap1falling(&self) -> bool {
        *self == SELCCR::CAP1FALLING
    }
}
#[doc = "Values that can be written to the field `CTM`"]
pub enum CTMW {
    #[doc = "Timer Mode.  Increments every rising PCLK edge"]
    TIMER_MODE,
    #[doc = "Counter Moderising edge. . TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    RISING,
    #[doc = "Counter Mode falling edge: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    FALLING,
    #[doc = "Counter Mode dual edge: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    DUAL,
}
impl CTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTMW::TIMER_MODE => 0,
            CTMW::RISING => 1,
            CTMW::FALLING => 2,
            CTMW::DUAL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTMW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer Mode. Increments every rising PCLK edge"]
    #[inline]
    pub fn timer_mode(self) -> &'a mut W {
        self.variant(CTMW::TIMER_MODE)
    }
    #[doc = "Counter Moderising edge. . TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(CTMW::RISING)
    }
    #[doc = "Counter Mode falling edge: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(CTMW::FALLING)
    }
    #[doc = "Counter Mode dual edge: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn dual(self) -> &'a mut W {
        self.variant(CTMW::DUAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CIS`"]
pub enum CISW {
    #[doc = "Capture channel 0."]
    CAPTURE_CHANNEL_0,
    #[doc = "Capture channel 1."]
    CAPTURE_CHANNEL_1,
    #[doc = "Capture channel 2."]
    CAPTURE_CHANNEL_2,
}
impl CISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CISW::CAPTURE_CHANNEL_0 => 0,
            CISW::CAPTURE_CHANNEL_1 => 1,
            CISW::CAPTURE_CHANNEL_2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CISW<'a> {
    w: &'a mut W,
}
impl<'a> _CISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Capture channel 0."]
    #[inline]
    pub fn capture_channel_0(self) -> &'a mut W {
        self.variant(CISW::CAPTURE_CHANNEL_0)
    }
    #[doc = "Capture channel 1."]
    #[inline]
    pub fn capture_channel_1(self) -> &'a mut W {
        self.variant(CISW::CAPTURE_CHANNEL_1)
    }
    #[doc = "Capture channel 2."]
    #[inline]
    pub fn capture_channel_2(self) -> &'a mut W {
        self.variant(CISW::CAPTURE_CHANNEL_2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENCCW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCCW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELCC`"]
pub enum SELCCW {
    #[doc = "Rising Edge of thesignal on capture channel 0 clears the timer (if bit 4 is set)."]
    CAP0RISING,
    #[doc = "Falling Edge of thesignal on capture channel 0 clears the timer (if bit 4 is set)."]
    CAP0FALLING,
    #[doc = "Rising Edge of thesignal on capture channel 1 clears the timer (if bit 4 is set)."]
    CAP1RISING,
    #[doc = "Falling Edge of thesignal on capture channel 1 clears the timer (if bit 4 is set)."]
    CAP1FALLING,
    #[doc = "Rising Edge of thesignal on capture channel 2 clears the timer (if bit 4 is set)."]
    CAP2RISING,
    #[doc = "Falling Edge of thesignal on capture channel 2 clears the timer (if bit 4 is set)."]
    CAP1FALLING,
}
impl SELCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELCCW::CAP0RISING => 0,
            SELCCW::CAP0FALLING => 1,
            SELCCW::CAP1RISING => 2,
            SELCCW::CAP1FALLING => 3,
            SELCCW::CAP2RISING => 4,
            SELCCW::CAP1FALLING => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELCCW<'a> {
    w: &'a mut W,
}
impl<'a> _SELCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELCCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Rising Edge of thesignal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn cap0rising(self) -> &'a mut W {
        self.variant(SELCCW::CAP0RISING)
    }
    #[doc = "Falling Edge of thesignal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn cap0falling(self) -> &'a mut W {
        self.variant(SELCCW::CAP0FALLING)
    }
    #[doc = "Rising Edge of thesignal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn cap1rising(self) -> &'a mut W {
        self.variant(SELCCW::CAP1RISING)
    }
    #[doc = "Falling Edge of thesignal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn cap1falling(self) -> &'a mut W {
        self.variant(SELCCW::CAP1FALLING)
    }
    #[doc = "Rising Edge of thesignal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn cap2rising(self) -> &'a mut W {
        self.variant(SELCCW::CAP2RISING)
    }
    #[doc = "Falling Edge of thesignal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn cap1falling(self) -> &'a mut W {
        self.variant(SELCCW::CAP1FALLING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
    #[inline]
    pub fn ctm(&self) -> CTMR {
        CTMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin is sampled for clocking. Value 0x3 isreserved."]
    #[inline]
    pub fn cis(&self) -> CISR {
        CISR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline]
    pub fn encc(&self) -> ENCCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENCCR { bits }
    }
    #[doc = "Bits 5:7 - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline]
    pub fn selcc(&self) -> SELCCR {
        SELCCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - Counter/Timer Mode. This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). If Counter mode is selected in the CTCR, bits 2:0 in the Capture Control Register (CCR) must be programmed as 000."]
    #[inline]
    pub fn ctm(&mut self) -> _CTMW {
        _CTMW { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select. In counter mode (when bits 1:0 in this register are not 00), these bits select which CAP pin is sampled for clocking. Value 0x3 isreserved."]
    #[inline]
    pub fn cis(&mut self) -> _CISW {
        _CISW { w: self }
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline]
    pub fn encc(&mut self) -> _ENCCW {
        _ENCCW { w: self }
    }
    #[doc = "Bits 5:7 - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline]
    pub fn selcc(&mut self) -> _SELCCW {
        _SELCCW { w: self }
    }
}
