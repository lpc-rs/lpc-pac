#[doc = "Reader of register USBCLKCTRL"]
pub type R = crate::R<u32, super::USBCLKCTRL>;
#[doc = "Writer for register USBCLKCTRL"]
pub type W = crate::W<u32, super::USBCLKCTRL>;
#[doc = "Register USBCLKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB need_clock signal control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_CLK_A {
    #[doc = "0: Under hardware control."]
    UNDER_HARDWARE_CONTROL = 0,
    #[doc = "1: Forced HIGH."]
    FORCED_HIGH = 1,
}
impl From<AP_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_CLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AP_CLK`"]
pub type AP_CLK_R = crate::R<bool, AP_CLK_A>;
impl AP_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_CLK_A {
        match self.bits {
            false => AP_CLK_A::UNDER_HARDWARE_CONTROL,
            true => AP_CLK_A::FORCED_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `UNDER_HARDWARE_CONTROL`"]
    #[inline(always)]
    pub fn is_under_hardware_control(&self) -> bool {
        *self == AP_CLK_A::UNDER_HARDWARE_CONTROL
    }
    #[doc = "Checks if the value of the field is `FORCED_HIGH`"]
    #[inline(always)]
    pub fn is_forced_high(&self) -> bool {
        *self == AP_CLK_A::FORCED_HIGH
    }
}
#[doc = "Write proxy for field `AP_CLK`"]
pub struct AP_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_CLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn under_hardware_control(self) -> &'a mut W {
        self.variant(AP_CLK_A::UNDER_HARDWARE_CONTROL)
    }
    #[doc = "Forced HIGH."]
    #[inline(always)]
    pub fn forced_high(self) -> &'a mut W {
        self.variant(AP_CLK_A::FORCED_HIGH)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "USB need_clock polarity for triggering the USB wake-up interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_CLK_A {
    #[doc = "0: Falling edge of the USB need_clock triggers the USB wake-up (default)."]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge of the USB need_clock triggers the USB wake-up."]
    RISING_EDGE = 1,
}
impl From<POL_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_CLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL_CLK`"]
pub type POL_CLK_R = crate::R<bool, POL_CLK_A>;
impl POL_CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_CLK_A {
        match self.bits {
            false => POL_CLK_A::FALLING_EDGE,
            true => POL_CLK_A::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == POL_CLK_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == POL_CLK_A::RISING_EDGE
    }
}
#[doc = "Write proxy for field `POL_CLK`"]
pub struct POL_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_CLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge of the USB need_clock triggers the USB wake-up (default)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(POL_CLK_A::FALLING_EDGE)
    }
    #[doc = "Rising edge of the USB need_clock triggers the USB wake-up."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(POL_CLK_A::RISING_EDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB need_clock signal control"]
    #[inline(always)]
    pub fn ap_clk(&self) -> AP_CLK_R {
        AP_CLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB need_clock polarity for triggering the USB wake-up interrupt"]
    #[inline(always)]
    pub fn pol_clk(&self) -> POL_CLK_R {
        POL_CLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB need_clock signal control"]
    #[inline(always)]
    pub fn ap_clk(&mut self) -> AP_CLK_W {
        AP_CLK_W { w: self }
    }
    #[doc = "Bit 1 - USB need_clock polarity for triggering the USB wake-up interrupt"]
    #[inline(always)]
    pub fn pol_clk(&mut self) -> POL_CLK_W {
        POL_CLK_W { w: self }
    }
}
