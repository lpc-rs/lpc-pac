#[doc = "Reader of register INSEL"]
pub type R = crate::R<u32, super::INSEL>;
#[doc = "Writer for register INSEL"]
pub type W = crate::W<u32, super::INSEL>;
#[doc = "Register INSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::INSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL_A::ADC0_IN0),
            3 => Val(SEL_A::TEMPERATURE_SENSOR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC0_IN0`"]
    #[inline(always)]
    pub fn is_adc0_in0(&self) -> bool {
        *self == SEL_A::ADC0_IN0
    }
    #[doc = "Checks if the value of the field is `TEMPERATURE_SENSOR`"]
    #[inline(always)]
    pub fn is_temperature_sensor(&self) -> bool {
        *self == SEL_A::TEMPERATURE_SENSOR
    }
}
#[doc = "Write proxy for field `SEL`"]
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
}
