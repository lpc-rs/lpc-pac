///Register `FIFOCFG` reader
pub struct R(crate::R<FIFOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOCFG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FIFOCFG` writer
pub struct W(crate::W<FIFOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOCFG_SPEC>;
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
impl From<crate::W<FIFOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOCFG_SPEC>) -> Self {
        W(writer)
    }
}
///Enable the transmit FIFO.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLETX_A {
    ///0: The transmit FIFO is not enabled.
    DISABLED = 0,
    ///1: The transmit FIFO is enabled.
    ENABLED = 1,
}
impl From<ENABLETX_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLETX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLETX` reader - Enable the transmit FIFO.
pub struct ENABLETX_R(crate::FieldReader<bool, ENABLETX_A>);
impl ENABLETX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLETX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENABLETX_A {
        match self.bits {
            false => ENABLETX_A::DISABLED,
            true => ENABLETX_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENABLETX_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENABLETX_A::ENABLED
    }
}
impl core::ops::Deref for ENABLETX_R {
    type Target = crate::FieldReader<bool, ENABLETX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ENABLETX` writer - Enable the transmit FIFO.
pub struct ENABLETX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLETX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ENABLETX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The transmit FIFO is not enabled.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLETX_A::DISABLED)
    }
    ///The transmit FIFO is enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLETX_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Enable the receive FIFO.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLERX_A {
    ///0: The receive FIFO is not enabled.
    DISABLED = 0,
    ///1: The receive FIFO is enabled.
    ENABLED = 1,
}
impl From<ENABLERX_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLERX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLERX` reader - Enable the receive FIFO.
pub struct ENABLERX_R(crate::FieldReader<bool, ENABLERX_A>);
impl ENABLERX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLERX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENABLERX_A {
        match self.bits {
            false => ENABLERX_A::DISABLED,
            true => ENABLERX_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENABLERX_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENABLERX_A::ENABLED
    }
}
impl core::ops::Deref for ENABLERX_R {
    type Target = crate::FieldReader<bool, ENABLERX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ENABLERX` writer - Enable the receive FIFO.
pub struct ENABLERX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLERX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ENABLERX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The receive FIFO is not enabled.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLERX_A::DISABLED)
    }
    ///The receive FIFO is enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLERX_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXI2SE0_A {
    ///0: If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair.
    LAST_VALUE = 0,
    ///1: If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred.
    ZERO = 1,
}
impl From<TXI2SE0_A> for bool {
    #[inline(always)]
    fn from(variant: TXI2SE0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXI2SE0` reader - Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused.
pub struct TXI2SE0_R(crate::FieldReader<bool, TXI2SE0_A>);
impl TXI2SE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXI2SE0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXI2SE0_A {
        match self.bits {
            false => TXI2SE0_A::LAST_VALUE,
            true => TXI2SE0_A::ZERO,
        }
    }
    ///Checks if the value of the field is `LAST_VALUE`
    #[inline(always)]
    pub fn is_last_value(&self) -> bool {
        **self == TXI2SE0_A::LAST_VALUE
    }
    ///Checks if the value of the field is `ZERO`
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == TXI2SE0_A::ZERO
    }
}
impl core::ops::Deref for TXI2SE0_R {
    type Target = crate::FieldReader<bool, TXI2SE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXI2SE0` writer - Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused.
pub struct TXI2SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXI2SE0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXI2SE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair.
    #[inline(always)]
    pub fn last_value(self) -> &'a mut W {
        self.variant(TXI2SE0_A::LAST_VALUE)
    }
    ///If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred.
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXI2SE0_A::ZERO)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACK48_A {
    ///0: 48-bit I2S FIFO entries are handled as all 24-bit values.
    BIT_24 = 0,
    ///1: 48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values.
    BIT_32_16 = 1,
}
impl From<PACK48_A> for bool {
    #[inline(always)]
    fn from(variant: PACK48_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PACK48` reader - Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA.
pub struct PACK48_R(crate::FieldReader<bool, PACK48_A>);
impl PACK48_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PACK48_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PACK48_A {
        match self.bits {
            false => PACK48_A::BIT_24,
            true => PACK48_A::BIT_32_16,
        }
    }
    ///Checks if the value of the field is `BIT_24`
    #[inline(always)]
    pub fn is_bit_24(&self) -> bool {
        **self == PACK48_A::BIT_24
    }
    ///Checks if the value of the field is `BIT_32_16`
    #[inline(always)]
    pub fn is_bit_32_16(&self) -> bool {
        **self == PACK48_A::BIT_32_16
    }
}
impl core::ops::Deref for PACK48_R {
    type Target = crate::FieldReader<bool, PACK48_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PACK48` writer - Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA.
pub struct PACK48_W<'a> {
    w: &'a mut W,
}
impl<'a> PACK48_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PACK48_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///48-bit I2S FIFO entries are handled as all 24-bit values.
    #[inline(always)]
    pub fn bit_24(self) -> &'a mut W {
        self.variant(PACK48_A::BIT_24)
    }
    ///48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values.
    #[inline(always)]
    pub fn bit_32_16(self) -> &'a mut W {
        self.variant(PACK48_A::BIT_32_16)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `SIZE` reader - FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART.
pub struct SIZE_R(crate::FieldReader<u8, u8>);
impl SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///DMA configuration for transmit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMATX_A {
    ///0: DMA is not used for the transmit function.
    DISABLED = 0,
    ///1: Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled.
    ENABLED = 1,
}
impl From<DMATX_A> for bool {
    #[inline(always)]
    fn from(variant: DMATX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMATX` reader - DMA configuration for transmit.
pub struct DMATX_R(crate::FieldReader<bool, DMATX_A>);
impl DMATX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMATX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMATX_A {
        match self.bits {
            false => DMATX_A::DISABLED,
            true => DMATX_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMATX_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMATX_A::ENABLED
    }
}
impl core::ops::Deref for DMATX_R {
    type Target = crate::FieldReader<bool, DMATX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMATX` writer - DMA configuration for transmit.
pub struct DMATX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMATX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA is not used for the transmit function.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMATX_A::DISABLED)
    }
    ///Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMATX_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///DMA configuration for receive.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMARX_A {
    ///0: DMA is not used for the receive function.
    DISABLED = 0,
    ///1: Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled.
    ENABLED = 1,
}
impl From<DMARX_A> for bool {
    #[inline(always)]
    fn from(variant: DMARX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMARX` reader - DMA configuration for receive.
pub struct DMARX_R(crate::FieldReader<bool, DMARX_A>);
impl DMARX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMARX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMARX_A {
        match self.bits {
            false => DMARX_A::DISABLED,
            true => DMARX_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMARX_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMARX_A::ENABLED
    }
}
impl core::ops::Deref for DMARX_R {
    type Target = crate::FieldReader<bool, DMARX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMARX` writer - DMA configuration for receive.
pub struct DMARX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMARX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA is not used for the receive function.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMARX_A::DISABLED)
    }
    ///Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMARX_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKETX_A {
    ///0: Only enabled interrupts will wake up the device form reduced power modes.
    DISABLED = 0,
    ///1: A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled.
    ENABLED = 1,
}
impl From<WAKETX_A> for bool {
    #[inline(always)]
    fn from(variant: WAKETX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WAKETX` reader - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.
pub struct WAKETX_R(crate::FieldReader<bool, WAKETX_A>);
impl WAKETX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKETX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAKETX_A {
        match self.bits {
            false => WAKETX_A::DISABLED,
            true => WAKETX_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WAKETX_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WAKETX_A::ENABLED
    }
}
impl core::ops::Deref for WAKETX_R {
    type Target = crate::FieldReader<bool, WAKETX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAKETX` writer - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.
pub struct WAKETX_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKETX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WAKETX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Only enabled interrupts will wake up the device form reduced power modes.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKETX_A::DISABLED)
    }
    ///A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKETX_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKERX_A {
    ///0: Only enabled interrupts will wake up the device form reduced power modes.
    DISABLED = 0,
    ///1: A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled.
    ENABLED = 1,
}
impl From<WAKERX_A> for bool {
    #[inline(always)]
    fn from(variant: WAKERX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WAKERX` reader - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.
pub struct WAKERX_R(crate::FieldReader<bool, WAKERX_A>);
impl WAKERX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKERX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAKERX_A {
        match self.bits {
            false => WAKERX_A::DISABLED,
            true => WAKERX_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WAKERX_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WAKERX_A::ENABLED
    }
}
impl core::ops::Deref for WAKERX_R {
    type Target = crate::FieldReader<bool, WAKERX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAKERX` writer - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.
pub struct WAKERX_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKERX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WAKERX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Only enabled interrupts will wake up the device form reduced power modes.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKERX_A::DISABLED)
    }
    ///A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKERX_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///Field `EMPTYTX` reader - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied.
pub struct EMPTYTX_R(crate::FieldReader<bool, bool>);
impl EMPTYTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPTYTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPTYTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EMPTYTX` writer - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied.
pub struct EMPTYTX_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTYTX_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Field `EMPTYRX` reader - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied.
pub struct EMPTYRX_R(crate::FieldReader<bool, bool>);
impl EMPTYRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPTYRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPTYRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EMPTYRX` writer - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied.
pub struct EMPTYRX_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTYRX_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Pop FIFO for debug reads.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POPDBG_A {
    ///0: Debug reads of the FIFO do not pop the FIFO.
    DO_NOT_POP = 0,
    ///1: A debug read will cause the FIFO to pop.
    POP = 1,
}
impl From<POPDBG_A> for bool {
    #[inline(always)]
    fn from(variant: POPDBG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `POPDBG` reader - Pop FIFO for debug reads.
pub struct POPDBG_R(crate::FieldReader<bool, POPDBG_A>);
impl POPDBG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POPDBG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POPDBG_A {
        match self.bits {
            false => POPDBG_A::DO_NOT_POP,
            true => POPDBG_A::POP,
        }
    }
    ///Checks if the value of the field is `DO_NOT_POP`
    #[inline(always)]
    pub fn is_do_not_pop(&self) -> bool {
        **self == POPDBG_A::DO_NOT_POP
    }
    ///Checks if the value of the field is `POP`
    #[inline(always)]
    pub fn is_pop(&self) -> bool {
        **self == POPDBG_A::POP
    }
}
impl core::ops::Deref for POPDBG_R {
    type Target = crate::FieldReader<bool, POPDBG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `POPDBG` writer - Pop FIFO for debug reads.
pub struct POPDBG_W<'a> {
    w: &'a mut W,
}
impl<'a> POPDBG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: POPDBG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Debug reads of the FIFO do not pop the FIFO.
    #[inline(always)]
    pub fn do_not_pop(self) -> &'a mut W {
        self.variant(POPDBG_A::DO_NOT_POP)
    }
    ///A debug read will cause the FIFO to pop.
    #[inline(always)]
    pub fn pop(self) -> &'a mut W {
        self.variant(POPDBG_A::POP)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    ///Bit 0 - Enable the transmit FIFO.
    #[inline(always)]
    pub fn enabletx(&self) -> ENABLETX_R {
        ENABLETX_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Enable the receive FIFO.
    #[inline(always)]
    pub fn enablerx(&self) -> ENABLERX_R {
        ENABLERX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused.
    #[inline(always)]
    pub fn txi2se0(&self) -> TXI2SE0_R {
        TXI2SE0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA.
    #[inline(always)]
    pub fn pack48(&self) -> PACK48_R {
        PACK48_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 4:5 - FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART.
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 12 - DMA configuration for transmit.
    #[inline(always)]
    pub fn dmatx(&self) -> DMATX_R {
        DMATX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - DMA configuration for receive.
    #[inline(always)]
    pub fn dmarx(&self) -> DMARX_R {
        DMARX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.
    #[inline(always)]
    pub fn waketx(&self) -> WAKETX_R {
        WAKETX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.
    #[inline(always)]
    pub fn wakerx(&self) -> WAKERX_R {
        WAKERX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied.
    #[inline(always)]
    pub fn emptytx(&self) -> EMPTYTX_R {
        EMPTYTX_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied.
    #[inline(always)]
    pub fn emptyrx(&self) -> EMPTYRX_R {
        EMPTYRX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Pop FIFO for debug reads.
    #[inline(always)]
    pub fn popdbg(&self) -> POPDBG_R {
        POPDBG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Enable the transmit FIFO.
    #[inline(always)]
    pub fn enabletx(&mut self) -> ENABLETX_W {
        ENABLETX_W { w: self }
    }
    ///Bit 1 - Enable the receive FIFO.
    #[inline(always)]
    pub fn enablerx(&mut self) -> ENABLERX_W {
        ENABLERX_W { w: self }
    }
    ///Bit 2 - Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused.
    #[inline(always)]
    pub fn txi2se0(&mut self) -> TXI2SE0_W {
        TXI2SE0_W { w: self }
    }
    ///Bit 3 - Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA.
    #[inline(always)]
    pub fn pack48(&mut self) -> PACK48_W {
        PACK48_W { w: self }
    }
    ///Bit 12 - DMA configuration for transmit.
    #[inline(always)]
    pub fn dmatx(&mut self) -> DMATX_W {
        DMATX_W { w: self }
    }
    ///Bit 13 - DMA configuration for receive.
    #[inline(always)]
    pub fn dmarx(&mut self) -> DMARX_W {
        DMARX_W { w: self }
    }
    ///Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.
    #[inline(always)]
    pub fn waketx(&mut self) -> WAKETX_W {
        WAKETX_W { w: self }
    }
    ///Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.
    #[inline(always)]
    pub fn wakerx(&mut self) -> WAKERX_W {
        WAKERX_W { w: self }
    }
    ///Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied.
    #[inline(always)]
    pub fn emptytx(&mut self) -> EMPTYTX_W {
        EMPTYTX_W { w: self }
    }
    ///Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied.
    #[inline(always)]
    pub fn emptyrx(&mut self) -> EMPTYRX_W {
        EMPTYRX_W { w: self }
    }
    ///Bit 18 - Pop FIFO for debug reads.
    #[inline(always)]
    pub fn popdbg(&mut self) -> POPDBG_W {
        POPDBG_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FIFO configuration and enable register.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fifocfg](index.html) module
pub struct FIFOCFG_SPEC;
impl crate::RegisterSpec for FIFOCFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [fifocfg::R](R) reader structure
impl crate::Readable for FIFOCFG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fifocfg::W](W) writer structure
impl crate::Writable for FIFOCFG_SPEC {
    type Writer = W;
}
///`reset()` method sets FIFOCFG to value 0
impl crate::Resettable for FIFOCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
