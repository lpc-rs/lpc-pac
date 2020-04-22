#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Break Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBRKEN_A {
    #[doc = "0: Normal operation."]
    NORMAL = 0,
    #[doc = "1: Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINOUS = 1,
}
impl From<TXBRKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXBRKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXBRKEN`"]
pub type TXBRKEN_R = crate::R<bool, TXBRKEN_A>;
impl TXBRKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXBRKEN_A {
        match self.bits {
            false => TXBRKEN_A::NORMAL,
            true => TXBRKEN_A::CONTINOUS,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TXBRKEN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `CONTINOUS`"]
    #[inline(always)]
    pub fn is_continous(&self) -> bool {
        *self == TXBRKEN_A::CONTINOUS
    }
}
#[doc = "Write proxy for field `TXBRKEN`"]
pub struct TXBRKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBRKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXBRKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TXBRKEN_A::NORMAL)
    }
    #[doc = "Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    #[inline(always)]
    pub fn continous(self) -> &'a mut W {
        self.variant(TXBRKEN_A::CONTINOUS)
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
#[doc = "Enable address detect mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRDET_A {
    #[doc = "0: Disabled. The USART presents all incoming data."]
    DISABLED = 0,
    #[doc = "1: Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    ENABLED = 1,
}
impl From<ADDRDET_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRDET`"]
pub type ADDRDET_R = crate::R<bool, ADDRDET_A>;
impl ADDRDET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRDET_A {
        match self.bits {
            false => ADDRDET_A::DISABLED,
            true => ADDRDET_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRDET_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRDET_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDRDET`"]
pub struct ADDRDET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRDET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The USART presents all incoming data."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRDET_A::DISABLED)
    }
    #[doc = "Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRDET_A::ENABLED)
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
#[doc = "Transmit Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDIS_A {
    #[doc = "0: Not disabled. USART transmitter is not disabled."]
    ENABLED = 0,
    #[doc = "1: Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED = 1,
}
impl From<TXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TXDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDIS`"]
pub type TXDIS_R = crate::R<bool, TXDIS_A>;
impl TXDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDIS_A {
        match self.bits {
            false => TXDIS_A::ENABLED,
            true => TXDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDIS_A::DISABLED
    }
}
#[doc = "Write proxy for field `TXDIS`"]
pub struct TXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not disabled. USART transmitter is not disabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDIS_A::ENABLED)
    }
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDIS_A::DISABLED)
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
#[doc = "Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC_A {
    #[doc = "0: Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER = 0,
    #[doc = "1: Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINOUS_CLOCK = 1,
}
impl From<CC_A> for bool {
    #[inline(always)]
    fn from(variant: CC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC`"]
pub type CC_R = crate::R<bool, CC_A>;
impl CC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC_A {
        match self.bits {
            false => CC_A::CLOCK_ON_CHARACTER,
            true => CC_A::CONTINOUS_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `CLOCK_ON_CHARACTER`"]
    #[inline(always)]
    pub fn is_clock_on_character(&self) -> bool {
        *self == CC_A::CLOCK_ON_CHARACTER
    }
    #[doc = "Checks if the value of the field is `CONTINOUS_CLOCK`"]
    #[inline(always)]
    pub fn is_continous_clock(&self) -> bool {
        *self == CC_A::CONTINOUS_CLOCK
    }
}
#[doc = "Write proxy for field `CC`"]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    #[inline(always)]
    pub fn clock_on_character(self) -> &'a mut W {
        self.variant(CC_A::CLOCK_ON_CHARACTER)
    }
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    #[inline(always)]
    pub fn continous_clock(self) -> &'a mut W {
        self.variant(CC_A::CONTINOUS_CLOCK)
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
#[doc = "Clear Continuous Clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRCCONRX_A {
    #[doc = "0: No effect. No effect on the CC bit."]
    NO_EFFECT = 0,
    #[doc = "1: Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR = 1,
}
impl From<CLRCCONRX_A> for bool {
    #[inline(always)]
    fn from(variant: CLRCCONRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLRCCONRX`"]
pub type CLRCCONRX_R = crate::R<bool, CLRCCONRX_A>;
impl CLRCCONRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRCCONRX_A {
        match self.bits {
            false => CLRCCONRX_A::NO_EFFECT,
            true => CLRCCONRX_A::AUTO_CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLRCCONRX_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `AUTO_CLEAR`"]
    #[inline(always)]
    pub fn is_auto_clear(&self) -> bool {
        *self == CLRCCONRX_A::AUTO_CLEAR
    }
}
#[doc = "Write proxy for field `CLRCCONRX`"]
pub struct CLRCCONRX_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRCCONRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRCCONRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect. No effect on the CC bit."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLRCCONRX_A::NO_EFFECT)
    }
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    #[inline(always)]
    pub fn auto_clear(self) -> &'a mut W {
        self.variant(CLRCCONRX_A::AUTO_CLEAR)
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
#[doc = "Autobaud enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOBAUD_A {
    #[doc = "0: Disabled. USART is in normal operating mode."]
    DISABLED = 0,
    #[doc = "1: Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    ENABLED = 1,
}
impl From<AUTOBAUD_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOBAUD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTOBAUD`"]
pub type AUTOBAUD_R = crate::R<bool, AUTOBAUD_A>;
impl AUTOBAUD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOBAUD_A {
        match self.bits {
            false => AUTOBAUD_A::DISABLED,
            true => AUTOBAUD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOBAUD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOBAUD_A::ENABLED
    }
}
#[doc = "Write proxy for field `AUTOBAUD`"]
pub struct AUTOBAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOBAUD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOBAUD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. USART is in normal operating mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOBAUD_A::DISABLED)
    }
    #[doc = "Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOBAUD_A::ENABLED)
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
impl R {
    #[doc = "Bit 1 - Break Enable."]
    #[inline(always)]
    pub fn txbrken(&self) -> TXBRKEN_R {
        TXBRKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline(always)]
    pub fn addrdet(&self) -> ADDRDET_R {
        ADDRDET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline(always)]
    pub fn txdis(&self) -> TXDIS_R {
        TXDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline(always)]
    pub fn clrcconrx(&self) -> CLRCCONRX_R {
        CLRCCONRX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Autobaud enable."]
    #[inline(always)]
    pub fn autobaud(&self) -> AUTOBAUD_R {
        AUTOBAUD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Break Enable."]
    #[inline(always)]
    pub fn txbrken(&mut self) -> TXBRKEN_W {
        TXBRKEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline(always)]
    pub fn addrdet(&mut self) -> ADDRDET_W {
        ADDRDET_W { w: self }
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline(always)]
    pub fn txdis(&mut self) -> TXDIS_W {
        TXDIS_W { w: self }
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline(always)]
    pub fn clrcconrx(&mut self) -> CLRCCONRX_W {
        CLRCCONRX_W { w: self }
    }
    #[doc = "Bit 16 - Autobaud enable."]
    #[inline(always)]
    pub fn autobaud(&mut self) -> AUTOBAUD_W {
        AUTOBAUD_W { w: self }
    }
}
