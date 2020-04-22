#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `RBS_1`"]
pub type RBS_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DOS_1`"]
pub type DOS_1_R = crate::R<bool, bool>;
#[doc = "Transmit Buffer Status 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBS1_1_A {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 1 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED_SOFTWARE_CAN = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 1 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_SOFTWARE_M = 1,
}
impl From<TBS1_1_A> for bool {
    #[inline(always)]
    fn from(variant: TBS1_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBS1_1`"]
pub type TBS1_1_R = crate::R<bool, TBS1_1_A>;
impl TBS1_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBS1_1_A {
        match self.bits {
            false => TBS1_1_A::LOCKED_SOFTWARE_CAN,
            true => TBS1_1_A::RELEASED_SOFTWARE_M,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`"]
    #[inline(always)]
    pub fn is_locked_software_can(&self) -> bool {
        *self == TBS1_1_A::LOCKED_SOFTWARE_CAN
    }
    #[doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`"]
    #[inline(always)]
    pub fn is_released_software_m(&self) -> bool {
        *self == TBS1_1_A::RELEASED_SOFTWARE_M
    }
}
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCS1_1_A {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 1 is not complete."]
    INCOMPLETE_THE_PREV = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 1 has been successfully completed."]
    COMPLETE_THE_PREVIO = 1,
}
impl From<TCS1_1_A> for bool {
    #[inline(always)]
    fn from(variant: TCS1_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCS1_1`"]
pub type TCS1_1_R = crate::R<bool, TCS1_1_A>;
impl TCS1_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCS1_1_A {
        match self.bits {
            false => TCS1_1_A::INCOMPLETE_THE_PREV,
            true => TCS1_1_A::COMPLETE_THE_PREVIO,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`"]
    #[inline(always)]
    pub fn is_incomplete_the_prev(&self) -> bool {
        *self == TCS1_1_A::INCOMPLETE_THE_PREV
    }
    #[doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`"]
    #[inline(always)]
    pub fn is_complete_the_previo(&self) -> bool {
        *self == TCS1_1_A::COMPLETE_THE_PREVIO
    }
}
#[doc = "Reader of field `RS_1`"]
pub type RS_1_R = crate::R<bool, bool>;
#[doc = "Transmit Status 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS1_1_A {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 1."]
    IDLE_THERE_IS_NO_TR = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 1."]
    TRANSMIT_THE_CAN_CO = 1,
}
impl From<TS1_1_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TS1_1`"]
pub type TS1_1_R = crate::R<bool, TS1_1_A>;
impl TS1_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS1_1_A {
        match self.bits {
            false => TS1_1_A::IDLE_THERE_IS_NO_TR,
            true => TS1_1_A::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`"]
    #[inline(always)]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        *self == TS1_1_A::IDLE_THERE_IS_NO_TR
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == TS1_1_A::TRANSMIT_THE_CAN_CO
    }
}
#[doc = "Reader of field `ES_1`"]
pub type ES_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BS_1`"]
pub type BS_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBS_2`"]
pub type RBS_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DOS_2`"]
pub type DOS_2_R = crate::R<bool, bool>;
#[doc = "Transmit Buffer Status 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBS2_2_A {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 2 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED_SOFTWARE_CAN = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 2 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_SOFTWARE_M = 1,
}
impl From<TBS2_2_A> for bool {
    #[inline(always)]
    fn from(variant: TBS2_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBS2_2`"]
