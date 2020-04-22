#[doc = "Reader of register SCS"]
pub type R = crate::R<u32, super::SCS>;
#[doc = "Writer for register SCS"]
pub type W = crate::W<u32, super::SCS>;
#[doc = "Register SCS `reset()`'s with value 0"]
impl crate::ResetValue for super::SCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Main oscillator range select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCRANGE_A {
    #[doc = "0: Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    LOW = 0,
    #[doc = "1: High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    HIGH = 1,
}
impl From<OSCRANGE_A> for bool {
    #[inline(always)]
    fn from(variant: OSCRANGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCRANGE`"]
pub type OSCRANGE_R = crate::R<bool, OSCRANGE_A>;
impl OSCRANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCRANGE_A {
        match self.bits {
            false => OSCRANGE_A::LOW,
            true => OSCRANGE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OSCRANGE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OSCRANGE_A::HIGH
    }
}
#[doc = "Write proxy for field `OSCRANGE`"]
pub struct OSCRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCRANGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OSCRANGE_A::LOW)
    }
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OSCRANGE_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Main oscillator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCEN_A {
    #[doc = "0: Disabled. The main oscillator is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    ENABLED = 1,
}
impl From<OSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCEN`"]
pub type OSCEN_R = crate::R<bool, OSCEN_A>;
impl OSCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCEN_A {
        match self.bits {
            false => OSCEN_A::DISABLED,
            true => OSCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `OSCEN`"]
pub struct OSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The main oscillator is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSCEN_A::DISABLED)
    }
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Main oscillator status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSTAT_A {
    #[doc = "0: Not ready. The main oscillator is not ready to be used as a clock source."]
    NOT_READY = 0,
    #[doc = "1: Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    READY = 1,
}
impl From<OSCSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCSTAT`"]
pub type OSCSTAT_R = crate::R<bool, OSCSTAT_A>;
impl OSCSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSTAT_A {
        match self.bits {
            false => OSCSTAT_A::NOT_READY,
            true => OSCSTAT_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == OSCSTAT_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == OSCSTAT_A::READY
    }
}
#[doc = "Write proxy for field `OSCSTAT`"]
pub struct OSCSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSTAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(OSCSTAT_A::NOT_READY)
    }
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(OSCSTAT_A::READY)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrange(&self) -> OSCRANGE_R {
        OSCRANGE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&self) -> OSCEN_R {
        OSCEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&self) -> OSCSTAT_R {
        OSCSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrange(&mut self) -> OSCRANGE_W {
        OSCRANGE_W { w: self }
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&mut self) -> OSCEN_W {
        OSCEN_W { w: self }
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&mut self) -> OSCSTAT_W {
        OSCSTAT_W { w: self }
    }
}
