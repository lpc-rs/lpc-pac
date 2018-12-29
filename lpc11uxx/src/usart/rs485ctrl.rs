#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RS485CTRL {
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
#[doc = "Possible values of the field `NMMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMMENR {
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    DISABLED,
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    ENABLED,
}
impl NMMENR {
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
            NMMENR::DISABLED => false,
            NMMENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMMENR {
        match value {
            false => NMMENR::DISABLED,
            true => NMMENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NMMENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == NMMENR::ENABLED
    }
}
#[doc = "Possible values of the field `RXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDISR {
    #[doc = "The receiver is enabled."]
    THE_RECEIVER_IS_ENAB,
    #[doc = "The receiver is disabled."]
    THE_RECEIVER_IS_DISA,
}
impl RXDISR {
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
            RXDISR::THE_RECEIVER_IS_ENAB => false,
            RXDISR::THE_RECEIVER_IS_DISA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDISR {
        match value {
            false => RXDISR::THE_RECEIVER_IS_ENAB,
            true => RXDISR::THE_RECEIVER_IS_DISA,
        }
    }
    #[doc = "Checks if the value of the field is `THE_RECEIVER_IS_ENAB`"]
    #[inline]
    pub fn is_the_receiver_is_enab(&self) -> bool {
        *self == RXDISR::THE_RECEIVER_IS_ENAB
    }
    #[doc = "Checks if the value of the field is `THE_RECEIVER_IS_DISA`"]
    #[inline]
    pub fn is_the_receiver_is_disa(&self) -> bool {
        *self == RXDISR::THE_RECEIVER_IS_DISA
    }
}
#[doc = "Possible values of the field `AADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AADENR {
    #[doc = "Auto Address Detect (AAD) is disabled."]
    AUTO_ADDRESS_DETECT_DISABLE,
    #[doc = "Auto Address Detect (AAD) is enabled."]
    AUTO_ADDRESS_DETECT_ENABLE,
}
impl AADENR {
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
            AADENR::AUTO_ADDRESS_DETECT_DISABLE => false,
            AADENR::AUTO_ADDRESS_DETECT_ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AADENR {
        match value {
            false => AADENR::AUTO_ADDRESS_DETECT_DISABLE,
            true => AADENR::AUTO_ADDRESS_DETECT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_ADDRESS_DETECT_DISABLE`"]
    #[inline]
    pub fn is_auto_address_detect_disable(&self) -> bool {
        *self == AADENR::AUTO_ADDRESS_DETECT_DISABLE
    }
    #[doc = "Checks if the value of the field is `AUTO_ADDRESS_DETECT_ENABLE`"]
    #[inline]
    pub fn is_auto_address_detect_enable(&self) -> bool {
        *self == AADENR::AUTO_ADDRESS_DETECT_ENABLE
    }
}
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    RTS,
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    DTR,
}
impl SELR {
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
            SELR::RTS => false,
            SELR::DTR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELR {
        match value {
            false => SELR::RTS,
            true => SELR::DTR,
        }
    }
    #[doc = "Checks if the value of the field is `RTS`"]
    #[inline]
    pub fn is_rts(&self) -> bool {
        *self == SELR::RTS
    }
    #[doc = "Checks if the value of the field is `DTR`"]
    #[inline]
    pub fn is_dtr(&self) -> bool {
        *self == SELR::DTR
    }
}
#[doc = "Possible values of the field `DCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTRLR {
    #[doc = "Disable Auto Direction Control."]
    DISABLE_AUTO_DIRECTI,
    #[doc = "Enable Auto Direction Control."]
    ENABLE_AUTO_DIRECTIO,
}
impl DCTRLR {
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
            DCTRLR::DISABLE_AUTO_DIRECTI => false,
            DCTRLR::ENABLE_AUTO_DIRECTIO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCTRLR {
        match value {
            false => DCTRLR::DISABLE_AUTO_DIRECTI,
            true => DCTRLR::ENABLE_AUTO_DIRECTIO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_DIRECTI`"]
    #[inline]
    pub fn is_disable_auto_directi(&self) -> bool {
        *self == DCTRLR::DISABLE_AUTO_DIRECTI
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_DIRECTIO`"]
    #[inline]
    pub fn is_enable_auto_directio(&self) -> bool {
        *self == DCTRLR::ENABLE_AUTO_DIRECTIO
    }
}
#[doc = "Possible values of the field `OINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OINVR {
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW,
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH,
}
impl OINVR {
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
            OINVR::LOW => false,
            OINVR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OINVR {
        match value {
            false => OINVR::LOW,
            true => OINVR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == OINVR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == OINVR::HIGH
    }
}
#[doc = "Values that can be written to the field `NMMEN`"]
pub enum NMMENW {
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    DISABLED,
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    ENABLED,
}
impl NMMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NMMENW::DISABLED => false,
            NMMENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NMMENW<'a> {
    w: &'a mut W,
}
impl<'a> _NMMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NMMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NMMENW::DISABLED)
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NMMENW::ENABLED)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXDIS`"]
pub enum RXDISW {
    #[doc = "The receiver is enabled."]
    THE_RECEIVER_IS_ENAB,
    #[doc = "The receiver is disabled."]
    THE_RECEIVER_IS_DISA,
}
impl RXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDISW::THE_RECEIVER_IS_ENAB => false,
            RXDISW::THE_RECEIVER_IS_DISA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The receiver is enabled."]
    #[inline]
    pub fn the_receiver_is_enab(self) -> &'a mut W {
        self.variant(RXDISW::THE_RECEIVER_IS_ENAB)
    }
    #[doc = "The receiver is disabled."]
    #[inline]
    pub fn the_receiver_is_disa(self) -> &'a mut W {
        self.variant(RXDISW::THE_RECEIVER_IS_DISA)
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
#[doc = "Values that can be written to the field `AADEN`"]
pub enum AADENW {
    #[doc = "Auto Address Detect (AAD) is disabled."]
    AUTO_ADDRESS_DETECT_DISABLE,
    #[doc = "Auto Address Detect (AAD) is enabled."]
    AUTO_ADDRESS_DETECT_ENABLE,
}
impl AADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AADENW::AUTO_ADDRESS_DETECT_DISABLE => false,
            AADENW::AUTO_ADDRESS_DETECT_ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AADENW<'a> {
    w: &'a mut W,
}
impl<'a> _AADENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AADENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto Address Detect (AAD) is disabled."]
    #[inline]
    pub fn auto_address_detect_disable(self) -> &'a mut W {
        self.variant(AADENW::AUTO_ADDRESS_DETECT_DISABLE)
    }
    #[doc = "Auto Address Detect (AAD) is enabled."]
    #[inline]
    pub fn auto_address_detect_enable(self) -> &'a mut W {
        self.variant(AADENW::AUTO_ADDRESS_DETECT_ENABLE)
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
#[doc = "Values that can be written to the field `SEL`"]
pub enum SELW {
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    RTS,
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    DTR,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELW::RTS => false,
            SELW::DTR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    #[inline]
    pub fn rts(self) -> &'a mut W {
        self.variant(SELW::RTS)
    }
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    #[inline]
    pub fn dtr(self) -> &'a mut W {
        self.variant(SELW::DTR)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCTRL`"]
pub enum DCTRLW {
    #[doc = "Disable Auto Direction Control."]
    DISABLE_AUTO_DIRECTI,
    #[doc = "Enable Auto Direction Control."]
    ENABLE_AUTO_DIRECTIO,
}
impl DCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCTRLW::DISABLE_AUTO_DIRECTI => false,
            DCTRLW::ENABLE_AUTO_DIRECTIO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _DCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Auto Direction Control."]
    #[inline]
    pub fn disable_auto_directi(self) -> &'a mut W {
        self.variant(DCTRLW::DISABLE_AUTO_DIRECTI)
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline]
    pub fn enable_auto_directio(self) -> &'a mut W {
        self.variant(DCTRLW::ENABLE_AUTO_DIRECTIO)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OINV`"]
pub enum OINVW {
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW,
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH,
}
impl OINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OINVW::LOW => false,
            OINVW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OINVW<'a> {
    w: &'a mut W,
}
impl<'a> _OINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(OINVW::LOW)
    }
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(OINVW::HIGH)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - NMM enable."]
    #[inline]
    pub fn nmmen(&self) -> NMMENR {
        NMMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline]
    pub fn rxdis(&self) -> RXDISR {
        RXDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline]
    pub fn aaden(&self) -> AADENR {
        AADENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Select direction control pin"]
    #[inline]
    pub fn sel(&self) -> SELR {
        SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Auto direction control enable."]
    #[inline]
    pub fn dctrl(&self) -> DCTRLR {
        DCTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
    #[inline]
    pub fn oinv(&self) -> OINVR {
        OINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - NMM enable."]
    #[inline]
    pub fn nmmen(&mut self) -> _NMMENW {
        _NMMENW { w: self }
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline]
    pub fn rxdis(&mut self) -> _RXDISW {
        _RXDISW { w: self }
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline]
    pub fn aaden(&mut self) -> _AADENW {
        _AADENW { w: self }
    }
    #[doc = "Bit 3 - Select direction control pin"]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
    #[doc = "Bit 4 - Auto direction control enable."]
    #[inline]
    pub fn dctrl(&mut self) -> _DCTRLW {
        _DCTRLW { w: self }
    }
    #[doc = "Bit 5 - Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
    #[inline]
    pub fn oinv(&mut self) -> _OINVW {
        _OINVW { w: self }
    }
}
