#[doc = "Register `STARTERP1` reader"]
pub struct R(crate::R<STARTERP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTERP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STARTERP1_SPEC>> for R {
    fn from(reader: crate::R<STARTERP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTERP1` writer"]
pub struct W(crate::W<STARTERP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTERP1_SPEC>;
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
impl core::convert::From<crate::W<STARTERP1_SPEC>> for W {
    fn from(writer: crate::W<STARTERP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WWDT interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDTINT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<WWDTINT_A> for bool {
    #[inline(always)]
    fn from(variant: WWDTINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTINT` reader - WWDT interrupt wake-up"]
pub struct WWDTINT_R(crate::FieldReader<bool, WWDTINT_A>);
impl WWDTINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDTINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDTINT_A {
        match self.bits {
            false => WWDTINT_A::DISABLED,
            true => WWDTINT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WWDTINT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WWDTINT_A::ENABLED
    }
}
impl core::ops::Deref for WWDTINT_R {
    type Target = crate::FieldReader<bool, WWDTINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDTINT` writer - WWDT interrupt wake-up"]
pub struct WWDTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDTINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDTINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDTINT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDTINT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Brown Out Detect (BOD) interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<BODINT_A> for bool {
    #[inline(always)]
    fn from(variant: BODINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODINT` reader - Brown Out Detect (BOD) interrupt wake-up"]
pub struct BODINT_R(crate::FieldReader<bool, BODINT_A>);
impl BODINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODINT_A {
        match self.bits {
            false => BODINT_A::DISABLED,
            true => BODINT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BODINT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BODINT_A::ENABLED
    }
}
impl core::ops::Deref for BODINT_R {
    type Target = crate::FieldReader<bool, BODINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODINT` writer - Brown Out Detect (BOD) interrupt wake-up"]
pub struct BODINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BODINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BODINT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BODINT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "USB need_clock signal wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_WAKEUP_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<USB_WAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: USB_WAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_WAKEUP` reader - USB need_clock signal wake-up"]
pub struct USB_WAKEUP_R(crate::FieldReader<bool, USB_WAKEUP_A>);
impl USB_WAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_WAKEUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_WAKEUP_A {
        match self.bits {
            false => USB_WAKEUP_A::DISABLED,
            true => USB_WAKEUP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USB_WAKEUP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USB_WAKEUP_A::ENABLED
    }
}
impl core::ops::Deref for USB_WAKEUP_R {
    type Target = crate::FieldReader<bool, USB_WAKEUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_WAKEUP` writer - USB need_clock signal wake-up"]
pub struct USB_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_WAKEUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_WAKEUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB_WAKEUP_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB_WAKEUP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "GPIO GROUP0 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOINT0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<GPIOINT0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOINT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOINT0` reader - GPIO GROUP0 interrupt wake-up"]
pub struct GPIOINT0_R(crate::FieldReader<bool, GPIOINT0_A>);
impl GPIOINT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOINT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOINT0_A {
        match self.bits {
            false => GPIOINT0_A::DISABLED,
            true => GPIOINT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GPIOINT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GPIOINT0_A::ENABLED
    }
}
impl core::ops::Deref for GPIOINT0_R {
    type Target = crate::FieldReader<bool, GPIOINT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOINT0` writer - GPIO GROUP0 interrupt wake-up"]
pub struct GPIOINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOINT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOINT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOINT0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOINT0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "GPIO GROUP1 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOINT1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<GPIOINT1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOINT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOINT1` reader - GPIO GROUP1 interrupt wake-up"]
pub struct GPIOINT1_R(crate::FieldReader<bool, GPIOINT1_A>);
impl GPIOINT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOINT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOINT1_A {
        match self.bits {
            false => GPIOINT1_A::DISABLED,
            true => GPIOINT1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GPIOINT1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GPIOINT1_A::ENABLED
    }
}
impl core::ops::Deref for GPIOINT1_R {
    type Target = crate::FieldReader<bool, GPIOINT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOINT1` writer - GPIO GROUP1 interrupt wake-up"]
pub struct GPIOINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOINT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOINT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOINT1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOINT1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - WWDT interrupt wake-up"]
    #[inline(always)]
    pub fn wwdtint(&self) -> WWDTINT_R {
        WWDTINT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Brown Out Detect (BOD) interrupt wake-up"]
    #[inline(always)]
    pub fn bodint(&self) -> BODINT_R {
        BODINT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 19 - USB need_clock signal wake-up"]
    #[inline(always)]
    pub fn usb_wakeup(&self) -> USB_WAKEUP_R {
        USB_WAKEUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GPIO GROUP0 interrupt wake-up"]
    #[inline(always)]
    pub fn gpioint0(&self) -> GPIOINT0_R {
        GPIOINT0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - GPIO GROUP1 interrupt wake-up"]
    #[inline(always)]
    pub fn gpioint1(&self) -> GPIOINT1_R {
        GPIOINT1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - WWDT interrupt wake-up"]
    #[inline(always)]
    pub fn wwdtint(&mut self) -> WWDTINT_W {
        WWDTINT_W { w: self }
    }
    #[doc = "Bit 13 - Brown Out Detect (BOD) interrupt wake-up"]
    #[inline(always)]
    pub fn bodint(&mut self) -> BODINT_W {
        BODINT_W { w: self }
    }
    #[doc = "Bit 19 - USB need_clock signal wake-up"]
    #[inline(always)]
    pub fn usb_wakeup(&mut self) -> USB_WAKEUP_W {
        USB_WAKEUP_W { w: self }
    }
    #[doc = "Bit 20 - GPIO GROUP0 interrupt wake-up"]
    #[inline(always)]
    pub fn gpioint0(&mut self) -> GPIOINT0_W {
        GPIOINT0_W { w: self }
    }
    #[doc = "Bit 21 - GPIO GROUP1 interrupt wake-up"]
    #[inline(always)]
    pub fn gpioint1(&mut self) -> GPIOINT1_W {
        GPIOINT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic 1 interrupt wake-up enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterp1](index.html) module"]
pub struct STARTERP1_SPEC;
impl crate::RegisterSpec for STARTERP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starterp1::R](R) reader structure"]
impl crate::Readable for STARTERP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starterp1::W](W) writer structure"]
impl crate::Writable for STARTERP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTERP1 to value 0"]
impl crate::Resettable for STARTERP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
