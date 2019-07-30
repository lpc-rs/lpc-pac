#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
#[doc = "Possible values of the field `TXBRKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBRKENR {
    #[doc = "Normal operation."]
    NORMAL_OPERATION,
    #[doc = "Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINUOUS_BREAK_IS,
}
impl crate::ToBits<bool> for TXBRKENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TXBRKENR::NORMAL_OPERATION => false,
            TXBRKENR::CONTINUOUS_BREAK_IS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TXBRKEN_R = crate::FR<bool, TXBRKENR>;
impl TXBRKEN_R {
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        *self == TXBRKENR::NORMAL_OPERATION
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS_BREAK_IS`"]
    #[inline(always)]
    pub fn is_continuous_break_is(&self) -> bool {
        *self == TXBRKENR::CONTINUOUS_BREAK_IS
    }
}
#[doc = "Values that can be written to the field `TXBRKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBRKENW {
    #[doc = "Normal operation."]
    NORMAL_OPERATION,
    #[doc = "Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINUOUS_BREAK_IS,
}
impl TXBRKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TXBRKENW::NORMAL_OPERATION => false,
            TXBRKENW::CONTINUOUS_BREAK_IS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TXBRKENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBRKENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXBRKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut W {
        self.variant(TXBRKENW::NORMAL_OPERATION)
    }
    #[doc = "Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    #[inline(always)]
    pub fn continuous_break_is(self) -> &'a mut W {
        self.variant(TXBRKENW::CONTINUOUS_BREAK_IS)
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
#[doc = "Possible values of the field `ADDRDET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRDETR {
    #[doc = "Disabled. The USART presents all incoming data."]
    DISABLED,
    #[doc = "Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    ENABLED,
}
impl crate::ToBits<bool> for ADDRDETR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADDRDETR::DISABLED => false,
            ADDRDETR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADDRDET_R = crate::FR<bool, ADDRDETR>;
impl ADDRDET_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRDETR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRDETR::ENABLED
    }
}
#[doc = "Values that can be written to the field `ADDRDET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRDETW {
    #[doc = "Disabled. The USART presents all incoming data."]
    DISABLED,
    #[doc = "Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    ENABLED,
}
impl ADDRDETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRDETW::DISABLED => false,
            ADDRDETW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADDRDETW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRDETW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRDETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The USART presents all incoming data."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRDETW::DISABLED)
    }
    #[doc = "Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRDETW::ENABLED)
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
#[doc = "Possible values of the field `TXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDISR {
    #[doc = "Not disabled. USART transmitter is not disabled."]
    NOT_DISABLED,
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED,
}
impl crate::ToBits<bool> for TXDISR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TXDISR::NOT_DISABLED => false,
            TXDISR::DISABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TXDIS_R = crate::FR<bool, TXDISR>;
impl TXDIS_R {
    #[doc = "Checks if the value of the field is `NOT_DISABLED`"]
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        *self == TXDISR::NOT_DISABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDISR::DISABLED
    }
}
#[doc = "Values that can be written to the field `TXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDISW {
    #[doc = "Not disabled. USART transmitter is not disabled."]
    NOT_DISABLED,
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED,
}
impl TXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDISW::NOT_DISABLED => false,
            TXDISW::DISABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDISW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not disabled. USART transmitter is not disabled."]
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut W {
        self.variant(TXDISW::NOT_DISABLED)
    }
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDISW::DISABLED)
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
#[doc = "Possible values of the field `CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCR {
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER,
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINUOUS_CLOCK,
}
impl crate::ToBits<bool> for CCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CCR::CLOCK_ON_CHARACTER => false,
            CCR::CONTINUOUS_CLOCK => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CC_R = crate::FR<bool, CCR>;
impl CC_R {
    #[doc = "Checks if the value of the field is `CLOCK_ON_CHARACTER`"]
    #[inline(always)]
    pub fn is_clock_on_character(&self) -> bool {
        *self == CCR::CLOCK_ON_CHARACTER
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS_CLOCK`"]
    #[inline(always)]
    pub fn is_continuous_clock(&self) -> bool {
        *self == CCR::CONTINUOUS_CLOCK
    }
}
#[doc = "Values that can be written to the field `CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCW {
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER,
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINUOUS_CLOCK,
}
impl CCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CCW::CLOCK_ON_CHARACTER => false,
            CCW::CONTINUOUS_CLOCK => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CCW<'a> {
    w: &'a mut W,
}
impl<'a> _CCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    #[inline(always)]
    pub fn clock_on_character(self) -> &'a mut W {
        self.variant(CCW::CLOCK_ON_CHARACTER)
    }
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    #[inline(always)]
    pub fn continuous_clock(self) -> &'a mut W {
        self.variant(CCW::CONTINUOUS_CLOCK)
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
#[doc = "Possible values of the field `CLRCCONRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRCCONRXR {
    #[doc = "No effect on the CC bit."]
    NO_EFFECT,
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR,
}
impl crate::ToBits<bool> for CLRCCONRXR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CLRCCONRXR::NO_EFFECT => false,
            CLRCCONRXR::AUTO_CLEAR => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CLRCCONRX_R = crate::FR<bool, CLRCCONRXR>;
