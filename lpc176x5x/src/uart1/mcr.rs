#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = r" Value of the field"]
pub struct DTRCTRLR {
    bits: bool,
}
impl DTRCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct RTSCTRLR {
    bits: bool,
}
impl RTSCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `LMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMSR {
    #[doc = "Disable modem loopback mode."]
    DISABLE_MODEM_LOOPBA,
    #[doc = "Enable modem loopback mode."]
    ENABLE_MODEM_LOOPBAC,
}
impl LMSR {
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
            LMSR::DISABLE_MODEM_LOOPBA => false,
            LMSR::ENABLE_MODEM_LOOPBAC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LMSR {
        match value {
            false => LMSR::DISABLE_MODEM_LOOPBA,
            true => LMSR::ENABLE_MODEM_LOOPBAC,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_MODEM_LOOPBA`"]
    #[inline]
    pub fn is_disable_modem_loopba(&self) -> bool {
        *self == LMSR::DISABLE_MODEM_LOOPBA
    }
    #[doc = "Checks if the value of the field is `ENABLE_MODEM_LOOPBAC`"]
    #[inline]
    pub fn is_enable_modem_loopbac(&self) -> bool {
        *self == LMSR::ENABLE_MODEM_LOOPBAC
    }
}
#[doc = "Possible values of the field `RTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSENR {
    #[doc = "Disable auto-rts flow control."]
    DISABLE_AUTO_RTS_FLO,
    #[doc = "Enable auto-rts flow control."]
    ENABLE_AUTO_RTS_FLOW,
}
impl RTSENR {
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
            RTSENR::DISABLE_AUTO_RTS_FLO => false,
            RTSENR::ENABLE_AUTO_RTS_FLOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTSENR {
        match value {
            false => RTSENR::DISABLE_AUTO_RTS_FLO,
            true => RTSENR::ENABLE_AUTO_RTS_FLOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_RTS_FLO`"]
    #[inline]
    pub fn is_disable_auto_rts_flo(&self) -> bool {
        *self == RTSENR::DISABLE_AUTO_RTS_FLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_RTS_FLOW`"]
    #[inline]
    pub fn is_enable_auto_rts_flow(&self) -> bool {
        *self == RTSENR::ENABLE_AUTO_RTS_FLOW
    }
}
#[doc = "Possible values of the field `CTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSENR {
    #[doc = "Disable auto-cts flow control."]
    DISABLE_AUTO_CTS_FLO,
    #[doc = "Enable auto-cts flow control."]
    ENABLE_AUTO_CTS_FLOW,
}
impl CTSENR {
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
            CTSENR::DISABLE_AUTO_CTS_FLO => false,
            CTSENR::ENABLE_AUTO_CTS_FLOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTSENR {
        match value {
            false => CTSENR::DISABLE_AUTO_CTS_FLO,
            true => CTSENR::ENABLE_AUTO_CTS_FLOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_CTS_FLO`"]
    #[inline]
    pub fn is_disable_auto_cts_flo(&self) -> bool {
        *self == CTSENR::DISABLE_AUTO_CTS_FLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_CTS_FLOW`"]
    #[inline]
    pub fn is_enable_auto_cts_flow(&self) -> bool {
        *self == CTSENR::ENABLE_AUTO_CTS_FLOW
    }
}
#[doc = r" Proxy"]
pub struct _DTRCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _DTRCTRLW<'a> {
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
#[doc = r" Proxy"]
pub struct _RTSCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSCTRLW<'a> {
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
#[doc = "Values that can be written to the field `LMS`"]
pub enum LMSW {
    #[doc = "Disable modem loopback mode."]
    DISABLE_MODEM_LOOPBA,
    #[doc = "Enable modem loopback mode."]
    ENABLE_MODEM_LOOPBAC,
}
impl LMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LMSW::DISABLE_MODEM_LOOPBA => false,
            LMSW::ENABLE_MODEM_LOOPBAC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LMSW<'a> {
    w: &'a mut W,
}
impl<'a> _LMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable modem loopback mode."]
    #[inline]
    pub fn disable_modem_loopba(self) -> &'a mut W {
        self.variant(LMSW::DISABLE_MODEM_LOOPBA)
    }
    #[doc = "Enable modem loopback mode."]
    #[inline]
    pub fn enable_modem_loopbac(self) -> &'a mut W {
        self.variant(LMSW::ENABLE_MODEM_LOOPBAC)
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
#[doc = "Values that can be written to the field `RTSEN`"]
pub enum RTSENW {
    #[doc = "Disable auto-rts flow control."]
    DISABLE_AUTO_RTS_FLO,
    #[doc = "Enable auto-rts flow control."]
    ENABLE_AUTO_RTS_FLOW,
}
impl RTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSENW::DISABLE_AUTO_RTS_FLO => false,
            RTSENW::ENABLE_AUTO_RTS_FLOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable auto-rts flow control."]
    #[inline]
    pub fn disable_auto_rts_flo(self) -> &'a mut W {
        self.variant(RTSENW::DISABLE_AUTO_RTS_FLO)
    }
    #[doc = "Enable auto-rts flow control."]
    #[inline]
    pub fn enable_auto_rts_flow(self) -> &'a mut W {
        self.variant(RTSENW::ENABLE_AUTO_RTS_FLOW)
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
#[doc = "Values that can be written to the field `CTSEN`"]
pub enum CTSENW {
    #[doc = "Disable auto-cts flow control."]
    DISABLE_AUTO_CTS_FLO,
    #[doc = "Enable auto-cts flow control."]
    ENABLE_AUTO_CTS_FLOW,
}
impl CTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSENW::DISABLE_AUTO_CTS_FLO => false,
            CTSENW::ENABLE_AUTO_CTS_FLOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable auto-cts flow control."]
    #[inline]
    pub fn disable_auto_cts_flo(self) -> &'a mut W {
        self.variant(CTSENW::DISABLE_AUTO_CTS_FLO)
    }
    #[doc = "Enable auto-cts flow control."]
    #[inline]
    pub fn enable_auto_cts_flow(self) -> &'a mut W {
        self.variant(CTSENW::ENABLE_AUTO_CTS_FLOW)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline]
    pub fn dtrctrl(&self) -> DTRCTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTRCTRLR { bits }
    }
    #[doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline]
    pub fn rtsctrl(&self) -> RTSCTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTSCTRLR { bits }
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR."]
    #[inline]
    pub fn lms(&self) -> LMSR {
        LMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RTS enable."]
    #[inline]
    pub fn rtsen(&self) -> RTSENR {
        RTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CTS enable."]
    #[inline]
    pub fn ctsen(&self) -> CTSENR {
        CTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline]
    pub fn dtrctrl(&mut self) -> _DTRCTRLW {
        _DTRCTRLW { w: self }
    }
    #[doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline]
    pub fn rtsctrl(&mut self) -> _RTSCTRLW {
        _RTSCTRLW { w: self }
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR."]
    #[inline]
    pub fn lms(&mut self) -> _LMSW {
        _LMSW { w: self }
    }
    #[doc = "Bit 6 - RTS enable."]
    #[inline]
    pub fn rtsen(&mut self) -> _RTSENW {
        _RTSENW { w: self }
    }
    #[doc = "Bit 7 - CTS enable."]
    #[inline]
    pub fn ctsen(&mut self) -> _CTSENW {
        _CTSENW { w: self }
    }
}
