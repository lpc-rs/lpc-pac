#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDAWAKECFG {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `IRCOUT_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCOUT_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl crate::ToBits<bool> for IRCOUT_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IRCOUT_PDR::POWERED_DOWN => true,
            IRCOUT_PDR::POWERED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IRCOUT_PD_R = crate::FR<bool, IRCOUT_PDR>;
impl IRCOUT_PD_R {
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == IRCOUT_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == IRCOUT_PDR::POWERED
    }
}
#[doc = "Values that can be written to the field `IRCOUT_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCOUT_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl IRCOUT_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            IRCOUT_PDW::POWERED_DOWN => true,
            IRCOUT_PDW::POWERED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _IRCOUT_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _IRCOUT_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRCOUT_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(IRCOUT_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(IRCOUT_PDW::POWERED)
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
#[doc = "Possible values of the field `IRC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl crate::ToBits<bool> for IRC_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IRC_PDR::POWERED_DOWN => true,
            IRC_PDR::POWERED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IRC_PD_R = crate::FR<bool, IRC_PDR>;
impl IRC_PD_R {
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == IRC_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == IRC_PDR::POWERED
    }
}
#[doc = "Values that can be written to the field `IRC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl IRC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            IRC_PDW::POWERED_DOWN => true,
            IRC_PDW::POWERED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _IRC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _IRC_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(IRC_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(IRC_PDW::POWERED)
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
#[doc = "Possible values of the field `FLASH_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl crate::ToBits<bool> for FLASH_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FLASH_PDR::POWERED_DOWN => true,
            FLASH_PDR::POWERED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FLASH_PD_R = crate::FR<bool, FLASH_PDR>;
impl FLASH_PD_R {
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == FLASH_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == FLASH_PDR::POWERED
    }
}
#[doc = "Values that can be written to the field `FLASH_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl FLASH_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH_PDW::POWERED_DOWN => true,
            FLASH_PDW::POWERED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FLASH_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(FLASH_PDW::POWERED)
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
#[doc = "Possible values of the field `BOD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl crate::ToBits<bool> for BOD_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BOD_PDR::POWERED_DOWN => true,
            BOD_PDR::POWERED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BOD_PD_R = crate::FR<bool, BOD_PDR>;
impl BOD_PD_R {
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PDR::POWERED
    }
}
#[doc = "Values that can be written to the field `BOD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl BOD_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BOD_PDW::POWERED_DOWN => true,
            BOD_PDW::POWERED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BOD_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOD_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PDW::POWERED)
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
#[doc = "Possible values of the field `ADC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl crate::ToBits<bool> for ADC_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADC_PDR::POWERED_DOWN => true,
            ADC_PDR::POWERED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADC_PD_R = crate::FR<bool, ADC_PDR>;
impl ADC_PD_R {
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == ADC_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == ADC_PDR::POWERED
    }
}
#[doc = "Values that can be written to the field `ADC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl ADC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_PDW::POWERED_DOWN => true,
            ADC_PDW::POWERED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(ADC_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(ADC_PDW::POWERED)
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
#[doc = "Possible values of the field `SYSOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSOSC_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl crate::ToBits<bool> for SYSOSC_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SYSOSC_PDR::POWERED_DOWN => true,
            SYSOSC_PDR::POWERED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SYSOSC_PD_R = crate::FR<bool, SYSOSC_PDR>;
impl SYSOSC_PD_R {
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == SYSOSC_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == SYSOSC_PDR::POWERED
    }
}
#[doc = "Values that can be written to the field `SYSOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSOSC_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl SYSOSC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSOSC_PDW::POWERED_DOWN => true,
            SYSOSC_PDW::POWERED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSOSC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSOSC_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSOSC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSOSC_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSOSC_PDW::POWERED)
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
#[doc = "Possible values of the field `WDTOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl crate::ToBits<bool> for WDTOSC_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WDTOSC_PDR::POWERED_DOWN => true,
            WDTOSC_PDR::POWERED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDTOSC_PD_R = crate::FR<bool, WDTOSC_PDR>;
impl WDTOSC_PD_R {
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == WDTOSC_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == WDTOSC_PDR::POWERED
    }
}
#[doc = "Values that can be written to the field `WDTOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl WDTOSC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTOSC_PDW::POWERED_DOWN => true,
            WDTOSC_PDW::POWERED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WDTOSC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTOSC_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTOSC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(WDTOSC_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(WDTOSC_PDW::POWERED)
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
#[doc = "Possible values of the field `SYSPLL_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSPLL_PDR {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl crate::ToBits<bool> for SYSPLL_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SYSPLL_PDR::POWERED_DOWN => true,
            SYSPLL_PDR::POWERED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SYSPLL_PD_R = crate::FR<bool, SYSPLL_PDR>;
impl SYSPLL_PD_R {
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == SYSPLL_PDR::POWERED_DOWN
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == SYSPLL_PDR::POWERED
    }
}
#[doc = "Values that can be written to the field `SYSPLL_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSPLL_PDW {
    #[doc = "Powered down"]
    POWERED_DOWN,
    #[doc = "Powered"]
    POWERED,
}
impl SYSPLL_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSPLL_PDW::POWERED_DOWN => true,
            SYSPLL_PDW::POWERED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSPLL_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSPLL_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSPLL_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSPLL_PDW::POWERED_DOWN)
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSPLL_PDW::POWERED)
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
#[doc = "Possible values of the field `USBPLL_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPLL_PDR {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl crate::ToBits<bool> for USBPLL_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USBPLL_PDR::POWERED => false,
            USBPLL_PDR::POWERED_DOWN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USBPLL_PD_R = crate::FR<bool, USBPLL_PDR>;
