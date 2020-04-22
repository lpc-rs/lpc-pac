#[doc = "Reader of register GSR"]
pub type R = crate::R<u32, super::GSR>;
#[doc = "Receive Buffer Status. After reading all messages and releasing their memory space with the command 'Release Receive Buffer,' this bit is cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBS_A {
    #[doc = "0: Empty. No message is available."]
    EMPTY_NO_MESSAGE_IS = 0,
    #[doc = "1: Full. At least one complete message is received by the Double Receive Buffer and available in the CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers. This bit is cleared by the Release Receive Buffer command in CANxCMR, if no subsequent received message is available."]
    FULL_AT_LEAST_ONE_C = 1,
}
impl From<RBS_A> for bool {
    #[inline(always)]
    fn from(variant: RBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RBS`"]
pub type RBS_R = crate::R<bool, RBS_A>;
impl RBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBS_A {
        match self.bits {
            false => RBS_A::EMPTY_NO_MESSAGE_IS,
            true => RBS_A::FULL_AT_LEAST_ONE_C,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY_NO_MESSAGE_IS`"]
    #[inline(always)]
    pub fn is_empty_no_message_is(&self) -> bool {
        *self == RBS_A::EMPTY_NO_MESSAGE_IS
    }
    #[doc = "Checks if the value of the field is `FULL_AT_LEAST_ONE_C`"]
    #[inline(always)]
    pub fn is_full_at_least_one_c(&self) -> bool {
        *self == RBS_A::FULL_AT_LEAST_ONE_C
    }
}
#[doc = "Data Overrun Status. If there is not enough space to store the message within the Receive Buffer, that message is dropped and the Data Overrun condition is signalled to the CPU in the moment this message becomes valid. If this message is not completed successfully (e.g. because of an error), no overrun condition is signalled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOS_A {
    #[doc = "0: Absent. No data overrun has occurred since the last Clear Data Overrun command was given/written to CANxCMR (or since Reset)."]
    ABSENT_NO_DATA_OVER = 0,
    #[doc = "1: Overrun. A message was lost because the preceding message to this CAN controller was not read and released quickly enough (there was not enough space for a new message in the Double Receive Buffer)."]
    OVERRUN_A_MESSAGE_W = 1,
}
impl From<DOS_A> for bool {
    #[inline(always)]
    fn from(variant: DOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOS`"]
pub type DOS_R = crate::R<bool, DOS_A>;
impl DOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOS_A {
        match self.bits {
            false => DOS_A::ABSENT_NO_DATA_OVER,
            true => DOS_A::OVERRUN_A_MESSAGE_W,
        }
    }
    #[doc = "Checks if the value of the field is `ABSENT_NO_DATA_OVER`"]
    #[inline(always)]
    pub fn is_absent_no_data_over(&self) -> bool {
        *self == DOS_A::ABSENT_NO_DATA_OVER
    }
    #[doc = "Checks if the value of the field is `OVERRUN_A_MESSAGE_W`"]
    #[inline(always)]
    pub fn is_overrun_a_message_w(&self) -> bool {
        *self == DOS_A::OVERRUN_A_MESSAGE_W
    }
}
#[doc = "Transmit Buffer Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBS_A {
    #[doc = "0: Locked. At least one of the Transmit Buffers is not available for the CPU, i.e. at least one previously queued message for this CAN controller has not yet been sent, and therefore software should not write to the CANxTFI, CANxTID, CANxTDA, nor CANxTDB registers of that (those) Tx buffer(s)."]
    LOCKED_AT_LEAST_ONE = 0,
    #[doc = "1: Released. All three Transmit Buffers are available for the CPU. No transmit message is pending for this CAN controller (in any of the 3 Tx buffers), and software may write to any of the CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_ALL_THREE_ = 1,
}
impl From<TBS_A> for bool {
    #[inline(always)]
    fn from(variant: TBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBS`"]
pub type TBS_R = crate::R<bool, TBS_A>;
impl TBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBS_A {
        match self.bits {
            false => TBS_A::LOCKED_AT_LEAST_ONE,
            true => TBS_A::RELEASED_ALL_THREE_,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_AT_LEAST_ONE`"]
    #[inline(always)]
    pub fn is_locked_at_least_one(&self) -> bool {
        *self == TBS_A::LOCKED_AT_LEAST_ONE
    }
    #[doc = "Checks if the value of the field is `RELEASED_ALL_THREE_`"]
    #[inline(always)]
    pub fn is_released_all_three_(&self) -> bool {
        *self == TBS_A::RELEASED_ALL_THREE_
    }
}
#[doc = "Transmit Complete Status. The Transmission Complete Status bit is set '0' (incomplete) whenever the Transmission Request bit or the Self Reception Request bit is set '1' at least for one of the three Transmit Buffers. The Transmission Complete Status bit will remain '0' until all messages are transmitted successfully.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCS_A {
    #[doc = "0: Incomplete. At least one requested transmission has not been successfully completed yet."]
    INCOMPLETE_AT_LEAST = 0,
    #[doc = "1: Complete. All requested transmission(s) has (have) been successfully completed."]
    COMPLETE_ALL_REQUES = 1,
}
impl From<TCS_A> for bool {
    #[inline(always)]
    fn from(variant: TCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCS`"]
pub type TCS_R = crate::R<bool, TCS_A>;
impl TCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCS_A {
        match self.bits {
            false => TCS_A::INCOMPLETE_AT_LEAST,
            true => TCS_A::COMPLETE_ALL_REQUES,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_AT_LEAST`"]
    #[inline(always)]
    pub fn is_incomplete_at_least(&self) -> bool {
        *self == TCS_A::INCOMPLETE_AT_LEAST
    }
    #[doc = "Checks if the value of the field is `COMPLETE_ALL_REQUES`"]
    #[inline(always)]
    pub fn is_complete_all_reques(&self) -> bool {
        *self == TCS_A::COMPLETE_ALL_REQUES
    }
}
#[doc = "Receive Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RS_A {
    #[doc = "0: Idle. The CAN controller is idle."]
    IDLE_THE_CAN_CONTRO = 0,
    #[doc = "1: Receive. The CAN controller is receiving a message."]
    RECEIVE_THE_CAN_CON = 1,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RS`"]
pub type RS_R = crate::R<bool, RS_A>;
impl RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS_A {
        match self.bits {
            false => RS_A::IDLE_THE_CAN_CONTRO,
            true => RS_A::RECEIVE_THE_CAN_CON,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THE_CAN_CONTRO`"]
    #[inline(always)]
    pub fn is_idle_the_can_contro(&self) -> bool {
        *self == RS_A::IDLE_THE_CAN_CONTRO
    }
    #[doc = "Checks if the value of the field is `RECEIVE_THE_CAN_CON`"]
    #[inline(always)]
    pub fn is_receive_the_can_con(&self) -> bool {
        *self == RS_A::RECEIVE_THE_CAN_CON
    }
}
#[doc = "Transmit Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS_A {
    #[doc = "0: Idle. The CAN controller is idle."]
    IDLE_THE_CAN_CONTRO = 0,
    #[doc = "1: Transmit. The CAN controller is sending a message."]
    TRANSMIT_THE_CAN_CO = 1,
}
impl From<TS_A> for bool {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TS`"]
pub type TS_R = crate::R<bool, TS_A>;
impl TS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS_A {
        match self.bits {
            false => TS_A::IDLE_THE_CAN_CONTRO,
            true => TS_A::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THE_CAN_CONTRO`"]
    #[inline(always)]
    pub fn is_idle_the_can_contro(&self) -> bool {
        *self == TS_A::IDLE_THE_CAN_CONTRO
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == TS_A::TRANSMIT_THE_CAN_CO
    }
}
#[doc = "Error Status. Errors detected during reception or transmission will effect the error counters according to the CAN specification. The Error Status bit is set when at least one of the error counters has reached or exceeded the Error Warning Limit. An Error Warning Interrupt is generated, if enabled. The default value of the Error Warning Limit after hardware reset is 96 decimal, see also Section 21.7.7 CAN Error Warning Limit register (CAN1EWL - 0x4004 4018, CAN2EWL - 0x4004 8018).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ES_A {
    #[doc = "0: OK. Both error counters are below the Error Warning Limit."]
    OK_BOTH_ERROR_COUNT = 0,
    #[doc = "1: Error. One or both of the Transmit and Receive Error Counters has reached the limit set in the Error Warning Limit register."]
    ERROR_ONE_OR_BOTH_O = 1,
}
impl From<ES_A> for bool {
    #[inline(always)]
    fn from(variant: ES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ES`"]
pub type ES_R = crate::R<bool, ES_A>;
impl ES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ES_A {
        match self.bits {
            false => ES_A::OK_BOTH_ERROR_COUNT,
            true => ES_A::ERROR_ONE_OR_BOTH_O,
        }
    }
    #[doc = "Checks if the value of the field is `OK_BOTH_ERROR_COUNT`"]
    #[inline(always)]
    pub fn is_ok_both_error_count(&self) -> bool {
        *self == ES_A::OK_BOTH_ERROR_COUNT
    }
    #[doc = "Checks if the value of the field is `ERROR_ONE_OR_BOTH_O`"]
    #[inline(always)]
    pub fn is_error_one_or_both_o(&self) -> bool {
        *self == ES_A::ERROR_ONE_OR_BOTH_O
    }
}
#[doc = "Bus Status. Mode bit '1' (present) and an Error Warning Interrupt is generated, if enabled. Afterwards the Transmit Error Counter is set to '127', and the Receive Error Counter is cleared. It will stay in this mode until the CPU clears the Reset Mode bit. Once this is completed the CAN Controller will wait the minimum protocol-defined time (128 occurrences of the Bus-Free signal) counting down the Transmit Error Counter. After that, the Bus Status bit is cleared (Bus-On), the Error Status bit is set '0' (ok), the Error Counters are reset, and an Error Warning Interrupt is generated, if enabled. Reading the TX Error Counter during this time gives information about the status of the Bus-Off recovery.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BS_A {
    #[doc = "0: Bus-on. The CAN Controller is involved in bus activities"]
    BUS_ON_THE_CAN_CONT = 0,
    #[doc = "1: Bus-off. The CAN controller is currently not involved/prohibited from bus activity because the Transmit Error Counter reached its limiting value of 255."]
    BUS_OFF_THE_CAN_CON = 1,
}
impl From<BS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BS`"]
pub type BS_R = crate::R<bool, BS_A>;
impl BS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BS_A {
        match self.bits {
            false => BS_A::BUS_ON_THE_CAN_CONT,
            true => BS_A::BUS_OFF_THE_CAN_CON,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_ON_THE_CAN_CONT`"]
    #[inline(always)]
    pub fn is_bus_on_the_can_cont(&self) -> bool {
        *self == BS_A::BUS_ON_THE_CAN_CONT
    }
    #[doc = "Checks if the value of the field is `BUS_OFF_THE_CAN_CON`"]
    #[inline(always)]
    pub fn is_bus_off_the_can_con(&self) -> bool {
        *self == BS_A::BUS_OFF_THE_CAN_CON
    }
}
#[doc = "Reader of field `RXERR`"]
pub type RXERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXERR`"]
pub type TXERR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Receive Buffer Status. After reading all messages and releasing their memory space with the command 'Release Receive Buffer,' this bit is cleared."]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Overrun Status. If there is not enough space to store the message within the Receive Buffer, that message is dropped and the Data Overrun condition is signalled to the CPU in the moment this message becomes valid. If this message is not completed successfully (e.g. because of an error), no overrun condition is signalled."]
    #[inline(always)]
    pub fn dos(&self) -> DOS_R {
        DOS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Status."]
    #[inline(always)]
    pub fn tbs(&self) -> TBS_R {
        TBS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Complete Status. The Transmission Complete Status bit is set '0' (incomplete) whenever the Transmission Request bit or the Self Reception Request bit is set '1' at least for one of the three Transmit Buffers. The Transmission Complete Status bit will remain '0' until all messages are transmitted successfully."]
    #[inline(always)]
    pub fn tcs(&self) -> TCS_R {
        TCS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error Status. Errors detected during reception or transmission will effect the error counters according to the CAN specification. The Error Status bit is set when at least one of the error counters has reached or exceeded the Error Warning Limit. An Error Warning Interrupt is generated, if enabled. The default value of the Error Warning Limit after hardware reset is 96 decimal, see also Section 21.7.7 CAN Error Warning Limit register (CAN1EWL - 0x4004 4018, CAN2EWL - 0x4004 8018)."]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus Status. Mode bit '1' (present) and an Error Warning Interrupt is generated, if enabled. Afterwards the Transmit Error Counter is set to '127', and the Receive Error Counter is cleared. It will stay in this mode until the CPU clears the Reset Mode bit. Once this is completed the CAN Controller will wait the minimum protocol-defined time (128 occurrences of the Bus-Free signal) counting down the Transmit Error Counter. After that, the Bus Status bit is cleared (Bus-On), the Error Status bit is set '0' (ok), the Error Counters are reset, and an Error Warning Interrupt is generated, if enabled. Reading the TX Error Counter during this time gives information about the status of the Bus-Off recovery."]
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - The current value of the Rx Error Counter (an 8-bit value)."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The current value of the Tx Error Counter (an 8-bit value)."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
