#[doc = "Register `PWMC` reader"]
pub struct R(crate::R<PWMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMC` writer"]
pub struct W(crate::W<PWMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMC_SPEC>;
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
impl From<crate::W<PWMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM mode enable for channel0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN0_A {
    #[doc = "0: CT16Bn_MAT0 is controlled by EM0."]
    EM0 = 0,
    #[doc = "1: PWM mode is enabled for CT16Bn_MAT0."]
    ENABLED = 1,
}
impl From<PWMEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN0` reader - PWM mode enable for channel0."]
pub struct PWMEN0_R(crate::FieldReader<bool, PWMEN0_A>);
impl PWMEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN0_R(crate::FieldReader::new(bits))
    }
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
        **self == PWMEN0_A::EM0
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PWMEN0_A::ENABLED
    }
}
impl core::ops::Deref for PWMEN0_R {
    type Target = crate::FieldReader<bool, PWMEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN0` writer - PWM mode enable for channel0."]
pub struct PWMEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CT16Bn_MAT0 is controlled by EM0."]
    #[inline(always)]
    pub fn em0(self) -> &'a mut W {
        self.variant(PWMEN0_A::EM0)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT0."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "PWM mode enable for channel1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN1_A {
    #[doc = "0: CT16Bn_MAT01 is controlled by EM1."]
    EM1 = 0,
    #[doc = "1: PWM mode is enabled for CT16Bn_MAT1."]
    ENABLED = 1,
}
impl From<PWMEN1_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN1` reader - PWM mode enable for channel1."]
pub struct PWMEN1_R(crate::FieldReader<bool, PWMEN1_A>);
impl PWMEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN1_R(crate::FieldReader::new(bits))
    }
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
        **self == PWMEN1_A::EM1
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PWMEN1_A::ENABLED
    }
}
impl core::ops::Deref for PWMEN1_R {
    type Target = crate::FieldReader<bool, PWMEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN1` writer - PWM mode enable for channel1."]
pub struct PWMEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CT16Bn_MAT01 is controlled by EM1."]
    #[inline(always)]
    pub fn em1(self) -> &'a mut W {
        self.variant(PWMEN1_A::EM1)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT1."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "PWM mode enable for channel2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN2_A {
    #[doc = "0: CT16Bn_MAT2 is controlled by EM2."]
    EM2 = 0,
    #[doc = "1: PWM mode is enabled for CT16Bn_MAT2."]
    ENABLED = 1,
}
impl From<PWMEN2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN2` reader - PWM mode enable for channel2."]
pub struct PWMEN2_R(crate::FieldReader<bool, PWMEN2_A>);
impl PWMEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN2_R(crate::FieldReader::new(bits))
    }
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
        **self == PWMEN2_A::EM2
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PWMEN2_A::ENABLED
    }
}
impl core::ops::Deref for PWMEN2_R {
    type Target = crate::FieldReader<bool, PWMEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN2` writer - PWM mode enable for channel2."]
pub struct PWMEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CT16Bn_MAT2 is controlled by EM2."]
    #[inline(always)]
    pub fn em2(self) -> &'a mut W {
        self.variant(PWMEN2_A::EM2)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT2."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "PWM mode enable for channel3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN3_A {
    #[doc = "0: CT16Bn_MAT3 is controlled by EM3."]
    EM3 = 0,
    #[doc = "1: PWM mode is enabled for CT16Bn_MAT3."]
    ENABLED = 1,
}
impl From<PWMEN3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN3` reader - PWM mode enable for channel3."]
pub struct PWMEN3_R(crate::FieldReader<bool, PWMEN3_A>);
impl PWMEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN3_R(crate::FieldReader::new(bits))
    }
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
        **self == PWMEN3_A::EM3
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PWMEN3_A::ENABLED
    }
}
impl core::ops::Deref for PWMEN3_R {
    type Target = crate::FieldReader<bool, PWMEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN3` writer - PWM mode enable for channel3."]
pub struct PWMEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CT16Bn_MAT3 is controlled by EM3."]
    #[inline(always)]
    pub fn em3(self) -> &'a mut W {
        self.variant(PWMEN3_A::EM3)
    }
    #[doc = "PWM mode is enabled for CT16Bn_MAT3."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
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
    #[doc = "Bit 3 - PWM mode enable for channel3."]
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
    #[doc = "Bit 3 - PWM mode enable for channel3."]
    #[inline(always)]
    pub fn pwmen3(&mut self) -> PWMEN3_W {
        PWMEN3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Control Register. The PWMCON enables PWM mode for the external match pins CT16B0_MAT\\[1:0\\]
and CT16B1_MAT\\[1:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmc](index.html) module"]
pub struct PWMC_SPEC;
impl crate::RegisterSpec for PWMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmc::R](R) reader structure"]
impl crate::Readable for PWMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmc::W](W) writer structure"]
impl crate::Writable for PWMC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWMC to value 0"]
impl crate::Resettable for PWMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
