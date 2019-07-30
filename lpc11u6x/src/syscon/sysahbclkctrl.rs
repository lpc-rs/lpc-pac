#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSAHBCLKCTRL {
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
        0x3f
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type SYS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SYSW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSW<'a> {
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
#[doc = "Possible values of the field `ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROMR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for ROMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ROMR::DISABLE => false,
            ROMR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ROM_R = crate::FR<bool, ROMR>;
impl ROM_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ROMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ROMR::ENABLE
    }
}
#[doc = "Values that can be written to the field `ROM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROMW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl ROMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ROMW::DISABLE => false,
            ROMW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ROMW<'a> {
    w: &'a mut W,
}
impl<'a> _ROMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROMW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROMW::ENABLE)
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
#[doc = "Possible values of the field `RAM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for RAM0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RAM0R::DISABLE => false,
            RAM0R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RAM0_R = crate::FR<bool, RAM0R>;
impl RAM0_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAM0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAM0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `RAM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl RAM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM0W::DISABLE => false,
            RAM0W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RAM0W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM0W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM0W::ENABLE)
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
#[doc = "Possible values of the field `FLASHREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHREGR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for FLASHREGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FLASHREGR::DISABLED => false,
            FLASHREGR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FLASHREG_R = crate::FR<bool, FLASHREGR>;
impl FLASHREG_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHREGR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHREGR::ENABLED
    }
}
#[doc = "Values that can be written to the field `FLASHREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHREGW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl FLASHREGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHREGW::DISABLED => false,
            FLASHREGW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASHREGW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHREGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHREGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHREGW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHREGW::ENABLED)
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
#[doc = "Possible values of the field `FLASHARRAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHARRAYR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for FLASHARRAYR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FLASHARRAYR::DISABLED => false,
            FLASHARRAYR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FLASHARRAY_R = crate::FR<bool, FLASHARRAYR>;
impl FLASHARRAY_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHARRAYR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHARRAYR::ENABLED
    }
}
#[doc = "Values that can be written to the field `FLASHARRAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHARRAYW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl FLASHARRAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHARRAYW::DISABLED => false,
            FLASHARRAYW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASHARRAYW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHARRAYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHARRAYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHARRAYW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHARRAYW::ENABLED)
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
#[doc = "Possible values of the field `I2C0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for I2C0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2C0R::DISABLE => false,
            I2C0R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2C0_R = crate::FR<bool, I2C0R>;
impl I2C0_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2C0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2C0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `I2C0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl I2C0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C0W::DISABLE => false,
            I2C0W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _I2C0W<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C0W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C0W::ENABLE)
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
#[doc = "Possible values of the field `GPIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for GPIOR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            GPIOR::DISABLE => false,
            GPIOR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type GPIO_R = crate::FR<bool, GPIOR>;
impl GPIO_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIOR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIOR::ENABLE
    }
}
#[doc = "Values that can be written to the field `GPIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl GPIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            GPIOW::DISABLE => false,
            GPIOW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _GPIOW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIOW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIOW::ENABLE)
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
#[doc = "Possible values of the field `CT16B0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B0R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for CT16B0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CT16B0R::DISABLE => false,
            CT16B0R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CT16B0_R = crate::FR<bool, CT16B0R>;
impl CT16B0_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT16B0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT16B0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `CT16B0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B0W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT16B0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CT16B0W::DISABLE => false,
            CT16B0W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CT16B0W<'a> {
    w: &'a mut W,
}
impl<'a> _CT16B0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT16B0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT16B0W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT16B0W::ENABLE)
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
#[doc = "Possible values of the field `CT16B1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B1R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for CT16B1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CT16B1R::DISABLE => false,
            CT16B1R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CT16B1_R = crate::FR<bool, CT16B1R>;
impl CT16B1_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT16B1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT16B1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `CT16B1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B1W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT16B1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CT16B1W::DISABLE => false,
            CT16B1W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CT16B1W<'a> {
    w: &'a mut W,
}
impl<'a> _CT16B1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT16B1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT16B1W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT16B1W::ENABLE)
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
#[doc = "Possible values of the field `CT32B0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B0R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for CT32B0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CT32B0R::DISABLE => false,
            CT32B0R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CT32B0_R = crate::FR<bool, CT32B0R>;
