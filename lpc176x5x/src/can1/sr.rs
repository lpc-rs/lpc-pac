#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RBS_1R {
    bits: bool,
}
impl RBS_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DOS_1R {
    bits: bool,
}
impl DOS_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `TBS1_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBS1_1R {
    #[doc = "Locked. Software cannot access the Tx Buffer 1 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED_SOFTWARE_CAN,
    #[doc = "Released. Software may write a message into the Transmit Buffer 1 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_SOFTWARE_M,
}
impl TBS1_1R {
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
            TBS1_1R::LOCKED_SOFTWARE_CAN => false,
            TBS1_1R::RELEASED_SOFTWARE_M => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBS1_1R {
        match value {
            false => TBS1_1R::LOCKED_SOFTWARE_CAN,
            true => TBS1_1R::RELEASED_SOFTWARE_M,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`"]
    #[inline]
    pub fn is_locked_software_can(&self) -> bool {
        *self == TBS1_1R::LOCKED_SOFTWARE_CAN
    }
    #[doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`"]
    #[inline]
    pub fn is_released_software_m(&self) -> bool {
        *self == TBS1_1R::RELEASED_SOFTWARE_M
    }
}
#[doc = "Possible values of the field `TCS1_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCS1_1R {
    #[doc = "Incomplete. The previously requested transmission for Tx Buffer 1 is not complete."]
    INCOMPLETE_THE_PREV,
    #[doc = "Complete. The previously requested transmission for Tx Buffer 1 has been successfully completed."]
    COMPLETE_THE_PREVIO,
}
impl TCS1_1R {
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
            TCS1_1R::INCOMPLETE_THE_PREV => false,
            TCS1_1R::COMPLETE_THE_PREVIO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCS1_1R {
        match value {
            false => TCS1_1R::INCOMPLETE_THE_PREV,
            true => TCS1_1R::COMPLETE_THE_PREVIO,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`"]
    #[inline]
    pub fn is_incomplete_the_prev(&self) -> bool {
        *self == TCS1_1R::INCOMPLETE_THE_PREV
    }
    #[doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`"]
    #[inline]
    pub fn is_complete_the_previo(&self) -> bool {
        *self == TCS1_1R::COMPLETE_THE_PREVIO
    }
}
#[doc = r" Value of the field"]
pub struct RS_1R {
    bits: bool,
}
impl RS_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `TS1_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS1_1R {
    #[doc = "Idle. There is no transmission from Tx Buffer 1."]
    IDLE_THERE_IS_NO_TR,
    #[doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 1."]
    TRANSMIT_THE_CAN_CO,
}
impl TS1_1R {
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
            TS1_1R::IDLE_THERE_IS_NO_TR => false,
            TS1_1R::TRANSMIT_THE_CAN_CO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TS1_1R {
        match value {
            false => TS1_1R::IDLE_THERE_IS_NO_TR,
            true => TS1_1R::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`"]
    #[inline]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        *self == TS1_1R::IDLE_THERE_IS_NO_TR
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == TS1_1R::TRANSMIT_THE_CAN_CO
    }
}
#[doc = r" Value of the field"]
pub struct ES_1R {
    bits: bool,
}
impl ES_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct BS_1R {
    bits: bool,
}
impl BS_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct RBS_2R {
    bits: bool,
}
impl RBS_2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DOS_2R {
    bits: bool,
}
impl DOS_2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `TBS2_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBS2_2R {
    #[doc = "Locked. Software cannot access the Tx Buffer 2 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED_SOFTWARE_CAN,
    #[doc = "Released. Software may write a message into the Transmit Buffer 2 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_SOFTWARE_M,
}
impl TBS2_2R {
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
            TBS2_2R::LOCKED_SOFTWARE_CAN => false,
            TBS2_2R::RELEASED_SOFTWARE_M => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBS2_2R {
        match value {
            false => TBS2_2R::LOCKED_SOFTWARE_CAN,
            true => TBS2_2R::RELEASED_SOFTWARE_M,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`"]
    #[inline]
    pub fn is_locked_software_can(&self) -> bool {
        *self == TBS2_2R::LOCKED_SOFTWARE_CAN
    }
    #[doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`"]
    #[inline]
    pub fn is_released_software_m(&self) -> bool {
        *self == TBS2_2R::RELEASED_SOFTWARE_M
    }
}
#[doc = "Possible values of the field `TCS2_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCS2_2R {
    #[doc = "Incomplete. The previously requested transmission for Tx Buffer 2 is not complete."]
    INCOMPLETE_THE_PREV,
    #[doc = "Complete. The previously requested transmission for Tx Buffer 2 has been successfully completed."]
    COMPLETE_THE_PREVIO,
}
impl TCS2_2R {
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
            TCS2_2R::INCOMPLETE_THE_PREV => false,
            TCS2_2R::COMPLETE_THE_PREVIO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCS2_2R {
        match value {
            false => TCS2_2R::INCOMPLETE_THE_PREV,
            true => TCS2_2R::COMPLETE_THE_PREVIO,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`"]
    #[inline]
    pub fn is_incomplete_the_prev(&self) -> bool {
        *self == TCS2_2R::INCOMPLETE_THE_PREV
    }
    #[doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`"]
    #[inline]
    pub fn is_complete_the_previo(&self) -> bool {
        *self == TCS2_2R::COMPLETE_THE_PREVIO
    }
}
#[doc = r" Value of the field"]
pub struct RS_2R {
    bits: bool,
}
impl RS_2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `TS2_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS2_2R {
    #[doc = "Idle. There is no transmission from Tx Buffer 2."]
    IDLE_THERE_IS_NO_TR,
    #[doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 2."]
    TRANSMIT_THE_CAN_CO,
}
impl TS2_2R {
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
            TS2_2R::IDLE_THERE_IS_NO_TR => false,
            TS2_2R::TRANSMIT_THE_CAN_CO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TS2_2R {
        match value {
            false => TS2_2R::IDLE_THERE_IS_NO_TR,
            true => TS2_2R::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`"]
    #[inline]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        *self == TS2_2R::IDLE_THERE_IS_NO_TR
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == TS2_2R::TRANSMIT_THE_CAN_CO
    }
}
#[doc = r" Value of the field"]
pub struct ES_2R {
    bits: bool,
}
impl ES_2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct BS_2R {
    bits: bool,
}
impl BS_2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct RBS_3R {
    bits: bool,
}
impl RBS_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DOS_3R {
    bits: bool,
}
impl DOS_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `TBS3_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBS3_3R {
    #[doc = "Locked. Software cannot access the Tx Buffer 3 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED_SOFTWARE_CAN,
    #[doc = "Released. Software may write a message into the Transmit Buffer 3 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED_SOFTWARE_M,
}
impl TBS3_3R {
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
            TBS3_3R::LOCKED_SOFTWARE_CAN => false,
            TBS3_3R::RELEASED_SOFTWARE_M => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBS3_3R {
        match value {
            false => TBS3_3R::LOCKED_SOFTWARE_CAN,
            true => TBS3_3R::RELEASED_SOFTWARE_M,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED_SOFTWARE_CAN`"]
    #[inline]
    pub fn is_locked_software_can(&self) -> bool {
        *self == TBS3_3R::LOCKED_SOFTWARE_CAN
    }
    #[doc = "Checks if the value of the field is `RELEASED_SOFTWARE_M`"]
    #[inline]
    pub fn is_released_software_m(&self) -> bool {
        *self == TBS3_3R::RELEASED_SOFTWARE_M
    }
}
#[doc = "Possible values of the field `TCS3_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCS3_3R {
    #[doc = "Incomplete. The previously requested transmission for Tx Buffer 3 is not complete."]
    INCOMPLETE_THE_PREV,
    #[doc = "Complete. The previously requested transmission for Tx Buffer 3 has been successfully completed."]
    COMPLETE_THE_PREVIO,
}
impl TCS3_3R {
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
            TCS3_3R::INCOMPLETE_THE_PREV => false,
            TCS3_3R::COMPLETE_THE_PREVIO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCS3_3R {
        match value {
            false => TCS3_3R::INCOMPLETE_THE_PREV,
            true => TCS3_3R::COMPLETE_THE_PREVIO,
        }
    }
    #[doc = "Checks if the value of the field is `INCOMPLETE_THE_PREV`"]
    #[inline]
    pub fn is_incomplete_the_prev(&self) -> bool {
        *self == TCS3_3R::INCOMPLETE_THE_PREV
    }
    #[doc = "Checks if the value of the field is `COMPLETE_THE_PREVIO`"]
    #[inline]
    pub fn is_complete_the_previo(&self) -> bool {
        *self == TCS3_3R::COMPLETE_THE_PREVIO
    }
}
#[doc = r" Value of the field"]
pub struct RS_3R {
    bits: bool,
}
impl RS_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `TS3_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS3_3R {
    #[doc = "Idle. There is no transmission from Tx Buffer 3."]
    IDLE_THERE_IS_NO_TR,
    #[doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 3."]
    TRANSMIT_THE_CAN_CO,
}
impl TS3_3R {
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
            TS3_3R::IDLE_THERE_IS_NO_TR => false,
            TS3_3R::TRANSMIT_THE_CAN_CO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TS3_3R {
        match value {
            false => TS3_3R::IDLE_THERE_IS_NO_TR,
            true => TS3_3R::TRANSMIT_THE_CAN_CO,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_THERE_IS_NO_TR`"]
    #[inline]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        *self == TS3_3R::IDLE_THERE_IS_NO_TR
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_THE_CAN_CO`"]
    #[inline]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == TS3_3R::TRANSMIT_THE_CAN_CO
    }
}
#[doc = r" Value of the field"]
pub struct ES_3R {
    bits: bool,
}
impl ES_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct BS_3R {
    bits: bool,
}
impl BS_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline]
    pub fn rbs_1(&self) -> RBS_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RBS_1R { bits }
    }
    #[doc = "Bit 1 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline]
    pub fn dos_1(&self) -> DOS_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DOS_1R { bits }
    }
    #[doc = "Bit 2 - Transmit Buffer Status 1."]
    #[inline]
    pub fn tbs1_1(&self) -> TBS1_1R {
        TBS1_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmission Complete Status."]
    #[inline]
    pub fn tcs1_1(&self) -> TCS1_1R {
        TCS1_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline]
    pub fn rs_1(&self) -> RS_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RS_1R { bits }
    }
    #[doc = "Bit 5 - Transmit Status 1."]
    #[inline]
    pub fn ts1_1(&self) -> TS1_1R {
        TS1_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline]
    pub fn es_1(&self) -> ES_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ES_1R { bits }
    }
    #[doc = "Bit 7 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline]
    pub fn bs_1(&self) -> BS_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BS_1R { bits }
    }
    #[doc = "Bit 8 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline]
    pub fn rbs_2(&self) -> RBS_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RBS_2R { bits }
    }
    #[doc = "Bit 9 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline]
    pub fn dos_2(&self) -> DOS_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DOS_2R { bits }
    }
    #[doc = "Bit 10 - Transmit Buffer Status 2."]
    #[inline]
    pub fn tbs2_2(&self) -> TBS2_2R {
        TBS2_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmission Complete Status."]
    #[inline]
    pub fn tcs2_2(&self) -> TCS2_2R {
        TCS2_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline]
    pub fn rs_2(&self) -> RS_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RS_2R { bits }
    }
    #[doc = "Bit 13 - Transmit Status 2."]
    #[inline]
    pub fn ts2_2(&self) -> TS2_2R {
        TS2_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline]
    pub fn es_2(&self) -> ES_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ES_2R { bits }
    }
    #[doc = "Bit 15 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline]
    pub fn bs_2(&self) -> BS_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BS_2R { bits }
    }
    #[doc = "Bit 16 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline]
    pub fn rbs_3(&self) -> RBS_3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RBS_3R { bits }
    }
    #[doc = "Bit 17 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline]
    pub fn dos_3(&self) -> DOS_3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DOS_3R { bits }
    }
    #[doc = "Bit 18 - Transmit Buffer Status 3."]
    #[inline]
    pub fn tbs3_3(&self) -> TBS3_3R {
        TBS3_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Transmission Complete Status."]
    #[inline]
    pub fn tcs3_3(&self) -> TCS3_3R {
        TCS3_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline]
    pub fn rs_3(&self) -> RS_3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RS_3R { bits }
    }
    #[doc = "Bit 21 - Transmit Status 3."]
    #[inline]
    pub fn ts3_3(&self) -> TS3_3R {
        TS3_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline]
    pub fn es_3(&self) -> ES_3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ES_3R { bits }
    }
    #[doc = "Bit 23 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline]
    pub fn bs_3(&self) -> BS_3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BS_3R { bits }
    }
}
