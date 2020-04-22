#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 6)) | (((value as u32) & 0x03ff) << 6);
        self.w
    }
}
#[doc = "Settling time The settling times noted in the description of the BIAS bit are valid for a capacitance load on the DAC_OUT pin not exceeding 100 pF. A load impedance value greater than that value will cause settling time longer than the specified time. One or more graphs of load impedance vs. settling time will be included in the final data sheet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIAS_A {
    #[doc = "0: The settling time of the DAC is 1 us max, and the maximum current is 700 uA. This allows a maximum update rate of 1 MHz."]
    FAST = 0,
    #[doc = "1: The settling time of the DAC is 2.5 us and the maximum current is 350 uA. This allows a maximum update rate of 400 kHz."]
    SLOW = 1,
}
impl From<BIAS_A> for bool {
    #[inline(always)]
    fn from(variant: BIAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIAS`"]
pub type BIAS_R = crate::R<bool, BIAS_A>;
impl BIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIAS_A {
        match self.bits {
            false => BIAS_A::FAST,
            true => BIAS_A::SLOW,
        }
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == BIAS_A::FAST
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == BIAS_A::SLOW
    }
}
#[doc = "Write proxy for field `BIAS`"]
pub struct BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The settling time of the DAC is 1 us max, and the maximum current is 700 uA. This allows a maximum update rate of 1 MHz."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(BIAS_A::FAST)
    }
    #[doc = "The settling time of the DAC is 2.5 us and the maximum current is 350 uA. This allows a maximum update rate of 400 kHz."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(BIAS_A::SLOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:15 - After the selected settling time after this field is written with a new VALUE, the voltage on the DAC_OUT pin (with respect to VSSA) is VALUE x ((VREFP - V REFN)/1024) + VREFN."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Settling time The settling times noted in the description of the BIAS bit are valid for a capacitance load on the DAC_OUT pin not exceeding 100 pF. A load impedance value greater than that value will cause settling time longer than the specified time. One or more graphs of load impedance vs. settling time will be included in the final data sheet."]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:15 - After the selected settling time after this field is written with a new VALUE, the voltage on the DAC_OUT pin (with respect to VSSA) is VALUE x ((VREFP - V REFN)/1024) + VREFN."]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
    #[doc = "Bit 16 - Settling time The settling times noted in the description of the BIAS bit are valid for a capacitance load on the DAC_OUT pin not exceeding 100 pF. A load impedance value greater than that value will cause settling time longer than the specified time. One or more graphs of load impedance vs. settling time will be included in the final data sheet."]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W {
        BIAS_W { w: self }
    }
}
