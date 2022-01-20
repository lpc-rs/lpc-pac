#[doc = "Register `SYSAHBCLKCTRL1` reader"]
pub struct R(crate::R<SYSAHBCLKCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSAHBCLKCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSAHBCLKCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSAHBCLKCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSAHBCLKCTRL1` writer"]
pub struct W(crate::W<SYSAHBCLKCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSAHBCLKCTRL1_SPEC>;
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
impl From<crate::W<SYSAHBCLKCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSAHBCLKCTRL1_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `CAPT` reader - Enables clock for CAPT."]
pub struct CAPT_R(crate::FieldReader<bool, CAPT_A>);
impl CAPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPT_R(crate::FieldReader::new(bits))
    }
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
        **self == CAPT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CAPT_A::ENABLE
    }
}
impl core::ops::Deref for CAPT_R {
    type Target = crate::FieldReader<bool, CAPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPT` writer - Enables clock for CAPT."]
pub struct CAPT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
#[doc = "Field `DAC1` reader - Enables clock for DAC1."]
pub struct DAC1_R(crate::FieldReader<bool, DAC1_A>);
impl DAC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC1_R(crate::FieldReader::new(bits))
    }
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
        **self == DAC1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DAC1_A::ENABLE
    }
}
impl core::ops::Deref for DAC1_R {
    type Target = crate::FieldReader<bool, DAC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC1` writer - Enables clock for DAC1."]
pub struct DAC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC1_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System clock group 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysahbclkctrl1](index.html) module"]
pub struct SYSAHBCLKCTRL1_SPEC;
impl crate::RegisterSpec for SYSAHBCLKCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysahbclkctrl1::R](R) reader structure"]
impl crate::Readable for SYSAHBCLKCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysahbclkctrl1::W](W) writer structure"]
impl crate::Writable for SYSAHBCLKCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSAHBCLKCTRL1 to value 0"]
impl crate::Resettable for SYSAHBCLKCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
