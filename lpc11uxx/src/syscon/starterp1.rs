#[doc = "Reader of register STARTERP1"]
pub type R = crate::R<u32, super::STARTERP1>;
#[doc = "Writer for register STARTERP1"]
pub type W = crate::W<u32, super::STARTERP1>;
#[doc = "Register STARTERP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTERP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `WWDTINT`"]
pub type WWDTINT_R = crate::R<bool, WWDTINT_A>;
impl WWDTINT_R {
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
        *self == WWDTINT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WWDTINT_A::ENABLED
    }
}
#[doc = "Write proxy for field `WWDTINT`"]
pub struct WWDTINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDTINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDTINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
#[doc = "Reader of field `BODINT`"]
pub type BODINT_R = crate::R<bool, BODINT_A>;
impl BODINT_R {
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
        *self == BODINT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BODINT_A::ENABLED
    }
}
#[doc = "Write proxy for field `BODINT`"]
pub struct BODINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BODINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
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
#[doc = "Reader of field `USB_WAKEUP`"]
pub type USB_WAKEUP_R = crate::R<bool, USB_WAKEUP_A>;
impl USB_WAKEUP_R {
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
        *self == USB_WAKEUP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USB_WAKEUP_A::ENABLED
    }
}
#[doc = "Write proxy for field `USB_WAKEUP`"]
pub struct USB_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_WAKEUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_WAKEUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
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
#[doc = "Reader of field `GPIOINT0`"]
pub type GPIOINT0_R = crate::R<bool, GPIOINT0_A>;
impl GPIOINT0_R {
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
        *self == GPIOINT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOINT0_A::ENABLED
    }
}
#[doc = "Write proxy for field `GPIOINT0`"]
pub struct GPIOINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOINT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOINT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
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
#[doc = "Reader of field `GPIOINT1`"]
pub type GPIOINT1_R = crate::R<bool, GPIOINT1_A>;
impl GPIOINT1_R {
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
        *self == GPIOINT1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOINT1_A::ENABLED
    }
}
#[doc = "Write proxy for field `GPIOINT1`"]
pub struct GPIOINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOINT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOINT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
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
}
