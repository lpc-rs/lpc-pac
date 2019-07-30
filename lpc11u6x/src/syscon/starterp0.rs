#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTERP0 {
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
#[doc = "Possible values of the field `PINT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT0R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for PINT0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PINT0R::DISABLED => false,
            PINT0R::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PINT0_R = crate::FR<bool, PINT0R>;
impl PINT0_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT0R::ENABLED
    }
}
#[doc = "Values that can be written to the field `PINT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT0W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PINT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PINT0W::DISABLED => false,
            PINT0W::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PINT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT0W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT0W::ENABLED)
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
#[doc = "Possible values of the field `PINT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT1R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for PINT1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PINT1R::DISABLED => false,
            PINT1R::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PINT1_R = crate::FR<bool, PINT1R>;
impl PINT1_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT1R::ENABLED
    }
}
#[doc = "Values that can be written to the field `PINT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT1W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PINT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PINT1W::DISABLED => false,
            PINT1W::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PINT1W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT1W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT1W::ENABLED)
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
#[doc = "Possible values of the field `PINT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT2R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for PINT2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PINT2R::DISABLED => false,
            PINT2R::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PINT2_R = crate::FR<bool, PINT2R>;
impl PINT2_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT2R::ENABLED
    }
}
#[doc = "Values that can be written to the field `PINT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT2W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PINT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PINT2W::DISABLED => false,
            PINT2W::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PINT2W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT2W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT2W::ENABLED)
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
#[doc = "Possible values of the field `PINT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT3R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for PINT3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PINT3R::DISABLED => false,
            PINT3R::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PINT3_R = crate::FR<bool, PINT3R>;
impl PINT3_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT3R::ENABLED
    }
}
#[doc = "Values that can be written to the field `PINT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT3W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PINT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PINT3W::DISABLED => false,
            PINT3W::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PINT3W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT3W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT3W::ENABLED)
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
#[doc = "Possible values of the field `PINT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT4R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for PINT4R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PINT4R::DISABLED => false,
            PINT4R::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PINT4_R = crate::FR<bool, PINT4R>;
impl PINT4_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT4R::ENABLED
    }
}
#[doc = "Values that can be written to the field `PINT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT4W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PINT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PINT4W::DISABLED => false,
            PINT4W::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PINT4W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT4W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT4W::ENABLED)
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
#[doc = "Possible values of the field `PINT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT5R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for PINT5R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PINT5R::DISABLED => false,
            PINT5R::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PINT5_R = crate::FR<bool, PINT5R>;
impl PINT5_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT5R::ENABLED
    }
}
#[doc = "Values that can be written to the field `PINT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT5W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PINT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PINT5W::DISABLED => false,
            PINT5W::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PINT5W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT5W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT5W::ENABLED)
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
#[doc = "Possible values of the field `PINT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT6R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for PINT6R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PINT6R::DISABLED => false,
            PINT6R::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PINT6_R = crate::FR<bool, PINT6R>;
impl PINT6_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT6R::ENABLED
    }
}
#[doc = "Values that can be written to the field `PINT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT6W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PINT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PINT6W::DISABLED => false,
            PINT6W::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PINT6W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT6W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT6W::ENABLED)
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
#[doc = "Possible values of the field `PINT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT7R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl crate::ToBits<bool> for PINT7R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PINT7R::DISABLED => false,
            PINT7R::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PINT7_R = crate::FR<bool, PINT7R>;
impl PINT7_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT7R::ENABLED
    }
}
#[doc = "Values that can be written to the field `PINT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT7W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PINT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PINT7W::DISABLED => false,
            PINT7W::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PINT7W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT7W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT7W::ENABLED)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Pin interrupt 0 wake-up"]
    #[inline(always)]
    pub fn pint0(&self) -> PINT0_R {
        PINT0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pin interrupt 1 wake-up"]
    #[inline(always)]
    pub fn pint1(&self) -> PINT1_R {
        PINT1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pin interrupt 2 wake-up"]
    #[inline(always)]
    pub fn pint2(&self) -> PINT2_R {
        PINT2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pin interrupt 3 wake-up"]
    #[inline(always)]
    pub fn pint3(&self) -> PINT3_R {
        PINT3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pin interrupt 4 wake-up"]
    #[inline(always)]
    pub fn pint4(&self) -> PINT4_R {
        PINT4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pin interrupt 5 wake-up"]
    #[inline(always)]
    pub fn pint5(&self) -> PINT5_R {
        PINT5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pin interrupt 6 wake-up"]
    #[inline(always)]
    pub fn pint6(&self) -> PINT6_R {
        PINT6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pin interrupt 7 wake-up"]
    #[inline(always)]
    pub fn pint7(&self) -> PINT7_R {
        PINT7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Pin interrupt 0 wake-up"]
    #[inline(always)]
    pub fn pint0(&mut self) -> _PINT0W {
        _PINT0W { w: self }
    }
    #[doc = "Bit 1 - Pin interrupt 1 wake-up"]
    #[inline(always)]
    pub fn pint1(&mut self) -> _PINT1W {
        _PINT1W { w: self }
    }
    #[doc = "Bit 2 - Pin interrupt 2 wake-up"]
    #[inline(always)]
    pub fn pint2(&mut self) -> _PINT2W {
        _PINT2W { w: self }
    }
    #[doc = "Bit 3 - Pin interrupt 3 wake-up"]
    #[inline(always)]
    pub fn pint3(&mut self) -> _PINT3W {
        _PINT3W { w: self }
    }
    #[doc = "Bit 4 - Pin interrupt 4 wake-up"]
    #[inline(always)]
    pub fn pint4(&mut self) -> _PINT4W {
        _PINT4W { w: self }
    }
    #[doc = "Bit 5 - Pin interrupt 5 wake-up"]
    #[inline(always)]
    pub fn pint5(&mut self) -> _PINT5W {
        _PINT5W { w: self }
    }
    #[doc = "Bit 6 - Pin interrupt 6 wake-up"]
    #[inline(always)]
    pub fn pint6(&mut self) -> _PINT6W {
        _PINT6W { w: self }
    }
    #[doc = "Bit 7 - Pin interrupt 7 wake-up"]
    #[inline(always)]
    pub fn pint7(&mut self) -> _PINT7W {
        _PINT7W { w: self }
    }
}
