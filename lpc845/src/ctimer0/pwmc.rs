#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWMC {
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
#[doc = "Possible values of the field `PWMEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN0R {
    #[doc = "Match. CTIMERn_MAT0 is controlled by EM0."]
    MATCH,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT0."]
    PWM,
}
impl PWMEN0R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PWMEN0R::MATCH => false,
            PWMEN0R::PWM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN0R {
        match value {
            false => PWMEN0R::MATCH,
            true => PWMEN0R::PWM,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == PWMEN0R::MATCH
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == PWMEN0R::PWM
    }
}
#[doc = "Possible values of the field `PWMEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN1R {
    #[doc = "Match. CTIMERn_MAT01 is controlled by EM1."]
    MATCH,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT1."]
    PWM,
}
impl PWMEN1R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PWMEN1R::MATCH => false,
            PWMEN1R::PWM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN1R {
        match value {
            false => PWMEN1R::MATCH,
            true => PWMEN1R::PWM,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == PWMEN1R::MATCH
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == PWMEN1R::PWM
    }
}
#[doc = "Possible values of the field `PWMEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN2R {
    #[doc = "Match. CTIMERn_MAT2 is controlled by EM2."]
    MATCH,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT2."]
    PWM,
}
impl PWMEN2R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PWMEN2R::MATCH => false,
            PWMEN2R::PWM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN2R {
        match value {
            false => PWMEN2R::MATCH,
            true => PWMEN2R::PWM,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == PWMEN2R::MATCH
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == PWMEN2R::PWM
    }
}
#[doc = "Possible values of the field `PWMEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN3R {
    #[doc = "Match. CTIMERn_MAT3 is controlled by EM3."]
    MATCH,
    #[doc = "PWM. PWM mode is enabled for CT132Bn_MAT3."]
    PWM,
}
impl PWMEN3R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PWMEN3R::MATCH => false,
            PWMEN3R::PWM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN3R {
        match value {
            false => PWMEN3R::MATCH,
            true => PWMEN3R::PWM,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == PWMEN3R::MATCH
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline]
    pub fn is_pwm(&self) -> bool {
        *self == PWMEN3R::PWM
    }
}
#[doc = "Values that can be written to the field `PWMEN0`"]
pub enum PWMEN0W {
    #[doc = "Match. CTIMERn_MAT0 is controlled by EM0."]
    MATCH,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT0."]
    PWM,
}
impl PWMEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN0W::MATCH => false,
            PWMEN0W::PWM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Match. CTIMERn_MAT0 is controlled by EM0."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(PWMEN0W::MATCH)
    }
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT0."]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(PWMEN0W::PWM)
    }
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMEN1`"]
pub enum PWMEN1W {
    #[doc = "Match. CTIMERn_MAT01 is controlled by EM1."]
    MATCH,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT1."]
    PWM,
}
impl PWMEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN1W::MATCH => false,
            PWMEN1W::PWM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Match. CTIMERn_MAT01 is controlled by EM1."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(PWMEN1W::MATCH)
    }
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT1."]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(PWMEN1W::PWM)
    }
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMEN2`"]
pub enum PWMEN2W {
    #[doc = "Match. CTIMERn_MAT2 is controlled by EM2."]
    MATCH,
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT2."]
    PWM,
}
impl PWMEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN2W::MATCH => false,
            PWMEN2W::PWM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Match. CTIMERn_MAT2 is controlled by EM2."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(PWMEN2W::MATCH)
    }
    #[doc = "PWM. PWM mode is enabled for CTIMERn_MAT2."]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(PWMEN2W::PWM)
    }
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMEN3`"]
pub enum PWMEN3W {
    #[doc = "Match. CTIMERn_MAT3 is controlled by EM3."]
    MATCH,
    #[doc = "PWM. PWM mode is enabled for CT132Bn_MAT3."]
    PWM,
}
impl PWMEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN3W::MATCH => false,
            PWMEN3W::PWM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Match. CTIMERn_MAT3 is controlled by EM3."]
    #[inline]
    pub fn match_(self) -> &'a mut W {
        self.variant(PWMEN3W::MATCH)
    }
    #[doc = "PWM. PWM mode is enabled for CT132Bn_MAT3."]
    #[inline]
    pub fn pwm(self) -> &'a mut W {
        self.variant(PWMEN3W::PWM)
    }
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - PWM mode enable for channel0."]
    #[inline]
    pub fn pwmen0(&self) -> PWMEN0R {
        PWMEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PWM mode enable for channel1."]
    #[inline]
    pub fn pwmen1(&self) -> PWMEN1R {
        PWMEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - PWM mode enable for channel2."]
    #[inline]
    pub fn pwmen2(&self) -> PWMEN2R {
        PWMEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[inline]
    pub fn pwmen3(&self) -> PWMEN3R {
        PWMEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - PWM mode enable for channel0."]
    #[inline]
    pub fn pwmen0(&mut self) -> _PWMEN0W {
        _PWMEN0W { w: self }
    }
    #[doc = "Bit 1 - PWM mode enable for channel1."]
    #[inline]
    pub fn pwmen1(&mut self) -> _PWMEN1W {
        _PWMEN1W { w: self }
    }
    #[doc = "Bit 2 - PWM mode enable for channel2."]
    #[inline]
    pub fn pwmen2(&mut self) -> _PWMEN2W {
        _PWMEN2W { w: self }
    }
    #[doc = "Bit 3 - PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[inline]
    pub fn pwmen3(&mut self) -> _PWMEN3W {
        _PWMEN3W { w: self }
    }
}