impl CLRCCONRX_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLRCCONRXR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `AUTO_CLEAR`"]
    #[inline(always)]
    pub fn is_auto_clear(&self) -> bool {
        *self == CLRCCONRXR::AUTO_CLEAR
    }
}
#[doc = "Values that can be written to the field `CLRCCONRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRCCONRXW {
    #[doc = "No effect on the CC bit."]
    NO_EFFECT,
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR,
}
impl CLRCCONRXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CLRCCONRXW::NO_EFFECT => false,
            CLRCCONRXW::AUTO_CLEAR => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CLRCCONRXW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRCCONRXW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRCCONRXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect on the CC bit."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLRCCONRXW::NO_EFFECT)
    }
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    #[inline(always)]
    pub fn auto_clear(self) -> &'a mut W {
        self.variant(CLRCCONRXW::AUTO_CLEAR)
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
#[doc = "Possible values of the field `AUTOBAUD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOBAUDR {
    #[doc = "Disabled. UART is in normal operating mode."]
    DISABLED,
    #[doc = "Enabled. UART is in autobaud mode. This bit should only be set when the UART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR. This bit can be cleared by software when set, but only when the UART receiver is idle."]
    ENABLED,
}
impl crate::ToBits<bool> for AUTOBAUDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AUTOBAUDR::DISABLED => false,
            AUTOBAUDR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AUTOBAUD_R = crate::FR<bool, AUTOBAUDR>;
impl AUTOBAUD_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOBAUDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOBAUDR::ENABLED
    }
}
#[doc = "Values that can be written to the field `AUTOBAUD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOBAUDW {
    #[doc = "Disabled. UART is in normal operating mode."]
    DISABLED,
    #[doc = "Enabled. UART is in autobaud mode. This bit should only be set when the UART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR. This bit can be cleared by software when set, but only when the UART receiver is idle."]
    ENABLED,
}
impl AUTOBAUDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOBAUDW::DISABLED => false,
            AUTOBAUDW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AUTOBAUDW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOBAUDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOBAUDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. UART is in normal operating mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOBAUDW::DISABLED)
    }
    #[doc = "Enabled. UART is in autobaud mode. This bit should only be set when the UART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR. This bit can be cleared by software when set, but only when the UART receiver is idle."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOBAUDW::ENABLED)
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
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Break Enable."]
    #[inline(always)]
    pub fn txbrken(&self) -> TXBRKEN_R {
        TXBRKEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline(always)]
    pub fn addrdet(&self) -> ADDRDET_R {
        ADDRDET_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline(always)]
    pub fn txdis(&self) -> TXDIS_R {
        TXDIS_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline(always)]
    pub fn clrcconrx(&self) -> CLRCCONRX_R {
        CLRCCONRX_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Autobaud enable."]
    #[inline(always)]
    pub fn autobaud(&self) -> AUTOBAUD_R {
        AUTOBAUD_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Break Enable."]
    #[inline(always)]
    pub fn txbrken(&mut self) -> _TXBRKENW {
        _TXBRKENW { w: self }
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline(always)]
    pub fn addrdet(&mut self) -> _ADDRDETW {
        _ADDRDETW { w: self }
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline(always)]
    pub fn txdis(&mut self) -> _TXDISW {
        _TXDISW { w: self }
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline(always)]
    pub fn cc(&mut self) -> _CCW {
        _CCW { w: self }
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline(always)]
    pub fn clrcconrx(&mut self) -> _CLRCCONRXW {
        _CLRCCONRXW { w: self }
    }
    #[doc = "Bit 16 - Autobaud enable."]
    #[inline(always)]
    pub fn autobaud(&mut self) -> _AUTOBAUDW {
        _AUTOBAUDW { w: self }
    }
}
