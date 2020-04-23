#[doc = "Reader of register PHY_CTRL"]
pub type R = crate::R<u32, super::PHY_CTRL>;
#[doc = "Writer for register PHY_CTRL"]
pub type W = crate::W<u32, super::PHY_CTRL>;
#[doc = "Register PHY_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PHY_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Capture PDM_DATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_FALL_A {
    #[doc = "0: Capture PDM_DATA on the rising edge of PDM_CLK."]
    RISING_EDGE = 0,
    #[doc = "1: Capture PDM_DATA on the falling edge of PDM_CLK."]
    FALLING_EDGE = 1,
}
impl From<PHY_FALL_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_FALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHY_FALL`"]
pub type PHY_FALL_R = crate::R<bool, PHY_FALL_A>;
impl PHY_FALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHY_FALL_A {
        match self.bits {
            false => PHY_FALL_A::RISING_EDGE,
            true => PHY_FALL_A::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PHY_FALL_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PHY_FALL_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `PHY_FALL`"]
pub struct PHY_FALL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_FALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHY_FALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture PDM_DATA on the rising edge of PDM_CLK."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(PHY_FALL_A::RISING_EDGE)
    }
    #[doc = "Capture PDM_DATA on the falling edge of PDM_CLK."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(PHY_FALL_A::FALLING_EDGE)
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
#[doc = "Half rate sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_HALF_A {
    #[doc = "0: Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing."]
    STANDARD = 0,
    #[doc = "1: Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing."]
    HALF_RATE = 1,
}
impl From<PHY_HALF_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_HALF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHY_HALF`"]
pub type PHY_HALF_R = crate::R<bool, PHY_HALF_A>;
impl PHY_HALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHY_HALF_A {
        match self.bits {
            false => PHY_HALF_A::STANDARD,
            true => PHY_HALF_A::HALF_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == PHY_HALF_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `HALF_RATE`"]
    #[inline(always)]
    pub fn is_half_rate(&self) -> bool {
        *self == PHY_HALF_A::HALF_RATE
    }
}
#[doc = "Write proxy for field `PHY_HALF`"]
pub struct PHY_HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_HALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHY_HALF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(PHY_HALF_A::STANDARD)
    }
    #[doc = "Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing."]
    #[inline(always)]
    pub fn half_rate(self) -> &'a mut W {
        self.variant(PHY_HALF_A::HALF_RATE)
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
    #[doc = "Bit 0 - Capture PDM_DATA"]
    #[inline(always)]
    pub fn phy_fall(&self) -> PHY_FALL_R {
        PHY_FALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Half rate sampling"]
    #[inline(always)]
    pub fn phy_half(&self) -> PHY_HALF_R {
        PHY_HALF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture PDM_DATA"]
    #[inline(always)]
    pub fn phy_fall(&mut self) -> PHY_FALL_W {
        PHY_FALL_W { w: self }
    }
    #[doc = "Bit 1 - Half rate sampling"]
    #[inline(always)]
    pub fn phy_half(&mut self) -> PHY_HALF_W {
        PHY_HALF_W { w: self }
    }
}
