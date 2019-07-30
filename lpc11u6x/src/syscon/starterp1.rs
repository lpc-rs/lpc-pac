#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTERP1 {
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
#[doc = "Possible values of the field `RTCINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCINTR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for RTCINTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RTCINTR::DISABLED => false,
            RTCINTR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RTCINT_R = crate::FR<bool, RTCINTR>;
impl RTCINT_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCINTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCINTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `RTCINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCINTW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl RTCINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCINTW::DISABLED => false,
            RTCINTW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RTCINTW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCINTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCINTW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCINTW::ENABLED)
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
#[doc = "Possible values of the field `WWDT_BODINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_BODINTR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for WWDT_BODINTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WWDT_BODINTR::DISABLED => false,
            WWDT_BODINTR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WWDT_BODINT_R = crate::FR<bool, WWDT_BODINTR>;
impl WWDT_BODINT_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WWDT_BODINTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WWDT_BODINTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `WWDT_BODINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_BODINTW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl WWDT_BODINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WWDT_BODINTW::DISABLED => false,
            WWDT_BODINTW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WWDT_BODINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_BODINTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDT_BODINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDT_BODINTW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDT_BODINTW::ENABLED)
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
#[doc = "Possible values of the field `USB_WAKEUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_WAKEUPR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for USB_WAKEUPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USB_WAKEUPR::DISABLED => false,
            USB_WAKEUPR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USB_WAKEUP_R = crate::FR<bool, USB_WAKEUPR>;
impl USB_WAKEUP_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USB_WAKEUPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USB_WAKEUPR::ENABLED
    }
}
#[doc = "Values that can be written to the field `USB_WAKEUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_WAKEUPW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl USB_WAKEUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USB_WAKEUPW::DISABLED => false,
            USB_WAKEUPW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_WAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_WAKEUPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_WAKEUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB_WAKEUPW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB_WAKEUPW::ENABLED)
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
#[doc = "Possible values of the field `GROUP0INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP0INTR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for GROUP0INTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            GROUP0INTR::DISABLED => false,
            GROUP0INTR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type GROUP0INT_R = crate::FR<bool, GROUP0INTR>;
impl GROUP0INT_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GROUP0INTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GROUP0INTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `GROUP0INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP0INTW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl GROUP0INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            GROUP0INTW::DISABLED => false,
            GROUP0INTW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _GROUP0INTW<'a> {
    w: &'a mut W,
}
impl<'a> _GROUP0INTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROUP0INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GROUP0INTW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GROUP0INTW::ENABLED)
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
#[doc = "Possible values of the field `GROUP1INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP1INTR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for GROUP1INTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            GROUP1INTR::DISABLED => false,
            GROUP1INTR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type GROUP1INT_R = crate::FR<bool, GROUP1INTR>;
impl GROUP1INT_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GROUP1INTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GROUP1INTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `GROUP1INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP1INTW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl GROUP1INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            GROUP1INTW::DISABLED => false,
            GROUP1INTW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _GROUP1INTW<'a> {
    w: &'a mut W,
}
impl<'a> _GROUP1INTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROUP1INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GROUP1INTW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GROUP1INTW::ENABLED)
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
#[doc = "Possible values of the field `USART1_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1_4R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for USART1_4R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USART1_4R::DISABLED => false,
            USART1_4R::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART1_4_R = crate::FR<bool, USART1_4R>;
impl USART1_4_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART1_4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART1_4R::ENABLED
    }
}
#[doc = "Values that can be written to the field `USART1_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1_4W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl USART1_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART1_4W::DISABLED => false,
            USART1_4W::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART1_4W<'a> {
    w: &'a mut W,
}
impl<'a> _USART1_4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1_4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART1_4W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART1_4W::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `USART2_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2_3R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for USART2_3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USART2_3R::DISABLED => false,
            USART2_3R::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART2_3_R = crate::FR<bool, USART2_3R>;
impl USART2_3_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART2_3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART2_3R::ENABLED
    }
}
#[doc = "Values that can be written to the field `USART2_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2_3W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl USART2_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART2_3W::DISABLED => false,
            USART2_3W::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART2_3W<'a> {
    w: &'a mut W,
}
impl<'a> _USART2_3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2_3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART2_3W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART2_3W::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 12 - RTC interrupt wake-up"]
    #[inline(always)]
    pub fn rtcint(&self) -> RTCINT_R {
        RTCINT_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Combined WWDT interrupt or Brown Out Detect (BOD) interrupt wake-up"]
    #[inline(always)]
    pub fn wwdt_bodint(&self) -> WWDT_BODINT_R {
        WWDT_BODINT_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 19 - USB need_clock signal wake-up"]
    #[inline(always)]
    pub fn usb_wakeup(&self) -> USB_WAKEUP_R {
        USB_WAKEUP_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GPIO GROUP0 interrupt wake-up"]
    #[inline(always)]
    pub fn group0int(&self) -> GROUP0INT_R {
        GROUP0INT_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - GPIO GROUP1 interrupt wake-up"]
    #[inline(always)]
    pub fn group1int(&self) -> GROUP1INT_R {
        GROUP1INT_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Combined USART1 and USART4 interrupt wake-up"]
    #[inline(always)]
    pub fn usart1_4(&self) -> USART1_4_R {
        USART1_4_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Combined USART2 and USART3 interrupt wake-up"]
    #[inline(always)]
    pub fn usart2_3(&self) -> USART2_3_R {
        USART2_3_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 12 - RTC interrupt wake-up"]
    #[inline(always)]
    pub fn rtcint(&mut self) -> _RTCINTW {
        _RTCINTW { w: self }
    }
    #[doc = "Bit 13 - Combined WWDT interrupt or Brown Out Detect (BOD) interrupt wake-up"]
    #[inline(always)]
    pub fn wwdt_bodint(&mut self) -> _WWDT_BODINTW {
        _WWDT_BODINTW { w: self }
    }
    #[doc = "Bit 19 - USB need_clock signal wake-up"]
    #[inline(always)]
    pub fn usb_wakeup(&mut self) -> _USB_WAKEUPW {
        _USB_WAKEUPW { w: self }
    }
    #[doc = "Bit 20 - GPIO GROUP0 interrupt wake-up"]
    #[inline(always)]
    pub fn group0int(&mut self) -> _GROUP0INTW {
        _GROUP0INTW { w: self }
    }
    #[doc = "Bit 21 - GPIO GROUP1 interrupt wake-up"]
    #[inline(always)]
    pub fn group1int(&mut self) -> _GROUP1INTW {
        _GROUP1INTW { w: self }
    }
    #[doc = "Bit 23 - Combined USART1 and USART4 interrupt wake-up"]
    #[inline(always)]
    pub fn usart1_4(&mut self) -> _USART1_4W {
        _USART1_4W { w: self }
    }
    #[doc = "Bit 24 - Combined USART2 and USART3 interrupt wake-up"]
    #[inline(always)]
    pub fn usart2_3(&mut self) -> _USART2_3W {
        _USART2_3W { w: self }
    }
}
