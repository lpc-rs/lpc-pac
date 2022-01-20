#[doc = "Register `INSEL` reader"]
pub struct R(crate::R<INSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INSEL` writer"]
pub struct W(crate::W<INSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSEL_SPEC>;
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
impl From<crate::W<INSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects the input source for channel 0. All other values are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: ADC0_IN0 function."]
    ADC0_IN0 = 0,
    #[doc = "3: Internal temperature sensor."]
    TEMPERATURE_SENSOR = 3,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SEL` reader - Selects the input source for channel 0. All other values are reserved."]
pub struct SEL_R(crate::FieldReader<u8, SEL_A>);
impl SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::ADC0_IN0),
            3 => Some(SEL_A::TEMPERATURE_SENSOR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC0_IN0`"]
    #[inline(always)]
    pub fn is_adc0_in0(&self) -> bool {
        **self == SEL_A::ADC0_IN0
    }
    #[doc = "Checks if the value of the field is `TEMPERATURE_SENSOR`"]
    #[inline(always)]
    pub fn is_temperature_sensor(&self) -> bool {
        **self == SEL_A::TEMPERATURE_SENSOR
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Selects the input source for channel 0. All other values are reserved."]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADC0_IN0 function."]
    #[inline(always)]
    pub fn adc0_in0(self) -> &'a mut W {
        self.variant(SEL_A::ADC0_IN0)
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn temperature_sensor(self) -> &'a mut W {
        self.variant(SEL_A::TEMPERATURE_SENSOR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects the input source for channel 0. All other values are reserved."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the input source for channel 0. All other values are reserved."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Select. Allows selection of the temperature sensor as an alternate input to ADC channel 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [insel](index.html) module"]
pub struct INSEL_SPEC;
impl crate::RegisterSpec for INSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [insel::R](R) reader structure"]
impl crate::Readable for INSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [insel::W](W) writer structure"]
impl crate::Writable for INSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INSEL to value 0"]
impl crate::Resettable for INSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
