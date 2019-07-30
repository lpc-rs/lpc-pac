#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `CAP0RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0RER {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl crate::ToBits<bool> for CAP0RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP0RER::ENABLED => true,
            CAP0RER::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP0RE_R = crate::FR<bool, CAP0RER>;
impl CAP0RE_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP0RER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0RER::DISABLED
    }
}
#[doc = "Values that can be written to the field `CAP0RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0REW {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl CAP0REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0REW::ENABLED => true,
            CAP0REW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP0REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0REW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP0REW::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0REW::DISABLED)
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
#[doc = "Possible values of the field `CAP0FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0FER {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl crate::ToBits<bool> for CAP0FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP0FER::ENABLED => true,
            CAP0FER::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP0FE_R = crate::FR<bool, CAP0FER>;
impl CAP0FE_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP0FER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0FER::DISABLED
    }
}
#[doc = "Values that can be written to the field `CAP0FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0FEW {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl CAP0FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0FEW::ENABLED => true,
            CAP0FEW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP0FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0FEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP0FEW::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0FEW::DISABLED)
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
#[doc = "Possible values of the field `CAP0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0IR {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl crate::ToBits<bool> for CAP0IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP0IR::ENABLED => true,
            CAP0IR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP0I_R = crate::FR<bool, CAP0IR>;
impl CAP0I_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP0IR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0IR::DISABLED
    }
}
#[doc = "Values that can be written to the field `CAP0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0IW {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl CAP0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0IW::ENABLED => true,
            CAP0IW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP0IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP0IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP0IW::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP0IW::DISABLED)
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
#[doc = "Possible values of the field `CAP1RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1RER {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl crate::ToBits<bool> for CAP1RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP1RER::ENABLED => true,
            CAP1RER::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP1RE_R = crate::FR<bool, CAP1RER>;
impl CAP1RE_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP1RER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1RER::DISABLED
    }
}
#[doc = "Values that can be written to the field `CAP1RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1REW {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl CAP1REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1REW::ENABLED => true,
            CAP1REW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP1REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1REW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP1REW::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1REW::DISABLED)
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
#[doc = "Possible values of the field `CAP1FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1FER {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl crate::ToBits<bool> for CAP1FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP1FER::ENABLED => true,
            CAP1FER::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP1FE_R = crate::FR<bool, CAP1FER>;
impl CAP1FE_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP1FER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1FER::DISABLED
    }
}
#[doc = "Values that can be written to the field `CAP1FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1FEW {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl CAP1FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1FEW::ENABLED => true,
            CAP1FEW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP1FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1FEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP1FEW::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1FEW::DISABLED)
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
#[doc = "Possible values of the field `CAP1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1IR {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl crate::ToBits<bool> for CAP1IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP1IR::ENABLED => true,
            CAP1IR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP1I_R = crate::FR<bool, CAP1IR>;
impl CAP1I_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP1IR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1IR::DISABLED
    }
}
#[doc = "Values that can be written to the field `CAP1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1IW {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl CAP1IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1IW::ENABLED => true,
            CAP1IW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP1IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP1IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP1IW::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP1IW::DISABLED)
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
#[doc = "Possible values of the field `CAP2RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP2RER {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl crate::ToBits<bool> for CAP2RER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP2RER::ENABLED => true,
            CAP2RER::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP2RE_R = crate::FR<bool, CAP2RER>;
impl CAP2RE_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP2RER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP2RER::DISABLED
    }
}
#[doc = "Values that can be written to the field `CAP2RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP2REW {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl CAP2REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP2REW::ENABLED => true,
            CAP2REW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP2REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2REW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP2REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP2REW::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP2REW::DISABLED)
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
#[doc = "Possible values of the field `CAP2FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP2FER {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl crate::ToBits<bool> for CAP2FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP2FER::ENABLED => true,
            CAP2FER::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP2FE_R = crate::FR<bool, CAP2FER>;
impl CAP2FE_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP2FER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP2FER::DISABLED
    }
}
#[doc = "Values that can be written to the field `CAP2FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP2FEW {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl CAP2FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP2FEW::ENABLED => true,
            CAP2FEW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP2FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2FEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP2FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP2FEW::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP2FEW::DISABLED)
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
#[doc = "Possible values of the field `CAP2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP2IR {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl crate::ToBits<bool> for CAP2IR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CAP2IR::ENABLED => true,
            CAP2IR::DISABLED => false,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP2I_R = crate::FR<bool, CAP2IR>;
impl CAP2I_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAP2IR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP2IR::DISABLED
    }
}
#[doc = "Values that can be written to the field `CAP2I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP2IW {
    #[doc = "Enabled."]
    ENABLED,
    #[doc = "Disabled."]
    DISABLED,
}
impl CAP2IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP2IW::ENABLED => true,
            CAP2IW::DISABLED => false,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAP2IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP2IW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAP2IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAP2IW::ENABLED)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAP2IW::DISABLED)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap0re(&self) -> CAP0RE_R {
        CAP0RE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Falling edge of capture channel 0:: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap0fe(&self) -> CAP0FE_R {
        CAP0FE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[inline(always)]
    pub fn cap0i(&self) -> CAP0I_R {
        CAP0I_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap1re(&self) -> CAP1RE_R {
        CAP1RE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Falling edge of capture channel 1:: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap1fe(&self) -> CAP1FE_R {
        CAP1FE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[inline(always)]
    pub fn cap1i(&self) -> CAP1I_R {
        CAP1I_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap2re(&self) -> CAP2RE_R {
        CAP2RE_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap2fe(&self) -> CAP2FE_R {
        CAP2FE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[inline(always)]
    pub fn cap2i(&self) -> CAP2I_R {
        CAP2I_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap0re(&mut self) -> _CAP0REW {
        _CAP0REW { w: self }
    }
    #[doc = "Bit 1 - Falling edge of capture channel 0:: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap0fe(&mut self) -> _CAP0FEW {
        _CAP0FEW { w: self }
    }
    #[doc = "Bit 2 - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[inline(always)]
    pub fn cap0i(&mut self) -> _CAP0IW {
        _CAP0IW { w: self }
    }
    #[doc = "Bit 3 - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap1re(&mut self) -> _CAP1REW {
        _CAP1REW { w: self }
    }
    #[doc = "Bit 4 - Falling edge of capture channel 1:: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap1fe(&mut self) -> _CAP1FEW {
        _CAP1FEW { w: self }
    }
    #[doc = "Bit 5 - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[inline(always)]
    pub fn cap1i(&mut self) -> _CAP1IW {
        _CAP1IW { w: self }
    }
    #[doc = "Bit 6 - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap2re(&mut self) -> _CAP2REW {
        _CAP2REW { w: self }
    }
    #[doc = "Bit 7 - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn cap2fe(&mut self) -> _CAP2FEW {
        _CAP2FEW { w: self }
    }
    #[doc = "Bit 8 - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[inline(always)]
    pub fn cap2i(&mut self) -> _CAP2IW {
        _CAP2IW { w: self }
    }
}
