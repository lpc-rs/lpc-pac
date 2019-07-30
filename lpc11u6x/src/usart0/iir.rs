#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IIR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `INTSTATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSTATUSR {
    #[doc = "At least one interrupt is pending."]
    AT_LEAST_ONE_INTERRU,
    #[doc = "No interrupt is pending."]
    NO_INTERRUPT_IS_PEND,
}
impl crate::ToBits<bool> for INTSTATUSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INTSTATUSR::AT_LEAST_ONE_INTERRU => false,
            INTSTATUSR::NO_INTERRUPT_IS_PEND => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTSTATUS_R = crate::FR<bool, INTSTATUSR>;
impl INTSTATUS_R {
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_INTERRU`"]
    #[inline(always)]
    pub fn is_at_least_one_interru(&self) -> bool {
        *self == INTSTATUSR::AT_LEAST_ONE_INTERRU
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_IS_PEND`"]
    #[inline(always)]
    pub fn is_no_interrupt_is_pend(&self) -> bool {
        *self == INTSTATUSR::NO_INTERRUPT_IS_PEND
    }
}
#[doc = "Possible values of the field `INTID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTIDR {
    #[doc = "1   - Receive Line Status (RLS)."]
    _1_RECEIVE_LINE_S,
    #[doc = "2a - Receive Data Available (RDA)."]
    _2A_RECEIVE_DATA_AV,
    #[doc = "2b - Character Time-out Indicator (CTI)."]
    _2B_CHARACTER_TIME_,
    #[doc = "3   - THRE Interrupt."]
    _3_THRE_INTERRUPT,
    #[doc = "4   - Modem status"]
    _4_MODEM_STATUS,
}
impl crate::ToBits<u8> for INTIDR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            INTIDR::_1_RECEIVE_LINE_S => 3,
            INTIDR::_2A_RECEIVE_DATA_AV => 2,
            INTIDR::_2B_CHARACTER_TIME_ => 6,
            INTIDR::_3_THRE_INTERRUPT => 1,
            INTIDR::_4_MODEM_STATUS => 0,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTID_R = crate::FR<u8, INTIDR>;
impl INTID_R {
    #[doc = "Checks if the value of the field is `_1_RECEIVE_LINE_S`"]
    #[inline(always)]
    pub fn is_1_receive_line_s(&self) -> bool {
        *self == INTIDR::_1_RECEIVE_LINE_S
    }
    #[doc = "Checks if the value of the field is `_2A_RECEIVE_DATA_AV`"]
    #[inline(always)]
    pub fn is_2a_receive_data_av(&self) -> bool {
        *self == INTIDR::_2A_RECEIVE_DATA_AV
    }
    #[doc = "Checks if the value of the field is `_2B_CHARACTER_TIME_`"]
    #[inline(always)]
    pub fn is_2b_character_time_(&self) -> bool {
        *self == INTIDR::_2B_CHARACTER_TIME_
    }
    #[doc = "Checks if the value of the field is `_3_THRE_INTERRUPT`"]
    #[inline(always)]
    pub fn is_3_thre_interrupt(&self) -> bool {
        *self == INTIDR::_3_THRE_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `_4_MODEM_STATUS`"]
    #[inline(always)]
    pub fn is_4_modem_status(&self) -> bool {
        *self == INTIDR::_4_MODEM_STATUS
    }
}
#[doc = r"Reader of the field"]
pub type FIFOEN_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type ABEOINT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ABTOINT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt status. Note that IIR\\[0\\] is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification. IER\\[3:1\\] identifies an interrupt corresponding to the USART Rx FIFO. All other values of IER\\[3:1\\] not listed below are reserved."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits() >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - These bits are equivalent to FCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
    #[inline(always)]
    pub fn abeoint(&self) -> ABEOINT_R {
        ABEOINT_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
    #[inline(always)]
    pub fn abtoint(&self) -> ABTOINT_R {
        ABTOINT_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
}
