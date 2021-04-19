#[doc = "Register `TRM` reader"]
pub struct R(crate::R<TRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TRM_SPEC>> for R {
    fn from(reader: crate::R<TRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRM` writer"]
pub struct W(crate::W<TRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRM_SPEC>;
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
impl core::convert::From<crate::W<TRM_SPEC>> for W {
    fn from(writer: crate::W<TRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "2.4V to 3.6V Vdd range: This bit MUST be set to '1' if operation below 2.7V is to be used. Failure to set this bit will result in invalid ADC results. Note: This bit will not be spec'd on parts that do not support operation below 2.7V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRANGE_A {
    #[doc = "0: High voltage"]
    HIGH_VOLTAGE = 0,
    #[doc = "1: Low voltage"]
    LOW_VOLTAGE = 1,
}
impl From<VRANGE_A> for bool {
    #[inline(always)]
    fn from(variant: VRANGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRANGE` reader - 2.4V to 3.6V Vdd range: This bit MUST be set to '1' if operation below 2.7V is to be used. Failure to set this bit will result in invalid ADC results. Note: This bit will not be spec'd on parts that do not support operation below 2.7V"]
pub struct VRANGE_R(crate::FieldReader<bool, VRANGE_A>);
impl VRANGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VRANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRANGE_A {
        match self.bits {
            false => VRANGE_A::HIGH_VOLTAGE,
            true => VRANGE_A::LOW_VOLTAGE,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_VOLTAGE`"]
    #[inline(always)]
    pub fn is_high_voltage(&self) -> bool {
        **self == VRANGE_A::HIGH_VOLTAGE
    }
    #[doc = "Checks if the value of the field is `LOW_VOLTAGE`"]
    #[inline(always)]
    pub fn is_low_voltage(&self) -> bool {
        **self == VRANGE_A::LOW_VOLTAGE
    }
}
impl core::ops::Deref for VRANGE_R {
    type Target = crate::FieldReader<bool, VRANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VRANGE` writer - 2.4V to 3.6V Vdd range: This bit MUST be set to '1' if operation below 2.7V is to be used. Failure to set this bit will result in invalid ADC results. Note: This bit will not be spec'd on parts that do not support operation below 2.7V"]
pub struct VRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> VRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VRANGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "High voltage"]
    #[inline(always)]
    pub fn high_voltage(self) -> &'a mut W {
        self.variant(VRANGE_A::HIGH_VOLTAGE)
    }
    #[doc = "Low voltage"]
    #[inline(always)]
    pub fn low_voltage(self) -> &'a mut W {
        self.variant(VRANGE_A::LOW_VOLTAGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - 2.4V to 3.6V Vdd range: This bit MUST be set to '1' if operation below 2.7V is to be used. Failure to set this bit will result in invalid ADC results. Note: This bit will not be spec'd on parts that do not support operation below 2.7V"]
    #[inline(always)]
    pub fn vrange(&self) -> VRANGE_R {
        VRANGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - 2.4V to 3.6V Vdd range: This bit MUST be set to '1' if operation below 2.7V is to be used. Failure to set this bit will result in invalid ADC results. Note: This bit will not be spec'd on parts that do not support operation below 2.7V"]
    #[inline(always)]
    pub fn vrange(&mut self) -> VRANGE_W {
        VRANGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Startup register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trm](index.html) module"]
pub struct TRM_SPEC;
impl crate::RegisterSpec for TRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trm::R](R) reader structure"]
impl crate::Readable for TRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trm::W](W) writer structure"]
impl crate::Writable for TRM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRM to value 0"]
impl crate::Resettable for TRM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
