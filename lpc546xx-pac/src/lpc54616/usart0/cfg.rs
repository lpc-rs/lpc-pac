///Register `CFG` reader
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFG` writer
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
///USART Enable.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    ///0: Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. When re-enabled, the USART will immediately be ready to transmit because the transmitter has been reset and is therefore available.
    DISABLED = 0,
    ///1: Enabled. The USART is enabled for operation.
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - USART Enable.
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENABLE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENABLE_A::ENABLED
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ENABLE` writer - USART Enable.
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. When re-enabled, the USART will immediately be ready to transmit because the transmitter has been reset and is therefore available.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    ///Enabled. The USART is enabled for operation.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
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
///Selects the data size for the USART.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATALEN_A {
    ///0: 7 bit Data length.
    BIT_7 = 0,
    ///1: 8 bit Data length.
    BIT_8 = 1,
    ///2: 9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register.
    BIT_9 = 2,
}
impl From<DATALEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATALEN_A) -> Self {
        variant as _
    }
}
///Field `DATALEN` reader - Selects the data size for the USART.
pub struct DATALEN_R(crate::FieldReader<u8, DATALEN_A>);
impl DATALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATALEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DATALEN_A> {
        match self.bits {
            0 => Some(DATALEN_A::BIT_7),
            1 => Some(DATALEN_A::BIT_8),
            2 => Some(DATALEN_A::BIT_9),
            _ => None,
        }
    }
    ///Checks if the value of the field is `BIT_7`
    #[inline(always)]
    pub fn is_bit_7(&self) -> bool {
        **self == DATALEN_A::BIT_7
    }
    ///Checks if the value of the field is `BIT_8`
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        **self == DATALEN_A::BIT_8
    }
    ///Checks if the value of the field is `BIT_9`
    #[inline(always)]
    pub fn is_bit_9(&self) -> bool {
        **self == DATALEN_A::BIT_9
    }
}
impl core::ops::Deref for DATALEN_R {
    type Target = crate::FieldReader<u8, DATALEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DATALEN` writer - Selects the data size for the USART.
pub struct DATALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DATALEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///7 bit Data length.
    #[inline(always)]
    pub fn bit_7(self) -> &'a mut W {
        self.variant(DATALEN_A::BIT_7)
    }
    ///8 bit Data length.
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut W {
        self.variant(DATALEN_A::BIT_8)
    }
    ///9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register.
    #[inline(always)]
    pub fn bit_9(self) -> &'a mut W {
        self.variant(DATALEN_A::BIT_9)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Selects what type of parity is used by the USART.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PARITYSEL_A {
    ///0: No parity.
    NO_PARITY = 0,
    ///2: Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even.
    EVEN_PARITY = 2,
    ///3: Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd.
    ODD_PARITY = 3,
}
impl From<PARITYSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITYSEL_A) -> Self {
        variant as _
    }
}
///Field `PARITYSEL` reader - Selects what type of parity is used by the USART.
pub struct PARITYSEL_R(crate::FieldReader<u8, PARITYSEL_A>);
impl PARITYSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PARITYSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PARITYSEL_A> {
        match self.bits {
            0 => Some(PARITYSEL_A::NO_PARITY),
            2 => Some(PARITYSEL_A::EVEN_PARITY),
            3 => Some(PARITYSEL_A::ODD_PARITY),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NO_PARITY`
    #[inline(always)]
    pub fn is_no_parity(&self) -> bool {
        **self == PARITYSEL_A::NO_PARITY
    }
    ///Checks if the value of the field is `EVEN_PARITY`
    #[inline(always)]
    pub fn is_even_parity(&self) -> bool {
        **self == PARITYSEL_A::EVEN_PARITY
    }
    ///Checks if the value of the field is `ODD_PARITY`
    #[inline(always)]
    pub fn is_odd_parity(&self) -> bool {
        **self == PARITYSEL_A::ODD_PARITY
    }
}
impl core::ops::Deref for PARITYSEL_R {
    type Target = crate::FieldReader<u8, PARITYSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PARITYSEL` writer - Selects what type of parity is used by the USART.
pub struct PARITYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITYSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PARITYSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No parity.
    #[inline(always)]
    pub fn no_parity(self) -> &'a mut W {
        self.variant(PARITYSEL_A::NO_PARITY)
    }
    ///Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even.
    #[inline(always)]
    pub fn even_parity(self) -> &'a mut W {
        self.variant(PARITYSEL_A::EVEN_PARITY)
    }
    ///Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd.
    #[inline(always)]
    pub fn odd_parity(self) -> &'a mut W {
        self.variant(PARITYSEL_A::ODD_PARITY)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
///Number of stop bits appended to transmitted data. Only a single stop bit is required for received data.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPLEN_A {
    ///0: 1 stop bit.
    BIT_1 = 0,
    ///1: 2 stop bits. This setting should only be used for asynchronous communication.
    BITS_2 = 1,
}
impl From<STOPLEN_A> for bool {
    #[inline(always)]
    fn from(variant: STOPLEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPLEN` reader - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data.
pub struct STOPLEN_R(crate::FieldReader<bool, STOPLEN_A>);
impl STOPLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOPLEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPLEN_A {
        match self.bits {
            false => STOPLEN_A::BIT_1,
            true => STOPLEN_A::BITS_2,
        }
    }
    ///Checks if the value of the field is `BIT_1`
    #[inline(always)]
    pub fn is_bit_1(&self) -> bool {
        **self == STOPLEN_A::BIT_1
    }
    ///Checks if the value of the field is `BITS_2`
    #[inline(always)]
    pub fn is_bits_2(&self) -> bool {
        **self == STOPLEN_A::BITS_2
    }
}
impl core::ops::Deref for STOPLEN_R {
    type Target = crate::FieldReader<bool, STOPLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `STOPLEN` writer - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data.
pub struct STOPLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPLEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STOPLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///1 stop bit.
    #[inline(always)]
    pub fn bit_1(self) -> &'a mut W {
        self.variant(STOPLEN_A::BIT_1)
    }
    ///2 stop bits. This setting should only be used for asynchronous communication.
    #[inline(always)]
    pub fn bits_2(self) -> &'a mut W {
        self.variant(STOPLEN_A::BITS_2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Selects standard or 32 kHz clocking mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE32K_A {
    ///0: Disabled. USART uses standard clocking.
    DISABLED = 0,
    ///1: Enabled. USART uses the 32 kHz clock from the RTC oscillator as the clock source to the BRG, and uses a special bit clocking scheme.
    ENABLED = 1,
}
impl From<MODE32K_A> for bool {
    #[inline(always)]
    fn from(variant: MODE32K_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MODE32K` reader - Selects standard or 32 kHz clocking mode.
pub struct MODE32K_R(crate::FieldReader<bool, MODE32K_A>);
impl MODE32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE32K_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE32K_A {
        match self.bits {
            false => MODE32K_A::DISABLED,
            true => MODE32K_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MODE32K_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MODE32K_A::ENABLED
    }
}
impl core::ops::Deref for MODE32K_R {
    type Target = crate::FieldReader<bool, MODE32K_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MODE32K` writer - Selects standard or 32 kHz clocking mode.
pub struct MODE32K_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE32K_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE32K_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. USART uses standard clocking.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE32K_A::DISABLED)
    }
    ///Enabled. USART uses the 32 kHz clock from the RTC oscillator as the clock source to the BRG, and uses a special bit clocking scheme.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MODE32K_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///LIN break mode enable.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINMODE_A {
    ///0: Disabled. Break detect and generate is configured for normal operation.
    DISABLED = 0,
    ///1: Enabled. Break detect and generate is configured for LIN bus operation.
    ENABLED = 1,
}
impl From<LINMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LINMODE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LINMODE` reader - LIN break mode enable.
pub struct LINMODE_R(crate::FieldReader<bool, LINMODE_A>);
impl LINMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LINMODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LINMODE_A {
        match self.bits {
            false => LINMODE_A::DISABLED,
            true => LINMODE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LINMODE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LINMODE_A::ENABLED
    }
}
impl core::ops::Deref for LINMODE_R {
    type Target = crate::FieldReader<bool, LINMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LINMODE` writer - LIN break mode enable.
