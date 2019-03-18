#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TXBRKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBRKENR {
    #[doc = "Normal operation."]
    NORMAL,
    #[doc = "Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINOUS,
}
impl TXBRKENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXBRKENR::NORMAL => false,
            TXBRKENR::CONTINOUS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXBRKENR {
        match value {
            false => TXBRKENR::NORMAL,
            true => TXBRKENR::CONTINOUS,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TXBRKENR::NORMAL
    }
    #[doc = "Checks if the value of the field is `CONTINOUS`"]
    #[inline]
    pub fn is_continous(&self) -> bool {
        *self == TXBRKENR::CONTINOUS
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
impl ADDRDETR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ADDRDETR::DISABLED => false,
            ADDRDETR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRDETR {
        match value {
            false => ADDRDETR::DISABLED,
            true => ADDRDETR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRDETR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRDETR::ENABLED
    }
}
#[doc = "Possible values of the field `TXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDISR {
    #[doc = "Not disabled. USART transmitter is not disabled."]
    ENABLED,
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED,
}
impl TXDISR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXDISR::ENABLED => false,
            TXDISR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDISR {
        match value {
            false => TXDISR::ENABLED,
            true => TXDISR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXDISR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXDISR::DISABLED
    }
}
#[doc = "Possible values of the field `CC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCR {
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER,
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINOUS_CLOCK,
}
impl CCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CCR::CLOCK_ON_CHARACTER => false,
            CCR::CONTINOUS_CLOCK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCR {
        match value {
            false => CCR::CLOCK_ON_CHARACTER,
            true => CCR::CONTINOUS_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `CLOCK_ON_CHARACTER`"]
    #[inline]
    pub fn is_clock_on_character(&self) -> bool {
        *self == CCR::CLOCK_ON_CHARACTER
    }
    #[doc = "Checks if the value of the field is `CONTINOUS_CLOCK`"]
    #[inline]
    pub fn is_continous_clock(&self) -> bool {
        *self == CCR::CONTINOUS_CLOCK
    }
}
#[doc = "Possible values of the field `CLRCCONRX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRCCONRXR {
    #[doc = "No effect. No effect on the CC bit."]
    NO_EFFECT,
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR,
}
impl CLRCCONRXR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CLRCCONRXR::NO_EFFECT => false,
            CLRCCONRXR::AUTO_CLEAR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLRCCONRXR {
        match value {
            false => CLRCCONRXR::NO_EFFECT,
            true => CLRCCONRXR::AUTO_CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == CLRCCONRXR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `AUTO_CLEAR`"]
    #[inline]
    pub fn is_auto_clear(&self) -> bool {
        *self == CLRCCONRXR::AUTO_CLEAR
    }
}
#[doc = "Possible values of the field `AUTOBAUD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOBAUDR {
    #[doc = "Disabled. USART is in normal operating mode."]
    DISABLED,
    #[doc = "Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    ENABLED,
}
impl AUTOBAUDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AUTOBAUDR::DISABLED => false,
            AUTOBAUDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOBAUDR {
        match value {
            false => AUTOBAUDR::DISABLED,
            true => AUTOBAUDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOBAUDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOBAUDR::ENABLED
    }
}
#[doc = "Values that can be written to the field `TXBRKEN`"]
pub enum TXBRKENW {
    #[doc = "Normal operation."]
    NORMAL,
    #[doc = "Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINOUS,
}
impl TXBRKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXBRKENW::NORMAL => false,
            TXBRKENW::CONTINOUS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXBRKENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBRKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXBRKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TXBRKENW::NORMAL)
    }
    #[doc = "Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    #[inline]
    pub fn continous(self) -> &'a mut W {
        self.variant(TXBRKENW::CONTINOUS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADDRDET`"]
pub enum ADDRDETW {
    #[doc = "Disabled. The USART presents all incoming data."]
    DISABLED,
    #[doc = "Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    ENABLED,
}
impl ADDRDETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRDETW::DISABLED => false,
            ADDRDETW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRDETW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRDETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRDETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The USART presents all incoming data."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRDETW::DISABLED)
    }
    #[doc = "Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRDETW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXDIS`"]
pub enum TXDISW {
    #[doc = "Not disabled. USART transmitter is not disabled."]
    ENABLED,
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED,
}
impl TXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDISW::ENABLED => false,
            TXDISW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not disabled. USART transmitter is not disabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDISW::ENABLED)
    }
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDISW::DISABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC`"]
pub enum CCW {
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER,
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINOUS_CLOCK,
}
impl CCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCW::CLOCK_ON_CHARACTER => false,
            CCW::CONTINOUS_CLOCK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCW<'a> {
    w: &'a mut W,
}
impl<'a> _CCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    #[inline]
    pub fn clock_on_character(self) -> &'a mut W {
        self.variant(CCW::CLOCK_ON_CHARACTER)
    }
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    #[inline]
    pub fn continous_clock(self) -> &'a mut W {
        self.variant(CCW::CONTINOUS_CLOCK)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLRCCONRX`"]
pub enum CLRCCONRXW {
    #[doc = "No effect. No effect on the CC bit."]
    NO_EFFECT,
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR,
}
impl CLRCCONRXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLRCCONRXW::NO_EFFECT => false,
            CLRCCONRXW::AUTO_CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLRCCONRXW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRCCONRXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLRCCONRXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. No effect on the CC bit."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLRCCONRXW::NO_EFFECT)
    }
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    #[inline]
    pub fn auto_clear(self) -> &'a mut W {
        self.variant(CLRCCONRXW::AUTO_CLEAR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUTOBAUD`"]
pub enum AUTOBAUDW {
    #[doc = "Disabled. USART is in normal operating mode."]
    DISABLED,
    #[doc = "Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    ENABLED,
}
impl AUTOBAUDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOBAUDW::DISABLED => false,
            AUTOBAUDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOBAUDW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOBAUDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOBAUDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. USART is in normal operating mode."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOBAUDW::DISABLED)
    }
    #[doc = "Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOBAUDW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Break Enable."]
    #[inline]
    pub fn txbrken(&self) -> TXBRKENR {
        TXBRKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline]
    pub fn addrdet(&self) -> ADDRDETR {
        ADDRDETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline]
    pub fn txdis(&self) -> TXDISR {
        TXDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline]
    pub fn cc(&self) -> CCR {
        CCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline]
    pub fn clrcconrx(&self) -> CLRCCONRXR {
        CLRCCONRXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Autobaud enable."]
    #[inline]
    pub fn autobaud(&self) -> AUTOBAUDR {
        AUTOBAUDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Break Enable."]
    #[inline]
    pub fn txbrken(&mut self) -> _TXBRKENW {
        _TXBRKENW { w: self }
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline]
    pub fn addrdet(&mut self) -> _ADDRDETW {
        _ADDRDETW { w: self }
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline]
    pub fn txdis(&mut self) -> _TXDISW {
        _TXDISW { w: self }
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline]
    pub fn cc(&mut self) -> _CCW {
        _CCW { w: self }
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline]
    pub fn clrcconrx(&mut self) -> _CLRCCONRXW {
        _CLRCCONRXW { w: self }
    }
    #[doc = "Bit 16 - Autobaud enable."]
    #[inline]
    pub fn autobaud(&mut self) -> _AUTOBAUDW {
        _AUTOBAUDW { w: self }
    }
}
