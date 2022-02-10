///Register `PHY_CTRL` reader
pub struct R(crate::R<PHY_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PHY_CTRL` writer
pub struct W(crate::W<PHY_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_CTRL_SPEC>;
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
impl From<crate::W<PHY_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Capture PDM_DATA
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_FALL_A {
    ///0: Capture PDM_DATA on the rising edge of PDM_CLK.
    RISING_EDGE = 0,
    ///1: Capture PDM_DATA on the falling edge of PDM_CLK.
    FALLING_EDGE = 1,
}
impl From<PHY_FALL_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_FALL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PHY_FALL` reader - Capture PDM_DATA
pub struct PHY_FALL_R(crate::FieldReader<bool, PHY_FALL_A>);
impl PHY_FALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHY_FALL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PHY_FALL_A {
        match self.bits {
            false => PHY_FALL_A::RISING_EDGE,
            true => PHY_FALL_A::FALLING_EDGE,
        }
    }
    ///Checks if the value of the field is `RISING_EDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == PHY_FALL_A::RISING_EDGE
    }
    ///Checks if the value of the field is `FALLING_EDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == PHY_FALL_A::FALLING_EDGE
    }
}
impl core::ops::Deref for PHY_FALL_R {
    type Target = crate::FieldReader<bool, PHY_FALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PHY_FALL` writer - Capture PDM_DATA
pub struct PHY_FALL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_FALL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PHY_FALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Capture PDM_DATA on the rising edge of PDM_CLK.
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(PHY_FALL_A::RISING_EDGE)
    }
    ///Capture PDM_DATA on the falling edge of PDM_CLK.
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(PHY_FALL_A::FALLING_EDGE)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Half rate sampling
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_HALF_A {
    ///0: Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing.
    STANDARD = 0,
    ///1: Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing.
    HALF_RATE = 1,
}
impl From<PHY_HALF_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_HALF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PHY_HALF` reader - Half rate sampling
pub struct PHY_HALF_R(crate::FieldReader<bool, PHY_HALF_A>);
impl PHY_HALF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHY_HALF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PHY_HALF_A {
        match self.bits {
            false => PHY_HALF_A::STANDARD,
            true => PHY_HALF_A::HALF_RATE,
        }
    }
    ///Checks if the value of the field is `STANDARD`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == PHY_HALF_A::STANDARD
    }
    ///Checks if the value of the field is `HALF_RATE`
    #[inline(always)]
    pub fn is_half_rate(&self) -> bool {
        **self == PHY_HALF_A::HALF_RATE
    }
}
impl core::ops::Deref for PHY_HALF_R {
    type Target = crate::FieldReader<bool, PHY_HALF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PHY_HALF` writer - Half rate sampling
pub struct PHY_HALF_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_HALF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PHY_HALF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing.
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(PHY_HALF_A::STANDARD)
    }
    ///Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing.
    #[inline(always)]
    pub fn half_rate(self) -> &'a mut W {
        self.variant(PHY_HALF_A::HALF_RATE)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    ///Bit 0 - Capture PDM_DATA
    #[inline(always)]
    pub fn phy_fall(&self) -> PHY_FALL_R {
        PHY_FALL_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Half rate sampling
    #[inline(always)]
    pub fn phy_half(&self) -> PHY_HALF_R {
        PHY_HALF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Capture PDM_DATA
    #[inline(always)]
    pub fn phy_fall(&mut self) -> PHY_FALL_W {
        PHY_FALL_W { w: self }
    }
    ///Bit 1 - Half rate sampling
    #[inline(always)]
    pub fn phy_half(&mut self) -> PHY_HALF_W {
        PHY_HALF_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PDM Source Configuration register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [phy_ctrl](index.html) module
pub struct PHY_CTRL_SPEC;
impl crate::RegisterSpec for PHY_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [phy_ctrl::R](R) reader structure
impl crate::Readable for PHY_CTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [phy_ctrl::W](W) writer structure
impl crate::Writable for PHY_CTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets PHY_CTRL to value 0
impl crate::Resettable for PHY_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
