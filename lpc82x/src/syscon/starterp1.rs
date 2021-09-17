#[doc = "Register `STARTERP1` reader"]
pub struct R(crate::R<STARTERP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTERP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTERP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTERP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTERP1` writer"]
pub struct W(crate::W<STARTERP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTERP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STARTERP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTERP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SPI0 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0` reader - SPI0 interrupt wake-up"]
pub struct SPI0_R(crate::FieldReader<bool, SPI0_A>);
impl SPI0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::DISABLED,
            true => SPI0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SPI0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SPI0_A::ENABLED
    }
}
impl core::ops::Deref for SPI0_R {
    type Target = crate::FieldReader<bool, SPI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0` writer - SPI0 interrupt wake-up"]
pub struct SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "SPI1 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1` reader - SPI1 interrupt wake-up"]
pub struct SPI1_R(crate::FieldReader<bool, SPI1_A>);
impl SPI1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::DISABLED,
            true => SPI1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SPI1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SPI1_A::ENABLED
    }
}
impl core::ops::Deref for SPI1_R {
    type Target = crate::FieldReader<bool, SPI1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1` writer - SPI1 interrupt wake-up"]
pub struct SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "USART0 interrupt wake-up. Configure USART in synchronous slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<USART0_A> for bool {
    #[inline(always)]
    fn from(variant: USART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART0` reader - USART0 interrupt wake-up. Configure USART in synchronous slave mode."]
pub struct USART0_R(crate::FieldReader<bool, USART0_A>);
impl USART0_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART0_A {
        match self.bits {
            false => USART0_A::DISABLED,
            true => USART0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USART0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USART0_A::ENABLED
    }
}
impl core::ops::Deref for USART0_R {
    type Target = crate::FieldReader<bool, USART0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART0` writer - USART0 interrupt wake-up. Configure USART in synchronous slave mode."]
pub struct USART0_W<'a> {
    w: &'a mut W,
}
impl<'a> USART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "USART1 interrupt wake-up. Configure USART in synchronous slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<USART1_A> for bool {
    #[inline(always)]
    fn from(variant: USART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1` reader - USART1 interrupt wake-up. Configure USART in synchronous slave mode."]
pub struct USART1_R(crate::FieldReader<bool, USART1_A>);
impl USART1_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1_A {
        match self.bits {
            false => USART1_A::DISABLED,
            true => USART1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USART1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USART1_A::ENABLED
    }
}
impl core::ops::Deref for USART1_R {
    type Target = crate::FieldReader<bool, USART1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1` writer - USART1 interrupt wake-up. Configure USART in synchronous slave mode."]
pub struct USART1_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "USART2 interrupt wake-up. Configure USART in synchronous slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<USART2_A> for bool {
    #[inline(always)]
    fn from(variant: USART2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART2` reader - USART2 interrupt wake-up. Configure USART in synchronous slave mode."]
pub struct USART2_R(crate::FieldReader<bool, USART2_A>);
impl USART2_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART2_A {
        match self.bits {
            false => USART2_A::DISABLED,
            true => USART2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USART2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USART2_A::ENABLED
    }
}
impl core::ops::Deref for USART2_R {
    type Target = crate::FieldReader<bool, USART2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2` writer - USART2 interrupt wake-up. Configure USART in synchronous slave mode."]
pub struct USART2_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART2_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "I2C1 interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<I2C1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1` reader - I2C1 interrupt wake-up."]
pub struct I2C1_R(crate::FieldReader<bool, I2C1_A>);
impl I2C1_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_A {
        match self.bits {
            false => I2C1_A::DISABLED,
            true => I2C1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == I2C1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == I2C1_A::ENABLED
    }
}
impl core::ops::Deref for I2C1_R {
    type Target = crate::FieldReader<bool, I2C1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1` writer - I2C1 interrupt wake-up."]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "I2C0 interrupt wake-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<I2C0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0` reader - I2C0 interrupt wake-up."]
pub struct I2C0_R(crate::FieldReader<bool, I2C0_A>);
impl I2C0_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_A {
        match self.bits {
            false => I2C0_A::DISABLED,
            true => I2C0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == I2C0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == I2C0_A::ENABLED
    }
}
impl core::ops::Deref for I2C0_R {
    type Target = crate::FieldReader<bool, I2C0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0` writer - I2C0 interrupt wake-up."]
pub struct I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "WWDT interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<WWDT_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT` reader - WWDT interrupt wake-up"]
pub struct WWDT_R(crate::FieldReader<bool, WWDT_A>);
impl WWDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_A {
        match self.bits {
            false => WWDT_A::DISABLED,
            true => WWDT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WWDT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WWDT_A::ENABLED
    }
}
impl core::ops::Deref for WWDT_R {
    type Target = crate::FieldReader<bool, WWDT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDT` writer - WWDT interrupt wake-up"]
pub struct WWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "BOD interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<BOD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOD` reader - BOD interrupt wake-up"]
pub struct BOD_R(crate::FieldReader<bool, BOD_A>);
impl BOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_A {
        match self.bits {
            false => BOD_A::DISABLED,
            true => BOD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BOD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BOD_A::ENABLED
    }
}
impl core::ops::Deref for BOD_R {
    type Target = crate::FieldReader<bool, BOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD` writer - BOD interrupt wake-up"]
pub struct BOD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOD_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Self-wake-up timer interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKT_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<WKT_A> for bool {
    #[inline(always)]
    fn from(variant: WKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKT` reader - Self-wake-up timer interrupt wake-up"]
pub struct WKT_R(crate::FieldReader<bool, WKT_A>);
impl WKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKT_A {
        match self.bits {
            false => WKT_A::DISABLED,
            true => WKT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKT_A::ENABLED
    }
}
impl core::ops::Deref for WKT_R {
    type Target = crate::FieldReader<bool, WKT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKT` writer - Self-wake-up timer interrupt wake-up"]
pub struct WKT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WKT_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WKT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "I2C2 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<I2C2_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2` reader - I2C2 interrupt wake-up"]
pub struct I2C2_R(crate::FieldReader<bool, I2C2_A>);
impl I2C2_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_A {
        match self.bits {
            false => I2C2_A::DISABLED,
            true => I2C2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == I2C2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == I2C2_A::ENABLED
    }
}
impl core::ops::Deref for I2C2_R {
    type Target = crate::FieldReader<bool, I2C2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2` writer - I2C2 interrupt wake-up"]
pub struct I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C2_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "I2C3 interrupt wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C3_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<I2C3_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3` reader - I2C3 interrupt wake-up"]
pub struct I2C3_R(crate::FieldReader<bool, I2C3_A>);
impl I2C3_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3_A {
        match self.bits {
            false => I2C3_A::DISABLED,
            true => I2C3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == I2C3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == I2C3_A::ENABLED
    }
}
impl core::ops::Deref for I2C3_R {
    type Target = crate::FieldReader<bool, I2C3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3` writer - I2C3 interrupt wake-up"]
pub struct I2C3_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C3_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C3_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI0 interrupt wake-up"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI1 interrupt wake-up"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USART0 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USART2 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C1 interrupt wake-up."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C0 interrupt wake-up."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - WWDT interrupt wake-up"]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - BOD interrupt wake-up"]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Self-wake-up timer interrupt wake-up"]
    #[inline(always)]
    pub fn wkt(&self) -> WKT_R {
        WKT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C2 interrupt wake-up"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C3 interrupt wake-up"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI0 interrupt wake-up"]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W {
        SPI0_W { w: self }
    }
    #[doc = "Bit 1 - SPI1 interrupt wake-up"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W {
        SPI1_W { w: self }
    }
    #[doc = "Bit 3 - USART0 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart0(&mut self) -> USART0_W {
        USART0_W { w: self }
    }
    #[doc = "Bit 4 - USART1 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart1(&mut self) -> USART1_W {
        USART1_W { w: self }
    }
    #[doc = "Bit 5 - USART2 interrupt wake-up. Configure USART in synchronous slave mode."]
    #[inline(always)]
    pub fn usart2(&mut self) -> USART2_W {
        USART2_W { w: self }
    }
    #[doc = "Bit 7 - I2C1 interrupt wake-up."]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Bit 8 - I2C0 interrupt wake-up."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W { w: self }
    }
    #[doc = "Bit 12 - WWDT interrupt wake-up"]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W {
        WWDT_W { w: self }
    }
    #[doc = "Bit 13 - BOD interrupt wake-up"]
    #[inline(always)]
    pub fn bod(&mut self) -> BOD_W {
        BOD_W { w: self }
    }
    #[doc = "Bit 15 - Self-wake-up timer interrupt wake-up"]
    #[inline(always)]
    pub fn wkt(&mut self) -> WKT_W {
        WKT_W { w: self }
    }
    #[doc = "Bit 21 - I2C2 interrupt wake-up"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2C2_W {
        I2C2_W { w: self }
    }
    #[doc = "Bit 22 - I2C3 interrupt wake-up"]
    #[inline(always)]
    pub fn i2c3(&mut self) -> I2C3_W {
        I2C3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic 1 interrupt wake-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterp1](index.html) module"]
pub struct STARTERP1_SPEC;
impl crate::RegisterSpec for STARTERP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starterp1::R](R) reader structure"]
impl crate::Readable for STARTERP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starterp1::W](W) writer structure"]
impl crate::Writable for STARTERP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTERP1 to value 0"]
impl crate::Resettable for STARTERP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
