#[doc = "Register `USBCLKCTRL` reader"]
pub struct R(crate::R<USBCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCLKCTRL` writer"]
pub struct W(crate::W<USBCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCLKCTRL_SPEC>;
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
impl From<crate::W<USBCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCLKCTRL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `AP_CLK` reader - USB need_clock signal control"]
pub struct AP_CLK_R(crate::FieldReader<bool, AP_CLK_A>);
impl AP_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        AP_CLK_R(crate::FieldReader::new(bits))
    }
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
        **self == AP_CLK_A::UNDER_HARDWARE_CONTROL
    }
    #[doc = "Checks if the value of the field is `FORCED_HIGH`"]
    #[inline(always)]
    pub fn is_forced_high(&self) -> bool {
        **self == AP_CLK_A::FORCED_HIGH
    }
}
impl core::ops::Deref for AP_CLK_R {
    type Target = crate::FieldReader<bool, AP_CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP_CLK` writer - USB need_clock signal control"]
pub struct AP_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_CLK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
#[doc = "Field `POL_CLK` reader - USB need_clock polarity for triggering the USB wake-up interrupt"]
pub struct POL_CLK_R(crate::FieldReader<bool, POL_CLK_A>);
impl POL_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_CLK_R(crate::FieldReader::new(bits))
    }
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
        **self == POL_CLK_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == POL_CLK_A::RISING_EDGE
    }
}
impl core::ops::Deref for POL_CLK_R {
    type Target = crate::FieldReader<bool, POL_CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_CLK` writer - USB need_clock polarity for triggering the USB wake-up interrupt"]
pub struct POL_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_CLK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclkctrl](index.html) module"]
pub struct USBCLKCTRL_SPEC;
impl crate::RegisterSpec for USBCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbclkctrl::R](R) reader structure"]
impl crate::Readable for USBCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbclkctrl::W](W) writer structure"]
impl crate::Writable for USBCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCLKCTRL to value 0"]
impl crate::Resettable for USBCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
