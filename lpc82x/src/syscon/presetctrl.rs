#[doc = "Register `PRESETCTRL` reader"]
pub struct R(crate::R<PRESETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL` writer"]
pub struct W(crate::W<PRESETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL_SPEC>;
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
impl From<crate::W<PRESETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SPI0 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_RST_N_A {
    #[doc = "0: Assert the SPI0 reset."]
    SPI0_RST_N_0 = 0,
    #[doc = "1: Clear the SPI0 reset."]
    SPI0_RST_N_1 = 1,
}
impl From<SPI0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0_RST_N` reader - SPI0 reset control."]
pub struct SPI0_RST_N_R(crate::FieldReader<bool, SPI0_RST_N_A>);
impl SPI0_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI0_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_RST_N_A {
        match self.bits {
            false => SPI0_RST_N_A::SPI0_RST_N_0,
            true => SPI0_RST_N_A::SPI0_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_RST_N_0`"]
    #[inline(always)]
    pub fn is_spi0_rst_n_0(&self) -> bool {
        **self == SPI0_RST_N_A::SPI0_RST_N_0
    }
    #[doc = "Checks if the value of the field is `SPI0_RST_N_1`"]
    #[inline(always)]
    pub fn is_spi0_rst_n_1(&self) -> bool {
        **self == SPI0_RST_N_A::SPI0_RST_N_1
    }
}
impl core::ops::Deref for SPI0_RST_N_R {
    type Target = crate::FieldReader<bool, SPI0_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0_RST_N` writer - SPI0 reset control."]
pub struct SPI0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the SPI0 reset."]
    #[inline(always)]
    pub fn spi0_rst_n_0(self) -> &'a mut W {
        self.variant(SPI0_RST_N_A::SPI0_RST_N_0)
    }
    #[doc = "Clear the SPI0 reset."]
    #[inline(always)]
    pub fn spi0_rst_n_1(self) -> &'a mut W {
        self.variant(SPI0_RST_N_A::SPI0_RST_N_1)
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
#[doc = "SPI1 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_RST_N_A {
    #[doc = "0: Assert the SPI1 reset."]
    SPI1_RST_N_0 = 0,
    #[doc = "1: Clear the SPI1 reset."]
    SPI1_RST_N_1 = 1,
}
impl From<SPI1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1_RST_N` reader - SPI1 reset control."]
pub struct SPI1_RST_N_R(crate::FieldReader<bool, SPI1_RST_N_A>);
impl SPI1_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_RST_N_A {
        match self.bits {
            false => SPI1_RST_N_A::SPI1_RST_N_0,
            true => SPI1_RST_N_A::SPI1_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPI1_RST_N_0`"]
    #[inline(always)]
    pub fn is_spi1_rst_n_0(&self) -> bool {
        **self == SPI1_RST_N_A::SPI1_RST_N_0
    }
    #[doc = "Checks if the value of the field is `SPI1_RST_N_1`"]
    #[inline(always)]
    pub fn is_spi1_rst_n_1(&self) -> bool {
        **self == SPI1_RST_N_A::SPI1_RST_N_1
    }
}
impl core::ops::Deref for SPI1_RST_N_R {
    type Target = crate::FieldReader<bool, SPI1_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1_RST_N` writer - SPI1 reset control."]
pub struct SPI1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the SPI1 reset."]
    #[inline(always)]
    pub fn spi1_rst_n_0(self) -> &'a mut W {
        self.variant(SPI1_RST_N_A::SPI1_RST_N_0)
    }
    #[doc = "Clear the SPI1 reset."]
    #[inline(always)]
    pub fn spi1_rst_n_1(self) -> &'a mut W {
        self.variant(SPI1_RST_N_A::SPI1_RST_N_1)
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
#[doc = "USART fractional baud rate generator(UARTFRG) reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTFRG_RST_N_A {
    #[doc = "0: Assert the UARTFRG reset."]
    UARTFRG_RST_N_0 = 0,
    #[doc = "1: Clear the UARTFRG reset."]
    UARTFRG_RST_N_1 = 1,
}
impl From<UARTFRG_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UARTFRG_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTFRG_RST_N` reader - USART fractional baud rate generator(UARTFRG) reset control."]
pub struct UARTFRG_RST_N_R(crate::FieldReader<bool, UARTFRG_RST_N_A>);
impl UARTFRG_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        UARTFRG_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTFRG_RST_N_A {
        match self.bits {
            false => UARTFRG_RST_N_A::UARTFRG_RST_N_0,
            true => UARTFRG_RST_N_A::UARTFRG_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `UARTFRG_RST_N_0`"]
    #[inline(always)]
    pub fn is_uartfrg_rst_n_0(&self) -> bool {
        **self == UARTFRG_RST_N_A::UARTFRG_RST_N_0
    }
    #[doc = "Checks if the value of the field is `UARTFRG_RST_N_1`"]
    #[inline(always)]
    pub fn is_uartfrg_rst_n_1(&self) -> bool {
        **self == UARTFRG_RST_N_A::UARTFRG_RST_N_1
    }
}
impl core::ops::Deref for UARTFRG_RST_N_R {
    type Target = crate::FieldReader<bool, UARTFRG_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UARTFRG_RST_N` writer - USART fractional baud rate generator(UARTFRG) reset control."]
pub struct UARTFRG_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTFRG_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTFRG_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the UARTFRG reset."]
    #[inline(always)]
    pub fn uartfrg_rst_n_0(self) -> &'a mut W {
        self.variant(UARTFRG_RST_N_A::UARTFRG_RST_N_0)
    }
    #[doc = "Clear the UARTFRG reset."]
    #[inline(always)]
    pub fn uartfrg_rst_n_1(self) -> &'a mut W {
        self.variant(UARTFRG_RST_N_A::UARTFRG_RST_N_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "USART0 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_RST_N_A {
    #[doc = "0: Assert the USART0 reset."]
    UART0_RST_N_0 = 0,
    #[doc = "1: Clear the USART0 reset."]
    UART0_RST_N_1 = 1,
}
impl From<UART0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0_RST_N` reader - USART0 reset control."]
pub struct UART0_RST_N_R(crate::FieldReader<bool, UART0_RST_N_A>);
impl UART0_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART0_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_RST_N_A {
        match self.bits {
            false => UART0_RST_N_A::UART0_RST_N_0,
            true => UART0_RST_N_A::UART0_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART0_RST_N_0`"]
    #[inline(always)]
    pub fn is_uart0_rst_n_0(&self) -> bool {
        **self == UART0_RST_N_A::UART0_RST_N_0
    }
    #[doc = "Checks if the value of the field is `UART0_RST_N_1`"]
    #[inline(always)]
    pub fn is_uart0_rst_n_1(&self) -> bool {
        **self == UART0_RST_N_A::UART0_RST_N_1
    }
}
impl core::ops::Deref for UART0_RST_N_R {
    type Target = crate::FieldReader<bool, UART0_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0_RST_N` writer - USART0 reset control."]
pub struct UART0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the USART0 reset."]
    #[inline(always)]
    pub fn uart0_rst_n_0(self) -> &'a mut W {
        self.variant(UART0_RST_N_A::UART0_RST_N_0)
    }
    #[doc = "Clear the USART0 reset."]
    #[inline(always)]
    pub fn uart0_rst_n_1(self) -> &'a mut W {
        self.variant(UART0_RST_N_A::UART0_RST_N_1)
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
#[doc = "USART1 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_RST_N_A {
    #[doc = "0: Assert the USART1 reset."]
    UART1_RST_N_0 = 0,
    #[doc = "1: Clear the USART1 reset."]
    UART1_RST_N_1 = 1,
}
impl From<UART1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1_RST_N` reader - USART1 reset control."]
pub struct UART1_RST_N_R(crate::FieldReader<bool, UART1_RST_N_A>);
impl UART1_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART1_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_RST_N_A {
        match self.bits {
            false => UART1_RST_N_A::UART1_RST_N_0,
            true => UART1_RST_N_A::UART1_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART1_RST_N_0`"]
    #[inline(always)]
    pub fn is_uart1_rst_n_0(&self) -> bool {
        **self == UART1_RST_N_A::UART1_RST_N_0
    }
    #[doc = "Checks if the value of the field is `UART1_RST_N_1`"]
    #[inline(always)]
    pub fn is_uart1_rst_n_1(&self) -> bool {
        **self == UART1_RST_N_A::UART1_RST_N_1
    }
}
impl core::ops::Deref for UART1_RST_N_R {
    type Target = crate::FieldReader<bool, UART1_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1_RST_N` writer - USART1 reset control."]
pub struct UART1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the USART1 reset."]
    #[inline(always)]
    pub fn uart1_rst_n_0(self) -> &'a mut W {
        self.variant(UART1_RST_N_A::UART1_RST_N_0)
    }
    #[doc = "Clear the USART1 reset."]
    #[inline(always)]
    pub fn uart1_rst_n_1(self) -> &'a mut W {
        self.variant(UART1_RST_N_A::UART1_RST_N_1)
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
#[doc = "USART2 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_RST_N_A {
    #[doc = "0: Assert the USART2 reset."]
    UART2_RST_N_0 = 0,
    #[doc = "1: Clear the USART2 reset."]
    UART2_RST_N_1 = 1,
}
impl From<UART2_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2_RST_N` reader - USART2 reset control."]
pub struct UART2_RST_N_R(crate::FieldReader<bool, UART2_RST_N_A>);
impl UART2_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART2_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_RST_N_A {
        match self.bits {
            false => UART2_RST_N_A::UART2_RST_N_0,
            true => UART2_RST_N_A::UART2_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART2_RST_N_0`"]
    #[inline(always)]
    pub fn is_uart2_rst_n_0(&self) -> bool {
        **self == UART2_RST_N_A::UART2_RST_N_0
    }
    #[doc = "Checks if the value of the field is `UART2_RST_N_1`"]
    #[inline(always)]
    pub fn is_uart2_rst_n_1(&self) -> bool {
        **self == UART2_RST_N_A::UART2_RST_N_1
    }
}
impl core::ops::Deref for UART2_RST_N_R {
    type Target = crate::FieldReader<bool, UART2_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2_RST_N` writer - USART2 reset control."]
pub struct UART2_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the USART2 reset."]
    #[inline(always)]
    pub fn uart2_rst_n_0(self) -> &'a mut W {
        self.variant(UART2_RST_N_A::UART2_RST_N_0)
    }
    #[doc = "Clear the USART2 reset."]
    #[inline(always)]
    pub fn uart2_rst_n_1(self) -> &'a mut W {
        self.variant(UART2_RST_N_A::UART2_RST_N_1)
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
#[doc = "I2C0 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_RST_N_A {
    #[doc = "0: Assert the I2C0 reset."]
    I2C0_RST_N_0 = 0,
    #[doc = "1: Clear the I2C0 reset."]
    I2C0_RST_N_1 = 1,
}
impl From<I2C0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0_RST_N` reader - I2C0 reset control."]
pub struct I2C0_RST_N_R(crate::FieldReader<bool, I2C0_RST_N_A>);
impl I2C0_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_RST_N_A {
        match self.bits {
            false => I2C0_RST_N_A::I2C0_RST_N_0,
            true => I2C0_RST_N_A::I2C0_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `I2C0_RST_N_0`"]
    #[inline(always)]
    pub fn is_i2c0_rst_n_0(&self) -> bool {
        **self == I2C0_RST_N_A::I2C0_RST_N_0
    }
    #[doc = "Checks if the value of the field is `I2C0_RST_N_1`"]
    #[inline(always)]
    pub fn is_i2c0_rst_n_1(&self) -> bool {
        **self == I2C0_RST_N_A::I2C0_RST_N_1
    }
}
impl core::ops::Deref for I2C0_RST_N_R {
    type Target = crate::FieldReader<bool, I2C0_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0_RST_N` writer - I2C0 reset control."]
pub struct I2C0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the I2C0 reset."]
    #[inline(always)]
    pub fn i2c0_rst_n_0(self) -> &'a mut W {
        self.variant(I2C0_RST_N_A::I2C0_RST_N_0)
    }
    #[doc = "Clear the I2C0 reset."]
    #[inline(always)]
    pub fn i2c0_rst_n_1(self) -> &'a mut W {
        self.variant(I2C0_RST_N_A::I2C0_RST_N_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Multi-rate timer (MRT) reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_RST_N_A {
    #[doc = "0: Assert the MRT reset."]
    MRT_RST_N_0 = 0,
    #[doc = "1: Clear the MRT reset."]
    MRT_RST_N_1 = 1,
}
impl From<MRT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT_RST_N` reader - Multi-rate timer (MRT) reset control."]
pub struct MRT_RST_N_R(crate::FieldReader<bool, MRT_RST_N_A>);
impl MRT_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRT_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_RST_N_A {
        match self.bits {
            false => MRT_RST_N_A::MRT_RST_N_0,
            true => MRT_RST_N_A::MRT_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `MRT_RST_N_0`"]
    #[inline(always)]
    pub fn is_mrt_rst_n_0(&self) -> bool {
        **self == MRT_RST_N_A::MRT_RST_N_0
    }
    #[doc = "Checks if the value of the field is `MRT_RST_N_1`"]
    #[inline(always)]
    pub fn is_mrt_rst_n_1(&self) -> bool {
        **self == MRT_RST_N_A::MRT_RST_N_1
    }
}
impl core::ops::Deref for MRT_RST_N_R {
    type Target = crate::FieldReader<bool, MRT_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRT_RST_N` writer - Multi-rate timer (MRT) reset control."]
pub struct MRT_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the MRT reset."]
    #[inline(always)]
    pub fn mrt_rst_n_0(self) -> &'a mut W {
        self.variant(MRT_RST_N_A::MRT_RST_N_0)
    }
    #[doc = "Clear the MRT reset."]
    #[inline(always)]
    pub fn mrt_rst_n_1(self) -> &'a mut W {
        self.variant(MRT_RST_N_A::MRT_RST_N_1)
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
#[doc = "SCT reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_RST_N_A {
    #[doc = "0: Assert the SCT reset."]
    SCT_RST_N_0 = 0,
    #[doc = "1: Clear the SCT reset."]
    SCT_RST_N_1 = 1,
}
impl From<SCT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT_RST_N` reader - SCT reset control."]
pub struct SCT_RST_N_R(crate::FieldReader<bool, SCT_RST_N_A>);
impl SCT_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCT_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_RST_N_A {
        match self.bits {
            false => SCT_RST_N_A::SCT_RST_N_0,
            true => SCT_RST_N_A::SCT_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCT_RST_N_0`"]
    #[inline(always)]
    pub fn is_sct_rst_n_0(&self) -> bool {
        **self == SCT_RST_N_A::SCT_RST_N_0
    }
    #[doc = "Checks if the value of the field is `SCT_RST_N_1`"]
    #[inline(always)]
    pub fn is_sct_rst_n_1(&self) -> bool {
        **self == SCT_RST_N_A::SCT_RST_N_1
    }
}
impl core::ops::Deref for SCT_RST_N_R {
    type Target = crate::FieldReader<bool, SCT_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_RST_N` writer - SCT reset control."]
pub struct SCT_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the SCT reset."]
    #[inline(always)]
    pub fn sct_rst_n_0(self) -> &'a mut W {
        self.variant(SCT_RST_N_A::SCT_RST_N_0)
    }
    #[doc = "Clear the SCT reset."]
    #[inline(always)]
    pub fn sct_rst_n_1(self) -> &'a mut W {
        self.variant(SCT_RST_N_A::SCT_RST_N_1)
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
#[doc = "Self-wake-up timer (WKT) reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKT_RST_N_A {
    #[doc = "0: Assert the WKT reset."]
    WKT_RST_N_0 = 0,
    #[doc = "1: Clear the WKT reset."]
    WKT_RST_N_1 = 1,
}
impl From<WKT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: WKT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKT_RST_N` reader - Self-wake-up timer (WKT) reset control."]
pub struct WKT_RST_N_R(crate::FieldReader<bool, WKT_RST_N_A>);
impl WKT_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKT_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKT_RST_N_A {
        match self.bits {
            false => WKT_RST_N_A::WKT_RST_N_0,
            true => WKT_RST_N_A::WKT_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKT_RST_N_0`"]
    #[inline(always)]
    pub fn is_wkt_rst_n_0(&self) -> bool {
        **self == WKT_RST_N_A::WKT_RST_N_0
    }
    #[doc = "Checks if the value of the field is `WKT_RST_N_1`"]
    #[inline(always)]
    pub fn is_wkt_rst_n_1(&self) -> bool {
        **self == WKT_RST_N_A::WKT_RST_N_1
    }
}
impl core::ops::Deref for WKT_RST_N_R {
    type Target = crate::FieldReader<bool, WKT_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKT_RST_N` writer - Self-wake-up timer (WKT) reset control."]
pub struct WKT_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> WKT_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKT_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the WKT reset."]
    #[inline(always)]
    pub fn wkt_rst_n_0(self) -> &'a mut W {
        self.variant(WKT_RST_N_A::WKT_RST_N_0)
    }
    #[doc = "Clear the WKT reset."]
    #[inline(always)]
    pub fn wkt_rst_n_1(self) -> &'a mut W {
        self.variant(WKT_RST_N_A::WKT_RST_N_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "GPIO and GPIO pin interrupt reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_RST_N_A {
    #[doc = "0: Assert the GPIO reset."]
    GPIO_RST_N_0 = 0,
    #[doc = "1: Clear the GPIO reset."]
    GPIO_RST_N_1 = 1,
}
impl From<GPIO_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_RST_N` reader - GPIO and GPIO pin interrupt reset control."]
pub struct GPIO_RST_N_R(crate::FieldReader<bool, GPIO_RST_N_A>);
impl GPIO_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_RST_N_A {
        match self.bits {
            false => GPIO_RST_N_A::GPIO_RST_N_0,
            true => GPIO_RST_N_A::GPIO_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_RST_N_0`"]
    #[inline(always)]
    pub fn is_gpio_rst_n_0(&self) -> bool {
        **self == GPIO_RST_N_A::GPIO_RST_N_0
    }
    #[doc = "Checks if the value of the field is `GPIO_RST_N_1`"]
    #[inline(always)]
    pub fn is_gpio_rst_n_1(&self) -> bool {
        **self == GPIO_RST_N_A::GPIO_RST_N_1
    }
}
impl core::ops::Deref for GPIO_RST_N_R {
    type Target = crate::FieldReader<bool, GPIO_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_RST_N` writer - GPIO and GPIO pin interrupt reset control."]
pub struct GPIO_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the GPIO reset."]
    #[inline(always)]
    pub fn gpio_rst_n_0(self) -> &'a mut W {
        self.variant(GPIO_RST_N_A::GPIO_RST_N_0)
    }
    #[doc = "Clear the GPIO reset."]
    #[inline(always)]
    pub fn gpio_rst_n_1(self) -> &'a mut W {
        self.variant(GPIO_RST_N_A::GPIO_RST_N_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Flash controller reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_RST_N_A {
    #[doc = "0: Assert the flash controller reset."]
    FLASH_RST_N_0 = 0,
    #[doc = "1: Clear the flash controller reset."]
    FLASH_RST_N_1 = 1,
}
impl From<FLASH_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH_RST_N` reader - Flash controller reset control."]
pub struct FLASH_RST_N_R(crate::FieldReader<bool, FLASH_RST_N_A>);
impl FLASH_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_RST_N_A {
        match self.bits {
            false => FLASH_RST_N_A::FLASH_RST_N_0,
            true => FLASH_RST_N_A::FLASH_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_RST_N_0`"]
    #[inline(always)]
    pub fn is_flash_rst_n_0(&self) -> bool {
        **self == FLASH_RST_N_A::FLASH_RST_N_0
    }
    #[doc = "Checks if the value of the field is `FLASH_RST_N_1`"]
    #[inline(always)]
    pub fn is_flash_rst_n_1(&self) -> bool {
        **self == FLASH_RST_N_A::FLASH_RST_N_1
    }
}
impl core::ops::Deref for FLASH_RST_N_R {
    type Target = crate::FieldReader<bool, FLASH_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_RST_N` writer - Flash controller reset control."]
pub struct FLASH_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the flash controller reset."]
    #[inline(always)]
    pub fn flash_rst_n_0(self) -> &'a mut W {
        self.variant(FLASH_RST_N_A::FLASH_RST_N_0)
    }
    #[doc = "Clear the flash controller reset."]
    #[inline(always)]
    pub fn flash_rst_n_1(self) -> &'a mut W {
        self.variant(FLASH_RST_N_A::FLASH_RST_N_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Analog comparator reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_RST_N_A {
    #[doc = "0: Assert the analog comparator reset."]
    ACMP_RST_N_0 = 0,
    #[doc = "1: Clear the analog comparator controller reset."]
    ACMP_RST_N_1 = 1,
}
impl From<ACMP_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP_RST_N` reader - Analog comparator reset control."]
pub struct ACMP_RST_N_R(crate::FieldReader<bool, ACMP_RST_N_A>);
impl ACMP_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMP_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_RST_N_A {
        match self.bits {
            false => ACMP_RST_N_A::ACMP_RST_N_0,
            true => ACMP_RST_N_A::ACMP_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_RST_N_0`"]
    #[inline(always)]
    pub fn is_acmp_rst_n_0(&self) -> bool {
        **self == ACMP_RST_N_A::ACMP_RST_N_0
    }
    #[doc = "Checks if the value of the field is `ACMP_RST_N_1`"]
    #[inline(always)]
    pub fn is_acmp_rst_n_1(&self) -> bool {
        **self == ACMP_RST_N_A::ACMP_RST_N_1
    }
}
impl core::ops::Deref for ACMP_RST_N_R {
    type Target = crate::FieldReader<bool, ACMP_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP_RST_N` writer - Analog comparator reset control."]
pub struct ACMP_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the analog comparator reset."]
    #[inline(always)]
    pub fn acmp_rst_n_0(self) -> &'a mut W {
        self.variant(ACMP_RST_N_A::ACMP_RST_N_0)
    }
    #[doc = "Clear the analog comparator controller reset."]
    #[inline(always)]
    pub fn acmp_rst_n_1(self) -> &'a mut W {
        self.variant(ACMP_RST_N_A::ACMP_RST_N_1)
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
#[doc = "I2C1 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_RST_N_A {
    #[doc = "0: Assert the I2C1 reset."]
    I2C1_RST_N_0 = 0,
    #[doc = "1: Clear the I2C1 reset."]
    I2C1_RST_N_1 = 1,
}
impl From<I2C1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1_RST_N` reader - I2C1 reset control."]
pub struct I2C1_RST_N_R(crate::FieldReader<bool, I2C1_RST_N_A>);
impl I2C1_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_RST_N_A {
        match self.bits {
            false => I2C1_RST_N_A::I2C1_RST_N_0,
            true => I2C1_RST_N_A::I2C1_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `I2C1_RST_N_0`"]
    #[inline(always)]
    pub fn is_i2c1_rst_n_0(&self) -> bool {
        **self == I2C1_RST_N_A::I2C1_RST_N_0
    }
    #[doc = "Checks if the value of the field is `I2C1_RST_N_1`"]
    #[inline(always)]
    pub fn is_i2c1_rst_n_1(&self) -> bool {
        **self == I2C1_RST_N_A::I2C1_RST_N_1
    }
}
impl core::ops::Deref for I2C1_RST_N_R {
    type Target = crate::FieldReader<bool, I2C1_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_RST_N` writer - I2C1 reset control."]
pub struct I2C1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the I2C1 reset."]
    #[inline(always)]
    pub fn i2c1_rst_n_0(self) -> &'a mut W {
        self.variant(I2C1_RST_N_A::I2C1_RST_N_0)
    }
    #[doc = "Clear the I2C1 reset."]
    #[inline(always)]
    pub fn i2c1_rst_n_1(self) -> &'a mut W {
        self.variant(I2C1_RST_N_A::I2C1_RST_N_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "I2C2 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_RST_N_A {
    #[doc = "0: Assert the I2C2 reset."]
    I2C2_RST_N_0 = 0,
    #[doc = "1: Clear the I2C2 reset."]
    I2C2_RST_N_1 = 1,
}
impl From<I2C2_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2_RST_N` reader - I2C2 reset control."]
pub struct I2C2_RST_N_R(crate::FieldReader<bool, I2C2_RST_N_A>);
impl I2C2_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_RST_N_A {
        match self.bits {
            false => I2C2_RST_N_A::I2C2_RST_N_0,
            true => I2C2_RST_N_A::I2C2_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `I2C2_RST_N_0`"]
    #[inline(always)]
    pub fn is_i2c2_rst_n_0(&self) -> bool {
        **self == I2C2_RST_N_A::I2C2_RST_N_0
    }
    #[doc = "Checks if the value of the field is `I2C2_RST_N_1`"]
    #[inline(always)]
    pub fn is_i2c2_rst_n_1(&self) -> bool {
        **self == I2C2_RST_N_A::I2C2_RST_N_1
    }
}
impl core::ops::Deref for I2C2_RST_N_R {
    type Target = crate::FieldReader<bool, I2C2_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2_RST_N` writer - I2C2 reset control."]
pub struct I2C2_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the I2C2 reset."]
    #[inline(always)]
    pub fn i2c2_rst_n_0(self) -> &'a mut W {
        self.variant(I2C2_RST_N_A::I2C2_RST_N_0)
    }
    #[doc = "Clear the I2C2 reset."]
    #[inline(always)]
    pub fn i2c2_rst_n_1(self) -> &'a mut W {
        self.variant(I2C2_RST_N_A::I2C2_RST_N_1)
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
#[doc = "I2C3 reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C3_RST_N_A {
    #[doc = "0: Assert the I2C3 reset."]
    I2C3_RST_N_0 = 0,
    #[doc = "1: Clear the I2C3 reset."]
    I2C3_RST_N_1 = 1,
}
impl From<I2C3_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3_RST_N` reader - I2C3 reset control."]
pub struct I2C3_RST_N_R(crate::FieldReader<bool, I2C3_RST_N_A>);
impl I2C3_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3_RST_N_A {
        match self.bits {
            false => I2C3_RST_N_A::I2C3_RST_N_0,
            true => I2C3_RST_N_A::I2C3_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `I2C3_RST_N_0`"]
    #[inline(always)]
    pub fn is_i2c3_rst_n_0(&self) -> bool {
        **self == I2C3_RST_N_A::I2C3_RST_N_0
    }
    #[doc = "Checks if the value of the field is `I2C3_RST_N_1`"]
    #[inline(always)]
    pub fn is_i2c3_rst_n_1(&self) -> bool {
        **self == I2C3_RST_N_A::I2C3_RST_N_1
    }
}
impl core::ops::Deref for I2C3_RST_N_R {
    type Target = crate::FieldReader<bool, I2C3_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3_RST_N` writer - I2C3 reset control."]
pub struct I2C3_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the I2C3 reset."]
    #[inline(always)]
    pub fn i2c3_rst_n_0(self) -> &'a mut W {
        self.variant(I2C3_RST_N_A::I2C3_RST_N_0)
    }
    #[doc = "Clear the I2C3 reset."]
    #[inline(always)]
    pub fn i2c3_rst_n_1(self) -> &'a mut W {
        self.variant(I2C3_RST_N_A::I2C3_RST_N_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "ADC reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_RST_N_A {
    #[doc = "0: Assert the ADC reset."]
    ADC_RST_N_0 = 0,
    #[doc = "1: Clear the ADC reset."]
    ADC_RST_N_1 = 1,
}
impl From<ADC_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_RST_N` reader - ADC reset control."]
pub struct ADC_RST_N_R(crate::FieldReader<bool, ADC_RST_N_A>);
impl ADC_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_RST_N_A {
        match self.bits {
            false => ADC_RST_N_A::ADC_RST_N_0,
            true => ADC_RST_N_A::ADC_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_RST_N_0`"]
    #[inline(always)]
    pub fn is_adc_rst_n_0(&self) -> bool {
        **self == ADC_RST_N_A::ADC_RST_N_0
    }
    #[doc = "Checks if the value of the field is `ADC_RST_N_1`"]
    #[inline(always)]
    pub fn is_adc_rst_n_1(&self) -> bool {
        **self == ADC_RST_N_A::ADC_RST_N_1
    }
}
impl core::ops::Deref for ADC_RST_N_R {
    type Target = crate::FieldReader<bool, ADC_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_RST_N` writer - ADC reset control."]
pub struct ADC_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the ADC reset."]
    #[inline(always)]
    pub fn adc_rst_n_0(self) -> &'a mut W {
        self.variant(ADC_RST_N_A::ADC_RST_N_0)
    }
    #[doc = "Clear the ADC reset."]
    #[inline(always)]
    pub fn adc_rst_n_1(self) -> &'a mut W {
        self.variant(ADC_RST_N_A::ADC_RST_N_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "DMA reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_RST_N_A {
    #[doc = "0: Assert the DMA reset."]
    DMA_RST_N_0 = 0,
    #[doc = "1: Clear the DMA reset."]
    DMA_RST_N_1 = 1,
}
impl From<DMA_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RST_N` reader - DMA reset control."]
pub struct DMA_RST_N_R(crate::FieldReader<bool, DMA_RST_N_A>);
impl DMA_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_RST_N_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_RST_N_A {
        match self.bits {
            false => DMA_RST_N_A::DMA_RST_N_0,
            true => DMA_RST_N_A::DMA_RST_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_RST_N_0`"]
    #[inline(always)]
    pub fn is_dma_rst_n_0(&self) -> bool {
        **self == DMA_RST_N_A::DMA_RST_N_0
    }
    #[doc = "Checks if the value of the field is `DMA_RST_N_1`"]
    #[inline(always)]
    pub fn is_dma_rst_n_1(&self) -> bool {
        **self == DMA_RST_N_A::DMA_RST_N_1
    }
}
impl core::ops::Deref for DMA_RST_N_R {
    type Target = crate::FieldReader<bool, DMA_RST_N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_RST_N` writer - DMA reset control."]
pub struct DMA_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_RST_N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Assert the DMA reset."]
    #[inline(always)]
    pub fn dma_rst_n_0(self) -> &'a mut W {
        self.variant(DMA_RST_N_A::DMA_RST_N_0)
    }
    #[doc = "Clear the DMA reset."]
    #[inline(always)]
    pub fn dma_rst_n_1(self) -> &'a mut W {
        self.variant(DMA_RST_N_A::DMA_RST_N_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI0 reset control."]
    #[inline(always)]
    pub fn spi0_rst_n(&self) -> SPI0_RST_N_R {
        SPI0_RST_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI1 reset control."]
    #[inline(always)]
    pub fn spi1_rst_n(&self) -> SPI1_RST_N_R {
        SPI1_RST_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USART fractional baud rate generator(UARTFRG) reset control."]
    #[inline(always)]
    pub fn uartfrg_rst_n(&self) -> UARTFRG_RST_N_R {
        UARTFRG_RST_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USART0 reset control."]
    #[inline(always)]
    pub fn uart0_rst_n(&self) -> UART0_RST_N_R {
        UART0_RST_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1 reset control."]
    #[inline(always)]
    pub fn uart1_rst_n(&self) -> UART1_RST_N_R {
        UART1_RST_N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USART2 reset control."]
    #[inline(always)]
    pub fn uart2_rst_n(&self) -> UART2_RST_N_R {
        UART2_RST_N_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C0 reset control."]
    #[inline(always)]
    pub fn i2c0_rst_n(&self) -> I2C0_RST_N_R {
        I2C0_RST_N_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Multi-rate timer (MRT) reset control."]
    #[inline(always)]
    pub fn mrt_rst_n(&self) -> MRT_RST_N_R {
        MRT_RST_N_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SCT reset control."]
    #[inline(always)]
    pub fn sct_rst_n(&self) -> SCT_RST_N_R {
        SCT_RST_N_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Self-wake-up timer (WKT) reset control."]
    #[inline(always)]
    pub fn wkt_rst_n(&self) -> WKT_RST_N_R {
        WKT_RST_N_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO and GPIO pin interrupt reset control."]
    #[inline(always)]
    pub fn gpio_rst_n(&self) -> GPIO_RST_N_R {
        GPIO_RST_N_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Flash controller reset control."]
    #[inline(always)]
    pub fn flash_rst_n(&self) -> FLASH_RST_N_R {
        FLASH_RST_N_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Analog comparator reset control."]
    #[inline(always)]
    pub fn acmp_rst_n(&self) -> ACMP_RST_N_R {
        ACMP_RST_N_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C1 reset control."]
    #[inline(always)]
    pub fn i2c1_rst_n(&self) -> I2C1_RST_N_R {
        I2C1_RST_N_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C2 reset control."]
    #[inline(always)]
    pub fn i2c2_rst_n(&self) -> I2C2_RST_N_R {
        I2C2_RST_N_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C3 reset control."]
    #[inline(always)]
    pub fn i2c3_rst_n(&self) -> I2C3_RST_N_R {
        I2C3_RST_N_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC reset control."]
    #[inline(always)]
    pub fn adc_rst_n(&self) -> ADC_RST_N_R {
        ADC_RST_N_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA reset control."]
    #[inline(always)]
    pub fn dma_rst_n(&self) -> DMA_RST_N_R {
        DMA_RST_N_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI0 reset control."]
    #[inline(always)]
    pub fn spi0_rst_n(&mut self) -> SPI0_RST_N_W {
        SPI0_RST_N_W { w: self }
    }
    #[doc = "Bit 1 - SPI1 reset control."]
    #[inline(always)]
    pub fn spi1_rst_n(&mut self) -> SPI1_RST_N_W {
        SPI1_RST_N_W { w: self }
    }
    #[doc = "Bit 2 - USART fractional baud rate generator(UARTFRG) reset control."]
    #[inline(always)]
    pub fn uartfrg_rst_n(&mut self) -> UARTFRG_RST_N_W {
        UARTFRG_RST_N_W { w: self }
    }
    #[doc = "Bit 3 - USART0 reset control."]
    #[inline(always)]
    pub fn uart0_rst_n(&mut self) -> UART0_RST_N_W {
        UART0_RST_N_W { w: self }
    }
    #[doc = "Bit 4 - USART1 reset control."]
    #[inline(always)]
    pub fn uart1_rst_n(&mut self) -> UART1_RST_N_W {
        UART1_RST_N_W { w: self }
    }
    #[doc = "Bit 5 - USART2 reset control."]
    #[inline(always)]
    pub fn uart2_rst_n(&mut self) -> UART2_RST_N_W {
        UART2_RST_N_W { w: self }
    }
    #[doc = "Bit 6 - I2C0 reset control."]
    #[inline(always)]
    pub fn i2c0_rst_n(&mut self) -> I2C0_RST_N_W {
        I2C0_RST_N_W { w: self }
    }
    #[doc = "Bit 7 - Multi-rate timer (MRT) reset control."]
    #[inline(always)]
    pub fn mrt_rst_n(&mut self) -> MRT_RST_N_W {
        MRT_RST_N_W { w: self }
    }
    #[doc = "Bit 8 - SCT reset control."]
    #[inline(always)]
    pub fn sct_rst_n(&mut self) -> SCT_RST_N_W {
        SCT_RST_N_W { w: self }
    }
    #[doc = "Bit 9 - Self-wake-up timer (WKT) reset control."]
    #[inline(always)]
    pub fn wkt_rst_n(&mut self) -> WKT_RST_N_W {
        WKT_RST_N_W { w: self }
    }
    #[doc = "Bit 10 - GPIO and GPIO pin interrupt reset control."]
    #[inline(always)]
    pub fn gpio_rst_n(&mut self) -> GPIO_RST_N_W {
        GPIO_RST_N_W { w: self }
    }
    #[doc = "Bit 11 - Flash controller reset control."]
    #[inline(always)]
    pub fn flash_rst_n(&mut self) -> FLASH_RST_N_W {
        FLASH_RST_N_W { w: self }
    }
    #[doc = "Bit 12 - Analog comparator reset control."]
    #[inline(always)]
    pub fn acmp_rst_n(&mut self) -> ACMP_RST_N_W {
        ACMP_RST_N_W { w: self }
    }
    #[doc = "Bit 14 - I2C1 reset control."]
    #[inline(always)]
    pub fn i2c1_rst_n(&mut self) -> I2C1_RST_N_W {
        I2C1_RST_N_W { w: self }
    }
    #[doc = "Bit 15 - I2C2 reset control."]
    #[inline(always)]
    pub fn i2c2_rst_n(&mut self) -> I2C2_RST_N_W {
        I2C2_RST_N_W { w: self }
    }
    #[doc = "Bit 16 - I2C3 reset control."]
    #[inline(always)]
    pub fn i2c3_rst_n(&mut self) -> I2C3_RST_N_W {
        I2C3_RST_N_W { w: self }
    }
    #[doc = "Bit 24 - ADC reset control."]
    #[inline(always)]
    pub fn adc_rst_n(&mut self) -> ADC_RST_N_W {
        ADC_RST_N_W { w: self }
    }
    #[doc = "Bit 29 - DMA reset control."]
    #[inline(always)]
    pub fn dma_rst_n(&mut self) -> DMA_RST_N_W {
        DMA_RST_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl](index.html) module"]
pub struct PRESETCTRL_SPEC;
impl crate::RegisterSpec for PRESETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl::R](R) reader structure"]
impl crate::Readable for PRESETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl::W](W) writer structure"]
impl crate::Writable for PRESETCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL to value 0x2101_dfff"]
impl crate::Resettable for PRESETCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2101_dfff
    }
}
