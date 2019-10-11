#[doc = "Reader of register IIR"]
pub type R = crate::R<u32, super::IIR>;
#[doc = "Interrupt status. Note that IIR\\[0\\] is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\].\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTSTATUS_A {
    #[doc = "0: At least one interrupt is pending."]
    PENDING,
    #[doc = "1: No interrupt is pending."]
    NONE,
}
impl From<INTSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: INTSTATUS_A) -> Self {
        match variant {
            INTSTATUS_A::PENDING => false,
            INTSTATUS_A::NONE => true,
        }
    }
}
#[doc = "Reader of field `INTSTATUS`"]
pub type INTSTATUS_R = crate::R<bool, INTSTATUS_A>;
impl INTSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTSTATUS_A {
        match self.bits {
            false => INTSTATUS_A::PENDING,
            true => INTSTATUS_A::NONE,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == INTSTATUS_A::PENDING
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == INTSTATUS_A::NONE
    }
}
#[doc = "Interrupt identification. IER\\[3:1\\] identifies an interrupt corresponding to the USART Rx FIFO. All other values of IER\\[3:1\\] not listed below are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTID_A {
    #[doc = "3: 1   - Receive Line Status (RLS)."]
    RECEIVE_LINE_STATUS,
    #[doc = "2: 2a - Receive Data Available (RDA)."]
    RECEIVE_DATA_AVAILABLE,
    #[doc = "6: 2b - Character Time-out Indicator (CTI)."]
    CHARACTER_TIMEOUT_INDICATOR,
    #[doc = "1: 3   - THRE Interrupt."]
    THRE_INTERRUPT,
    #[doc = "0: 4   - Modem status"]
    MODEM_STATUS,
}
impl From<INTID_A> for u8 {
    #[inline(always)]
    fn from(variant: INTID_A) -> Self {
        match variant {
            INTID_A::RECEIVE_LINE_STATUS => 3,
            INTID_A::RECEIVE_DATA_AVAILABLE => 2,
            INTID_A::CHARACTER_TIMEOUT_INDICATOR => 6,
            INTID_A::THRE_INTERRUPT => 1,
            INTID_A::MODEM_STATUS => 0,
        }
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
            3 => Val(INTID_A::RECEIVE_LINE_STATUS),
            2 => Val(INTID_A::RECEIVE_DATA_AVAILABLE),
            6 => Val(INTID_A::CHARACTER_TIMEOUT_INDICATOR),
            1 => Val(INTID_A::THRE_INTERRUPT),
            0 => Val(INTID_A::MODEM_STATUS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVE_LINE_STATUS`"]
    #[inline(always)]
    pub fn is_receive_line_status(&self) -> bool {
        *self == INTID_A::RECEIVE_LINE_STATUS
    }
    #[doc = "Checks if the value of the field is `RECEIVE_DATA_AVAILABLE`"]
    #[inline(always)]
    pub fn is_receive_data_available(&self) -> bool {
        *self == INTID_A::RECEIVE_DATA_AVAILABLE
    }
    #[doc = "Checks if the value of the field is `CHARACTER_TIMEOUT_INDICATOR`"]
    #[inline(always)]
    pub fn is_character_timeout_indicator(&self) -> bool {
        *self == INTID_A::CHARACTER_TIMEOUT_INDICATOR
    }
    #[doc = "Checks if the value of the field is `THRE_INTERRUPT`"]
    #[inline(always)]
    pub fn is_thre_interrupt(&self) -> bool {
        *self == INTID_A::THRE_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `MODEM_STATUS`"]
    #[inline(always)]
    pub fn is_modem_status(&self) -> bool {
        *self == INTID_A::MODEM_STATUS
    }
}
#[doc = "Reader of field `FIFOEN`"]
pub type FIFOEN_R = crate::R<u8, u8>;
#[doc = "Reader of field `ABEOINT`"]
pub type ABEOINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ABTOINT`"]
pub type ABTOINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt status. Note that IIR\\[0\\] is active low. The pending interrupt can be determined by evaluating IIR\\[3:1\\]."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification. IER\\[3:1\\] identifies an interrupt corresponding to the USART Rx FIFO. All other values of IER\\[3:1\\] not listed below are reserved."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - These bits are equivalent to FCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 6) & 0x03) as u8)
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
