#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RS485CTRL {
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
#[doc = "Possible values of the field `NMMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMMENR {
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    DISABLED,
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    ENABLED,
}
impl crate::ToBits<bool> for NMMENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NMMENR::DISABLED => false,
            NMMENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NMMEN_R = crate::FR<bool, NMMENR>;
impl NMMEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NMMENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NMMENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `NMMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMMENW {
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    DISABLED,
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    ENABLED,
}
impl NMMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NMMENW::DISABLED => false,
            NMMENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NMMENW<'a> {
    w: &'a mut W,
}
impl<'a> _NMMENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NMMENW::DISABLED)
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte causes the USART to set the parity error and generate an interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NMMENW::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
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
impl crate::ToBits<bool> for RXDISR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RXDISR::THE_RECEIVER_IS_ENAB => false,
            RXDISR::THE_RECEIVER_IS_DISA => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXDIS_R = crate::FR<bool, RXDISR>;
impl RXDIS_R {
    #[doc = "Checks if the value of the field is `THE_RECEIVER_IS_ENAB`"]
    #[inline(always)]
    pub fn is_the_receiver_is_enab(&self) -> bool {
        *self == RXDISR::THE_RECEIVER_IS_ENAB
    }
    #[doc = "Checks if the value of the field is `THE_RECEIVER_IS_DISA`"]
    #[inline(always)]
    pub fn is_the_receiver_is_disa(&self) -> bool {
        *self == RXDISR::THE_RECEIVER_IS_DISA
    }
}
#[doc = "Values that can be written to the field `RXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDISW {
    #[doc = "The receiver is enabled."]
    THE_RECEIVER_IS_ENAB,
    #[doc = "The receiver is disabled."]
    THE_RECEIVER_IS_DISA,
}
impl RXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDISW::THE_RECEIVER_IS_ENAB => false,
            RXDISW::THE_RECEIVER_IS_DISA => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDISW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The receiver is enabled."]
    #[inline(always)]
    pub fn the_receiver_is_enab(self) -> &'a mut W {
        self.variant(RXDISW::THE_RECEIVER_IS_ENAB)
    }
    #[doc = "The receiver is disabled."]
    #[inline(always)]
    pub fn the_receiver_is_disa(self) -> &'a mut W {
        self.variant(RXDISW::THE_RECEIVER_IS_DISA)
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
#[doc = "Possible values of the field `AADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AADENR {
    #[doc = "Auto Address Detect (AAD) is disabled."]
    AUTO_ADDRESS_DETECT_DISABLED,
    #[doc = "Auto Address Detect (AAD) is enabled."]
    AUTO_ADDRESS_DETECT_ENABLED,
}
impl crate::ToBits<bool> for AADENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AADENR::AUTO_ADDRESS_DETECT_DISABLED => false,
            AADENR::AUTO_ADDRESS_DETECT_ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AADEN_R = crate::FR<bool, AADENR>;
impl AADEN_R {
    #[doc = "Checks if the value of the field is `AUTO_ADDRESS_DETECT_DISABLED`"]
    #[inline(always)]
    pub fn is_auto_address_detect_disabled(&self) -> bool {
        *self == AADENR::AUTO_ADDRESS_DETECT_DISABLED
    }
    #[doc = "Checks if the value of the field is `AUTO_ADDRESS_DETECT_ENABLED`"]
    #[inline(always)]
    pub fn is_auto_address_detect_enabled(&self) -> bool {
        *self == AADENR::AUTO_ADDRESS_DETECT_ENABLED
    }
}
#[doc = "Values that can be written to the field `AADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AADENW {
    #[doc = "Auto Address Detect (AAD) is disabled."]
    AUTO_ADDRESS_DETECT_DISABLED,
    #[doc = "Auto Address Detect (AAD) is enabled."]
    AUTO_ADDRESS_DETECT_ENABLED,
}
impl AADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AADENW::AUTO_ADDRESS_DETECT_DISABLED => false,
            AADENW::AUTO_ADDRESS_DETECT_ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AADENW<'a> {
    w: &'a mut W,
}
impl<'a> _AADENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AADENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Auto Address Detect (AAD) is disabled."]
    #[inline(always)]
    pub fn auto_address_detect_disabled(self) -> &'a mut W {
        self.variant(AADENW::AUTO_ADDRESS_DETECT_DISABLED)
    }
    #[doc = "Auto Address Detect (AAD) is enabled."]
    #[inline(always)]
    pub fn auto_address_detect_enabled(self) -> &'a mut W {
        self.variant(AADENW::AUTO_ADDRESS_DETECT_ENABLED)
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    RTS,
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    DTR,
}
impl crate::ToBits<bool> for SELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SELR::RTS => false,
            SELR::DTR => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEL_R = crate::FR<bool, SELR>;
