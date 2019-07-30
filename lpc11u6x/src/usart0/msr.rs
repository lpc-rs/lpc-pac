#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DCTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCTSR {
    #[doc = "No change detected on modem input, CTS."]
    NO_CHANGE_DETECTED_O,
    #[doc = "State change detected on modem input, CTS."]
    STATE_CHANGE_DETECTE,
}
impl crate::ToBits<bool> for DCTSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DCTSR::NO_CHANGE_DETECTED_O => false,
            DCTSR::STATE_CHANGE_DETECTE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DCTS_R = crate::FR<bool, DCTSR>;
impl DCTS_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == DCTSR::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == DCTSR::STATE_CHANGE_DETECTE
    }
}
#[doc = "Possible values of the field `DDSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDSRR {
    #[doc = "No change detected on modem input, DSR."]
    NO_CHANGE_DETECTED_O,
    #[doc = "State change detected on modem input, DSR."]
    STATE_CHANGE_DETECTE,
}
impl crate::ToBits<bool> for DDSRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DDSRR::NO_CHANGE_DETECTED_O => false,
            DDSRR::STATE_CHANGE_DETECTE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DDSR_R = crate::FR<bool, DDSRR>;
impl DDSR_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == DDSRR::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == DDSRR::STATE_CHANGE_DETECTE
    }
}
#[doc = "Possible values of the field `TERI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERIR {
    #[doc = "No change detected on modem input, RI."]
    NO_CHANGE_DETECTED_O,
    #[doc = "Low-to-high transition detected on RI."]
    LOW_TO_HIGH_TRANSITI,
}
impl crate::ToBits<bool> for TERIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TERIR::NO_CHANGE_DETECTED_O => false,
            TERIR::LOW_TO_HIGH_TRANSITI => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TERI_R = crate::FR<bool, TERIR>;
impl TERI_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == TERIR::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH_TRANSITI`"]
    #[inline(always)]
    pub fn is_low_to_high_transiti(&self) -> bool {
        *self == TERIR::LOW_TO_HIGH_TRANSITI
    }
}
#[doc = "Possible values of the field `DDCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDCDR {
    #[doc = "No change detected on modem input, DCD."]
    NO_CHANGE_DETECTED_O,
    #[doc = "State change detected on modem input, DCD."]
    STATE_CHANGE_DETECTE,
}
impl crate::ToBits<bool> for DDCDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DDCDR::NO_CHANGE_DETECTED_O => false,
            DDCDR::STATE_CHANGE_DETECTE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DDCD_R = crate::FR<bool, DDCDR>;
impl DDCD_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE_DETECTED_O`"]
    #[inline(always)]
    pub fn is_no_change_detected_o(&self) -> bool {
        *self == DDCDR::NO_CHANGE_DETECTED_O
    }
    #[doc = "Checks if the value of the field is `STATE_CHANGE_DETECTE`"]
    #[inline(always)]
    pub fn is_state_change_detecte(&self) -> bool {
        *self == DDCDR::STATE_CHANGE_DETECTE
    }
}
#[doc = r"Reader of the field"]
pub type CTS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DSR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DCD_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Delta CTS. Set upon state change of input CTS. Cleared on an MSR read."]
    #[inline(always)]
    pub fn dcts(&self) -> DCTS_R {
        DCTS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Delta DSR. Set upon state change of input DSR. Cleared on an MSR read."]
    #[inline(always)]
    pub fn ddsr(&self) -> DDSR_R {
        DDSR_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge RI. Set upon low to high transition of input RI. Cleared on an MSR read."]
    #[inline(always)]
    pub fn teri(&self) -> TERI_R {
        TERI_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Delta DCD. Set upon state change of input DCD. Cleared on an MSR read."]
    #[inline(always)]
    pub fn ddcd(&self) -> DDCD_R {
        DDCD_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear To Send State. Complement of input signal CTS. This bit is connected to MCR\\[1\\] in modem loopback mode."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Set Ready State. Complement of input signal DSR. This bit is connected to MCR\\[0\\] in modem loopback mode."]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Ring Indicator State. Complement of input RI. This bit is connected to MCR\\[2\\] in modem loopback mode."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data Carrier Detect State. Complement of input DCD. This bit is connected to MCR\\[3\\] in modem loopback mode."]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
