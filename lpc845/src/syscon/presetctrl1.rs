#[doc = "Register `PRESETCTRL1` reader"]
pub struct R(crate::R<PRESETCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL1` writer"]
pub struct W(crate::W<PRESETCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL1_SPEC>;
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
impl From<crate::W<PRESETCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capacitive touch reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_RST_N_A {
    #[doc = "0: Assert the capacitive touch reset."]
    ASSERT = 0,
    #[doc = "1: Clear the capacitive touch reset."]
    CLEAR = 1,
}
impl From<CAPT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPT_RST_N` reader - Capacitive touch reset control"]
pub struct CAPT_RST_N_R(crate::FieldReader<bool, CAPT_RST_N_A>);
impl CAPT_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPT_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_RST_N_A {
        match self.bits {
            false => CAPT_RST_N_A::ASSERT,
            true => CAPT_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == CAPT_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CAPT_RST_N_A::CLEAR
    }
}
impl core::ops::Deref for CAPT_RST_N_R {
    type Target = crate::FieldReader<bool, CAPT_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPT_RST_N` writer - Capacitive touch reset control"]
pub struct CAPT_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the capacitive touch reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(CAPT_RST_N_A::ASSERT)
    }
    #[doc = "Clear the capacitive touch reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAPT_RST_N_A::CLEAR)
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
#[doc = "DAC1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC1_RST_N_A {
    #[doc = "0: Assert the DAC1 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the DAC1 reset."]
    CLEAR = 1,
}
impl From<DAC1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1_RST_N` reader - DAC1 reset control"]
pub struct DAC1_RST_N_R(crate::FieldReader<bool, DAC1_RST_N_A>);
impl DAC1_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC1_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1_RST_N_A {
        match self.bits {
            false => DAC1_RST_N_A::ASSERT,
            true => DAC1_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == DAC1_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == DAC1_RST_N_A::CLEAR
    }
}
impl core::ops::Deref for DAC1_RST_N_R {
    type Target = crate::FieldReader<bool, DAC1_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC1_RST_N` writer - DAC1 reset control"]
pub struct DAC1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC1_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the DAC1 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(DAC1_RST_N_A::ASSERT)
    }
    #[doc = "Clear the DAC1 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DAC1_RST_N_A::CLEAR)
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
#[doc = "Fractional baud rate generator 0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRG0_RST_N_A {
    #[doc = "0: Assert the FRG0 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the FRG0 reset."]
    CLEAR = 1,
}
impl From<FRG0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: FRG0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRG0_RST_N` reader - Fractional baud rate generator 0 reset control"]
pub struct FRG0_RST_N_R(crate::FieldReader<bool, FRG0_RST_N_A>);
impl FRG0_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRG0_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRG0_RST_N_A {
        match self.bits {
            false => FRG0_RST_N_A::ASSERT,
            true => FRG0_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == FRG0_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == FRG0_RST_N_A::CLEAR
    }
}
impl core::ops::Deref for FRG0_RST_N_R {
    type Target = crate::FieldReader<bool, FRG0_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRG0_RST_N` writer - Fractional baud rate generator 0 reset control"]
pub struct FRG0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> FRG0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRG0_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the FRG0 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(FRG0_RST_N_A::ASSERT)
    }
    #[doc = "Clear the FRG0 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FRG0_RST_N_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Fractional baud rate generator 1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRG1_RST_N_A {
    #[doc = "0: Assert the FRG1 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the FRG1 reset."]
    CLEAR = 1,
}
impl From<FRG1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: FRG1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRG1_RST_N` reader - Fractional baud rate generator 1 reset control"]
pub struct FRG1_RST_N_R(crate::FieldReader<bool, FRG1_RST_N_A>);
impl FRG1_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRG1_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRG1_RST_N_A {
        match self.bits {
            false => FRG1_RST_N_A::ASSERT,
            true => FRG1_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == FRG1_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == FRG1_RST_N_A::CLEAR
    }
}
impl core::ops::Deref for FRG1_RST_N_R {
    type Target = crate::FieldReader<bool, FRG1_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRG1_RST_N` writer - Fractional baud rate generator 1 reset control"]
pub struct FRG1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> FRG1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRG1_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the FRG1 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(FRG1_RST_N_A::ASSERT)
    }
    #[doc = "Clear the FRG1 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FRG1_RST_N_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Capacitive touch reset control"]
    #[inline(always)]
    pub fn capt_rst_n(&self) -> CAPT_RST_N_R {
        CAPT_RST_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC1 reset control"]
    #[inline(always)]
    pub fn dac1_rst_n(&self) -> DAC1_RST_N_R {
        DAC1_RST_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fractional baud rate generator 0 reset control"]
    #[inline(always)]
    pub fn frg0_rst_n(&self) -> FRG0_RST_N_R {
        FRG0_RST_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fractional baud rate generator 1 reset control"]
    #[inline(always)]
    pub fn frg1_rst_n(&self) -> FRG1_RST_N_R {
        FRG1_RST_N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capacitive touch reset control"]
    #[inline(always)]
    pub fn capt_rst_n(&mut self) -> CAPT_RST_N_W {
        CAPT_RST_N_W { w: self }
    }
    #[doc = "Bit 1 - DAC1 reset control"]
    #[inline(always)]
    pub fn dac1_rst_n(&mut self) -> DAC1_RST_N_W {
        DAC1_RST_N_W { w: self }
    }
    #[doc = "Bit 3 - Fractional baud rate generator 0 reset control"]
    #[inline(always)]
    pub fn frg0_rst_n(&mut self) -> FRG0_RST_N_W {
        FRG0_RST_N_W { w: self }
    }
    #[doc = "Bit 4 - Fractional baud rate generator 1 reset control"]
    #[inline(always)]
    pub fn frg1_rst_n(&mut self) -> FRG1_RST_N_W {
        FRG1_RST_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset group 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl1](index.html) module"]
pub struct PRESETCTRL1_SPEC;
impl crate::RegisterSpec for PRESETCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl1::R](R) reader structure"]
impl crate::Readable for PRESETCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl1::W](W) writer structure"]
impl crate::Writable for PRESETCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL1 to value 0x1f"]
impl crate::Resettable for PRESETCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