pub type TBS2_2_R = crate::R<bool, TBS2_2_A>;
impl TBS2_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBS2_2_A {
        match self.bits {
            false => TBS2_2_A::LOCKED_SOFTWARE_CAN,
            true => TBS2_2_A::RELEASED_SOFTWARE_M,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`"]
    #[inline(always)]
    pub fn is_locked_software_can(&self) -> bool {
        *self == TBS2_2_A::LOCKED_SOFTWARE_CAN
    }
    #[doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`"]
    #[inline(always)]
    pub fn is_released_software_m(&self) -> bool {
        *self == TBS2_2_A::RELEASED_SOFTWARE_M
    }
}
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCS2_2_A {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 2 is not complete."]
    INCOMPLETE_THE_PREV = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 2 has been successfully completed."]
    COMPLETE_THE_PREVIO = 1,
}
impl From<TCS2_2_A> for bool {
    #[inline(always)]
    fn from(variant: TCS2_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCS2_2`"]
pub type TCS2_2_R = crate::R<bool, TCS2_2_A>;
impl TCS2_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCS2_2_A {
        match self.bits {
            false => TCS2_2_A::INCOMPLETE_THE_PREV,
            true => TCS2_2_A::COMPLETE_THE_PREVIO,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`"]
    #[inline(always)]
    pub fn is_incomplete_the_prev(&self) -> bool {
        *self == TCS2_2_A::INCOMPLETE_THE_PREV
    }
    #[doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`"]
    #[inline(always)]
    pub fn is_complete_the_previo(&self) -> bool {
        *self == TCS2_2_A::COMPLETE_THE_PREVIO
    }
}
#[doc = "Reader of field `RS_2`"]
pub type RS_2_R = crate::R<bool, bool>;
#[doc = "Transmit Status 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS2_2_A {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 2."]
    IDLE_THERE_IS_NO_TR = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 2."]
    TRANSMIT_THE_CAN_CO = 1,
}
impl From<TS2_2_A> for bool {
    #[inline(always)]
    fn from(variant: TS2_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TS2_2`"]
pub type TS2_2_R = crate::R<bool, TS2_2_A>;
impl TS2_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS2_2_A {
        match self.bits {
            false => TS2_2_A::IDLE_THERE_IS_NO_TR,
            true => TS2_2_A::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`"]
    #[inline(always)]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        *self == TS2_2_A::IDLE_THERE_IS_NO_TR
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == TS2_2_A::TRANSMIT_THE_CAN_CO
    }
}
#[doc = "Reader of field `ES_2`"]
pub type ES_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `BS_2`"]
pub type BS_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBS_3`"]
pub type RBS_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `DOS_3`"]
pub type DOS_3_R = crate::R<bool, bool>;
#[doc = "Transmit Buffer Status 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBS3_3_A {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 3 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED_SOFTWARE_CAN = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 3 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_SOFTWARE_M = 1,
}
impl From<TBS3_3_A> for bool {
    #[inline(always)]
    fn from(variant: TBS3_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBS3_3`"]
pub type TBS3_3_R = crate::R<bool, TBS3_3_A>;
impl TBS3_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBS3_3_A {
        match self.bits {
            false => TBS3_3_A::LOCKED_SOFTWARE_CAN,
            true => TBS3_3_A::RELEASED_SOFTWARE_M,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`"]
    #[inline(always)]
    pub fn is_locked_software_can(&self) -> bool {
        *self == TBS3_3_A::LOCKED_SOFTWARE_CAN
    }
    #[doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`"]
    #[inline(always)]
    pub fn is_released_software_m(&self) -> bool {
        *self == TBS3_3_A::RELEASED_SOFTWARE_M
    }
}
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCS3_3_A {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 3 is not complete."]
    INCOMPLETE_THE_PREV = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 3 has been successfully completed."]
    COMPLETE_THE_PREVIO = 1,
}
impl From<TCS3_3_A> for bool {
    #[inline(always)]
    fn from(variant: TCS3_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCS3_3`"]
pub type TCS3_3_R = crate::R<bool, TCS3_3_A>;
impl TCS3_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCS3_3_A {
        match self.bits {
            false => TCS3_3_A::INCOMPLETE_THE_PREV,
            true => TCS3_3_A::COMPLETE_THE_PREVIO,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`"]
    #[inline(always)]
    pub fn is_incomplete_the_prev(&self) -> bool {
        *self == TCS3_3_A::INCOMPLETE_THE_PREV
    }
    #[doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`"]
    #[inline(always)]
    pub fn is_complete_the_previo(&self) -> bool {
        *self == TCS3_3_A::COMPLETE_THE_PREVIO
    }
}
#[doc = "Reader of field `RS_3`"]
pub type RS_3_R = crate::R<bool, bool>;
#[doc = "Transmit Status 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS3_3_A {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 3."]
    IDLE_THERE_IS_NO_TR = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 3."]
    TRANSMIT_THE_CAN_CO = 1,
}
impl From<TS3_3_A> for bool {
    #[inline(always)]
    fn from(variant: TS3_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TS3_3`"]
pub type TS3_3_R = crate::R<bool, TS3_3_A>;
impl TS3_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS3_3_A {
        match self.bits {
            false => TS3_3_A::IDLE_THERE_IS_NO_TR,
            true => TS3_3_A::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`"]
    #[inline(always)]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        *self == TS3_3_A::IDLE_THERE_IS_NO_TR
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == TS3_3_A::TRANSMIT_THE_CAN_CO
    }
}
#[doc = "Reader of field `ES_3`"]
pub type ES_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `BS_3`"]
pub type BS_3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_1(&self) -> RBS_1_R {
        RBS_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_1(&self) -> DOS_1_R {
        DOS_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Status 1."]
    #[inline(always)]
    pub fn tbs1_1(&self) -> TBS1_1_R {
        TBS1_1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs1_1(&self) -> TCS1_1_R {
        TCS1_1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_1(&self) -> RS_1_R {
        RS_1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Status 1."]
    #[inline(always)]
    pub fn ts1_1(&self) -> TS1_1_R {
        TS1_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_1(&self) -> ES_1_R {
        ES_1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_1(&self) -> BS_1_R {
        BS_1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_2(&self) -> RBS_2_R {
        RBS_2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_2(&self) -> DOS_2_R {
        DOS_2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Buffer Status 2."]
    #[inline(always)]
    pub fn tbs2_2(&self) -> TBS2_2_R {
        TBS2_2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs2_2(&self) -> TCS2_2_R {
        TCS2_2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_2(&self) -> RS_2_R {
        RS_2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Status 2."]
    #[inline(always)]
    pub fn ts2_2(&self) -> TS2_2_R {
        TS2_2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_2(&self) -> ES_2_R {
        ES_2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_2(&self) -> BS_2_R {
        BS_2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_3(&self) -> RBS_3_R {
        RBS_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_3(&self) -> DOS_3_R {
        DOS_3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit Buffer Status 3."]
    #[inline(always)]
    pub fn tbs3_3(&self) -> TBS3_3_R {
        TBS3_3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs3_3(&self) -> TCS3_3_R {
        TCS3_3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_3(&self) -> RS_3_R {
        RS_3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmit Status 3."]
    #[inline(always)]
    pub fn ts3_3(&self) -> TS3_3_R {
        TS3_3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_3(&self) -> ES_3_R {
        ES_3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_3(&self) -> BS_3_R {
        BS_3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
