#[doc = "Register `PRESETCTRL` reader"]
pub struct R(crate::R<PRESETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL` writer"]
pub struct W(crate::W<PRESETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PRESETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SSP0_RST_N` reader - SSP0 reset control"]
pub struct SSP0_RST_N_R(crate::FieldReader<bool, SSP0_RST_N_A>);
impl SSP0_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSP0_RST_N_R(crate::FieldReader::new(bits))
    }
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
        **self == SSP0_RST_N_A::RESETS_THE_SSP0_PERI
    }
    #[doc = "Checks if the value of the field is `SSP0_RESET_DE_ASSERT`"]
    #[inline(always)]
    pub fn is_ssp0_reset_de_assert(&self) -> bool {
        **self == SSP0_RST_N_A::SSP0_RESET_DE_ASSERT
    }
}
impl core::ops::Deref for SSP0_RST_N_R {
    type Target = crate::FieldReader<bool, SSP0_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSP0_RST_N` writer - SSP0 reset control"]
pub struct SSP0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP0_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
#[doc = "Field `I2C_RST_N` reader - I2C reset control"]
pub struct I2C_RST_N_R(crate::FieldReader<bool, I2C_RST_N_A>);
impl I2C_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_RST_N_R(crate::FieldReader::new(bits))
    }
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
        **self == I2C_RST_N_A::RESETS_THE_I2C_PERIP
    }
    #[doc = "Checks if the value of the field is `I2C_RESET_DE_ASSERTE`"]
    #[inline(always)]
    pub fn is_i2c_reset_de_asserte(&self) -> bool {
        **self == I2C_RST_N_A::I2C_RESET_DE_ASSERTE
    }
}
impl core::ops::Deref for I2C_RST_N_R {
    type Target = crate::FieldReader<bool, I2C_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_RST_N` writer - I2C reset control"]
pub struct I2C_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
#[doc = "Field `SSP1_RST_N` reader - SSP1 reset control"]
pub struct SSP1_RST_N_R(crate::FieldReader<bool, SSP1_RST_N_A>);
impl SSP1_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSP1_RST_N_R(crate::FieldReader::new(bits))
    }
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
        **self == SSP1_RST_N_A::RESETS_THE_SSP1_PERI
    }
    #[doc = "Checks if the value of the field is `SSP1_RESET_DE_ASSERT`"]
    #[inline(always)]
    pub fn is_ssp1_reset_de_assert(&self) -> bool {
        **self == SSP1_RST_N_A::SSP1_RESET_DE_ASSERT
    }
}
impl core::ops::Deref for SSP1_RST_N_R {
    type Target = crate::FieldReader<bool, SSP1_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSP1_RST_N` writer - SSP1 reset control"]
pub struct SSP1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP1_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl](index.html) module"]
pub struct PRESETCTRL_SPEC;
impl crate::RegisterSpec for PRESETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl::R](R) reader structure"]
impl crate::Readable for PRESETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl::W](W) writer structure"]
impl crate::Writable for PRESETCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL to value 0"]
impl crate::Resettable for PRESETCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
