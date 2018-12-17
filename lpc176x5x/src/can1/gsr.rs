#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::GSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBSR {
    #[doc = "Empty. No message is available."]
    EMPTY_NO_MESSAGE_IS,
    #[doc = "Full. At least one complete message is received by the Double Receive Buffer and available in the CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers. This bit is cleared by the Release Receive Buffer command in CANxCMR, if no subsequent received message is available."]
    FULL_AT_LEAST_ONE_C,
}
impl RBSR {
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
            RBSR::EMPTY_NO_MESSAGE_IS => false,
            RBSR::FULL_AT_LEAST_ONE_C => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBSR {
        match value {
            false => RBSR::EMPTY_NO_MESSAGE_IS,
            true => RBSR::FULL_AT_LEAST_ONE_C,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY_NO_MESSAGE_IS`"]
    #[inline]
    pub fn is_empty_no_message_is(&self) -> bool {
        *self == RBSR::EMPTY_NO_MESSAGE_IS
    }
    #[doc = "Checks if the value of the field is `FULL_AT_LEAST_ONE_C`"]
    #[inline]
    pub fn is_full_at_least_one_c(&self) -> bool {
        *self == RBSR::FULL_AT_LEAST_ONE_C
    }
}
#[doc = "Possible values of the field `DOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOSR {
    #[doc = "Absent. No data overrun has occurred since the last Clear Data Overrun command was given/written to CANxCMR (or since Reset)."]
    ABSENT_NO_DATA_OVER,
    #[doc = "Overrun. A message was lost because the preceding message to this CAN controller was not read and released quickly enough (there was not enough space for a new message in the Double Receive Buffer)."]
    OVERRUN_A_MESSAGE_W,
}
impl DOSR {
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
            DOSR::ABSENT_NO_DATA_OVER => false,
            DOSR::OVERRUN_A_MESSAGE_W => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOSR {
        match value {
            false => DOSR::ABSENT_NO_DATA_OVER,
            true => DOSR::OVERRUN_A_MESSAGE_W,
        }
    }
    #[doc = "Checks if the value of the field is `ABSENT_NO_DATA_OVER`"]
    #[inline]
    pub fn is_absent_no_data_over(&self) -> bool {
        *self == DOSR::ABSENT_NO_DATA_OVER
    }
    #[doc = "Checks if the value of the field is `OVERRUN_A_MESSAGE_W`"]
    #[inline]
    pub fn is_overrun_a_message_w(&self) -> bool {
        *self == DOSR::OVERRUN_A_MESSAGE_W
    }
}
#[doc = "Possible values of the field `TBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBSR {
    #[doc = "Locked. At least one of the Transmit Buffers is not available for the CPU, i.e. at least one previously queued message for this CAN controller has not yet been sent, and therefore software should not write to the CANxTFI, CANxTID, CANxTDA, nor CANxTDB registers of that (those) Tx buffer(s)."]
    LOCKED_AT_LEAST_ONE,
    #[doc = "Released. All three Transmit Buffers are available for the CPU. No transmit message is pending for this CAN controller (in any of the 3 Tx buffers), and software may write to any of the CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_ALL_THREE_,
}
impl TBSR {
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
            TBSR::LOCKED_AT_LEAST_ONE => false,
            TBSR::RELEASED_ALL_THREE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBSR {
        match value {
            false => TBSR::LOCKED_AT_LEAST_ONE,
            true => TBSR::RELEASED_ALL_THREE_,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_AT_LEAST_ONE`"]
    #[inline]
    pub fn is_locked_at_least_one(&self) -> bool {
        *self == TBSR::LOCKED_AT_LEAST_ONE
    }
    #[doc = "Checks if the value of the field is `RELEASED_ALL_THREE_`"]
    #[inline]
    pub fn is_released_all_three_(&self) -> bool {
        *self == TBSR::RELEASED_ALL_THREE_
    }
}
#[doc = "Possible values of the field `TCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCSR {
    #[doc = "Incomplete. At least one requested transmission has not been successfully completed yet."]
    INCOMPLETE_AT_LEAST,
    #[doc = "Complete. All requested transmission(s) has (have) been successfully completed."]
    COMPLETE_ALL_REQUES,
}
impl TCSR {
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
            TCSR::INCOMPLETE_AT_LEAST => false,
            TCSR::COMPLETE_ALL_REQUES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCSR {
        match value {
            false => TCSR::INCOMPLETE_AT_LEAST,
            true => TCSR::COMPLETE_ALL_REQUES,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_AT_LEAST`"]
    #[inline]
    pub fn is_incomplete_at_least(&self) -> bool {
        *self == TCSR::INCOMPLETE_AT_LEAST
    }
    #[doc = "Checks if the value of the field is `COMPLETE_ALL_REQUES`"]
    #[inline]
    pub fn is_complete_all_reques(&self) -> bool {
        *self == TCSR::COMPLETE_ALL_REQUES
    }
}
#[doc = "Possible values of the field `RS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSR {
    #[doc = "Idle. The CAN controller is idle."]
    IDLE_THE_CAN_CONTRO,
    #[doc = "Receive. The CAN controller is receiving a message."]
    RECEIVE_THE_CAN_CON,
}
impl RSR {
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
            RSR::IDLE_THE_CAN_CONTRO => false,
            RSR::RECEIVE_THE_CAN_CON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSR {
        match value {
            false => RSR::IDLE_THE_CAN_CONTRO,
            true => RSR::RECEIVE_THE_CAN_CON,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THE_CAN_CONTRO`"]
    #[inline]
    pub fn is_idle_the_can_contro(&self) -> bool {
        *self == RSR::IDLE_THE_CAN_CONTRO
    }
    #[doc = "Checks if the value of the field is `RECEIVE_THE_CAN_CON`"]
    #[inline]
    pub fn is_receive_the_can_con(&self) -> bool {
        *self == RSR::RECEIVE_THE_CAN_CON
    }
}
#[doc = "Possible values of the field `TS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSR {
    #[doc = "Idle. The CAN controller is idle."]
    IDLE_THE_CAN_CONTRO,
    #[doc = "Transmit. The CAN controller is sending a message."]
    TRANSMIT_THE_CAN_CO,
}
impl TSR {
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
            TSR::IDLE_THE_CAN_CONTRO => false,
            TSR::TRANSMIT_THE_CAN_CO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSR {
        match value {
            false => TSR::IDLE_THE_CAN_CONTRO,
            true => TSR::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THE_CAN_CONTRO`"]
    #[inline]
    pub fn is_idle_the_can_contro(&self) -> bool {
        *self == TSR::IDLE_THE_CAN_CONTRO
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == TSR::TRANSMIT_THE_CAN_CO
    }
}
#[doc = "Possible values of the field `ES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESR {
    #[doc = "OK. Both error counters are below the Error Warning Limit."]
    OK_BOTH_ERROR_COUNT,
    #[doc = "Error. One or both of the Transmit and Receive Error Counters has reached the limit set in the Error Warning Limit register."]
    ERROR_ONE_OR_BOTH_O,
}
impl ESR {
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
            ESR::OK_BOTH_ERROR_COUNT => false,
            ESR::ERROR_ONE_OR_BOTH_O => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESR {
        match value {
            false => ESR::OK_BOTH_ERROR_COUNT,
            true => ESR::ERROR_ONE_OR_BOTH_O,
        }
    }
    #[doc = "Checks if the value of the field is `OK_BOTH_ERROR_COUNT`"]
    #[inline]
    pub fn is_ok_both_error_count(&self) -> bool {
        *self == ESR::OK_BOTH_ERROR_COUNT
    }
    #[doc = "Checks if the value of the field is `ERROR_ONE_OR_BOTH_O`"]
    #[inline]
    pub fn is_error_one_or_both_o(&self) -> bool {
        *self == ESR::ERROR_ONE_OR_BOTH_O
    }
}
#[doc = "Possible values of the field `BS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSR {
    #[doc = "Bus-on. The CAN Controller is involved in bus activities"]
    BUS_ON_THE_CAN_CONT,
    #[doc = "Bus-off. The CAN controller is currently not involved/prohibited from bus activity because the Transmit Error Counter reached its limiting value of 255."]
    BUS_OFF_THE_CAN_CON,
}
impl BSR {
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
            BSR::BUS_ON_THE_CAN_CONT => false,
            BSR::BUS_OFF_THE_CAN_CON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BSR {
        match value {
            false => BSR::BUS_ON_THE_CAN_CONT,
            true => BSR::BUS_OFF_THE_CAN_CON,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_ON_THE_CAN_CONT`"]
    #[inline]
    pub fn is_bus_on_the_can_cont(&self) -> bool {
        *self == BSR::BUS_ON_THE_CAN_CONT
    }
    #[doc = "Checks if the value of the field is `BUS_OFF_THE_CAN_CON`"]
    #[inline]
    pub fn is_bus_off_the_can_con(&self) -> bool {
        *self == BSR::BUS_OFF_THE_CAN_CON
    }
}
#[doc = r" Value of the field"]
pub struct RXERRR {
    bits: u8,
}
impl RXERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXERRR {
    bits: u8,
}
impl TXERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receive Buffer Status. After reading all messages and releasing their memory space with the command 'Release Receive Buffer,' this bit is cleared."]
    #[inline]
    pub fn rbs(&self) -> RBSR {
        RBSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Data Overrun Status. If there is not enough space to store the message within the Receive Buffer, that message is dropped and the Data Overrun condition is signalled to the CPU in the moment this message becomes valid. If this message is not completed successfully (e.g. because of an error), no overrun condition is signalled."]
    #[inline]
    pub fn dos(&self) -> DOSR {
        DOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Transmit Buffer Status."]
    #[inline]
    pub fn tbs(&self) -> TBSR {
        TBSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit Complete Status. The Transmission Complete Status bit is set '0' (incomplete) whenever the Transmission Request bit or the Self Reception Request bit is set '1' at least for one of the three Transmit Buffers. The Transmission Complete Status bit will remain '0' until all messages are transmitted successfully."]
    #[inline]
    pub fn tcs(&self) -> TCSR {
        TCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Receive Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
    #[inline]
    pub fn rs(&self) -> RSR {
        RSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Transmit Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
    #[inline]
    pub fn ts(&self) -> TSR {
        TSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Error Status. Errors detected during reception or transmission will effect the error counters according to the CAN specification. The Error Status bit is set when at least one of the error counters has reached or exceeded the Error Warning Limit. An Error Warning Interrupt is generated, if enabled. The default value of the Error Warning Limit after hardware reset is 96 decimal, see also Section 21.7.7 CAN Error Warning Limit register (CAN1EWL - 0x4004 4018, CAN2EWL - 0x4004 8018)."]
    #[inline]
    pub fn es(&self) -> ESR {
        ESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Bus Status. Mode bit '1' (present) and an Error Warning Interrupt is generated, if enabled. Afterwards the Transmit Error Counter is set to '127', and the Receive Error Counter is cleared. It will stay in this mode until the CPU clears the Reset Mode bit. Once this is completed the CAN Controller will wait the minimum protocol-defined time (128 occurrences of the Bus-Free signal) counting down the Transmit Error Counter. After that, the Bus Status bit is cleared (Bus-On), the Error Status bit is set '0' (ok), the Error Counters are reset, and an Error Warning Interrupt is generated, if enabled. Reading the TX Error Counter during this time gives information about the status of the Bus-Off recovery."]
    #[inline]
    pub fn bs(&self) -> BSR {
        BSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - The current value of the Rx Error Counter (an 8-bit value)."]
    #[inline]
    pub fn rxerr(&self) -> RXERRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXERRR { bits }
    }
    #[doc = "Bits 24:31 - The current value of the Tx Error Counter (an 8-bit value)."]
    #[inline]
    pub fn txerr(&self) -> TXERRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXERRR { bits }
    }
}