impl CT32B0_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT32B0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT32B0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `CT32B0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B0W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT32B0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CT32B0W::DISABLE => false,
            CT32B0W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CT32B0W<'a> {
    w: &'a mut W,
}
impl<'a> _CT32B0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT32B0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32B0W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32B0W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `CT32B1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B1R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for CT32B1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CT32B1R::DISABLE => false,
            CT32B1R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CT32B1_R = crate::FR<bool, CT32B1R>;
impl CT32B1_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CT32B1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CT32B1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `CT32B1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B1W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CT32B1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CT32B1W::DISABLE => false,
            CT32B1W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CT32B1W<'a> {
    w: &'a mut W,
}
impl<'a> _CT32B1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT32B1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CT32B1W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CT32B1W::ENABLE)
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
#[doc = "Possible values of the field `SSP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for SSP0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SSP0R::DISABLE => false,
            SSP0R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SSP0_R = crate::FR<bool, SSP0R>;
impl SSP0_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SSP0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SSP0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `SSP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl SSP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP0W::DISABLE => false,
            SSP0W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSP0W<'a> {
    w: &'a mut W,
}
impl<'a> _SSP0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSP0W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSP0W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `USART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART0R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for USART0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USART0R::DISABLE => false,
            USART0R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART0_R = crate::FR<bool, USART0R>;
impl USART0_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USART0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USART0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `USART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART0W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl USART0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART0W::DISABLE => false,
            USART0W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART0W<'a> {
    w: &'a mut W,
}
impl<'a> _USART0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USART0W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USART0W::ENABLE)
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
#[doc = "Possible values of the field `ADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for ADCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADCR::DISABLE => false,
            ADCR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADC_R = crate::FR<bool, ADCR>;
impl ADC_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADCR::ENABLE
    }
}
#[doc = "Values that can be written to the field `ADC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl ADCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCW::DISABLE => false,
            ADCW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADCW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADCW::ENABLE)
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
#[doc = "Possible values of the field `USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for USBR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USBR::DISABLE => false,
            USBR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USB_R = crate::FR<bool, USBR>;
impl USB_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USBR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USBR::ENABLE
    }
}
#[doc = "Values that can be written to the field `USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl USBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USBW::DISABLE => false,
            USBW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USBW<'a> {
    w: &'a mut W,
}
impl<'a> _USBW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USBW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USBW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `WWDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDTR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for WWDTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WWDTR::DISABLE => false,
            WWDTR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WWDT_R = crate::FR<bool, WWDTR>;
impl WWDT_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WWDTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WWDTR::ENABLE
    }
}
#[doc = "Values that can be written to the field `WWDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDTW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl WWDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WWDTW::DISABLE => false,
            WWDTW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WWDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WWDTW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WWDTW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `IOCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCONR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for IOCONR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IOCONR::DISABLE => false,
            IOCONR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IOCON_R = crate::FR<bool, IOCONR>;
impl IOCON_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IOCONR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IOCONR::ENABLE
    }
}
#[doc = "Values that can be written to the field `IOCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCONW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl IOCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            IOCONW::DISABLE => false,
            IOCONW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _IOCONW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCONW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IOCONW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IOCONW::ENABLE)
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
#[doc = "Possible values of the field `SSP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for SSP1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SSP1R::DISABLE => false,
            SSP1R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SSP1_R = crate::FR<bool, SSP1R>;
impl SSP1_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SSP1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SSP1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `SSP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl SSP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP1W::DISABLE => false,
            SSP1W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSP1W<'a> {
    w: &'a mut W,
}
impl<'a> _SSP1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SSP1W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SSP1W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `PINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINTR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for PINTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PINTR::DISABLE => false,
            PINTR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PINT_R = crate::FR<bool, PINTR>;
impl PINT_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PINTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PINTR::ENABLE
    }
}
#[doc = "Values that can be written to the field `PINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINTW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl PINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PINTW::DISABLE => false,
            PINTW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PINTW<'a> {
    w: &'a mut W,
}
impl<'a> _PINTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PINTW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PINTW::ENABLE)
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
#[doc = "Possible values of the field `USART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for USART1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USART1R::DISABLE => false,
            USART1R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART1_R = crate::FR<bool, USART1R>;
