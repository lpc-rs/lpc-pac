#[doc = "Reader of register SYSAHBCLKCTRL1"]
pub type R = crate::R<u32, super::SYSAHBCLKCTRL1>;
#[doc = "Writer for register SYSAHBCLKCTRL1"]
pub type W = crate::W<u32, super::SYSAHBCLKCTRL1>;
#[doc = "Register SYSAHBCLKCTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSAHBCLKCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enables clock for CAPT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<CAPT_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPT`"]
pub type CAPT_R = crate::R<bool, CAPT_A>;
impl CAPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_A {
        match self.bits {
            false => CAPT_A::DISABLE,
            true => CAPT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAPT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAPT_A::ENABLE
    }
}
#[doc = "Write proxy for field `CAPT`"]
pub struct CAPT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPT_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPT_A::ENABLE)
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
#[doc = "Enables clock for DAC1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC1_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<DAC1_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DAC1`"]
pub type DAC1_R = crate::R<bool, DAC1_A>;
impl DAC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1_A {
        match self.bits {
            false => DAC1_A::DISABLE,
            true => DAC1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DAC1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DAC1_A::ENABLE
    }
}
#[doc = "Write proxy for field `DAC1`"]
pub struct DAC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DAC1_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DAC1_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Enables clock for CAPT."]
    #[inline(always)]
    pub fn capt(&self) -> CAPT_R {
        CAPT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables clock for DAC1."]
    #[inline(always)]
    pub fn dac1(&self) -> DAC1_R {
        DAC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables clock for CAPT."]
    #[inline(always)]
    pub fn capt(&mut self) -> CAPT_W {
        CAPT_W { w: self }
    }
    #[doc = "Bit 1 - Enables clock for DAC1."]
    #[inline(always)]
    pub fn dac1(&mut self) -> DAC1_W {
        DAC1_W { w: self }
    }
}
