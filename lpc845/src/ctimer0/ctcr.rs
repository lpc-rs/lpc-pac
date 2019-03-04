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
    #[doc = "Timer Mode. Incremented every rising APB bus clock edge."]
    TIMER,
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    COUNTER_RISING_EDGE,
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    COUNTER_FALLING_EDGE,
    #[doc = "Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    COUNTER_DUAL_EDGE,
}
impl CTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTMODER::TIMER => 0,
            CTMODER::COUNTER_RISING_EDGE => 1,
            CTMODER::COUNTER_FALLING_EDGE => 2,
            CTMODER::COUNTER_DUAL_EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTMODER {
        match value {
            0 => CTMODER::TIMER,
            1 => CTMODER::COUNTER_RISING_EDGE,
            2 => CTMODER::COUNTER_FALLING_EDGE,
            3 => CTMODER::COUNTER_DUAL_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline]
    pub fn is_timer(&self) -> bool {
        *self == CTMODER::TIMER
    }
    #[doc = "Checks if the value of the field is `COUNTER_RISING_EDGE`"]
    #[inline]
    pub fn is_counter_rising_edge(&self) -> bool {
        *self == CTMODER::COUNTER_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `COUNTER_FALLING_EDGE`"]
    #[inline]
    pub fn is_counter_falling_edge(&self) -> bool {
        *self == CTMODER::COUNTER_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `COUNTER_DUAL_EDGE`"]
    #[inline]
    pub fn is_counter_dual_edge(&self) -> bool {
        *self == CTMODER::COUNTER_DUAL_EDGE
    }
}
#[doc = "Possible values of the field `CINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSELR {
    #[doc = "Channel 0. CAPn.0 for CTIMERn"]
    CHANNEL_0,
    #[doc = "Channel 1. CAPn.1 for CTIMERn"]
    CHANNEL_1,
    #[doc = "Channel 2. CAPn.2 for CTIMERn"]
    CHANNEL_2,
    #[doc = "Channel 3. CAPn.3 for CTIMERn"]
    CHANNEL_3,
}
impl CINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CINSELR::CHANNEL_0 => 0,
            CINSELR::CHANNEL_1 => 1,
            CINSELR::CHANNEL_2 => 2,
            CINSELR::CHANNEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CINSELR {
        match value {
            0 => CINSELR::CHANNEL_0,
            1 => CINSELR::CHANNEL_1,
            2 => CINSELR::CHANNEL_2,
            3 => CINSELR::CHANNEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0`"]
    #[inline]
    pub fn is_channel_0(&self) -> bool {
        *self == CINSELR::CHANNEL_0
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1`"]
    #[inline]
    pub fn is_channel_1(&self) -> bool {
        *self == CINSELR::CHANNEL_1
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2`"]
    #[inline]
    pub fn is_channel_2(&self) -> bool {
        *self == CINSELR::CHANNEL_2
    }
    #[doc = "Checks if the value of the field is `CHANNEL_3`"]
    #[inline]
    pub fn is_channel_3(&self) -> bool {
        *self == CINSELR::CHANNEL_3
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
    #[doc = "Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    CHANNEL_0_RISING,
    #[doc = "Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    CHANNEL_0_FALLING,
    #[doc = "Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    CHANNEL_1_RISING,
    #[doc = "Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    CHANNEL_1_FALLING,
    #[doc = "Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    CHANNEL_2_RISING,
    #[doc = "Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    CHANNEL_2_FALLING,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SELCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELCCR::CHANNEL_0_RISING => 0,
            SELCCR::CHANNEL_0_FALLING => 1,
            SELCCR::CHANNEL_1_RISING => 2,
            SELCCR::CHANNEL_1_FALLING => 3,
            SELCCR::CHANNEL_2_RISING => 4,
            SELCCR::CHANNEL_2_FALLING => 5,
            SELCCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELCCR {
        match value {
            0 => SELCCR::CHANNEL_0_RISING,
            1 => SELCCR::CHANNEL_0_FALLING,
            2 => SELCCR::CHANNEL_1_RISING,
            3 => SELCCR::CHANNEL_1_FALLING,
            4 => SELCCR::CHANNEL_2_RISING,
            5 => SELCCR::CHANNEL_2_FALLING,
            i => SELCCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_RISING`"]
    #[inline]
    pub fn is_channel_0_rising(&self) -> bool {
        *self == SELCCR::CHANNEL_0_RISING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_FALLING`"]
    #[inline]
    pub fn is_channel_0_falling(&self) -> bool {
        *self == SELCCR::CHANNEL_0_FALLING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_RISING`"]
    #[inline]
    pub fn is_channel_1_rising(&self) -> bool {
        *self == SELCCR::CHANNEL_1_RISING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_FALLING`"]
    #[inline]
    pub fn is_channel_1_falling(&self) -> bool {
        *self == SELCCR::CHANNEL_1_FALLING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_RISING`"]
    #[inline]
    pub fn is_channel_2_rising(&self) -> bool {
        *self == SELCCR::CHANNEL_2_RISING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_FALLING`"]
    #[inline]
    pub fn is_channel_2_falling(&self) -> bool {
        *self == SELCCR::CHANNEL_2_FALLING
    }
}
#[doc = "Values that can be written to the field `CTMODE`"]
pub enum CTMODEW {
    #[doc = "Timer Mode. Incremented every rising APB bus clock edge."]
    TIMER,
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    COUNTER_RISING_EDGE,
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    COUNTER_FALLING_EDGE,
    #[doc = "Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    COUNTER_DUAL_EDGE,
}
impl CTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTMODEW::TIMER => 0,
            CTMODEW::COUNTER_RISING_EDGE => 1,
            CTMODEW::COUNTER_FALLING_EDGE => 2,
            CTMODEW::COUNTER_DUAL_EDGE => 3,
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
    #[doc = "Timer Mode. Incremented every rising APB bus clock edge."]
    #[inline]
    pub fn timer(self) -> &'a mut W {
        self.variant(CTMODEW::TIMER)
    }
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn counter_rising_edge(self) -> &'a mut W {
        self.variant(CTMODEW::COUNTER_RISING_EDGE)
    }
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn counter_falling_edge(self) -> &'a mut W {
        self.variant(CTMODEW::COUNTER_FALLING_EDGE)
    }
    #[doc = "Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline]
    pub fn counter_dual_edge(self) -> &'a mut W {
        self.variant(CTMODEW::COUNTER_DUAL_EDGE)
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
    #[doc = "Channel 0. CAPn.0 for CTIMERn"]
    CHANNEL_0,
    #[doc = "Channel 1. CAPn.1 for CTIMERn"]
    CHANNEL_1,
    #[doc = "Channel 2. CAPn.2 for CTIMERn"]
    CHANNEL_2,
    #[doc = "Channel 3. CAPn.3 for CTIMERn"]
    CHANNEL_3,
}
impl CINSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CINSELW::CHANNEL_0 => 0,
            CINSELW::CHANNEL_1 => 1,
            CINSELW::CHANNEL_2 => 2,
            CINSELW::CHANNEL_3 => 3,
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
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Channel 0. CAPn.0 for CTIMERn"]
    #[inline]
    pub fn channel_0(self) -> &'a mut W {
        self.variant(CINSELW::CHANNEL_0)
    }
    #[doc = "Channel 1. CAPn.1 for CTIMERn"]
    #[inline]
    pub fn channel_1(self) -> &'a mut W {
        self.variant(CINSELW::CHANNEL_1)
    }
    #[doc = "Channel 2. CAPn.2 for CTIMERn"]
    #[inline]
    pub fn channel_2(self) -> &'a mut W {
        self.variant(CINSELW::CHANNEL_2)
    }
    #[doc = "Channel 3. CAPn.3 for CTIMERn"]
    #[inline]
    pub fn channel_3(self) -> &'a mut W {
        self.variant(CINSELW::CHANNEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    CHANNEL_0_RISING,
    #[doc = "Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    CHANNEL_0_FALLING,
    #[doc = "Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    CHANNEL_1_RISING,
    #[doc = "Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    CHANNEL_1_FALLING,
    #[doc = "Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    CHANNEL_2_RISING,
    #[doc = "Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    CHANNEL_2_FALLING,
}
impl SELCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELCCW::CHANNEL_0_RISING => 0,
            SELCCW::CHANNEL_0_FALLING => 1,
            SELCCW::CHANNEL_1_RISING => 2,
            SELCCW::CHANNEL_1_FALLING => 3,
            SELCCW::CHANNEL_2_RISING => 4,
            SELCCW::CHANNEL_2_FALLING => 5,
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
    #[doc = "Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn channel_0_rising(self) -> &'a mut W {
        self.variant(SELCCW::CHANNEL_0_RISING)
    }
    #[doc = "Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn channel_0_falling(self) -> &'a mut W {
        self.variant(SELCCW::CHANNEL_0_FALLING)
    }
    #[doc = "Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn channel_1_rising(self) -> &'a mut W {
        self.variant(SELCCW::CHANNEL_1_RISING)
    }
    #[doc = "Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn channel_1_falling(self) -> &'a mut W {
        self.variant(SELCCW::CHANNEL_1_FALLING)
    }
    #[doc = "Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn channel_2_rising(self) -> &'a mut W {
        self.variant(SELCCW::CHANNEL_2_RISING)
    }
    #[doc = "Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline]
    pub fn channel_2_falling(self) -> &'a mut W {
        self.variant(SELCCW::CHANNEL_2_FALLING)
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
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline]
    pub fn ctmode(&self) -> CTMODER {
        CTMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline]
    pub fn cinsel(&self) -> CINSELR {
        CINSELR::_from({
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
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline]
    pub fn ctmode(&mut self) -> _CTMODEW {
        _CTMODEW { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline]
    pub fn cinsel(&mut self) -> _CINSELW {
        _CINSELW { w: self }
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
