#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTRCTRL`"]
pub type DTRCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTRCTRL`"]
pub struct DTRCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRCTRL_W<'a> {
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
#[doc = "Reader of field `RTSCTRL`"]
pub type RTSCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTSCTRL`"]
pub struct RTSCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSCTRL_W<'a> {
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
#[doc = "Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The DSR, CTS, DCD, and RI pins are ignored. Externally, DTR and RTS are set inactive. Internally, the upper four bits of the MSR are driven by the lower four bits of the MCR. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMS_A {
    #[doc = "0: Disable modem loopback mode."]
    DISABLE_MODEM_LOOPBA = 0,
    #[doc = "1: Enable modem loopback mode."]
    ENABLE_MODEM_LOOPBAC = 1,
}
impl From<LMS_A> for bool {
    #[inline(always)]
    fn from(variant: LMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LMS`"]
pub type LMS_R = crate::R<bool, LMS_A>;
impl LMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMS_A {
        match self.bits {
            false => LMS_A::DISABLE_MODEM_LOOPBA,
            true => LMS_A::ENABLE_MODEM_LOOPBAC,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_MODEM_LOOPBA`"]
    #[inline(always)]
    pub fn is_disable_modem_loopba(&self) -> bool {
        *self == LMS_A::DISABLE_MODEM_LOOPBA
    }
    #[doc = "Checks if the value of the field is `ENABLE_MODEM_LOOPBAC`"]
    #[inline(always)]
    pub fn is_enable_modem_loopbac(&self) -> bool {
        *self == LMS_A::ENABLE_MODEM_LOOPBAC
    }
}
#[doc = "Write proxy for field `LMS`"]
pub struct LMS_W<'a> {
    w: &'a mut W,
}
impl<'a> LMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LMS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable modem loopback mode."]
    #[inline(always)]
    pub fn disable_modem_loopba(self) -> &'a mut W {
        self.variant(LMS_A::DISABLE_MODEM_LOOPBA)
    }
    #[doc = "Enable modem loopback mode."]
    #[inline(always)]
    pub fn enable_modem_loopbac(self) -> &'a mut W {
        self.variant(LMS_A::ENABLE_MODEM_LOOPBAC)
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
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSEN_A {
    #[doc = "0: Disable auto-rts flow control."]
    DISABLE_AUTO_RTS_FLO = 0,
    #[doc = "1: Enable auto-rts flow control."]
    ENABLE_AUTO_RTS_FLOW = 1,
}
impl From<RTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTSEN`"]
pub type RTSEN_R = crate::R<bool, RTSEN_A>;
impl RTSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSEN_A {
        match self.bits {
            false => RTSEN_A::DISABLE_AUTO_RTS_FLO,
            true => RTSEN_A::ENABLE_AUTO_RTS_FLOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_RTS_FLO`"]
    #[inline(always)]
    pub fn is_disable_auto_rts_flo(&self) -> bool {
        *self == RTSEN_A::DISABLE_AUTO_RTS_FLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_RTS_FLOW`"]
    #[inline(always)]
    pub fn is_enable_auto_rts_flow(&self) -> bool {
        *self == RTSEN_A::ENABLE_AUTO_RTS_FLOW
    }
}
#[doc = "Write proxy for field `RTSEN`"]
pub struct RTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable auto-rts flow control."]
    #[inline(always)]
    pub fn disable_auto_rts_flo(self) -> &'a mut W {
        self.variant(RTSEN_A::DISABLE_AUTO_RTS_FLO)
    }
    #[doc = "Enable auto-rts flow control."]
    #[inline(always)]
    pub fn enable_auto_rts_flow(self) -> &'a mut W {
        self.variant(RTSEN_A::ENABLE_AUTO_RTS_FLOW)
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
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEN_A {
    #[doc = "0: Disable auto-cts flow control."]
    DISABLE_AUTO_CTS_FLO = 0,
    #[doc = "1: Enable auto-cts flow control."]
    ENABLE_AUTO_CTS_FLOW = 1,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTSEN`"]
pub type CTSEN_R = crate::R<bool, CTSEN_A>;
impl CTSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLE_AUTO_CTS_FLO,
            true => CTSEN_A::ENABLE_AUTO_CTS_FLOW,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_CTS_FLO`"]
    #[inline(always)]
    pub fn is_disable_auto_cts_flo(&self) -> bool {
        *self == CTSEN_A::DISABLE_AUTO_CTS_FLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_CTS_FLOW`"]
    #[inline(always)]
    pub fn is_enable_auto_cts_flow(&self) -> bool {
        *self == CTSEN_A::ENABLE_AUTO_CTS_FLOW
    }
}
#[doc = "Write proxy for field `CTSEN`"]
pub struct CTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable auto-cts flow control."]
    #[inline(always)]
    pub fn disable_auto_cts_flo(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLE_AUTO_CTS_FLO)
    }
    #[doc = "Enable auto-cts flow control."]
    #[inline(always)]
    pub fn enable_auto_cts_flow(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLE_AUTO_CTS_FLOW)
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
    #[doc = "Bit 0 - Source for modem output pin DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrctrl(&self) -> DTRCTRL_R {
        DTRCTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsctrl(&self) -> RTSCTRL_R {
        RTSCTRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The DSR, CTS, DCD, and RI pins are ignored. Externally, DTR and RTS are set inactive. Internally, the upper four bits of the MSR are driven by the lower four bits of the MCR. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
    #[inline(always)]
    pub fn lms(&self) -> LMS_R {
        LMS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source for modem output pin DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrctrl(&mut self) -> DTRCTRL_W {
        DTRCTRL_W { w: self }
    }
    #[doc = "Bit 1 - Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsctrl(&mut self) -> RTSCTRL_W {
        RTSCTRL_W { w: self }
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD, has no effect on loopback and output pin, TXD is held in marking state. The DSR, CTS, DCD, and RI pins are ignored. Externally, DTR and RTS are set inactive. Internally, the upper four bits of the MSR are driven by the lower four bits of the MCR. This permits modem status interrupts to be generated in loopback mode by writing the lower four bits of MCR."]
    #[inline(always)]
    pub fn lms(&mut self) -> LMS_W {
        LMS_W { w: self }
    }
    #[doc = "Bit 6 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W {
        RTSEN_W { w: self }
    }
    #[doc = "Bit 7 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W { w: self }
    }
}