impl USART1_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USART1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USART1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `USART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl USART1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART1W::DISABLE => false,
            USART1W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART1W<'a> {
    w: &'a mut W,
}
impl<'a> _USART1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USART1W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USART1W::ENABLE)
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
#[doc = "Possible values of the field `USART2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for USART2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USART2R::DISABLE => false,
            USART2R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART2_R = crate::FR<bool, USART2R>;
impl USART2_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USART2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USART2R::ENABLE
    }
}
#[doc = "Values that can be written to the field `USART2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl USART2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART2W::DISABLE => false,
            USART2W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART2W<'a> {
    w: &'a mut W,
}
impl<'a> _USART2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USART2W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USART2W::ENABLE)
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
#[doc = "Possible values of the field `USART3_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART3_4R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for USART3_4R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USART3_4R::DISABLE => false,
            USART3_4R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART3_4_R = crate::FR<bool, USART3_4R>;
impl USART3_4_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USART3_4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USART3_4R::ENABLE
    }
}
#[doc = "Values that can be written to the field `USART3_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART3_4W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl USART3_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART3_4W::DISABLE => false,
            USART3_4W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART3_4W<'a> {
    w: &'a mut W,
}
impl<'a> _USART3_4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3_4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USART3_4W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USART3_4W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `GROUP0INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP0INTR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for GROUP0INTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            GROUP0INTR::DISABLE => false,
            GROUP0INTR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type GROUP0INT_R = crate::FR<bool, GROUP0INTR>;
impl GROUP0INT_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GROUP0INTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GROUP0INTR::ENABLE
    }
}
#[doc = "Values that can be written to the field `GROUP0INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP0INTW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl GROUP0INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            GROUP0INTW::DISABLE => false,
            GROUP0INTW::ENABLE => true,
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GROUP0INTW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GROUP0INTW::ENABLE)
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
#[doc = "Possible values of the field `GROUP1INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP1INTR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for GROUP1INTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            GROUP1INTR::DISABLE => false,
            GROUP1INTR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type GROUP1INT_R = crate::FR<bool, GROUP1INTR>;
impl GROUP1INT_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GROUP1INTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GROUP1INTR::ENABLE
    }
}
#[doc = "Values that can be written to the field `GROUP1INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP1INTW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl GROUP1INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            GROUP1INTW::DISABLE => false,
            GROUP1INTW::ENABLE => true,
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GROUP1INTW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GROUP1INTW::ENABLE)
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
#[doc = "Possible values of the field `I2C1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for I2C1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2C1R::DISABLE => false,
            I2C1R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2C1_R = crate::FR<bool, I2C1R>;
impl I2C1_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2C1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2C1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `I2C1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl I2C1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C1W::DISABLE => false,
            I2C1W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _I2C1W<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C1W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C1W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Possible values of the field `RAM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM1R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for RAM1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RAM1R::DISABLE => false,
            RAM1R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RAM1_R = crate::FR<bool, RAM1R>;
impl RAM1_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAM1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAM1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `RAM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM1W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl RAM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM1W::DISABLE => false,
            RAM1W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RAM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM1W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM1W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `USBSRAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSRAMR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for USBSRAMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USBSRAMR::DISABLE => false,
            USBSRAMR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USBSRAM_R = crate::FR<bool, USBSRAMR>;
impl USBSRAM_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USBSRAMR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USBSRAMR::ENABLE
    }
}
#[doc = "Values that can be written to the field `USBSRAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSRAMW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl USBSRAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USBSRAMW::DISABLE => false,
            USBSRAMW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USBSRAMW<'a> {
    w: &'a mut W,
}
impl<'a> _USBSRAMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSRAMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USBSRAMW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USBSRAMW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Possible values of the field `CRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for CRCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CRCR::DISABLE => false,
            CRCR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CRC_R = crate::FR<bool, CRCR>;
impl CRC_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CRCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CRCR::ENABLE
    }
}
#[doc = "Values that can be written to the field `CRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl CRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCW::DISABLE => false,
            CRCW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRCW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for DMAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DMAR::DISABLE => false,
            DMAR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DMA_R = crate::FR<bool, DMAR>;
