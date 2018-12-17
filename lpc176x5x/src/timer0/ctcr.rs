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
#[doc = "Possible values of the field `CTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTMODER {
    #[doc = "Timer Mode: every rising PCLK edge"]
    TIMER_MODE_EVERY_RI,
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    RISING,
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    FALLING,
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    DUALEDGE,
}
impl CTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTMODER::TIMER_MODE_EVERY_RI => 0,
            CTMODER::RISING => 1,
            CTMODER::FALLING => 2,
            CTMODER::DUALEDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTMODER {
        match value {
            0 => CTMODER::TIMER_MODE_EVERY_RI,
            1 => CTMODER::RISING,
            2 => CTMODER::FALLING,
            3 => CTMODER::DUALEDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE_EVERY_RI`"]
    #[inline]
    pub fn is_timer_mode_every_ri(&self) -> bool {
        *self == CTMODER::TIMER_MODE_EVERY_RI
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == CTMODER::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == CTMODER::FALLING
    }
    #[doc = "Checks if the value of the field is `DUALEDGE`"]
    #[inline]
    pub fn is_dualedge(&self) -> bool {
        *self == CTMODER::DUALEDGE
    }
}
#[doc = "Possible values of the field `CINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSELR {
    #[doc = "CAPn.0 for TIMERn"]
    CAPN_0_FOR_TIMERN,
    #[doc = "CAPn.1 for TIMERn"]
    CAPN_1_FOR_TIMERN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CINSELR::CAPN_0_FOR_TIMERN => 0,
            CINSELR::CAPN_1_FOR_TIMERN => 1,
            CINSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CINSELR {
        match value {
            0 => CINSELR::CAPN_0_FOR_TIMERN,
            1 => CINSELR::CAPN_1_FOR_TIMERN,
            i => CINSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CAPN_0_FOR_TIMERN`"]
    #[inline]
    pub fn is_capn_0_for_timern(&self) -> bool {
        *self == CINSELR::CAPN_0_FOR_TIMERN
    }
    #[doc = "Checks if the value of the field is `CAPN_1_FOR_TIMERN`"]
    #[inline]
    pub fn is_capn_1_for_timern(&self) -> bool {
        *self == CINSELR::CAPN_1_FOR_TIMERN
    }
}
#[doc = "Values that can be written to the field `CTMODE`"]
pub enum CTMODEW {
    #[doc = "Timer Mode: every rising PCLK edge"]
    TIMER_MODE_EVERY_RI,
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    RISING,
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    FALLING,
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    DUALEDGE,
}
impl CTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTMODEW::TIMER_MODE_EVERY_RI => 0,
            CTMODEW::RISING => 1,
            CTMODEW::FALLING => 2,
            CTMODEW::DUALEDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline]
    pub fn timer_mode_every_ri(self) -> &'a mut W {
        self.variant(CTMODEW::TIMER_MODE_EVERY_RI)
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(CTMODEW::RISING)
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(CTMODEW::FALLING)
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn dualedge(self) -> &'a mut W {
        self.variant(CTMODEW::DUALEDGE)
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
#[doc = "Values that can be written to the field `CINSEL`"]
pub enum CINSELW {
    #[doc = "CAPn.0 for TIMERn"]
    CAPN_0_FOR_TIMERN,
    #[doc = "CAPn.1 for TIMERn"]
    CAPN_1_FOR_TIMERN,
}
impl CINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CINSELW::CAPN_0_FOR_TIMERN => 0,
            CINSELW::CAPN_1_FOR_TIMERN => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CINSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CAPn.0 for TIMERn"]
    #[inline]
    pub fn capn_0_for_timern(self) -> &'a mut W {
        self.variant(CINSELW::CAPN_0_FOR_TIMERN)
    }
    #[doc = "CAPn.1 for TIMERn"]
    #[inline]
    pub fn capn_1_for_timern(self) -> &'a mut W {
        self.variant(CINSELW::CAPN_1_FOR_TIMERN)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline]
    pub fn ctmode(&self) -> CTMODER {
        CTMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline]
    pub fn cinsel(&self) -> CINSELR {
        CINSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
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
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline]
    pub fn ctmode(&mut self) -> _CTMODEW {
        _CTMODEW { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline]
    pub fn cinsel(&mut self) -> _CINSELW {
        _CINSELW { w: self }
    }
}