impl SEL_R {
    #[doc = "Checks if the value of the field is `RTS`"]
    #[inline(always)]
    pub fn is_rts(&self) -> bool {
        *self == SELR::RTS
    }
    #[doc = "Checks if the value of the field is `DTR`"]
    #[inline(always)]
    pub fn is_dtr(&self) -> bool {
        *self == SELR::DTR
    }
}
#[doc = "Values that can be written to the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELW {
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    RTS,
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    DTR,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SELW::RTS => false,
            SELW::DTR => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    #[inline(always)]
    pub fn rts(self) -> &'a mut W {
        self.variant(SELW::RTS)
    }
    #[doc = "If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    #[inline(always)]
    pub fn dtr(self) -> &'a mut W {
        self.variant(SELW::DTR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
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
impl crate::ToBits<bool> for DCTRLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DCTRLR::DISABLE_AUTO_DIRECTI => false,
            DCTRLR::ENABLE_AUTO_DIRECTIO => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DCTRL_R = crate::FR<bool, DCTRLR>;
impl DCTRL_R {
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_DIRECTI`"]
    #[inline(always)]
    pub fn is_disable_auto_directi(&self) -> bool {
        *self == DCTRLR::DISABLE_AUTO_DIRECTI
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_DIRECTIO`"]
    #[inline(always)]
    pub fn is_enable_auto_directio(&self) -> bool {
        *self == DCTRLR::ENABLE_AUTO_DIRECTIO
    }
}
#[doc = "Values that can be written to the field `DCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTRLW {
    #[doc = "Disable Auto Direction Control."]
    DISABLE_AUTO_DIRECTI,
    #[doc = "Enable Auto Direction Control."]
    ENABLE_AUTO_DIRECTIO,
}
impl DCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DCTRLW::DISABLE_AUTO_DIRECTI => false,
            DCTRLW::ENABLE_AUTO_DIRECTIO => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _DCTRLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn disable_auto_directi(self) -> &'a mut W {
        self.variant(DCTRLW::DISABLE_AUTO_DIRECTI)
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn enable_auto_directio(self) -> &'a mut W {
        self.variant(DCTRLW::ENABLE_AUTO_DIRECTIO)
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
#[doc = "Possible values of the field `OINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OINVR {
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW,
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH,
}
impl crate::ToBits<bool> for OINVR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OINVR::LOW => false,
            OINVR::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OINV_R = crate::FR<bool, OINVR>;
impl OINV_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OINVR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OINVR::HIGH
    }
}
#[doc = "Values that can be written to the field `OINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OINVW {
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW,
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH,
}
impl OINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OINVW::LOW => false,
            OINVW::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OINVW<'a> {
    w: &'a mut W,
}
impl<'a> _OINVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OINVW::LOW)
    }
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OINVW::HIGH)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    pub fn nmmen(&self) -> NMMEN_R {
        NMMEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    pub fn rxdis(&self) -> RXDIS_R {
        RXDIS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline(always)]
    pub fn aaden(&self) -> AADEN_R {
        AADEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select direction control pin"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Auto direction control enable."]
    #[inline(always)]
    pub fn dctrl(&self) -> DCTRL_R {
        DCTRL_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
    #[inline(always)]
    pub fn oinv(&self) -> OINV_R {
        OINV_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    pub fn nmmen(&mut self) -> _NMMENW {
        _NMMENW { w: self }
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    pub fn rxdis(&mut self) -> _RXDISW {
        _RXDISW { w: self }
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline(always)]
    pub fn aaden(&mut self) -> _AADENW {
        _AADENW { w: self }
    }
    #[doc = "Bit 3 - Select direction control pin"]
    #[inline(always)]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
    #[doc = "Bit 4 - Auto direction control enable."]
    #[inline(always)]
    pub fn dctrl(&mut self) -> _DCTRLW {
        _DCTRLW { w: self }
    }
    #[doc = "Bit 5 - Polarity control. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
    #[inline(always)]
    pub fn oinv(&mut self) -> _OINVW {
        _OINVW { w: self }
    }
}