impl DMA_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAR::ENABLE
    }
}
#[doc = "Values that can be written to the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAW::DISABLE => false,
            DMAW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Possible values of the field `RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCR {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for RTCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RTCR::DISABLE => false,
            RTCR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RTC_R = crate::FR<bool, RTCR>;
impl RTC_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTCR::ENABLE
    }
}
#[doc = "Values that can be written to the field `RTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCW {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl RTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCW::DISABLE => false,
            RTCW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTCW::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Possible values of the field `SCT0_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT0_1R {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl crate::ToBits<bool> for SCT0_1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SCT0_1R::DISABLE => false,
            SCT0_1R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SCT0_1_R = crate::FR<bool, SCT0_1R>;
impl SCT0_1_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SCT0_1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SCT0_1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `SCT0_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT0_1W {
    #[doc = "Disable"]
    DISABLE,
    #[doc = "Enable"]
    ENABLE,
}
impl SCT0_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SCT0_1W::DISABLE => false,
            SCT0_1W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SCT0_1W<'a> {
    w: &'a mut W,
}
impl<'a> _SCT0_1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT0_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT0_1W::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT0_1W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - This bit is read-only and always reads as 1. It configures the always-on clock for the AHB, the APB bridges, the Cortex-M0 core clocks, SYSCON, reset control, SRAM0, and the PMU. Writes to this bit are ignored."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables clock for Main SRAM0."]
    #[inline(always)]
    pub fn ram0(&self) -> RAM0_R {
        RAM0_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&self) -> FLASHREG_R {
        FLASHREG_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables clock for flash access."]
    #[inline(always)]
    pub fn flasharray(&self) -> FLASHARRAY_R {
        FLASHARRAY_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables clock for GPIO port registers."]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct16b0(&self) -> CT16B0_R {
        CT16B0_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct16b1(&self) -> CT16B1_R {
        CT16B1_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct32b0(&self) -> CT32B0_R {
        CT32B0_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct32b1(&self) -> CT32B1_R {
        CT32B1_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables clock for SSP0."]
    #[inline(always)]
    pub fn ssp0(&self) -> SSP0_R {
        SSP0_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables clock for USART0."]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables clock to the USB register interface."]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables clock for I/O configuration block."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables clock for SSP1."]
    #[inline(always)]
    pub fn ssp1(&self) -> SSP1_R {
        SSP1_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables clock to GPIO Pin interrupt register interface."]
    #[inline(always)]
    pub fn pint(&self) -> PINT_R {
        PINT_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables clock to USART1 register interface."]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables clock to USART2 register interface."]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables clock to USART3 and USART4 register interfaces."]
    #[inline(always)]
    pub fn usart3_4(&self) -> USART3_4_R {
        USART3_4_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables clock to GPIO GROUP0 interrupt register interface."]
    #[inline(always)]
    pub fn group0int(&self) -> GROUP0INT_R {
        GROUP0INT_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enables clock to GPIO GROUP1 interrupt register interface."]
    #[inline(always)]
    pub fn group1int(&self) -> GROUP1INT_R {
        GROUP1INT_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enables clock for I2C1."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables clock for SRAM1 located at 0x2000 0000 to 0x2000 0800."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables USB SRAM/SRAM2 block located at 0x2000 4000 to 0x2000 4800."]
    #[inline(always)]
    pub fn usbsram(&self) -> USBSRAM_R {
        USBSRAM_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enables clock for CRC."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enables clock for DMA."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enables clock for RTC register interface."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables clock for SCT0 and SCT1."]
    #[inline(always)]
    pub fn sct0_1(&self) -> SCT0_1_R {
        SCT0_1_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - This bit is read-only and always reads as 1. It configures the always-on clock for the AHB, the APB bridges, the Cortex-M0 core clocks, SYSCON, reset control, SRAM0, and the PMU. Writes to this bit are ignored."]
    #[inline(always)]
    pub fn sys(&mut self) -> _SYSW {
        _SYSW { w: self }
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&mut self) -> _ROMW {
        _ROMW { w: self }
    }
    #[doc = "Bit 2 - Enables clock for Main SRAM0."]
    #[inline(always)]
    pub fn ram0(&mut self) -> _RAM0W {
        _RAM0W { w: self }
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&mut self) -> _FLASHREGW {
        _FLASHREGW { w: self }
    }
    #[doc = "Bit 4 - Enables clock for flash access."]
    #[inline(always)]
    pub fn flasharray(&mut self) -> _FLASHARRAYW {
        _FLASHARRAYW { w: self }
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> _I2C0W {
        _I2C0W { w: self }
    }
    #[doc = "Bit 6 - Enables clock for GPIO port registers."]
    #[inline(always)]
    pub fn gpio(&mut self) -> _GPIOW {
        _GPIOW { w: self }
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct16b0(&mut self) -> _CT16B0W {
        _CT16B0W { w: self }
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct16b1(&mut self) -> _CT16B1W {
        _CT16B1W { w: self }
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct32b0(&mut self) -> _CT32B0W {
        _CT32B0W { w: self }
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct32b1(&mut self) -> _CT32B1W {
        _CT32B1W { w: self }
    }
    #[doc = "Bit 11 - Enables clock for SSP0."]
    #[inline(always)]
    pub fn ssp0(&mut self) -> _SSP0W {
        _SSP0W { w: self }
    }
    #[doc = "Bit 12 - Enables clock for USART0."]
    #[inline(always)]
    pub fn usart0(&mut self) -> _USART0W {
        _USART0W { w: self }
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline(always)]
    pub fn adc(&mut self) -> _ADCW {
        _ADCW { w: self }
    }
    #[doc = "Bit 14 - Enables clock to the USB register interface."]
    #[inline(always)]
    pub fn usb(&mut self) -> _USBW {
        _USBW { w: self }
    }
    #[doc = "Bit 15 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> _WWDTW {
        _WWDTW { w: self }
    }
    #[doc = "Bit 16 - Enables clock for I/O configuration block."]
    #[inline(always)]
    pub fn iocon(&mut self) -> _IOCONW {
        _IOCONW { w: self }
    }
    #[doc = "Bit 18 - Enables clock for SSP1."]
    #[inline(always)]
    pub fn ssp1(&mut self) -> _SSP1W {
        _SSP1W { w: self }
    }
    #[doc = "Bit 19 - Enables clock to GPIO Pin interrupt register interface."]
    #[inline(always)]
    pub fn pint(&mut self) -> _PINTW {
        _PINTW { w: self }
    }
    #[doc = "Bit 20 - Enables clock to USART1 register interface."]
    #[inline(always)]
    pub fn usart1(&mut self) -> _USART1W {
        _USART1W { w: self }
    }
    #[doc = "Bit 21 - Enables clock to USART2 register interface."]
    #[inline(always)]
    pub fn usart2(&mut self) -> _USART2W {
        _USART2W { w: self }
    }
    #[doc = "Bit 22 - Enables clock to USART3 and USART4 register interfaces."]
    #[inline(always)]
    pub fn usart3_4(&mut self) -> _USART3_4W {
        _USART3_4W { w: self }
    }
    #[doc = "Bit 23 - Enables clock to GPIO GROUP0 interrupt register interface."]
    #[inline(always)]
    pub fn group0int(&mut self) -> _GROUP0INTW {
        _GROUP0INTW { w: self }
    }
    #[doc = "Bit 24 - Enables clock to GPIO GROUP1 interrupt register interface."]
    #[inline(always)]
    pub fn group1int(&mut self) -> _GROUP1INTW {
        _GROUP1INTW { w: self }
    }
    #[doc = "Bit 25 - Enables clock for I2C1."]
    #[inline(always)]
    pub fn i2c1(&mut self) -> _I2C1W {
        _I2C1W { w: self }
    }
    #[doc = "Bit 26 - Enables clock for SRAM1 located at 0x2000 0000 to 0x2000 0800."]
    #[inline(always)]
    pub fn ram1(&mut self) -> _RAM1W {
        _RAM1W { w: self }
    }
    #[doc = "Bit 27 - Enables USB SRAM/SRAM2 block located at 0x2000 4000 to 0x2000 4800."]
    #[inline(always)]
    pub fn usbsram(&mut self) -> _USBSRAMW {
        _USBSRAMW { w: self }
    }
    #[doc = "Bit 28 - Enables clock for CRC."]
    #[inline(always)]
    pub fn crc(&mut self) -> _CRCW {
        _CRCW { w: self }
    }
    #[doc = "Bit 29 - Enables clock for DMA."]
    #[inline(always)]
    pub fn dma(&mut self) -> _DMAW {
        _DMAW { w: self }
    }
    #[doc = "Bit 30 - Enables clock for RTC register interface."]
    #[inline(always)]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
    #[doc = "Bit 31 - Enables clock for SCT0 and SCT1."]
    #[inline(always)]
    pub fn sct0_1(&mut self) -> _SCT0_1W {
        _SCT0_1W { w: self }
    }
}
