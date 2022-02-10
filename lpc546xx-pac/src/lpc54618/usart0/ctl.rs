///Register `CTL` reader
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CTL` writer
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
///Break Enable.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBRKEN_A {
    ///0: Normal operation.
    NORMAL = 0,
    ///1: Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN.
    CONTINOUS = 1,
}
impl From<TXBRKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXBRKEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXBRKEN` reader - Break Enable.
pub struct TXBRKEN_R(crate::FieldReader<bool, TXBRKEN_A>);
impl TXBRKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXBRKEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXBRKEN_A {
        match self.bits {
            false => TXBRKEN_A::NORMAL,
            true => TXBRKEN_A::CONTINOUS,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TXBRKEN_A::NORMAL
    }
    ///Checks if the value of the field is `CONTINOUS`
    #[inline(always)]
    pub fn is_continous(&self) -> bool {
        **self == TXBRKEN_A::CONTINOUS
    }
}
impl core::ops::Deref for TXBRKEN_R {
    type Target = crate::FieldReader<bool, TXBRKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXBRKEN` writer - Break Enable.
pub struct TXBRKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBRKEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXBRKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal operation.
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TXBRKEN_A::NORMAL)
    }
    ///Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN.
    #[inline(always)]
    pub fn continous(self) -> &'a mut W {
        self.variant(TXBRKEN_A::CONTINOUS)
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
///Enable address detect mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRDET_A {
    ///0: Disabled. The USART presents all incoming data.
    DISABLED = 0,
    ///1: Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally.
    ENABLED = 1,
}
impl From<ADDRDET_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRDET_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDRDET` reader - Enable address detect mode.
pub struct ADDRDET_R(crate::FieldReader<bool, ADDRDET_A>);
impl ADDRDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDRDET_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDRDET_A {
        match self.bits {
            false => ADDRDET_A::DISABLED,
            true => ADDRDET_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADDRDET_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADDRDET_A::ENABLED
    }
}
impl core::ops::Deref for ADDRDET_R {
    type Target = crate::FieldReader<bool, ADDRDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ADDRDET` writer - Enable address detect mode.
pub struct ADDRDET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRDET_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADDRDET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. The USART presents all incoming data.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRDET_A::DISABLED)
    }
    ///Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRDET_A::ENABLED)
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
///Transmit Disable.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDIS_A {
    ///0: Not disabled. USART transmitter is not disabled.
    ENABLED = 0,
    ///1: Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control.
    DISABLED = 1,
}
impl From<TXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TXDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDIS` reader - Transmit Disable.
pub struct TXDIS_R(crate::FieldReader<bool, TXDIS_A>);
impl TXDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXDIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXDIS_A {
        match self.bits {
            false => TXDIS_A::ENABLED,
            true => TXDIS_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXDIS_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXDIS_A::DISABLED
    }
}
impl core::ops::Deref for TXDIS_R {
    type Target = crate::FieldReader<bool, TXDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXDIS` writer - Transmit Disable.
pub struct TXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Not disabled. USART transmitter is not disabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDIS_A::ENABLED)
    }
    ///Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDIS_A::DISABLED)
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
///Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC_A {
    ///0: Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received.
    CLOCK_ON_CHARACTER = 0,
    ///1: Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD).
    CONTINOUS_CLOCK = 1,
}
impl From<CC_A> for bool {
    #[inline(always)]
    fn from(variant: CC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC` reader - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode.
pub struct CC_R(crate::FieldReader<bool, CC_A>);
impl CC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC_A {
        match self.bits {
            false => CC_A::CLOCK_ON_CHARACTER,
            true => CC_A::CONTINOUS_CLOCK,
        }
    }
    ///Checks if the value of the field is `CLOCK_ON_CHARACTER`
    #[inline(always)]
    pub fn is_clock_on_character(&self) -> bool {
        **self == CC_A::CLOCK_ON_CHARACTER
    }
    ///Checks if the value of the field is `CONTINOUS_CLOCK`
    #[inline(always)]
    pub fn is_continous_clock(&self) -> bool {
        **self == CC_A::CONTINOUS_CLOCK
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<bool, CC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC` writer - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode.
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received.
    #[inline(always)]
    pub fn clock_on_character(self) -> &'a mut W {
        self.variant(CC_A::CLOCK_ON_CHARACTER)
    }
    ///Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD).
    #[inline(always)]
    pub fn continous_clock(self) -> &'a mut W {
        self.variant(CC_A::CONTINOUS_CLOCK)
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
///Clear Continuous Clock.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRCCONRX_A {
    ///0: No effect. No effect on the CC bit.
    NO_EFFECT = 0,
    ///1: Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time.
    AUTO_CLEAR = 1,
}
impl From<CLRCCONRX_A> for bool {
    #[inline(always)]
    fn from(variant: CLRCCONRX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CLRCCONRX` reader - Clear Continuous Clock.
pub struct CLRCCONRX_R(crate::FieldReader<bool, CLRCCONRX_A>);
impl CLRCCONRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLRCCONRX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CLRCCONRX_A {
        match self.bits {
            false => CLRCCONRX_A::NO_EFFECT,
            true => CLRCCONRX_A::AUTO_CLEAR,
        }
    }
    ///Checks if the value of the field is `NO_EFFECT`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == CLRCCONRX_A::NO_EFFECT
    }
    ///Checks if the value of the field is `AUTO_CLEAR`
    #[inline(always)]
    pub fn is_auto_clear(&self) -> bool {
        **self == CLRCCONRX_A::AUTO_CLEAR
    }
}
impl core::ops::Deref for CLRCCONRX_R {
    type Target = crate::FieldReader<bool, CLRCCONRX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CLRCCONRX` writer - Clear Continuous Clock.
pub struct CLRCCONRX_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRCCONRX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CLRCCONRX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect. No effect on the CC bit.
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLRCCONRX_A::NO_EFFECT)
    }
    ///Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time.
    #[inline(always)]
    pub fn auto_clear(self) -> &'a mut W {
        self.variant(CLRCCONRX_A::AUTO_CLEAR)
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
///Autobaud enable.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOBAUD_A {
    ///0: Disabled. USART is in normal operating mode.
    DISABLED = 0,
    ///1: Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR.
    ENABLED = 1,
}
impl From<AUTOBAUD_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOBAUD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOBAUD` reader - Autobaud enable.
pub struct AUTOBAUD_R(crate::FieldReader<bool, AUTOBAUD_A>);
impl AUTOBAUD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOBAUD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AUTOBAUD_A {
        match self.bits {
            false => AUTOBAUD_A::DISABLED,
            true => AUTOBAUD_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AUTOBAUD_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AUTOBAUD_A::ENABLED
    }
}
impl core::ops::Deref for AUTOBAUD_R {
    type Target = crate::FieldReader<bool, AUTOBAUD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AUTOBAUD` writer - Autobaud enable.
pub struct AUTOBAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOBAUD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AUTOBAUD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. USART is in normal operating mode.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOBAUD_A::DISABLED)
    }
    ///Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOBAUD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    ///Bit 1 - Break Enable.
    #[inline(always)]
    pub fn txbrken(&self) -> TXBRKEN_R {
        TXBRKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Enable address detect mode.
    #[inline(always)]
    pub fn addrdet(&self) -> ADDRDET_R {
        ADDRDET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 6 - Transmit Disable.
    #[inline(always)]
    pub fn txdis(&self) -> TXDIS_R {
        TXDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode.
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Clear Continuous Clock.
    #[inline(always)]
    pub fn clrcconrx(&self) -> CLRCCONRX_R {
        CLRCCONRX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 16 - Autobaud enable.
    #[inline(always)]
    pub fn autobaud(&self) -> AUTOBAUD_R {
        AUTOBAUD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Break Enable.
    #[inline(always)]
    pub fn txbrken(&mut self) -> TXBRKEN_W {
        TXBRKEN_W { w: self }
    }
    ///Bit 2 - Enable address detect mode.
    #[inline(always)]
    pub fn addrdet(&mut self) -> ADDRDET_W {
        ADDRDET_W { w: self }
    }
    ///Bit 6 - Transmit Disable.
    #[inline(always)]
    pub fn txdis(&mut self) -> TXDIS_W {
        TXDIS_W { w: self }
    }
    ///Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode.
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    ///Bit 9 - Clear Continuous Clock.
    #[inline(always)]
    pub fn clrcconrx(&mut self) -> CLRCCONRX_W {
        CLRCCONRX_W { w: self }
    }
    ///Bit 16 - Autobaud enable.
    #[inline(always)]
    pub fn autobaud(&mut self) -> AUTOBAUD_W {
        AUTOBAUD_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///USART Control register. USART control settings that are more likely to change during operation.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ctl](index.html) module
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [ctl::R](R) reader structure
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ctl::W](W) writer structure
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
///`reset()` method sets CTL to value 0
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