pub struct LINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LINMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. Break detect and generate is configured for normal operation.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LINMODE_A::DISABLED)
    }
    ///Enabled. Break detect and generate is configured for LIN bus operation.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LINMODE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEN_A {
    ///0: No flow control. The transmitter does not receive any automatic flow control signal.
    DISABLED = 0,
    ///1: Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes.
    ENABLED = 1,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSEN` reader - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled.
pub struct CTSEN_R(crate::FieldReader<bool, CTSEN_A>);
impl CTSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLED,
            true => CTSEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTSEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTSEN_A::ENABLED
    }
}
impl core::ops::Deref for CTSEN_R {
    type Target = crate::FieldReader<bool, CTSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTSEN` writer - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled.
pub struct CTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No flow control. The transmitter does not receive any automatic flow control signal.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLED)
    }
    ///Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Selects synchronous or asynchronous operation.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN_A {
    ///0: Asynchronous mode.
    ASYNCHRONOUS_MODE = 0,
    ///1: Synchronous mode.
    SYNCHRONOUS_MODE = 1,
}
impl From<SYNCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCEN` reader - Selects synchronous or asynchronous operation.
pub struct SYNCEN_R(crate::FieldReader<bool, SYNCEN_A>);
impl SYNCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN_A {
        match self.bits {
            false => SYNCEN_A::ASYNCHRONOUS_MODE,
            true => SYNCEN_A::SYNCHRONOUS_MODE,
        }
    }
    ///Checks if the value of the field is `ASYNCHRONOUS_MODE`
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        **self == SYNCEN_A::ASYNCHRONOUS_MODE
    }
    ///Checks if the value of the field is `SYNCHRONOUS_MODE`
    #[inline(always)]
    pub fn is_synchronous_mode(&self) -> bool {
        **self == SYNCEN_A::SYNCHRONOUS_MODE
    }
}
impl core::ops::Deref for SYNCEN_R {
    type Target = crate::FieldReader<bool, SYNCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNCEN` writer - Selects synchronous or asynchronous operation.
