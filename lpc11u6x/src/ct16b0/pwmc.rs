#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWMC {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `PWMEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN0R {
    #[doc = "CT16Bn_MAT0 is controlled by EM0."]
    EM0,
    #[doc = "PWM mode is enabled for CT16Bn_MAT0."]
    PWM,
}
impl crate::ToBits<bool> for PWMEN0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMEN0R::EM0 => false,
            PWMEN0R::PWM => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMEN0_R = crate::FR<bool, PWMEN0R>;
impl PWMEN0_R {
    #[doc = "Checks if the value of the field is `EM0`"]
    #[inline(always)]
    pub fn is_em0(&self) -> bool {
        *self == PWMEN0R::EM0
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == PWMEN0R::PWM
    }
}
#[doc = "Values that can be written to the field `PWMEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN0W {
    #[doc = "CT16Bn_MAT0 is controlled by EM0."]
    EM0,
    #[doc = "PWM mode is enabled for CT16Bn_MAT0."]
    PWM,
}
impl PWMEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN0W::EM0 => false,
            PWMEN0W::PWM => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CT16Bn_MAT0 is controlled by EM0."]
    #[inline(always)]
    pub fn em0(self) -> &'a mut W {
        self.variant(PWMEN0W::EM0)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT0."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(PWMEN0W::PWM)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `PWMEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN1R {
    #[doc = "CT16Bn_MAT01 is controlled by EM1."]
    EM1,
    #[doc = "PWM mode is enabled for CT16Bn_MAT1."]
    PWM,
}
impl crate::ToBits<bool> for PWMEN1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMEN1R::EM1 => false,
            PWMEN1R::PWM => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMEN1_R = crate::FR<bool, PWMEN1R>;
impl PWMEN1_R {
    #[doc = "Checks if the value of the field is `EM1`"]
    #[inline(always)]
    pub fn is_em1(&self) -> bool {
        *self == PWMEN1R::EM1
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == PWMEN1R::PWM
    }
}
#[doc = "Values that can be written to the field `PWMEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN1W {
    #[doc = "CT16Bn_MAT01 is controlled by EM1."]
    EM1,
    #[doc = "PWM mode is enabled for CT16Bn_MAT1."]
    PWM,
}
impl PWMEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN1W::EM1 => false,
            PWMEN1W::PWM => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CT16Bn_MAT01 is controlled by EM1."]
    #[inline(always)]
    pub fn em1(self) -> &'a mut W {
        self.variant(PWMEN1W::EM1)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT1."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(PWMEN1W::PWM)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `PWMEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN2R {
    #[doc = "CT16Bn_MAT2 is controlled by EM2."]
    EM2,
    #[doc = "PWM mode is enabled for CT16Bn_MAT2."]
    PWM,
}
impl crate::ToBits<bool> for PWMEN2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMEN2R::EM2 => false,
            PWMEN2R::PWM => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMEN2_R = crate::FR<bool, PWMEN2R>;
impl PWMEN2_R {
    #[doc = "Checks if the value of the field is `EM2`"]
    #[inline(always)]
    pub fn is_em2(&self) -> bool {
        *self == PWMEN2R::EM2
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == PWMEN2R::PWM
    }
}
#[doc = "Values that can be written to the field `PWMEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN2W {
    #[doc = "CT16Bn_MAT2 is controlled by EM2."]
    EM2,
    #[doc = "PWM mode is enabled for CT16Bn_MAT2."]
    PWM,
}
impl PWMEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN2W::EM2 => false,
            PWMEN2W::PWM => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CT16Bn_MAT2 is controlled by EM2."]
    #[inline(always)]
    pub fn em2(self) -> &'a mut W {
        self.variant(PWMEN2W::EM2)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT2."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(PWMEN2W::PWM)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `PWMEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN3R {
    #[doc = "CT16Bn_MAT3 is controlled by EM3."]
    EM3,
    #[doc = "PWM mode is enabled for CT16Bn_MAT3."]
    PWM,
}
impl crate::ToBits<bool> for PWMEN3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PWMEN3R::EM3 => false,
            PWMEN3R::PWM => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PWMEN3_R = crate::FR<bool, PWMEN3R>;
impl PWMEN3_R {
    #[doc = "Checks if the value of the field is `EM3`"]
    #[inline(always)]
    pub fn is_em3(&self) -> bool {
        *self == PWMEN3R::EM3
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == PWMEN3R::PWM
    }
}
#[doc = "Values that can be written to the field `PWMEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN3W {
    #[doc = "CT16Bn_MAT3 is controlled by EM3."]
    EM3,
    #[doc = "PWM mode is enabled for CT16Bn_MAT3."]
    PWM,
}
impl PWMEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN3W::EM3 => false,
            PWMEN3W::PWM => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWMEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CT16Bn_MAT3 is controlled by EM3."]
    #[inline(always)]
    pub fn em3(self) -> &'a mut W {
        self.variant(PWMEN3W::EM3)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT3."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(PWMEN3W::PWM)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - PWM mode enable for channel0."]
    #[inline(always)]
    pub fn pwmen0(&self) -> PWMEN0_R {
        PWMEN0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM mode enable for channel1."]
    #[inline(always)]
    pub fn pwmen1(&self) -> PWMEN1_R {
        PWMEN1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM mode enable for channel2."]
    #[inline(always)]
    pub fn pwmen2(&self) -> PWMEN2_R {
        PWMEN2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM mode enable for channel3."]
    #[inline(always)]
    pub fn pwmen3(&self) -> PWMEN3_R {
        PWMEN3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - PWM mode enable for channel0."]
    #[inline(always)]
    pub fn pwmen0(&mut self) -> _PWMEN0W {
        _PWMEN0W { w: self }
    }
    #[doc = "Bit 1 - PWM mode enable for channel1."]
    #[inline(always)]
    pub fn pwmen1(&mut self) -> _PWMEN1W {
        _PWMEN1W { w: self }
    }
    #[doc = "Bit 2 - PWM mode enable for channel2."]
    #[inline(always)]
    pub fn pwmen2(&mut self) -> _PWMEN2W {
        _PWMEN2W { w: self }
    }
    #[doc = "Bit 3 - PWM mode enable for channel3."]
    #[inline(always)]
    pub fn pwmen3(&mut self) -> _PWMEN3W {
        _PWMEN3W { w: self }
    }
}
