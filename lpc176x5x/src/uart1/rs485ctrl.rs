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
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt."]
    ENABLED_IN_THIS_MOD,
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
            NMMENR::DISABLED_ => false,
            NMMENR::ENABLED_IN_THIS_MOD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NMMENR {
        match value {
            false => NMMENR::DISABLED_,
            true => NMMENR::ENABLED_IN_THIS_MOD,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == NMMENR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `ENABLED_IN_THIS_MOD`"]
    #[inline]
    pub fn is_enabled_in_this_mod(&self) -> bool {
        *self == NMMENR::ENABLED_IN_THIS_MOD
    }
}
#[doc = "Possible values of the field `RXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDISR {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
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
            RXDISR::ENABLED_ => false,
            RXDISR::DISABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDISR {
        match value {
            false => RXDISR::ENABLED_,
            true => RXDISR::DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == RXDISR::ENABLED_
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == RXDISR::DISABLED_
    }
}
#[doc = "Possible values of the field `AADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AADENR {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Enabled."]
    ENABLED_,
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
            AADENR::DISABLED_ => false,
            AADENR::ENABLED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AADENR {
        match value {
            false => AADENR::DISABLED_,
            true => AADENR::ENABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == AADENR::DISABLED_
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == AADENR::ENABLED_
    }
}
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    RTS_IF_DIRECTION_CO,
    #[doc = "DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    DTR_IF_DIRECTION_CO,
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
            SELR::RTS_IF_DIRECTION_CO => false,
            SELR::DTR_IF_DIRECTION_CO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELR {
        match value {
            false => SELR::RTS_IF_DIRECTION_CO,
            true => SELR::DTR_IF_DIRECTION_CO,
        }
    }
    #[doc = "Checks if the value of the field is `RTS_IF_DIRECTION_CO`"]
    #[inline]
    pub fn is_rts_if_direction_co(&self) -> bool {
        *self == SELR::RTS_IF_DIRECTION_CO
    }
    #[doc = "Checks if the value of the field is `DTR_IF_DIRECTION_CO`"]
    #[inline]
    pub fn is_dtr_if_direction_co(&self) -> bool {
        *self == SELR::DTR_IF_DIRECTION_CO
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
    #[doc = "LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW_THE_DIRECTION_C,
    #[doc = "HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH_THE_DIRECTION_,
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
            OINVR::LOW_THE_DIRECTION_C => false,
            OINVR::HIGH_THE_DIRECTION_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OINVR {
        match value {
            false => OINVR::LOW_THE_DIRECTION_C,
            true => OINVR::HIGH_THE_DIRECTION_,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_THE_DIRECTION_C`"]
    #[inline]
    pub fn is_low_the_direction_c(&self) -> bool {
        *self == OINVR::LOW_THE_DIRECTION_C
    }
    #[doc = "Checks if the value of the field is `HIGH_THE_DIRECTION_`"]
    #[inline]
    pub fn is_high_the_direction_(&self) -> bool {
        *self == OINVR::HIGH_THE_DIRECTION_
    }
}
#[doc = "Values that can be written to the field `NMMEN`"]
pub enum NMMENW {
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt."]
    ENABLED_IN_THIS_MOD,
}
impl NMMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NMMENW::DISABLED_ => false,
            NMMENW::ENABLED_IN_THIS_MOD => true,
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
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(NMMENW::DISABLED_)
    }
    #[doc = "Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt."]
    #[inline]
    pub fn enabled_in_this_mod(self) -> &'a mut W {
        self.variant(NMMENW::ENABLED_IN_THIS_MOD)
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
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl RXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDISW::ENABLED_ => false,
            RXDISW::DISABLED_ => true,
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
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(RXDISW::ENABLED_)
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(RXDISW::DISABLED_)
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
    #[doc = "Disabled."]
    DISABLED_,
    #[doc = "Enabled."]
    ENABLED_,
}
impl AADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AADENW::DISABLED_ => false,
            AADENW::ENABLED_ => true,
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
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(AADENW::DISABLED_)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(AADENW::ENABLED_)
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
    #[doc = "RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    RTS_IF_DIRECTION_CO,
    #[doc = "DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    DTR_IF_DIRECTION_CO,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELW::RTS_IF_DIRECTION_CO => false,
            SELW::DTR_IF_DIRECTION_CO => true,
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
    #[doc = "RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    #[inline]
    pub fn rts_if_direction_co(self) -> &'a mut W {
        self.variant(SELW::RTS_IF_DIRECTION_CO)
    }
    #[doc = "DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    #[inline]
    pub fn dtr_if_direction_co(self) -> &'a mut W {
        self.variant(SELW::DTR_IF_DIRECTION_CO)
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
    #[doc = "LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LOW_THE_DIRECTION_C,
    #[doc = "HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HIGH_THE_DIRECTION_,
}
impl OINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OINVW::LOW_THE_DIRECTION_C => false,
            OINVW::HIGH_THE_DIRECTION_ => true,
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
    #[doc = "LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline]
    pub fn low_the_direction_c(self) -> &'a mut W {
        self.variant(OINVW::LOW_THE_DIRECTION_C)
    }
    #[doc = "HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline]
    pub fn high_the_direction_(self) -> &'a mut W {
        self.variant(OINVW::HIGH_THE_DIRECTION_)
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
    #[doc = "Bit 0 - RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select."]
    #[inline]
    pub fn nmmen(&self) -> NMMENR {
        NMMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive enable."]
    #[inline]
    pub fn rxdis(&self) -> RXDISR {
        RXDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Auto Address Detect (AAD) enable."]
    #[inline]
    pub fn aaden(&self) -> AADENR {
        AADENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Direction control."]
    #[inline]
    pub fn sel(&self) -> SELR {
        SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Direction control enable."]
    #[inline]
    pub fn dctrl(&self) -> DCTRLR {
        DCTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
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
    #[doc = "Bit 0 - RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select."]
    #[inline]
    pub fn nmmen(&mut self) -> _NMMENW {
        _NMMENW { w: self }
    }
    #[doc = "Bit 1 - Receive enable."]
    #[inline]
    pub fn rxdis(&mut self) -> _RXDISW {
        _RXDISW { w: self }
    }
    #[doc = "Bit 2 - Auto Address Detect (AAD) enable."]
    #[inline]
    pub fn aaden(&mut self) -> _AADENW {
        _AADENW { w: self }
    }
    #[doc = "Bit 3 - Direction control."]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
    #[doc = "Bit 4 - Direction control enable."]
    #[inline]
    pub fn dctrl(&mut self) -> _DCTRLW {
        _DCTRLW { w: self }
    }
    #[doc = "Bit 5 - Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
    #[inline]
    pub fn oinv(&mut self) -> _OINVW {
        _OINVW { w: self }
    }
}
