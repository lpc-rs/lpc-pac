#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = r"Reader of the field"]
pub type DTRCTRL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DTRCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _DTRCTRLW<'a> {
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
#[doc = r"Reader of the field"]
pub type RTSCTRL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RTSCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSCTRLW<'a> {
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
#[doc = "Possible values of the field `LMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMSR {
    #[doc = "Disable modem loopback mode."]
    DISABLE_MODEM_LOOPBA,
    #[doc = "Enable modem loopback mode."]
    ENABLE_MODEM_LOOPBAC,
}
impl crate::ToBits<bool> for LMSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LMSR::DISABLE_MODEM_LOOPBA => false,
            LMSR::ENABLE_MODEM_LOOPBAC => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LMS_R = crate::FR<bool, LMSR>;
impl LMS_R {
    #[doc = "Checks if the value of the field is `DISABLE_MODEM_LOOPBA`"]
    #[inline(always)]
    pub fn is_disable_modem_loopba(&self) -> bool {
        *self == LMSR::DISABLE_MODEM_LOOPBA
    }
    #[doc = "Checks if the value of the field is `ENABLE_MODEM_LOOPBAC`"]
    #[inline(always)]
    pub fn is_enable_modem_loopbac(&self) -> bool {
        *self == LMSR::ENABLE_MODEM_LOOPBAC
    }
}
#[doc = "Values that can be written to the field `LMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMSW {
    #[doc = "Disable modem loopback mode."]
    DISABLE_MODEM_LOOPBA,
    #[doc = "Enable modem loopback mode."]
    ENABLE_MODEM_LOOPBAC,
}
impl LMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LMSW::DISABLE_MODEM_LOOPBA => false,
            LMSW::ENABLE_MODEM_LOOPBAC => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LMSW<'a> {
    w: &'a mut W,
}
impl<'a> _LMSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable modem loopback mode."]
    #[inline(always)]
    pub fn disable_modem_loopba(self) -> &'a mut W {
        self.variant(LMSW::DISABLE_MODEM_LOOPBA)
    }
    #[doc = "Enable modem loopback mode."]
    #[inline(always)]
    pub fn enable_modem_loopbac(self) -> &'a mut W {
        self.variant(LMSW::ENABLE_MODEM_LOOPBAC)
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
#[doc = "Possible values of the field `RTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSENR {
    #[doc = "Disable auto-rts flow control."]
    DISABLE_AUTO_RTS_FLO,
    #[doc = "Enable auto-rts flow control."]
    ENABLE_AUTO_RTS_FLOW,
}
impl crate::ToBits<bool> for RTSENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RTSENR::DISABLE_AUTO_RTS_FLO => false,
            RTSENR::ENABLE_AUTO_RTS_FLOW => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RTSEN_R = crate::FR<bool, RTSENR>;
impl RTSEN_R {
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_RTS_FLO`"]
    #[inline(always)]
    pub fn is_disable_auto_rts_flo(&self) -> bool {
        *self == RTSENR::DISABLE_AUTO_RTS_FLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_RTS_FLOW`"]
    #[inline(always)]
    pub fn is_enable_auto_rts_flow(&self) -> bool {
        *self == RTSENR::ENABLE_AUTO_RTS_FLOW
    }
}
#[doc = "Values that can be written to the field `RTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSENW {
    #[doc = "Disable auto-rts flow control."]
    DISABLE_AUTO_RTS_FLO,
    #[doc = "Enable auto-rts flow control."]
    ENABLE_AUTO_RTS_FLOW,
}
impl RTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RTSENW::DISABLE_AUTO_RTS_FLO => false,
            RTSENW::ENABLE_AUTO_RTS_FLOW => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTSENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable auto-rts flow control."]
    #[inline(always)]
    pub fn disable_auto_rts_flo(self) -> &'a mut W {
        self.variant(RTSENW::DISABLE_AUTO_RTS_FLO)
    }
    #[doc = "Enable auto-rts flow control."]
    #[inline(always)]
    pub fn enable_auto_rts_flow(self) -> &'a mut W {
        self.variant(RTSENW::ENABLE_AUTO_RTS_FLOW)
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
#[doc = "Possible values of the field `CTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSENR {
    #[doc = "Disable auto-cts flow control."]
    DISABLE_AUTO_CTS_FLO,
    #[doc = "Enable auto-cts flow control."]
    ENABLE_AUTO_CTS_FLOW,
}
impl crate::ToBits<bool> for CTSENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CTSENR::DISABLE_AUTO_CTS_FLO => false,
            CTSENR::ENABLE_AUTO_CTS_FLOW => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CTSEN_R = crate::FR<bool, CTSENR>;
impl CTSEN_R {
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_CTS_FLO`"]
    #[inline(always)]
    pub fn is_disable_auto_cts_flo(&self) -> bool {
        *self == CTSENR::DISABLE_AUTO_CTS_FLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_CTS_FLOW`"]
    #[inline(always)]
    pub fn is_enable_auto_cts_flow(&self) -> bool {
        *self == CTSENR::ENABLE_AUTO_CTS_FLOW
    }
}
#[doc = "Values that can be written to the field `CTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSENW {
    #[doc = "Disable auto-cts flow control."]
    DISABLE_AUTO_CTS_FLO,
    #[doc = "Enable auto-cts flow control."]
    ENABLE_AUTO_CTS_FLOW,
}
impl CTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSENW::DISABLE_AUTO_CTS_FLO => false,
            CTSENW::ENABLE_AUTO_CTS_FLOW => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable auto-cts flow control."]
    #[inline(always)]
    pub fn disable_auto_cts_flo(self) -> &'a mut W {
        self.variant(CTSENW::DISABLE_AUTO_CTS_FLO)
    }
    #[doc = "Enable auto-cts flow control."]
    #[inline(always)]
    pub fn enable_auto_cts_flow(self) -> &'a mut W {
        self.variant(CTSENW::ENABLE_AUTO_CTS_FLOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Source for modem output pin DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrctrl(&self) -> DTRCTRL_R {
        DTRCTRL_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsctrl(&self) -> RTSCTRL_R {
        RTSCTRL_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The DSR, CTS, DCD, and RI pins are ignored. Externally, DTR and RTS are set inactive. Internally, the upper four bits of the MSR are driven by the lower four bits of the MCR. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
    #[inline(always)]
    pub fn lms(&self) -> LMS_R {
        LMS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Source for modem output pin DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrctrl(&mut self) -> _DTRCTRLW {
        _DTRCTRLW { w: self }
    }
    #[doc = "Bit 1 - Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsctrl(&mut self) -> _RTSCTRLW {
        _RTSCTRLW { w: self }
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The DSR, CTS, DCD, and RI pins are ignored. Externally, DTR and RTS are set inactive. Internally, the upper four bits of the MSR are driven by the lower four bits of the MCR. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
    #[inline(always)]
    pub fn lms(&mut self) -> _LMSW {
        _LMSW { w: self }
    }
    #[doc = "Bit 6 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> _RTSENW {
        _RTSENW { w: self }
    }
    #[doc = "Bit 7 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> _CTSENW {
        _CTSENW { w: self }
    }
}
