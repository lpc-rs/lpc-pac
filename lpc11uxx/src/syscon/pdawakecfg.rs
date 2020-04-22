#[doc = "Reader of register PDAWAKECFG"]
pub type R = crate::R<u32, super::PDAWAKECFG>;
#[doc = "Writer for register PDAWAKECFG"]
pub type W = crate::W<u32, super::PDAWAKECFG>;
#[doc = "Register PDAWAKECFG `reset()`'s with value 0xedf0"]
impl crate::ResetValue for super::PDAWAKECFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xedf0
    }
}
#[doc = "IRC oscillator output wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCOUT_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<IRCOUT_PD_A> for bool {
    #[inline(always)]
    fn from(variant: IRCOUT_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRCOUT_PD`"]
pub type IRCOUT_PD_R = crate::R<bool, IRCOUT_PD_A>;
impl IRCOUT_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCOUT_PD_A {
        match self.bits {
            false => IRCOUT_PD_A::POWERED,
            true => IRCOUT_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == IRCOUT_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == IRCOUT_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `IRCOUT_PD`"]
pub struct IRCOUT_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> IRCOUT_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRCOUT_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(IRCOUT_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(IRCOUT_PD_A::POWERED_DOWN)
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
#[doc = "IRC oscillator power-down wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<IRC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: IRC_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRC_PD`"]
pub type IRC_PD_R = crate::R<bool, IRC_PD_A>;
impl IRC_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRC_PD_A {
        match self.bits {
            false => IRC_PD_A::POWERED,
            true => IRC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == IRC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == IRC_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `IRC_PD`"]
pub struct IRC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRC_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(IRC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(IRC_PD_A::POWERED_DOWN)
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
#[doc = "Flash wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<FLASH_PD_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH_PD`"]
pub type FLASH_PD_R = crate::R<bool, FLASH_PD_A>;
impl FLASH_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_PD_A {
        match self.bits {
            false => FLASH_PD_A::POWERED,
            true => FLASH_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == FLASH_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == FLASH_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `FLASH_PD`"]
pub struct FLASH_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(FLASH_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FLASH_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "BOD wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<BOD_PD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOD_PD`"]
pub type BOD_PD_R = crate::R<bool, BOD_PD_A>;
impl BOD_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_PD_A {
        match self.bits {
            false => BOD_PD_A::POWERED,
            true => BOD_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `BOD_PD`"]
pub struct BOD_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOD_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "ADC wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<ADC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_PD`"]
pub type ADC_PD_R = crate::R<bool, ADC_PD_A>;
impl ADC_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_PD_A {
        match self.bits {
            false => ADC_PD_A::POWERED,
            true => ADC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == ADC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == ADC_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `ADC_PD`"]
pub struct ADC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(ADC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(ADC_PD_A::POWERED_DOWN)
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
#[doc = "Crystal oscillator wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSOSC_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<SYSOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSOSC_PD`"]
pub type SYSOSC_PD_R = crate::R<bool, SYSOSC_PD_A>;
impl SYSOSC_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSOSC_PD_A {
        match self.bits {
            false => SYSOSC_PD_A::POWERED,
            true => SYSOSC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == SYSOSC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == SYSOSC_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `SYSOSC_PD`"]
pub struct SYSOSC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSOSC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSOSC_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSOSC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSOSC_PD_A::POWERED_DOWN)
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
#[doc = "Watchdog oscillator wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<WDTOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDTOSC_PD`"]
pub type WDTOSC_PD_R = crate::R<bool, WDTOSC_PD_A>;
impl WDTOSC_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTOSC_PD_A {
        match self.bits {
            false => WDTOSC_PD_A::POWERED,
            true => WDTOSC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == WDTOSC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == WDTOSC_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `WDTOSC_PD`"]
pub struct WDTOSC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTOSC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTOSC_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::POWERED_DOWN)
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
#[doc = "System PLL wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSPLL_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<SYSPLL_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSPLL_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSPLL_PD`"]
pub type SYSPLL_PD_R = crate::R<bool, SYSPLL_PD_A>;
impl SYSPLL_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSPLL_PD_A {
        match self.bits {
            false => SYSPLL_PD_A::POWERED,
            true => SYSPLL_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == SYSPLL_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == SYSPLL_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `SYSPLL_PD`"]
pub struct SYSPLL_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSPLL_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSPLL_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSPLL_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSPLL_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "USB PLL wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPLL_PD_A {
    #[doc = "0: Powered"]
    POWERED = 0,
    #[doc = "1: Powered down"]
    POWERED_DOWN = 1,
}
impl From<USBPLL_PD_A> for bool {
    #[inline(always)]
    fn from(variant: USBPLL_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBPLL_PD`"]
pub type USBPLL_PD_R = crate::R<bool, USBPLL_PD_A>;
impl USBPLL_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPLL_PD_A {
        match self.bits {
            false => USBPLL_PD_A::POWERED,
            true => USBPLL_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == USBPLL_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == USBPLL_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `USBPLL_PD`"]
pub struct USBPLL_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPLL_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPLL_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(USBPLL_PD_A::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(USBPLL_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "USB transceiver wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPAD_PD_A {
    #[doc = "0: USB transceiver powered"]
    USB_TRANSCEIVER_POWERED = 0,
    #[doc = "1: USB transceiver powered down"]
    USB_TRANSCEIVER_POWERED_DOWN = 1,
}
impl From<USBPAD_PD_A> for bool {
    #[inline(always)]
    fn from(variant: USBPAD_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBPAD_PD`"]
pub type USBPAD_PD_R = crate::R<bool, USBPAD_PD_A>;
impl USBPAD_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPAD_PD_A {
        match self.bits {
            false => USBPAD_PD_A::USB_TRANSCEIVER_POWERED,
            true => USBPAD_PD_A::USB_TRANSCEIVER_POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `USB_TRANSCEIVER_POWERED`"]
    #[inline(always)]
    pub fn is_usb_transceiver_powered(&self) -> bool {
        *self == USBPAD_PD_A::USB_TRANSCEIVER_POWERED
    }
    #[doc = "Checks if the value of the field is `USB_TRANSCEIVER_POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_usb_transceiver_powered_down(&self) -> bool {
        *self == USBPAD_PD_A::USB_TRANSCEIVER_POWERED_DOWN
    }
}
#[doc = "Write proxy for field `USBPAD_PD`"]
pub struct USBPAD_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPAD_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPAD_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "USB transceiver powered"]
    #[inline(always)]
    pub fn usb_transceiver_powered(self) -> &'a mut W {
        self.variant(USBPAD_PD_A::USB_TRANSCEIVER_POWERED)
    }
    #[doc = "USB transceiver powered down"]
    #[inline(always)]
    pub fn usb_transceiver_powered_down(self) -> &'a mut W {
        self.variant(USBPAD_PD_A::USB_TRANSCEIVER_POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `RESERVED_11`"]
pub type RESERVED_11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED_11`"]
pub struct RESERVED_11_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RESERVED_12`"]
pub type RESERVED_12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED_12`"]
pub struct RESERVED_12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_12_W<'a> {
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
#[doc = "Reader of field `RESERVED_13_15`"]
pub type RESERVED_13_15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED_13_15`"]
pub struct RESERVED_13_15_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_13_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `RESERVED_16_31`"]
pub type RESERVED_16_31_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED_16_31`"]
pub struct RESERVED_16_31_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_16_31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IRC oscillator output wake-up configuration"]
    #[inline(always)]
    pub fn ircout_pd(&self) -> IRCOUT_PD_R {
        IRCOUT_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRC oscillator power-down wake-up configuration"]
    #[inline(always)]
    pub fn irc_pd(&self) -> IRC_PD_R {
        IRC_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline(always)]
    pub fn flash_pd(&self) -> FLASH_PD_R {
        FLASH_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline(always)]
    pub fn adc_pd(&self) -> ADC_PD_R {
        ADC_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline(always)]
    pub fn sysosc_pd(&self) -> SYSOSC_PD_R {
        SYSOSC_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration"]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline(always)]
    pub fn syspll_pd(&self) -> SYSPLL_PD_R {
        SYSPLL_PD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB PLL wake-up configuration"]
    #[inline(always)]
    pub fn usbpll_pd(&self) -> USBPLL_PD_R {
        USBPLL_PD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB transceiver wake-up configuration"]
    #[inline(always)]
    pub fn usbpad_pd(&self) -> USBPAD_PD_R {
        USBPAD_PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reserved. Always write this bit as 0."]
    #[inline(always)]
    pub fn reserved_12(&self) -> RESERVED_12_R {
        RESERVED_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Reserved. Always write these bits as 111."]
    #[inline(always)]
    pub fn reserved_13_15(&self) -> RESERVED_13_15_R {
        RESERVED_13_15_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn reserved_16_31(&self) -> RESERVED_16_31_R {
        RESERVED_16_31_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - IRC oscillator output wake-up configuration"]
    #[inline(always)]
    pub fn ircout_pd(&mut self) -> IRCOUT_PD_W {
        IRCOUT_PD_W { w: self }
    }
    #[doc = "Bit 1 - IRC oscillator power-down wake-up configuration"]
    #[inline(always)]
    pub fn irc_pd(&mut self) -> IRC_PD_W {
        IRC_PD_W { w: self }
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline(always)]
    pub fn flash_pd(&mut self) -> FLASH_PD_W {
        FLASH_PD_W { w: self }
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline(always)]
    pub fn bod_pd(&mut self) -> BOD_PD_W {
        BOD_PD_W { w: self }
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline(always)]
    pub fn adc_pd(&mut self) -> ADC_PD_W {
        ADC_PD_W { w: self }
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline(always)]
    pub fn sysosc_pd(&mut self) -> SYSOSC_PD_W {
        SYSOSC_PD_W { w: self }
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration"]
    #[inline(always)]
    pub fn wdtosc_pd(&mut self) -> WDTOSC_PD_W {
        WDTOSC_PD_W { w: self }
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline(always)]
    pub fn syspll_pd(&mut self) -> SYSPLL_PD_W {
        SYSPLL_PD_W { w: self }
    }
    #[doc = "Bit 8 - USB PLL wake-up configuration"]
    #[inline(always)]
    pub fn usbpll_pd(&mut self) -> USBPLL_PD_W {
        USBPLL_PD_W { w: self }
    }
    #[doc = "Bit 10 - USB transceiver wake-up configuration"]
    #[inline(always)]
    pub fn usbpad_pd(&mut self) -> USBPAD_PD_W {
        USBPAD_PD_W { w: self }
    }
    #[doc = "Bit 11 - Reserved. Always write this bit as 1."]
    #[inline(always)]
    pub fn reserved_11(&mut self) -> RESERVED_11_W {
        RESERVED_11_W { w: self }
    }
    #[doc = "Bit 12 - Reserved. Always write this bit as 0."]
    #[inline(always)]
    pub fn reserved_12(&mut self) -> RESERVED_12_W {
        RESERVED_12_W { w: self }
    }
    #[doc = "Bits 13:15 - Reserved. Always write these bits as 111."]
    #[inline(always)]
    pub fn reserved_13_15(&mut self) -> RESERVED_13_15_W {
        RESERVED_13_15_W { w: self }
    }
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn reserved_16_31(&mut self) -> RESERVED_16_31_W {
        RESERVED_16_31_W { w: self }
    }
}
