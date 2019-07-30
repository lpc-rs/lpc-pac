#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `MR0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0IR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR0IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR0IR::ENABLED => true,
            MR0IR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR0I_R = crate::FR<bool, MR0IR>;
impl MR0I_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR0IR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR0IR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0IW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR0IW::ENABLED => true,
            MR0IW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR0IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR0IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR0IW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR0IW::DISABLED)
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
#[doc = "Possible values of the field `MR0R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0RR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR0RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR0RR::ENABLED => true,
            MR0RR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR0R_R = crate::FR<bool, MR0RR>;
impl MR0R_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR0RR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR0RR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR0R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0RW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR0RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR0RW::ENABLED => true,
            MR0RW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR0RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR0RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR0RW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR0RW::DISABLED)
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
#[doc = "Possible values of the field `MR0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0SR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR0SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR0SR::ENABLED => true,
            MR0SR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR0S_R = crate::FR<bool, MR0SR>;
impl MR0S_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR0SR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR0SR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0SW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR0SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR0SW::ENABLED => true,
            MR0SW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR0SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR0SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR0SW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR0SW::DISABLED)
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
#[doc = "Possible values of the field `MR1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1IR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR1IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR1IR::ENABLED => true,
            MR1IR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR1I_R = crate::FR<bool, MR1IR>;
impl MR1I_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR1IR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR1IR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1IW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR1IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR1IW::ENABLED => true,
            MR1IW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR1IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR1IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR1IW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR1IW::DISABLED)
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
#[doc = "Possible values of the field `MR1R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1RR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR1RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR1RR::ENABLED => true,
            MR1RR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR1R_R = crate::FR<bool, MR1RR>;
impl MR1R_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR1RR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR1RR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR1R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1RW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR1RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR1RW::ENABLED => true,
            MR1RW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR1RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR1RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR1RW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR1RW::DISABLED)
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
#[doc = "Possible values of the field `MR1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1SR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR1SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR1SR::ENABLED => true,
            MR1SR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR1S_R = crate::FR<bool, MR1SR>;
impl MR1S_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR1SR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR1SR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1SW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR1SW::ENABLED => true,
            MR1SW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR1SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR1SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR1SW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR1SW::DISABLED)
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
#[doc = "Possible values of the field `MR2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2IR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR2IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR2IR::ENABLED => true,
            MR2IR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR2I_R = crate::FR<bool, MR2IR>;
impl MR2I_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR2IR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR2IR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2IW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR2IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR2IW::ENABLED => true,
            MR2IW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR2IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR2IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR2IW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR2IW::DISABLED)
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
#[doc = "Possible values of the field `MR2R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2RR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR2RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR2RR::ENABLED => true,
            MR2RR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR2R_R = crate::FR<bool, MR2RR>;
impl MR2R_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR2RR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR2RR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR2R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2RW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR2RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR2RW::ENABLED => true,
            MR2RW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR2RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR2RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR2RW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR2RW::DISABLED)
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
#[doc = "Possible values of the field `MR2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2SR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR2SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR2SR::ENABLED => true,
            MR2SR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR2S_R = crate::FR<bool, MR2SR>;
impl MR2S_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR2SR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR2SR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2SW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR2SW::ENABLED => true,
            MR2SW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR2SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR2SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR2SW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR2SW::DISABLED)
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
#[doc = "Possible values of the field `MR3I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3IR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR3IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR3IR::ENABLED => true,
            MR3IR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR3I_R = crate::FR<bool, MR3IR>;
impl MR3I_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR3IR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR3IR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR3I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3IW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR3IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR3IW::ENABLED => true,
            MR3IW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR3IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR3IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR3IW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR3IW::DISABLED)
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
#[doc = "Possible values of the field `MR3R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3RR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR3RR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR3RR::ENABLED => true,
            MR3RR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR3R_R = crate::FR<bool, MR3RR>;
impl MR3R_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR3RR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR3RR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR3R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3RW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR3RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR3RW::ENABLED => true,
            MR3RW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR3RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3RW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR3RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR3RW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR3RW::DISABLED)
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
#[doc = "Possible values of the field `MR3S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3SR {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl crate::ToBits<bool> for MR3SR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MR3SR::ENABLED => true,
            MR3SR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MR3S_R = crate::FR<bool, MR3SR>;
impl MR3S_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR3SR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR3SR::DISABLED
    }
}
#[doc = "Values that can be written to the field `MR3S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3SW {
    #[doc = "Enabled"]
    ENABLED,
    #[doc = "Disabled"]
    DISABLED,
}
impl MR3SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MR3SW::ENABLED => true,
            MR3SW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MR3SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR3SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MR3SW::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MR3SW::DISABLED)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn mr0i(&self) -> MR0I_R {
        MR0I_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn mr0r(&self) -> MR0R_R {
        MR0R_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn mr0s(&self) -> MR0S_R {
        MR0S_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn mr1i(&self) -> MR1I_R {
        MR1I_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn mr1r(&self) -> MR1R_R {
        MR1R_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn mr1s(&self) -> MR1S_R {
        MR1S_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn mr2i(&self) -> MR2I_R {
        MR2I_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn mr2r(&self) -> MR2R_R {
        MR2R_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub fn mr2s(&self) -> MR2S_R {
        MR2S_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn mr3i(&self) -> MR3I_R {
        MR3I_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn mr3r(&self) -> MR3R_R {
        MR3R_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn mr3s(&self) -> MR3S_R {
        MR3S_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn mr0i(&mut self) -> _MR0IW {
        _MR0IW { w: self }
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn mr0r(&mut self) -> _MR0RW {
        _MR0RW { w: self }
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn mr0s(&mut self) -> _MR0SW {
        _MR0SW { w: self }
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn mr1i(&mut self) -> _MR1IW {
        _MR1IW { w: self }
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn mr1r(&mut self) -> _MR1RW {
        _MR1RW { w: self }
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn mr1s(&mut self) -> _MR1SW {
        _MR1SW { w: self }
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn mr2i(&mut self) -> _MR2IW {
        _MR2IW { w: self }
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn mr2r(&mut self) -> _MR2RW {
        _MR2RW { w: self }
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub fn mr2s(&mut self) -> _MR2SW {
        _MR2SW { w: self }
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn mr3i(&mut self) -> _MR3IW {
        _MR3IW { w: self }
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn mr3r(&mut self) -> _MR3RW {
        _MR3RW { w: self }
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn mr3s(&mut self) -> _MR3SW {
        _MR3SW { w: self }
    }
}
