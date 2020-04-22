#[doc = "Reader of register PWMC"]
pub type R = crate::R<u32, super::PWMC>;
#[doc = "Writer for register PWMC"]
pub type W = crate::W<u32, super::PWMC>;
#[doc = "Register PWMC `reset()`'s with value 0"]
impl crate::ResetValue for super::PWMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PWM mode enable for channel0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN0_A {
    #[doc = "0: CT32Bn_MAT0 is controlled by EM0."]
    EM0 = 0,
    #[doc = "1: PWM mode is enabled for CT32Bn_MAT0."]
    ENABLED = 1,
}
impl From<PWMEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN0`"]
pub type PWMEN0_R = crate::R<bool, PWMEN0_A>;
impl PWMEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN0_A {
        match self.bits {
            false => PWMEN0_A::EM0,
            true => PWMEN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `EM0`"]
    #[inline(always)]
    pub fn is_em0(&self) -> bool {
        *self == PWMEN0_A::EM0
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMEN0_A::ENABLED
    }
}
#[doc = "Write proxy for field `PWMEN0`"]
pub struct PWMEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CT32Bn_MAT0 is controlled by EM0."]
    #[inline(always)]
    pub fn em0(self) -> &'a mut W {
        self.variant(PWMEN0_A::EM0)
    }
    #[doc = "PWM mode is enabled for CT32Bn_MAT0."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMEN0_A::ENABLED)
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
#[doc = "PWM mode enable for channel1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN1_A {
    #[doc = "0: CT32Bn_MAT01 is controlled by EM1."]
    EM1 = 0,
    #[doc = "1: PWM mode is enabled for CT32Bn_MAT1."]
    ENABLED = 1,
}
impl From<PWMEN1_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN1`"]
pub type PWMEN1_R = crate::R<bool, PWMEN1_A>;
impl PWMEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN1_A {
        match self.bits {
            false => PWMEN1_A::EM1,
            true => PWMEN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `EM1`"]
    #[inline(always)]
    pub fn is_em1(&self) -> bool {
        *self == PWMEN1_A::EM1
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMEN1_A::ENABLED
    }
}
#[doc = "Write proxy for field `PWMEN1`"]
pub struct PWMEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CT32Bn_MAT01 is controlled by EM1."]
    #[inline(always)]
    pub fn em1(self) -> &'a mut W {
        self.variant(PWMEN1_A::EM1)
    }
    #[doc = "PWM mode is enabled for CT32Bn_MAT1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMEN1_A::ENABLED)
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
#[doc = "PWM mode enable for channel2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN2_A {
    #[doc = "0: CT32Bn_MAT2 is controlled by EM2."]
    EM2 = 0,
    #[doc = "1: PWM mode is enabled for CT32Bn_MAT2."]
    ENABLED = 1,
}
impl From<PWMEN2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN2`"]
pub type PWMEN2_R = crate::R<bool, PWMEN2_A>;
impl PWMEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN2_A {
        match self.bits {
            false => PWMEN2_A::EM2,
            true => PWMEN2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `EM2`"]
    #[inline(always)]
    pub fn is_em2(&self) -> bool {
        *self == PWMEN2_A::EM2
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMEN2_A::ENABLED
    }
}
#[doc = "Write proxy for field `PWMEN2`"]
pub struct PWMEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CT32Bn_MAT2 is controlled by EM2."]
    #[inline(always)]
    pub fn em2(self) -> &'a mut W {
        self.variant(PWMEN2_A::EM2)
    }
    #[doc = "PWM mode is enabled for CT32Bn_MAT2."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMEN2_A::ENABLED)
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
#[doc = "PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN3_A {
    #[doc = "0: CT32Bn_MAT3 is controlled by EM3."]
    EM3 = 0,
    #[doc = "1: PWM mode is enabled for CT132Bn_MAT3."]
    ENABLED = 1,
}
impl From<PWMEN3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN3`"]
pub type PWMEN3_R = crate::R<bool, PWMEN3_A>;
impl PWMEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN3_A {
        match self.bits {
            false => PWMEN3_A::EM3,
            true => PWMEN3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `EM3`"]
    #[inline(always)]
    pub fn is_em3(&self) -> bool {
        *self == PWMEN3_A::EM3
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMEN3_A::ENABLED
    }
}
#[doc = "Write proxy for field `PWMEN3`"]
pub struct PWMEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CT32Bn_MAT3 is controlled by EM3."]
    #[inline(always)]
    pub fn em3(self) -> &'a mut W {
        self.variant(PWMEN3_A::EM3)
    }
    #[doc = "PWM mode is enabled for CT132Bn_MAT3."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWMEN3_A::ENABLED)
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
    #[doc = "Bit 0 - PWM mode enable for channel0."]
    #[inline(always)]
    pub fn pwmen0(&self) -> PWMEN0_R {
        PWMEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PWM mode enable for channel1."]
    #[inline(always)]
    pub fn pwmen1(&self) -> PWMEN1_R {
        PWMEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PWM mode enable for channel2."]
    #[inline(always)]
    pub fn pwmen2(&self) -> PWMEN2_R {
        PWMEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[inline(always)]
    pub fn pwmen3(&self) -> PWMEN3_R {
        PWMEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM mode enable for channel0."]
    #[inline(always)]
    pub fn pwmen0(&mut self) -> PWMEN0_W {
        PWMEN0_W { w: self }
    }
    #[doc = "Bit 1 - PWM mode enable for channel1."]
    #[inline(always)]
    pub fn pwmen1(&mut self) -> PWMEN1_W {
        PWMEN1_W { w: self }
    }
    #[doc = "Bit 2 - PWM mode enable for channel2."]
    #[inline(always)]
    pub fn pwmen2(&mut self) -> PWMEN2_W {
        PWMEN2_W { w: self }
    }
    #[doc = "Bit 3 - PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[inline(always)]
    pub fn pwmen3(&mut self) -> PWMEN3_W {
        PWMEN3_W { w: self }
    }
}
