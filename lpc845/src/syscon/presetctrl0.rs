#[doc = "Reader of register PRESETCTRL0"]
pub type R = crate::R<u32, super::PRESETCTRL0>;
#[doc = "Writer for register PRESETCTRL0"]
pub type W = crate::W<u32, super::PRESETCTRL0>;
#[doc = "Register PRESETCTRL0 `reset()`'s with value 0xfbfd_fff0"]
impl crate::ResetValue for super::PRESETCTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfbfd_fff0
    }
}
#[doc = "flash controller reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_RST_N_A {
    #[doc = "0: Assert the flash controller reset."]
    ASSERT = 0,
    #[doc = "1: Clear the flash controller reset."]
    CLEAR = 1,
}
impl From<FLASH_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH_RST_N`"]
pub type FLASH_RST_N_R = crate::R<bool, FLASH_RST_N_A>;
impl FLASH_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_RST_N_A {
        match self.bits {
            false => FLASH_RST_N_A::ASSERT,
            true => FLASH_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == FLASH_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FLASH_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `FLASH_RST_N`"]
pub struct FLASH_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the flash controller reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(FLASH_RST_N_A::ASSERT)
    }
    #[doc = "Clear the flash controller reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLASH_RST_N_A::CLEAR)
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
#[doc = "I2C0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_RST_N_A {
    #[doc = "0: Assert the I2C0 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the I2C0 reset."]
    CLEAR = 1,
}
impl From<I2C0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0_RST_N`"]
pub type I2C0_RST_N_R = crate::R<bool, I2C0_RST_N_A>;
impl I2C0_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_RST_N_A {
        match self.bits {
            false => I2C0_RST_N_A::ASSERT,
            true => I2C0_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == I2C0_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == I2C0_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `I2C0_RST_N`"]
pub struct I2C0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the I2C0 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(I2C0_RST_N_A::ASSERT)
    }
    #[doc = "Clear the I2C0 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(I2C0_RST_N_A::CLEAR)
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
#[doc = "GPIO0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_RST_N_A {
    #[doc = "0: Assert the GPIO0 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the GPIO0 reset."]
    CLEAR = 1,
}
impl From<GPIO0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO0_RST_N`"]
pub type GPIO0_RST_N_R = crate::R<bool, GPIO0_RST_N_A>;
impl GPIO0_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_RST_N_A {
        match self.bits {
            false => GPIO0_RST_N_A::ASSERT,
            true => GPIO0_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == GPIO0_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO0_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `GPIO0_RST_N`"]
pub struct GPIO0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the GPIO0 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(GPIO0_RST_N_A::ASSERT)
    }
    #[doc = "Clear the GPIO0 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO0_RST_N_A::CLEAR)
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
#[doc = "SWM reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWM_RST_N_A {
    #[doc = "0: Assert the SWM reset."]
    ASSERT = 0,
    #[doc = "1: Clear the SWM reset."]
    CLEAR = 1,
}
impl From<SWM_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SWM_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWM_RST_N`"]
pub type SWM_RST_N_R = crate::R<bool, SWM_RST_N_A>;
impl SWM_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWM_RST_N_A {
        match self.bits {
            false => SWM_RST_N_A::ASSERT,
            true => SWM_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == SWM_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SWM_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `SWM_RST_N`"]
pub struct SWM_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SWM_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWM_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the SWM reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(SWM_RST_N_A::ASSERT)
    }
    #[doc = "Clear the SWM reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SWM_RST_N_A::CLEAR)
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
#[doc = "SCT reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_RST_N_A {
    #[doc = "0: Assert the SCT reset."]
    ASSERT = 0,
    #[doc = "1: Clear the SCT reset."]
    CLEAR = 1,
}
impl From<SCT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCT_RST_N`"]
pub type SCT_RST_N_R = crate::R<bool, SCT_RST_N_A>;
impl SCT_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_RST_N_A {
        match self.bits {
            false => SCT_RST_N_A::ASSERT,
            true => SCT_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == SCT_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SCT_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `SCT_RST_N`"]
pub struct SCT_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the SCT reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(SCT_RST_N_A::ASSERT)
    }
    #[doc = "Clear the SCT reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SCT_RST_N_A::CLEAR)
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
#[doc = "Self-wake-up timer (WKT) reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKT_RST_N_A {
    #[doc = "0: Assert the WKT reset."]
    ASSERT = 0,
    #[doc = "1: Clear the WKT reset."]
    CLEAR = 1,
}
impl From<WKT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: WKT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKT_RST_N`"]
pub type WKT_RST_N_R = crate::R<bool, WKT_RST_N_A>;
impl WKT_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKT_RST_N_A {
        match self.bits {
            false => WKT_RST_N_A::ASSERT,
            true => WKT_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == WKT_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WKT_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `WKT_RST_N`"]
pub struct WKT_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> WKT_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKT_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the WKT reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(WKT_RST_N_A::ASSERT)
    }
    #[doc = "Clear the WKT reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WKT_RST_N_A::CLEAR)
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
#[doc = "Multi-rate timer (MRT) reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_RST_N_A {
    #[doc = "0: Assert the MRT reset."]
    ASSERT = 0,
    #[doc = "1: Clear the MRT reset."]
    CLEAR = 1,
}
impl From<MRT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MRT_RST_N`"]
pub type MRT_RST_N_R = crate::R<bool, MRT_RST_N_A>;
impl MRT_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_RST_N_A {
        match self.bits {
            false => MRT_RST_N_A::ASSERT,
            true => MRT_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == MRT_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == MRT_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `MRT_RST_N`"]
pub struct MRT_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the MRT reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(MRT_RST_N_A::ASSERT)
    }
    #[doc = "Clear the MRT reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MRT_RST_N_A::CLEAR)
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
#[doc = "SPI0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_RST_N_A {
    #[doc = "0: Assert the SPI0 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the SPI0 reset."]
    CLEAR = 1,
}
impl From<SPI0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_RST_N`"]
pub type SPI0_RST_N_R = crate::R<bool, SPI0_RST_N_A>;
impl SPI0_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_RST_N_A {
        match self.bits {
            false => SPI0_RST_N_A::ASSERT,
            true => SPI0_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == SPI0_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SPI0_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `SPI0_RST_N`"]
pub struct SPI0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the SPI0 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(SPI0_RST_N_A::ASSERT)
    }
    #[doc = "Clear the SPI0 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SPI0_RST_N_A::CLEAR)
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
#[doc = "SPI1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_RST_N_A {
    #[doc = "0: Assert the SPI1 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the SPI1 reset."]
    CLEAR = 1,
}
impl From<SPI1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1_RST_N`"]
pub type SPI1_RST_N_R = crate::R<bool, SPI1_RST_N_A>;
impl SPI1_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_RST_N_A {
        match self.bits {
            false => SPI1_RST_N_A::ASSERT,
            true => SPI1_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == SPI1_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SPI1_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `SPI1_RST_N`"]
pub struct SPI1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the SPI1 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(SPI1_RST_N_A::ASSERT)
    }
    #[doc = "Clear the SPI1 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SPI1_RST_N_A::CLEAR)
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
#[doc = "CRC engine reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_RST_N_A {
    #[doc = "0: Assert the CRC reset."]
    ASSERT = 0,
    #[doc = "1: Clear the CRC reset."]
    CLEAR = 1,
}
impl From<CRC_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC_RST_N`"]
pub type CRC_RST_N_R = crate::R<bool, CRC_RST_N_A>;
impl CRC_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_RST_N_A {
        match self.bits {
            false => CRC_RST_N_A::ASSERT,
            true => CRC_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == CRC_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CRC_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `CRC_RST_N`"]
pub struct CRC_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the CRC reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(CRC_RST_N_A::ASSERT)
    }
    #[doc = "Clear the CRC reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRC_RST_N_A::CLEAR)
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
#[doc = "UART0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_RST_N_A {
    #[doc = "0: Assert the UART0 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the UART0 reset."]
    CLEAR = 1,
}
impl From<UART0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0_RST_N`"]
pub type UART0_RST_N_R = crate::R<bool, UART0_RST_N_A>;
impl UART0_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_RST_N_A {
        match self.bits {
            false => UART0_RST_N_A::ASSERT,
            true => UART0_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == UART0_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_cl_ear(&self) -> bool {
        *self == UART0_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `UART0_RST_N`"]
pub struct UART0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the UART0 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART0_RST_N_A::ASSERT)
    }
    #[doc = "Clear the UART0 reset."]
    #[inline(always)]
    pub fn cl_ear(self) -> &'a mut W {
        self.variant(UART0_RST_N_A::CLEAR)
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
#[doc = "UART1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_RST_N_A {
    #[doc = "0: Assert the UART1 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the UART1 reset."]
    CLEAR = 1,
}
impl From<UART1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART1_RST_N`"]
pub type UART1_RST_N_R = crate::R<bool, UART1_RST_N_A>;
impl UART1_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_RST_N_A {
        match self.bits {
            false => UART1_RST_N_A::ASSERT,
            true => UART1_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == UART1_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UART1_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `UART1_RST_N`"]
pub struct UART1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the UART1 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART1_RST_N_A::ASSERT)
    }
    #[doc = "Clear the UART1 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UART1_RST_N_A::CLEAR)
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
#[doc = "UART2 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_RST_N_A {
    #[doc = "0: Assert the UART2 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the UART2 reset."]
    CLEAR = 1,
}
impl From<UART2_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART2_RST_N`"]
pub type UART2_RST_N_R = crate::R<bool, UART2_RST_N_A>;
impl UART2_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_RST_N_A {
        match self.bits {
            false => UART2_RST_N_A::ASSERT,
            true => UART2_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == UART2_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UART2_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `UART2_RST_N`"]
pub struct UART2_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the UART2 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART2_RST_N_A::ASSERT)
    }
    #[doc = "Clear the UART2 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UART2_RST_N_A::CLEAR)
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
#[doc = "IOCON reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_RST_N_A {
    #[doc = "0: Assert the IOCON reset."]
    ASSERT = 0,
    #[doc = "1: Clear the IOCON reset."]
    CLEAR = 1,
}
impl From<IOCON_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IOCON_RST_N`"]
pub type IOCON_RST_N_R = crate::R<bool, IOCON_RST_N_A>;
impl IOCON_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_RST_N_A {
        match self.bits {
            false => IOCON_RST_N_A::ASSERT,
            true => IOCON_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == IOCON_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == IOCON_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `IOCON_RST_N`"]
pub struct IOCON_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCON_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the IOCON reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(IOCON_RST_N_A::ASSERT)
    }
    #[doc = "Clear the IOCON reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IOCON_RST_N_A::CLEAR)
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
#[doc = "Analog comparator reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_RST_N_A {
    #[doc = "0: Assert the analog comparator reset."]
    ASSERT = 0,
    #[doc = "1: Clear the analog comparator reset."]
    CLEAR = 1,
}
impl From<ACMP_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMP_RST_N`"]
pub type ACMP_RST_N_R = crate::R<bool, ACMP_RST_N_A>;
impl ACMP_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_RST_N_A {
        match self.bits {
            false => ACMP_RST_N_A::ASSERT,
            true => ACMP_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == ACMP_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ACMP_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `ACMP_RST_N`"]
pub struct ACMP_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the analog comparator reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(ACMP_RST_N_A::ASSERT)
    }
    #[doc = "Clear the analog comparator reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACMP_RST_N_A::CLEAR)
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
#[doc = "GPIO1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1_RST_N_A {
    #[doc = "0: Assert the GPIO1 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the GPIO1 reset."]
    CLEAR = 1,
}
impl From<GPIO1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO1_RST_N`"]
pub type GPIO1_RST_N_R = crate::R<bool, GPIO1_RST_N_A>;
impl GPIO1_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1_RST_N_A {
        match self.bits {
            false => GPIO1_RST_N_A::ASSERT,
            true => GPIO1_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == GPIO1_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIO1_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `GPIO1_RST_N`"]
pub struct GPIO1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the GPIO1 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(GPIO1_RST_N_A::ASSERT)
    }
    #[doc = "Clear the GPIO1 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIO1_RST_N_A::CLEAR)
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
#[doc = "I2C1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_RST_N_A {
    #[doc = "0: Assert the I2C1 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the I2C1 reset."]
    CLEAR = 1,
}
impl From<I2C1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C1_RST_N`"]
pub type I2C1_RST_N_R = crate::R<bool, I2C1_RST_N_A>;
impl I2C1_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_RST_N_A {
        match self.bits {
            false => I2C1_RST_N_A::ASSERT,
            true => I2C1_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == I2C1_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == I2C1_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `I2C1_RST_N`"]
pub struct I2C1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the I2C1 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(I2C1_RST_N_A::ASSERT)
    }
    #[doc = "Clear the I2C1 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(I2C1_RST_N_A::CLEAR)
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
#[doc = "I2C2 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_RST_N_A {
    #[doc = "0: Assert the I2C2 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the I2C2 reset."]
    CLEAR = 1,
}
impl From<I2C2_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C2_RST_N`"]
pub type I2C2_RST_N_R = crate::R<bool, I2C2_RST_N_A>;
impl I2C2_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_RST_N_A {
        match self.bits {
            false => I2C2_RST_N_A::ASSERT,
            true => I2C2_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == I2C2_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == I2C2_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `I2C2_RST_N`"]
pub struct I2C2_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the I2C2 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(I2C2_RST_N_A::ASSERT)
    }
    #[doc = "Clear the I2C2 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(I2C2_RST_N_A::CLEAR)
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
#[doc = "I2C3 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C3_RST_N_A {
    #[doc = "0: Assert the I2C3 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the I2C3 reset."]
    CLEAR = 1,
}
impl From<I2C3_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C3_RST_N`"]
pub type I2C3_RST_N_R = crate::R<bool, I2C3_RST_N_A>;
impl I2C3_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3_RST_N_A {
        match self.bits {
            false => I2C3_RST_N_A::ASSERT,
            true => I2C3_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == I2C3_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == I2C3_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `I2C3_RST_N`"]
pub struct I2C3_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the I2C3 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(I2C3_RST_N_A::ASSERT)
    }
    #[doc = "Clear the I2C3 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(I2C3_RST_N_A::CLEAR)
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
#[doc = "ADC reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_RST_N_A {
    #[doc = "0: Assert the ADC reset."]
    ASSERT = 0,
    #[doc = "1: Clear the ADC reset."]
    CLEAR = 1,
}
impl From<ADC_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_RST_N`"]
pub type ADC_RST_N_R = crate::R<bool, ADC_RST_N_A>;
impl ADC_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_RST_N_A {
        match self.bits {
            false => ADC_RST_N_A::ASSERT,
            true => ADC_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == ADC_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ADC_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `ADC_RST_N`"]
pub struct ADC_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the ADC reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(ADC_RST_N_A::ASSERT)
    }
    #[doc = "Clear the ADC reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADC_RST_N_A::CLEAR)
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
#[doc = "CTIMER reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER_RST_N_A {
    #[doc = "0: Assert the CTIMER reset."]
    ASSERT = 0,
    #[doc = "1: Clear the CTIMER reset."]
    CLEAR = 1,
}
impl From<CTIMER_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTIMER_RST_N`"]
pub type CTIMER_RST_N_R = crate::R<bool, CTIMER_RST_N_A>;
impl CTIMER_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER_RST_N_A {
        match self.bits {
            false => CTIMER_RST_N_A::ASSERT,
            true => CTIMER_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == CTIMER_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTIMER_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `CTIMER_RST_N`"]
pub struct CTIMER_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the CTIMER reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(CTIMER_RST_N_A::ASSERT)
    }
    #[doc = "Clear the CTIMER reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTIMER_RST_N_A::CLEAR)
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
#[doc = "DAC0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0_RST_N_A {
    #[doc = "0: Assert the DAC0 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the DAC0 reset."]
    CLEAR = 1,
}
impl From<DAC0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: DAC0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DAC0_RST_N`"]
pub type DAC0_RST_N_R = crate::R<bool, DAC0_RST_N_A>;
impl DAC0_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC0_RST_N_A {
        match self.bits {
            false => DAC0_RST_N_A::ASSERT,
            true => DAC0_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == DAC0_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DAC0_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `DAC0_RST_N`"]
pub struct DAC0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC0_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the DAC0 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(DAC0_RST_N_A::ASSERT)
    }
    #[doc = "Clear the DAC0 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DAC0_RST_N_A::CLEAR)
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
#[doc = "GPIOINT reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOINT_RST_N_A {
    #[doc = "0: Assert the GPIOINT reset."]
    ASSERT = 0,
    #[doc = "1: Clear the GPIOINT reset."]
    CLEAR = 1,
}
impl From<GPIOINT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOINT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOINT_RST_N`"]
pub type GPIOINT_RST_N_R = crate::R<bool, GPIOINT_RST_N_A>;
impl GPIOINT_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOINT_RST_N_A {
        match self.bits {
            false => GPIOINT_RST_N_A::ASSERT,
            true => GPIOINT_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == GPIOINT_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GPIOINT_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `GPIOINT_RST_N`"]
pub struct GPIOINT_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOINT_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOINT_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the GPIOINT reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(GPIOINT_RST_N_A::ASSERT)
    }
    #[doc = "Clear the GPIOINT reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(GPIOINT_RST_N_A::CLEAR)
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
#[doc = "DMA reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_RST_N_A {
    #[doc = "0: Assert the DMA reset."]
    ASSERT = 0,
    #[doc = "1: Clear the DMA reset."]
    CLEAR = 1,
}
impl From<DMA_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_RST_N`"]
pub type DMA_RST_N_R = crate::R<bool, DMA_RST_N_A>;
impl DMA_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_RST_N_A {
        match self.bits {
            false => DMA_RST_N_A::ASSERT,
            true => DMA_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == DMA_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DMA_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `DMA_RST_N`"]
pub struct DMA_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the DMA reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(DMA_RST_N_A::ASSERT)
    }
    #[doc = "Clear the DMA reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DMA_RST_N_A::CLEAR)
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
#[doc = "UART3 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART3_RST_N_A {
    #[doc = "0: Assert the UART3 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the UART3 reset."]
    CLEAR = 1,
}
impl From<UART3_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART3_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART3_RST_N`"]
pub type UART3_RST_N_R = crate::R<bool, UART3_RST_N_A>;
impl UART3_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART3_RST_N_A {
        match self.bits {
            false => UART3_RST_N_A::ASSERT,
            true => UART3_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == UART3_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UART3_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `UART3_RST_N`"]
pub struct UART3_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART3_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the UART3 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART3_RST_N_A::ASSERT)
    }
    #[doc = "Clear the UART3 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UART3_RST_N_A::CLEAR)
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
#[doc = "UART4 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART4_RST_N_A {
    #[doc = "0: Assert the UART4 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the UART4 reset."]
    CLEAR = 1,
}
impl From<UART4_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART4_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART4_RST_N`"]
pub type UART4_RST_N_R = crate::R<bool, UART4_RST_N_A>;
impl UART4_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4_RST_N_A {
        match self.bits {
            false => UART4_RST_N_A::ASSERT,
            true => UART4_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == UART4_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UART4_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `UART4_RST_N`"]
pub struct UART4_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the UART4 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART4_RST_N_A::ASSERT)
    }
    #[doc = "Clear the UART4 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UART4_RST_N_A::CLEAR)
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
    #[doc = "Bit 4 - flash controller reset control"]
    #[inline(always)]
    pub fn flash_rst_n(&self) -> FLASH_RST_N_R {
        FLASH_RST_N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C0 reset control"]
    #[inline(always)]
    pub fn i2c0_rst_n(&self) -> I2C0_RST_N_R {
        I2C0_RST_N_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO0 reset control"]
    #[inline(always)]
    pub fn gpio0_rst_n(&self) -> GPIO0_RST_N_R {
        GPIO0_RST_N_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SWM reset control"]
    #[inline(always)]
    pub fn swm_rst_n(&self) -> SWM_RST_N_R {
        SWM_RST_N_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SCT reset control"]
    #[inline(always)]
    pub fn sct_rst_n(&self) -> SCT_RST_N_R {
        SCT_RST_N_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Self-wake-up timer (WKT) reset control"]
    #[inline(always)]
    pub fn wkt_rst_n(&self) -> WKT_RST_N_R {
        WKT_RST_N_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Multi-rate timer (MRT) reset control"]
    #[inline(always)]
    pub fn mrt_rst_n(&self) -> MRT_RST_N_R {
        MRT_RST_N_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SPI0 reset control"]
    #[inline(always)]
    pub fn spi0_rst_n(&self) -> SPI0_RST_N_R {
        SPI0_RST_N_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset control"]
    #[inline(always)]
    pub fn spi1_rst_n(&self) -> SPI1_RST_N_R {
        SPI1_RST_N_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CRC engine reset control"]
    #[inline(always)]
    pub fn crc_rst_n(&self) -> CRC_RST_N_R {
        CRC_RST_N_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - UART0 reset control"]
    #[inline(always)]
    pub fn uart0_rst_n(&self) -> UART0_RST_N_R {
        UART0_RST_N_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - UART1 reset control"]
    #[inline(always)]
    pub fn uart1_rst_n(&self) -> UART1_RST_N_R {
        UART1_RST_N_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART2 reset control"]
    #[inline(always)]
    pub fn uart2_rst_n(&self) -> UART2_RST_N_R {
        UART2_RST_N_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IOCON reset control"]
    #[inline(always)]
    pub fn iocon_rst_n(&self) -> IOCON_RST_N_R {
        IOCON_RST_N_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Analog comparator reset control"]
    #[inline(always)]
    pub fn acmp_rst_n(&self) -> ACMP_RST_N_R {
        ACMP_RST_N_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GPIO1 reset control"]
    #[inline(always)]
    pub fn gpio1_rst_n(&self) -> GPIO1_RST_N_R {
        GPIO1_RST_N_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset control"]
    #[inline(always)]
    pub fn i2c1_rst_n(&self) -> I2C1_RST_N_R {
        I2C1_RST_N_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset control"]
    #[inline(always)]
    pub fn i2c2_rst_n(&self) -> I2C2_RST_N_R {
        I2C2_RST_N_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C3 reset control"]
    #[inline(always)]
    pub fn i2c3_rst_n(&self) -> I2C3_RST_N_R {
        I2C3_RST_N_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC reset control"]
    #[inline(always)]
    pub fn adc_rst_n(&self) -> ADC_RST_N_R {
        ADC_RST_N_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CTIMER reset control"]
    #[inline(always)]
    pub fn ctimer_rst_n(&self) -> CTIMER_RST_N_R {
        CTIMER_RST_N_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DAC0 reset control"]
    #[inline(always)]
    pub fn dac0_rst_n(&self) -> DAC0_RST_N_R {
        DAC0_RST_N_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GPIOINT reset control"]
    #[inline(always)]
    pub fn gpioint_rst_n(&self) -> GPIOINT_RST_N_R {
        GPIOINT_RST_N_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA reset control"]
    #[inline(always)]
    pub fn dma_rst_n(&self) -> DMA_RST_N_R {
        DMA_RST_N_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - UART3 reset control"]
    #[inline(always)]
    pub fn uart3_rst_n(&self) -> UART3_RST_N_R {
        UART3_RST_N_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - UART4 reset control"]
    #[inline(always)]
    pub fn uart4_rst_n(&self) -> UART4_RST_N_R {
        UART4_RST_N_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - flash controller reset control"]
    #[inline(always)]
    pub fn flash_rst_n(&mut self) -> FLASH_RST_N_W {
        FLASH_RST_N_W { w: self }
    }
    #[doc = "Bit 5 - I2C0 reset control"]
    #[inline(always)]
    pub fn i2c0_rst_n(&mut self) -> I2C0_RST_N_W {
        I2C0_RST_N_W { w: self }
    }
    #[doc = "Bit 6 - GPIO0 reset control"]
    #[inline(always)]
    pub fn gpio0_rst_n(&mut self) -> GPIO0_RST_N_W {
        GPIO0_RST_N_W { w: self }
    }
    #[doc = "Bit 7 - SWM reset control"]
    #[inline(always)]
    pub fn swm_rst_n(&mut self) -> SWM_RST_N_W {
        SWM_RST_N_W { w: self }
    }
    #[doc = "Bit 8 - SCT reset control"]
    #[inline(always)]
    pub fn sct_rst_n(&mut self) -> SCT_RST_N_W {
        SCT_RST_N_W { w: self }
    }
    #[doc = "Bit 9 - Self-wake-up timer (WKT) reset control"]
    #[inline(always)]
    pub fn wkt_rst_n(&mut self) -> WKT_RST_N_W {
        WKT_RST_N_W { w: self }
    }
    #[doc = "Bit 10 - Multi-rate timer (MRT) reset control"]
    #[inline(always)]
    pub fn mrt_rst_n(&mut self) -> MRT_RST_N_W {
        MRT_RST_N_W { w: self }
    }
    #[doc = "Bit 11 - SPI0 reset control"]
    #[inline(always)]
    pub fn spi0_rst_n(&mut self) -> SPI0_RST_N_W {
        SPI0_RST_N_W { w: self }
    }
    #[doc = "Bit 12 - SPI1 reset control"]
    #[inline(always)]
    pub fn spi1_rst_n(&mut self) -> SPI1_RST_N_W {
        SPI1_RST_N_W { w: self }
    }
    #[doc = "Bit 13 - CRC engine reset control"]
    #[inline(always)]
    pub fn crc_rst_n(&mut self) -> CRC_RST_N_W {
        CRC_RST_N_W { w: self }
    }
    #[doc = "Bit 14 - UART0 reset control"]
    #[inline(always)]
    pub fn uart0_rst_n(&mut self) -> UART0_RST_N_W {
        UART0_RST_N_W { w: self }
    }
    #[doc = "Bit 15 - UART1 reset control"]
    #[inline(always)]
    pub fn uart1_rst_n(&mut self) -> UART1_RST_N_W {
        UART1_RST_N_W { w: self }
    }
    #[doc = "Bit 16 - UART2 reset control"]
    #[inline(always)]
    pub fn uart2_rst_n(&mut self) -> UART2_RST_N_W {
        UART2_RST_N_W { w: self }
    }
    #[doc = "Bit 18 - IOCON reset control"]
    #[inline(always)]
    pub fn iocon_rst_n(&mut self) -> IOCON_RST_N_W {
        IOCON_RST_N_W { w: self }
    }
    #[doc = "Bit 19 - Analog comparator reset control"]
    #[inline(always)]
    pub fn acmp_rst_n(&mut self) -> ACMP_RST_N_W {
        ACMP_RST_N_W { w: self }
    }
    #[doc = "Bit 20 - GPIO1 reset control"]
    #[inline(always)]
    pub fn gpio1_rst_n(&mut self) -> GPIO1_RST_N_W {
        GPIO1_RST_N_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 reset control"]
    #[inline(always)]
    pub fn i2c1_rst_n(&mut self) -> I2C1_RST_N_W {
        I2C1_RST_N_W { w: self }
    }
    #[doc = "Bit 22 - I2C2 reset control"]
    #[inline(always)]
    pub fn i2c2_rst_n(&mut self) -> I2C2_RST_N_W {
        I2C2_RST_N_W { w: self }
    }
    #[doc = "Bit 23 - I2C3 reset control"]
    #[inline(always)]
    pub fn i2c3_rst_n(&mut self) -> I2C3_RST_N_W {
        I2C3_RST_N_W { w: self }
    }
    #[doc = "Bit 24 - ADC reset control"]
    #[inline(always)]
    pub fn adc_rst_n(&mut self) -> ADC_RST_N_W {
        ADC_RST_N_W { w: self }
    }
    #[doc = "Bit 25 - CTIMER reset control"]
    #[inline(always)]
    pub fn ctimer_rst_n(&mut self) -> CTIMER_RST_N_W {
        CTIMER_RST_N_W { w: self }
    }
    #[doc = "Bit 27 - DAC0 reset control"]
    #[inline(always)]
    pub fn dac0_rst_n(&mut self) -> DAC0_RST_N_W {
        DAC0_RST_N_W { w: self }
    }
    #[doc = "Bit 28 - GPIOINT reset control"]
    #[inline(always)]
    pub fn gpioint_rst_n(&mut self) -> GPIOINT_RST_N_W {
        GPIOINT_RST_N_W { w: self }
    }
    #[doc = "Bit 29 - DMA reset control"]
    #[inline(always)]
    pub fn dma_rst_n(&mut self) -> DMA_RST_N_W {
        DMA_RST_N_W { w: self }
    }
    #[doc = "Bit 30 - UART3 reset control"]
    #[inline(always)]
    pub fn uart3_rst_n(&mut self) -> UART3_RST_N_W {
        UART3_RST_N_W { w: self }
    }
    #[doc = "Bit 31 - UART4 reset control"]
    #[inline(always)]
    pub fn uart4_rst_n(&mut self) -> UART4_RST_N_W {
        UART4_RST_N_W { w: self }
    }
}