impl USBPLL_PD_R {
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == USBPLL_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == USBPLL_PDR::POWERED_DOWN
    }
}
#[doc = "Values that can be written to the field `USBPLL_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPLL_PDW {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl USBPLL_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USBPLL_PDW::POWERED => false,
            USBPLL_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USBPLL_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _USBPLL_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPLL_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(USBPLL_PDW::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(USBPLL_PDW::POWERED_DOWN)
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
#[doc = "Possible values of the field `USBPAD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPAD_PDR {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl crate::ToBits<bool> for USBPAD_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USBPAD_PDR::POWERED => false,
            USBPAD_PDR::POWERED_DOWN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USBPAD_PD_R = crate::FR<bool, USBPAD_PDR>;
impl USBPAD_PD_R {
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == USBPAD_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == USBPAD_PDR::POWERED_DOWN
    }
}
#[doc = "Values that can be written to the field `USBPAD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPAD_PDW {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl USBPAD_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USBPAD_PDW::POWERED => false,
            USBPAD_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USBPAD_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _USBPAD_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPAD_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(USBPAD_PDW::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(USBPAD_PDW::POWERED_DOWN)
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
#[doc = "Possible values of the field `TEMPSENSE_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPSENSE_PDR {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl crate::ToBits<bool> for TEMPSENSE_PDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TEMPSENSE_PDR::POWERED => false,
            TEMPSENSE_PDR::POWERED_DOWN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TEMPSENSE_PD_R = crate::FR<bool, TEMPSENSE_PDR>;
impl TEMPSENSE_PD_R {
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == TEMPSENSE_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == TEMPSENSE_PDR::POWERED_DOWN
    }
}
#[doc = "Values that can be written to the field `TEMPSENSE_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPSENSE_PDW {
    #[doc = "Powered"]
    POWERED,
    #[doc = "Powered down"]
    POWERED_DOWN,
}
impl TEMPSENSE_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TEMPSENSE_PDW::POWERED => false,
            TEMPSENSE_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TEMPSENSE_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMPSENSE_PDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEMPSENSE_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(TEMPSENSE_PDW::POWERED)
    }
    #[doc = "Powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(TEMPSENSE_PDW::POWERED_DOWN)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - IRC oscillator output wake-up configuration"]
    #[inline(always)]
    pub fn ircout_pd(&self) -> IRCOUT_PD_R {
        IRCOUT_PD_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRC oscillator power-down wake-up configuration"]
    #[inline(always)]
    pub fn irc_pd(&self) -> IRC_PD_R {
        IRC_PD_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline(always)]
    pub fn flash_pd(&self) -> FLASH_PD_R {
        FLASH_PD_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline(always)]
    pub fn adc_pd(&self) -> ADC_PD_R {
        ADC_PD_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline(always)]
    pub fn sysosc_pd(&self) -> SYSOSC_PD_R {
        SYSOSC_PD_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration"]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline(always)]
    pub fn syspll_pd(&self) -> SYSPLL_PD_R {
        SYSPLL_PD_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB PLL wake-up configuration"]
    #[inline(always)]
    pub fn usbpll_pd(&self) -> USBPLL_PD_R {
        USBPLL_PD_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USB transceiver wake-up configuration"]
    #[inline(always)]
    pub fn usbpad_pd(&self) -> USBPAD_PD_R {
        USBPAD_PD_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Temperature sensor wake-up configuration"]
    #[inline(always)]
    pub fn tempsense_pd(&self) -> TEMPSENSE_PD_R {
        TEMPSENSE_PD_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - IRC oscillator output wake-up configuration"]
    #[inline(always)]
    pub fn ircout_pd(&mut self) -> _IRCOUT_PDW {
        _IRCOUT_PDW { w: self }
    }
    #[doc = "Bit 1 - IRC oscillator power-down wake-up configuration"]
    #[inline(always)]
    pub fn irc_pd(&mut self) -> _IRC_PDW {
        _IRC_PDW { w: self }
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline(always)]
    pub fn flash_pd(&mut self) -> _FLASH_PDW {
        _FLASH_PDW { w: self }
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline(always)]
    pub fn bod_pd(&mut self) -> _BOD_PDW {
        _BOD_PDW { w: self }
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline(always)]
    pub fn adc_pd(&mut self) -> _ADC_PDW {
        _ADC_PDW { w: self }
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline(always)]
    pub fn sysosc_pd(&mut self) -> _SYSOSC_PDW {
        _SYSOSC_PDW { w: self }
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration"]
    #[inline(always)]
    pub fn wdtosc_pd(&mut self) -> _WDTOSC_PDW {
        _WDTOSC_PDW { w: self }
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline(always)]
    pub fn syspll_pd(&mut self) -> _SYSPLL_PDW {
        _SYSPLL_PDW { w: self }
    }
    #[doc = "Bit 8 - USB PLL wake-up configuration"]
    #[inline(always)]
    pub fn usbpll_pd(&mut self) -> _USBPLL_PDW {
        _USBPLL_PDW { w: self }
    }
    #[doc = "Bit 10 - USB transceiver wake-up configuration"]
    #[inline(always)]
    pub fn usbpad_pd(&mut self) -> _USBPAD_PDW {
        _USBPAD_PDW { w: self }
    }
    #[doc = "Bit 13 - Temperature sensor wake-up configuration"]
    #[inline(always)]
    pub fn tempsense_pd(&mut self) -> _TEMPSENSE_PDW {
        _TEMPSENSE_PDW { w: self }
    }
}
