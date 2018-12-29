#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTERP1 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `WWDTINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDTINTR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl WWDTINTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WWDTINTR::DISABLED => false,
            WWDTINTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WWDTINTR {
        match value {
            false => WWDTINTR::DISABLED,
            true => WWDTINTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WWDTINTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WWDTINTR::ENABLED
    }
}
#[doc = "Possible values of the field `BODINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl BODINTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BODINTR::DISABLED => false,
            BODINTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODINTR {
        match value {
            false => BODINTR::DISABLED,
            true => BODINTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BODINTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BODINTR::ENABLED
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
impl USB_WAKEUPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            USB_WAKEUPR::DISABLED => false,
            USB_WAKEUPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_WAKEUPR {
        match value {
            false => USB_WAKEUPR::DISABLED,
            true => USB_WAKEUPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == USB_WAKEUPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == USB_WAKEUPR::ENABLED
    }
}
#[doc = "Possible values of the field `GPIOINT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOINT0R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl GPIOINT0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GPIOINT0R::DISABLED => false,
            GPIOINT0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIOINT0R {
        match value {
            false => GPIOINT0R::DISABLED,
            true => GPIOINT0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOINT0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOINT0R::ENABLED
    }
}
#[doc = "Possible values of the field `GPIOINT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOINT1R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl GPIOINT1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GPIOINT1R::DISABLED => false,
            GPIOINT1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIOINT1R {
        match value {
            false => GPIOINT1R::DISABLED,
            true => GPIOINT1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOINT1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOINT1R::ENABLED
    }
}
#[doc = "Values that can be written to the field `WWDTINT`"]
pub enum WWDTINTW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl WWDTINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WWDTINTW::DISABLED => false,
            WWDTINTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WWDTINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDTINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDTINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDTINTW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDTINTW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BODINT`"]
pub enum BODINTW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl BODINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODINTW::DISABLED => false,
            BODINTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODINTW<'a> {
    w: &'a mut W,
}
impl<'a> _BODINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BODINTW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BODINTW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USB_WAKEUP`"]
pub enum USB_WAKEUPW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl USB_WAKEUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USB_WAKEUPW::DISABLED => false,
            USB_WAKEUPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB_WAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_WAKEUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_WAKEUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB_WAKEUPW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB_WAKEUPW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOINT0`"]
pub enum GPIOINT0W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl GPIOINT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIOINT0W::DISABLED => false,
            GPIOINT0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIOINT0W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOINT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOINT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOINT0W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOINT0W::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GPIOINT1`"]
pub enum GPIOINT1W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl GPIOINT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIOINT1W::DISABLED => false,
            GPIOINT1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GPIOINT1W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOINT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOINT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOINT1W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOINT1W::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 12 - WWDT interrupt wake-up"]
    #[inline]
    pub fn wwdtint(&self) -> WWDTINTR {
        WWDTINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Brown Out Detect (BOD) interrupt wake-up"]
    #[inline]
    pub fn bodint(&self) -> BODINTR {
        BODINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - USB need_clock signal wake-up"]
    #[inline]
    pub fn usb_wakeup(&self) -> USB_WAKEUPR {
        USB_WAKEUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - GPIO GROUP0 interrupt wake-up"]
    #[inline]
    pub fn gpioint0(&self) -> GPIOINT0R {
        GPIOINT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - GPIO GROUP1 interrupt wake-up"]
    #[inline]
    pub fn gpioint1(&self) -> GPIOINT1R {
        GPIOINT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 12 - WWDT interrupt wake-up"]
    #[inline]
    pub fn wwdtint(&mut self) -> _WWDTINTW {
        _WWDTINTW { w: self }
    }
    #[doc = "Bit 13 - Brown Out Detect (BOD) interrupt wake-up"]
    #[inline]
    pub fn bodint(&mut self) -> _BODINTW {
        _BODINTW { w: self }
    }
    #[doc = "Bit 19 - USB need_clock signal wake-up"]
    #[inline]
    pub fn usb_wakeup(&mut self) -> _USB_WAKEUPW {
        _USB_WAKEUPW { w: self }
    }
    #[doc = "Bit 20 - GPIO GROUP0 interrupt wake-up"]
    #[inline]
    pub fn gpioint0(&mut self) -> _GPIOINT0W {
        _GPIOINT0W { w: self }
    }
    #[doc = "Bit 21 - GPIO GROUP1 interrupt wake-up"]
    #[inline]
    pub fn gpioint1(&mut self) -> _GPIOINT1W {
        _GPIOINT1W { w: self }
    }
}