pub struct SYNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Asynchronous mode.
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(SYNCEN_A::ASYNCHRONOUS_MODE)
    }
    ///Synchronous mode.
    #[inline(always)]
    pub fn synchronous_mode(self) -> &'a mut W {
        self.variant(SYNCEN_A::SYNCHRONOUS_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Selects the clock polarity and sampling edge of received data in synchronous mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKPOL_A {
    ///0: Falling edge. Un_RXD is sampled on the falling edge of SCLK.
    FALLING_EDGE = 0,
    ///1: Rising edge. Un_RXD is sampled on the rising edge of SCLK.
    RISING_EDGE = 1,
}
impl From<CLKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CLKPOL` reader - Selects the clock polarity and sampling edge of received data in synchronous mode.
pub struct CLKPOL_R(crate::FieldReader<bool, CLKPOL_A>);
impl CLKPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKPOL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CLKPOL_A {
        match self.bits {
            false => CLKPOL_A::FALLING_EDGE,
            true => CLKPOL_A::RISING_EDGE,
        }
    }
    ///Checks if the value of the field is `FALLING_EDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == CLKPOL_A::FALLING_EDGE
    }
    ///Checks if the value of the field is `RISING_EDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == CLKPOL_A::RISING_EDGE
    }
}
impl core::ops::Deref for CLKPOL_R {
    type Target = crate::FieldReader<bool, CLKPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CLKPOL` writer - Selects the clock polarity and sampling edge of received data in synchronous mode.
pub struct CLKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CLKPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge. Un_RXD is sampled on the falling edge of SCLK.
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CLKPOL_A::FALLING_EDGE)
    }
    ///Rising edge. Un_RXD is sampled on the rising edge of SCLK.
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CLKPOL_A::RISING_EDGE)
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
///Synchronous mode Master select.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCMST_A {
    ///0: Slave. When synchronous mode is enabled, the USART is a slave.
    SLAVE = 0,
    ///1: Master. When synchronous mode is enabled, the USART is a master.
    MASTER = 1,
}
impl From<SYNCMST_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCMST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNCMST` reader - Synchronous mode Master select.
pub struct SYNCMST_R(crate::FieldReader<bool, SYNCMST_A>);
impl SYNCMST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCMST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SYNCMST_A {
        match self.bits {
            false => SYNCMST_A::SLAVE,
            true => SYNCMST_A::MASTER,
        }
    }
    ///Checks if the value of the field is `SLAVE`
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        **self == SYNCMST_A::SLAVE
    }
    ///Checks if the value of the field is `MASTER`
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        **self == SYNCMST_A::MASTER
    }
}
impl core::ops::Deref for SYNCMST_R {
    type Target = crate::FieldReader<bool, SYNCMST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNCMST` writer - Synchronous mode Master select.
pub struct SYNCMST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCMST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SYNCMST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Slave. When synchronous mode is enabled, the USART is a slave.
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(SYNCMST_A::SLAVE)
    }
    ///Master. When synchronous mode is enabled, the USART is a master.
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(SYNCMST_A::MASTER)
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
///Selects data loopback mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOP_A {
    ///0: Normal operation.
    NORMAL = 0,
    ///1: Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN.
    LOOPBACK = 1,
}
impl From<LOOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LOOP` reader - Selects data loopback mode.
pub struct LOOP_R(crate::FieldReader<bool, LOOP_A>);
impl LOOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LOOP_A {
        match self.bits {
            false => LOOP_A::NORMAL,
            true => LOOP_A::LOOPBACK,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == LOOP_A::NORMAL
    }
    ///Checks if the value of the field is `LOOPBACK`
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        **self == LOOP_A::LOOPBACK
    }
}
impl core::ops::Deref for LOOP_R {
    type Target = crate::FieldReader<bool, LOOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LOOP` writer - Selects data loopback mode.
pub struct LOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LOOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal operation.
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LOOP_A::NORMAL)
    }
    ///Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN.
    #[inline(always)]
    pub fn loopback(self) -> &'a mut W {
        self.variant(LOOP_A::LOOPBACK)
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
///Output Enable Turnaround time enable for RS-485 operation.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OETA_A {
    ///0: Disabled. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission.
    DISABLED = 0,
    ///1: Enabled. If selected by OESEL, the Output Enable signal remains asserted for one character time after the end of the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted.
    ENABLED = 1,
}
impl From<OETA_A> for bool {
    #[inline(always)]
    fn from(variant: OETA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OETA` reader - Output Enable Turnaround time enable for RS-485 operation.
pub struct OETA_R(crate::FieldReader<bool, OETA_A>);
impl OETA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OETA_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OETA_A {
        match self.bits {
            false => OETA_A::DISABLED,
            true => OETA_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OETA_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OETA_A::ENABLED
    }
}
impl core::ops::Deref for OETA_R {
    type Target = crate::FieldReader<bool, OETA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OETA` writer - Output Enable Turnaround time enable for RS-485 operation.
pub struct OETA_W<'a> {
    w: &'a mut W,
}
impl<'a> OETA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OETA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OETA_A::DISABLED)
    }
    ///Enabled. If selected by OESEL, the Output Enable signal remains asserted for one character time after the end of the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OETA_A::ENABLED)
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
///Automatic Address matching enable.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOADDR_A {
    ///0: Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address).
    DISABLED = 0,
    ///1: Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match.
    ENABLED = 1,
}
impl From<AUTOADDR_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOADDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOADDR` reader - Automatic Address matching enable.
pub struct AUTOADDR_R(crate::FieldReader<bool, AUTOADDR_A>);
impl AUTOADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOADDR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AUTOADDR_A {
        match self.bits {
            false => AUTOADDR_A::DISABLED,
            true => AUTOADDR_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AUTOADDR_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AUTOADDR_A::ENABLED
    }
}
impl core::ops::Deref for AUTOADDR_R {
    type Target = crate::FieldReader<bool, AUTOADDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AUTOADDR` writer - Automatic Address matching enable.
pub struct AUTOADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOADDR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AUTOADDR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address).
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOADDR_A::DISABLED)
    }
    ///Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOADDR_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///Output Enable Select.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OESEL_A {
    ///0: Standard. The RTS signal is used as the standard flow control function.
    STANDARD = 0,
    ///1: RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver.
    RS_485 = 1,
}
impl From<OESEL_A> for bool {
    #[inline(always)]
    fn from(variant: OESEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OESEL` reader - Output Enable Select.
pub struct OESEL_R(crate::FieldReader<bool, OESEL_A>);
impl OESEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OESEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OESEL_A {
        match self.bits {
            false => OESEL_A::STANDARD,
            true => OESEL_A::RS_485,
        }
    }
    ///Checks if the value of the field is `STANDARD`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == OESEL_A::STANDARD
    }
    ///Checks if the value of the field is `RS_485`
    #[inline(always)]
    pub fn is_rs_485(&self) -> bool {
        **self == OESEL_A::RS_485
    }
}
impl core::ops::Deref for OESEL_R {
    type Target = crate::FieldReader<bool, OESEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OESEL` writer - Output Enable Select.
pub struct OESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OESEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OESEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Standard. The RTS signal is used as the standard flow control function.
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(OESEL_A::STANDARD)
    }
    ///RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver.
    #[inline(always)]
    pub fn rs_485(self) -> &'a mut W {
        self.variant(OESEL_A::RS_485)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///Output Enable Polarity.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEPOL_A {
    ///0: Low. If selected by OESEL, the output enable is active low.
    LOW = 0,
    ///1: High. If selected by OESEL, the output enable is active high.
    HIGH = 1,
}
impl From<OEPOL_A> for bool {
    #[inline(always)]
    fn from(variant: OEPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OEPOL` reader - Output Enable Polarity.
pub struct OEPOL_R(crate::FieldReader<bool, OEPOL_A>);
impl OEPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEPOL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OEPOL_A {
        match self.bits {
            false => OEPOL_A::LOW,
            true => OEPOL_A::HIGH,
        }
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == OEPOL_A::LOW
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == OEPOL_A::HIGH
    }
}
impl core::ops::Deref for OEPOL_R {
    type Target = crate::FieldReader<bool, OEPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OEPOL` writer - Output Enable Polarity.
pub struct OEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OEPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Low. If selected by OESEL, the output enable is active low.
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OEPOL_A::LOW)
    }
    ///High. If selected by OESEL, the output enable is active high.
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OEPOL_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///Receive data polarity.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPOL_A {
    ///0: Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1.
    STANDARD = 0,
    ///1: Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0.
    INVERTED = 1,
}
impl From<RXPOL_A> for bool {
    #[inline(always)]
    fn from(variant: RXPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXPOL` reader - Receive data polarity.
pub struct RXPOL_R(crate::FieldReader<bool, RXPOL_A>);
impl RXPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXPOL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXPOL_A {
        match self.bits {
            false => RXPOL_A::STANDARD,
            true => RXPOL_A::INVERTED,
        }
    }
    ///Checks if the value of the field is `STANDARD`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == RXPOL_A::STANDARD
    }
    ///Checks if the value of the field is `INVERTED`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == RXPOL_A::INVERTED
    }
}
impl core::ops::Deref for RXPOL_R {
    type Target = crate::FieldReader<bool, RXPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXPOL` writer - Receive data polarity.
pub struct RXPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1.
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(RXPOL_A::STANDARD)
    }
    ///Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0.
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RXPOL_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///Transmit data polarity.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPOL_A {
    ///0: Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1.
    STANDARD = 0,
    ///1: Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0.
    INVERTED = 1,
}
impl From<TXPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TXPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXPOL` reader - Transmit data polarity.
pub struct TXPOL_R(crate::FieldReader<bool, TXPOL_A>);
impl TXPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXPOL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXPOL_A {
        match self.bits {
            false => TXPOL_A::STANDARD,
            true => TXPOL_A::INVERTED,
        }
    }
    ///Checks if the value of the field is `STANDARD`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == TXPOL_A::STANDARD
    }
    ///Checks if the value of the field is `INVERTED`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TXPOL_A::INVERTED
    }
}
impl core::ops::Deref for TXPOL_R {
    type Target = crate::FieldReader<bool, TXPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXPOL` writer - Transmit data polarity.
pub struct TXPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1.
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(TXPOL_A::STANDARD)
    }
    ///Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0.
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TXPOL_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    ///Bit 0 - USART Enable.
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 2:3 - Selects the data size for the USART.
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 4:5 - Selects what type of parity is used by the USART.
    #[inline(always)]
    pub fn paritysel(&self) -> PARITYSEL_R {
        PARITYSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data.
    #[inline(always)]
    pub fn stoplen(&self) -> STOPLEN_R {
        STOPLEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Selects standard or 32 kHz clocking mode.
    #[inline(always)]
    pub fn mode32k(&self) -> MODE32K_R {
        MODE32K_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - LIN break mode enable.
    #[inline(always)]
    pub fn linmode(&self) -> LINMODE_R {
        LINMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled.
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 11 - Selects synchronous or asynchronous operation.
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode.
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 14 - Synchronous mode Master select.
    #[inline(always)]
    pub fn syncmst(&self) -> SYNCMST_R {
        SYNCMST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Selects data loopback mode.
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 18 - Output Enable Turnaround time enable for RS-485 operation.
    #[inline(always)]
    pub fn oeta(&self) -> OETA_R {
        OETA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Automatic Address matching enable.
    #[inline(always)]
    pub fn autoaddr(&self) -> AUTOADDR_R {
        AUTOADDR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Output Enable Select.
    #[inline(always)]
    pub fn oesel(&self) -> OESEL_R {
        OESEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - Output Enable Polarity.
    #[inline(always)]
    pub fn oepol(&self) -> OEPOL_R {
        OEPOL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - Receive data polarity.
    #[inline(always)]
    pub fn rxpol(&self) -> RXPOL_R {
        RXPOL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - Transmit data polarity.
    #[inline(always)]
    pub fn txpol(&self) -> TXPOL_R {
        TXPOL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - USART Enable.
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    ///Bits 2:3 - Selects the data size for the USART.
    #[inline(always)]
    pub fn datalen(&mut self) -> DATALEN_W {
        DATALEN_W { w: self }
    }
    ///Bits 4:5 - Selects what type of parity is used by the USART.
    #[inline(always)]
    pub fn paritysel(&mut self) -> PARITYSEL_W {
        PARITYSEL_W { w: self }
    }
    ///Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data.
    #[inline(always)]
    pub fn stoplen(&mut self) -> STOPLEN_W {
        STOPLEN_W { w: self }
    }
    ///Bit 7 - Selects standard or 32 kHz clocking mode.
    #[inline(always)]
    pub fn mode32k(&mut self) -> MODE32K_W {
        MODE32K_W { w: self }
    }
    ///Bit 8 - LIN break mode enable.
    #[inline(always)]
    pub fn linmode(&mut self) -> LINMODE_W {
        LINMODE_W { w: self }
    }
    ///Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled.
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W { w: self }
    }
    ///Bit 11 - Selects synchronous or asynchronous operation.
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W {
        SYNCEN_W { w: self }
    }
    ///Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode.
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W {
        CLKPOL_W { w: self }
    }
    ///Bit 14 - Synchronous mode Master select.
    #[inline(always)]
    pub fn syncmst(&mut self) -> SYNCMST_W {
        SYNCMST_W { w: self }
    }
    ///Bit 15 - Selects data loopback mode.
    #[inline(always)]
    pub fn loop_(&mut self) -> LOOP_W {
        LOOP_W { w: self }
    }
    ///Bit 18 - Output Enable Turnaround time enable for RS-485 operation.
    #[inline(always)]
    pub fn oeta(&mut self) -> OETA_W {
        OETA_W { w: self }
    }
    ///Bit 19 - Automatic Address matching enable.
    #[inline(always)]
    pub fn autoaddr(&mut self) -> AUTOADDR_W {
        AUTOADDR_W { w: self }
    }
    ///Bit 20 - Output Enable Select.
    #[inline(always)]
    pub fn oesel(&mut self) -> OESEL_W {
        OESEL_W { w: self }
    }
    ///Bit 21 - Output Enable Polarity.
    #[inline(always)]
    pub fn oepol(&mut self) -> OEPOL_W {
        OEPOL_W { w: self }
    }
    ///Bit 22 - Receive data polarity.
    #[inline(always)]
    pub fn rxpol(&mut self) -> RXPOL_W {
        RXPOL_W { w: self }
    }
    ///Bit 23 - Transmit data polarity.
    #[inline(always)]
    pub fn txpol(&mut self) -> TXPOL_W {
        TXPOL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///USART Configuration register. Basic USART configuration settings that typically are not changed during operation.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfg::R](R) reader structure
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfg::W](W) writer structure
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
///`reset()` method sets CFG to value 0
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
