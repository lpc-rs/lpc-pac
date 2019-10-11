#[doc = "Reader of register MSR"]
pub type R = crate::R<u32, super::MSR>;
#[doc = "Delta CTS. Set upon state change of input CTS. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTS_A {
    #[doc = "0: No change detected on modem input, CTS."]
    NO_CHANGE_DETECTED_O,
    #[doc = "1: State change detected on modem input, CTS."]
    STATE_CHANGE_DETECTE,
}
impl From<DCTS_A> for bool {
    #[inline(always)]
    fn from(variant: DCTS_A) -> Self {
        match variant {
            DCTS_A::NO_CHANGE_DETECTED_O => false,
            DCTS_A::STATE_CHANGE_DETECTE => true,
        }
    }
}
#[doc = "Reader of field `DCTS`"]
pub type DCTS_R = crate::R<bool, DCTS_A>;
impl DCTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCTS_A {
        match self.bits {
            false => DCTS_A::NO_CHANGE_DETECTED_O,
            true => DCTS_A::STATE_CHANGE_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == DCTS_A::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == DCTS_A::STATE_CHANGE_DETECTE
    }
}
#[doc = "Delta DSR. Set upon state change of input DSR. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDSR_A {
    #[doc = "0: No change detected on modem input, DSR."]
    NO_CHANGE_DETECTED_O,
    #[doc = "1: State change detected on modem input, DSR."]
    STATE_CHANGE_DETECTE,
}
impl From<DDSR_A> for bool {
    #[inline(always)]
    fn from(variant: DDSR_A) -> Self {
        match variant {
            DDSR_A::NO_CHANGE_DETECTED_O => false,
            DDSR_A::STATE_CHANGE_DETECTE => true,
        }
    }
}
#[doc = "Reader of field `DDSR`"]
pub type DDSR_R = crate::R<bool, DDSR_A>;
impl DDSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDSR_A {
        match self.bits {
            false => DDSR_A::NO_CHANGE_DETECTED_O,
            true => DDSR_A::STATE_CHANGE_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == DDSR_A::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == DDSR_A::STATE_CHANGE_DETECTE
    }
}
#[doc = "Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERI_A {
    #[doc = "0: No change detected on modem input, RI."]
    NO_CHANGE_DETECTED_O,
    #[doc = "1: Low-to-high transition detected on RI."]
    LOW_TO_HIGH_TRANSITI,
}
impl From<TERI_A> for bool {
    #[inline(always)]
    fn from(variant: TERI_A) -> Self {
        match variant {
            TERI_A::NO_CHANGE_DETECTED_O => false,
            TERI_A::LOW_TO_HIGH_TRANSITI => true,
        }
    }
}
#[doc = "Reader of field `TERI`"]
pub type TERI_R = crate::R<bool, TERI_A>;
impl TERI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERI_A {
        match self.bits {
            false => TERI_A::NO_CHANGE_DETECTED_O,
            true => TERI_A::LOW_TO_HIGH_TRANSITI,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == TERI_A::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH_TRANSITI`"]
    #[inline(always)]
    pub fn is_low_to_high_transiti(&self) -> bool {
        *self == TERI_A::LOW_TO_HIGH_TRANSITI
    }
}
#[doc = "Delta DCD. Set upon state change of input DCD. Cleared on an MSR read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDCD_A {
    #[doc = "0: No change detected on modem input, DCD."]
    NO_CHANGE_DETECTED_O,
    #[doc = "1: State change detected on modem input, DCD."]
    STATE_CHANGE_DETECTE,
}
impl From<DDCD_A> for bool {
    #[inline(always)]
    fn from(variant: DDCD_A) -> Self {
        match variant {
            DDCD_A::NO_CHANGE_DETECTED_O => false,
            DDCD_A::STATE_CHANGE_DETECTE => true,
        }
    }
}
#[doc = "Reader of field `DDCD`"]
pub type DDCD_R = crate::R<bool, DDCD_A>;
impl DDCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDCD_A {
        match self.bits {
            false => DDCD_A::NO_CHANGE_DETECTED_O,
            true => DDCD_A::STATE_CHANGE_DETECTE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == DDCD_A::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == DDCD_A::STATE_CHANGE_DETECTE
    }
}
#[doc = "Reader of field `CTS`"]
pub type CTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSR`"]
pub type DSR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RI`"]
pub type RI_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCD`"]
pub type DCD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Delta CTS. Set upon state change of input CTS. Cleared on an MSR read."]
    #[inline(always)]
    pub fn dcts(&self) -> DCTS_R {
        DCTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Delta DSR. Set upon state change of input DSR. Cleared on an MSR read."]
    #[inline(always)]
    pub fn ddsr(&self) -> DDSR_R {
        DDSR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read."]
    #[inline(always)]
    pub fn teri(&self) -> TERI_R {
        TERI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Delta DCD. Set upon state change of input DCD. Cleared on an MSR read."]
    #[inline(always)]
    pub fn ddcd(&self) -> DDCD_R {
        DDCD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR\\[1\\] in modem loopback mode."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR\\[0\\] in modem loopback mode."]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Ring Indicator State. Complement of input RI. This bit is connected to MCR\\[2\\] in modem loopback mode."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR\\[3\\] in modem loopback mode."]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
