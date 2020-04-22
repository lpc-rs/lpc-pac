#[doc = "Reader of register PRESETCTRL"]
pub type R = crate::R<u32, super::PRESETCTRL>;
#[doc = "Writer for register PRESETCTRL"]
pub type W = crate::W<u32, super::PRESETCTRL>;
#[doc = "Register PRESETCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESETCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SSP0 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_RST_N_A {
    #[doc = "0: Resets the SSP0 peripheral."]
    RESETS_THE_SSP0_PERI = 0,
    #[doc = "1: SSP0 reset de-asserted."]
    SSP0_RESET_DE_ASSERT = 1,
}
impl From<SSP0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SSP0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSP0_RST_N`"]
pub type SSP0_RST_N_R = crate::R<bool, SSP0_RST_N_A>;
impl SSP0_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP0_RST_N_A {
        match self.bits {
            false => SSP0_RST_N_A::RESETS_THE_SSP0_PERI,
            true => SSP0_RST_N_A::SSP0_RESET_DE_ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `RESETS_THE_SSP0_PERI`"]
    #[inline(always)]
    pub fn is_resets_the_ssp0_peri(&self) -> bool {
        *self == SSP0_RST_N_A::RESETS_THE_SSP0_PERI
    }
    #[doc = "Checks if the value of the field is `SSP0_RESET_DE_ASSERT`"]
    #[inline(always)]
    pub fn is_ssp0_reset_de_assert(&self) -> bool {
        *self == SSP0_RST_N_A::SSP0_RESET_DE_ASSERT
    }
}
#[doc = "Write proxy for field `SSP0_RST_N`"]
pub struct SSP0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP0_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resets the SSP0 peripheral."]
    #[inline(always)]
    pub fn resets_the_ssp0_peri(self) -> &'a mut W {
        self.variant(SSP0_RST_N_A::RESETS_THE_SSP0_PERI)
    }
    #[doc = "SSP0 reset de-asserted."]
    #[inline(always)]
    pub fn ssp0_reset_de_assert(self) -> &'a mut W {
        self.variant(SSP0_RST_N_A::SSP0_RESET_DE_ASSERT)
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
#[doc = "I2C reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_RST_N_A {
    #[doc = "0: Resets the I2C peripheral."]
    RESETS_THE_I2C_PERIP = 0,
    #[doc = "1: I2C reset de-asserted."]
    I2C_RESET_DE_ASSERTE = 1,
}
impl From<I2C_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C_RST_N`"]
pub type I2C_RST_N_R = crate::R<bool, I2C_RST_N_A>;
impl I2C_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_RST_N_A {
        match self.bits {
            false => I2C_RST_N_A::RESETS_THE_I2C_PERIP,
            true => I2C_RST_N_A::I2C_RESET_DE_ASSERTE,
        }
    }
    #[doc = "Checks if the value of the field is `RESETS_THE_I2C_PERIP`"]
    #[inline(always)]
    pub fn is_resets_the_i2c_perip(&self) -> bool {
        *self == I2C_RST_N_A::RESETS_THE_I2C_PERIP
    }
    #[doc = "Checks if the value of the field is `I2C_RESET_DE_ASSERTE`"]
    #[inline(always)]
    pub fn is_i2c_reset_de_asserte(&self) -> bool {
        *self == I2C_RST_N_A::I2C_RESET_DE_ASSERTE
    }
}
#[doc = "Write proxy for field `I2C_RST_N`"]
pub struct I2C_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resets the I2C peripheral."]
    #[inline(always)]
    pub fn resets_the_i2c_perip(self) -> &'a mut W {
        self.variant(I2C_RST_N_A::RESETS_THE_I2C_PERIP)
    }
    #[doc = "I2C reset de-asserted."]
    #[inline(always)]
    pub fn i2c_reset_de_asserte(self) -> &'a mut W {
        self.variant(I2C_RST_N_A::I2C_RESET_DE_ASSERTE)
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
#[doc = "SSP1 reset control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_RST_N_A {
    #[doc = "0: Resets the SSP1 peripheral."]
    RESETS_THE_SSP1_PERI = 0,
    #[doc = "1: SSP1 reset de-asserted."]
    SSP1_RESET_DE_ASSERT = 1,
}
impl From<SSP1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SSP1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSP1_RST_N`"]
pub type SSP1_RST_N_R = crate::R<bool, SSP1_RST_N_A>;
impl SSP1_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP1_RST_N_A {
        match self.bits {
            false => SSP1_RST_N_A::RESETS_THE_SSP1_PERI,
            true => SSP1_RST_N_A::SSP1_RESET_DE_ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `RESETS_THE_SSP1_PERI`"]
    #[inline(always)]
    pub fn is_resets_the_ssp1_peri(&self) -> bool {
        *self == SSP1_RST_N_A::RESETS_THE_SSP1_PERI
    }
    #[doc = "Checks if the value of the field is `SSP1_RESET_DE_ASSERT`"]
    #[inline(always)]
    pub fn is_ssp1_reset_de_assert(&self) -> bool {
        *self == SSP1_RST_N_A::SSP1_RESET_DE_ASSERT
    }
}
#[doc = "Write proxy for field `SSP1_RST_N`"]
pub struct SSP1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP1_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resets the SSP1 peripheral."]
    #[inline(always)]
    pub fn resets_the_ssp1_peri(self) -> &'a mut W {
        self.variant(SSP1_RST_N_A::RESETS_THE_SSP1_PERI)
    }
    #[doc = "SSP1 reset de-asserted."]
    #[inline(always)]
    pub fn ssp1_reset_de_assert(self) -> &'a mut W {
        self.variant(SSP1_RST_N_A::SSP1_RESET_DE_ASSERT)
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
impl R {
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline(always)]
    pub fn ssp0_rst_n(&self) -> SSP0_RST_N_R {
        SSP0_RST_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline(always)]
    pub fn i2c_rst_n(&self) -> I2C_RST_N_R {
        I2C_RST_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSP1 reset control"]
    #[inline(always)]
    pub fn ssp1_rst_n(&self) -> SSP1_RST_N_R {
        SSP1_RST_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline(always)]
    pub fn ssp0_rst_n(&mut self) -> SSP0_RST_N_W {
        SSP0_RST_N_W { w: self }
    }
    #[doc = "Bit 1 - I2C reset control"]
    #[inline(always)]
    pub fn i2c_rst_n(&mut self) -> I2C_RST_N_W {
        I2C_RST_N_W { w: self }
    }
    #[doc = "Bit 2 - SSP1 reset control"]
    #[inline(always)]
    pub fn ssp1_rst_n(&mut self) -> SSP1_RST_N_W {
        SSP1_RST_N_W { w: self }
    }
}
