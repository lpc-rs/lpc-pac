#[doc = "Reader of register IIR"]
pub type R = crate::R<u32, super::IIR>;
#[doc = "Interrupt status. Note that UnIIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating UnIIR\\[3:1\\].\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSTATUS_A {
    #[doc = "0: At least one interrupt is pending."]
    AT_LEAST_ONE_INTERRU = 0,
    #[doc = "1: No interrupt is pending."]
    NO_INTERRUPT_IS_PEND = 1,
}
impl From<INTSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: INTSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTSTATUS`"]
pub type INTSTATUS_R = crate::R<bool, INTSTATUS_A>;
impl INTSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSTATUS_A {
        match self.bits {
            false => INTSTATUS_A::AT_LEAST_ONE_INTERRU,
            true => INTSTATUS_A::NO_INTERRUPT_IS_PEND,
        }
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_INTERRU`"]
    #[inline(always)]
    pub fn is_at_least_one_interru(&self) -> bool {
        *self == INTSTATUS_A::AT_LEAST_ONE_INTERRU
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT_IS_PEND`"]
    #[inline(always)]
    pub fn is_no_interrupt_is_pend(&self) -> bool {
        *self == INTSTATUS_A::NO_INTERRUPT_IS_PEND
    }
}
#[doc = "Interrupt identification. UnIER\\[3:1\\]
identifies an interrupt corresponding to the UARTn Rx or TX FIFO. All other combinations of UnIER\\[3:1\\]
not listed below are reserved (000,100,101,111).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTID_A {
    #[doc = "3: 1   - Receive Line Status (RLS)."]
    _1_RECEIVE_LINE_S = 3,
    #[doc = "2: 2a - Receive Data Available (RDA)."]
    _2A__RECEIVE_DATA_AV = 2,
    #[doc = "6: 2b - Character Time-out Indicator (CTI)."]
    _2B__CHARACTER_TIME_ = 6,
    #[doc = "1: 3   - THRE Interrupt"]
    _3_THRE_INTERRUPT = 1,
}
impl From<INTID_A> for u8 {
    #[inline(always)]
    fn from(variant: INTID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTID`"]
pub type INTID_R = crate::R<u8, INTID_A>;
impl INTID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INTID_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(INTID_A::_1_RECEIVE_LINE_S),
            2 => Val(INTID_A::_2A__RECEIVE_DATA_AV),
            6 => Val(INTID_A::_2B__CHARACTER_TIME_),
            1 => Val(INTID_A::_3_THRE_INTERRUPT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_RECEIVE_LINE_S`"]
    #[inline(always)]
    pub fn is_1_receive_line_s(&self) -> bool {
        *self == INTID_A::_1_RECEIVE_LINE_S
    }
    #[doc = "Checks if the value of the field is `_2A__RECEIVE_DATA_AV`"]
    #[inline(always)]
    pub fn is_2a__receive_data_av(&self) -> bool {
        *self == INTID_A::_2A__RECEIVE_DATA_AV
    }
    #[doc = "Checks if the value of the field is `_2B__CHARACTER_TIME_`"]
    #[inline(always)]
    pub fn is_2b__character_time_(&self) -> bool {
        *self == INTID_A::_2B__CHARACTER_TIME_
    }
    #[doc = "Checks if the value of the field is `_3_THRE_INTERRUPT`"]
    #[inline(always)]
    pub fn is_3_thre_interrupt(&self) -> bool {
        *self == INTID_A::_3_THRE_INTERRUPT
    }
}
#[doc = "Reader of field `FIFOENABLE`"]
pub type FIFOENABLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ABEOINT`"]
pub type ABEOINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ABTOINT`"]
pub type ABTOINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt status. Note that UnIIR\\[0\\]
is active low. The pending interrupt can be determined by evaluating UnIIR\\[3:1\\]."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification. UnIER\\[3:1\\]
identifies an interrupt corresponding to the UARTn Rx or TX FIFO. All other combinations of UnIER\\[3:1\\]
not listed below are reserved (000,100,101,111)."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Copies of UnFCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoenable(&self) -> FIFOENABLE_R {
        FIFOENABLE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
    #[inline(always)]
    pub fn abeoint(&self) -> ABEOINT_R {
        ABEOINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
    #[inline(always)]
    pub fn abtoint(&self) -> ABTOINT_R {
        ABTOINT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
