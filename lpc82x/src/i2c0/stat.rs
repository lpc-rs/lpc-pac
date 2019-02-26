#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `MSTPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDINGR {
    #[doc = "In progress. Communication is in progress and the Master function is busy and cannot currently accept a command."]
    IN_PROGRESS,
    #[doc = "Pending. The Master function needs software service or is in the idle state. If the master is not in the idle state, it is waiting to receive or transmit data or the NACK bit."]
    PENDING,
}
impl MSTPENDINGR {
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
            MSTPENDINGR::IN_PROGRESS => false,
            MSTPENDINGR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTPENDINGR {
        match value {
            false => MSTPENDINGR::IN_PROGRESS,
            true => MSTPENDINGR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline]
    pub fn is_in_progress(&self) -> bool {
        *self == MSTPENDINGR::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == MSTPENDINGR::PENDING
    }
}
#[doc = "Possible values of the field `MSTSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTATER {
    #[doc = "Idle. The Master function is available to be used for a new transaction."]
    IDLE,
    #[doc = "Receive ready. Received data  available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    RECEIVE_READY,
    #[doc = "Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    TRANSMIT_READY,
    #[doc = "NACK Address. Slave NACKed address."]
    NACK_ADDRESS,
    #[doc = "NACK Data. Slave NACKed transmitted data."]
    NACK_DATA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MSTSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSTSTATER::IDLE => 0,
            MSTSTATER::RECEIVE_READY => 1,
            MSTSTATER::TRANSMIT_READY => 2,
            MSTSTATER::NACK_ADDRESS => 3,
            MSTSTATER::NACK_DATA => 4,
            MSTSTATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSTSTATER {
        match value {
            0 => MSTSTATER::IDLE,
            1 => MSTSTATER::RECEIVE_READY,
            2 => MSTSTATER::TRANSMIT_READY,
            3 => MSTSTATER::NACK_ADDRESS,
            4 => MSTSTATER::NACK_DATA,
            i => MSTSTATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == MSTSTATER::IDLE
    }
    #[doc = "Checks if the value of the field is `RECEIVE_READY`"]
    #[inline]
    pub fn is_receive_ready(&self) -> bool {
        *self == MSTSTATER::RECEIVE_READY
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_READY`"]
    #[inline]
    pub fn is_transmit_ready(&self) -> bool {
        *self == MSTSTATER::TRANSMIT_READY
    }
    #[doc = "Checks if the value of the field is `NACK_ADDRESS`"]
    #[inline]
    pub fn is_nack_address(&self) -> bool {
        *self == MSTSTATER::NACK_ADDRESS
    }
    #[doc = "Checks if the value of the field is `NACK_DATA`"]
    #[inline]
    pub fn is_nack_data(&self) -> bool {
        *self == MSTSTATER::NACK_DATA
    }
}
#[doc = "Possible values of the field `MSTARBLOSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSR {
    #[doc = "No loss. No Arbitration Loss has occurred."]
    NO_LOSS,
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    ARBITRATION_LOSS,
}
impl MSTARBLOSSR {
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
            MSTARBLOSSR::NO_LOSS => false,
            MSTARBLOSSR::ARBITRATION_LOSS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTARBLOSSR {
        match value {
            false => MSTARBLOSSR::NO_LOSS,
            true => MSTARBLOSSR::ARBITRATION_LOSS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_LOSS`"]
    #[inline]
    pub fn is_no_loss(&self) -> bool {
        *self == MSTARBLOSSR::NO_LOSS
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOSS`"]
    #[inline]
    pub fn is_arbitration_loss(&self) -> bool {
        *self == MSTARBLOSSR::ARBITRATION_LOSS
    }
}
#[doc = "Possible values of the field `MSTSTSTPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERRR {
    #[doc = "No Start/Stop Error has occurred."]
    NO_ERROR,
    #[doc = "Start/stop error has occurred. The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    STARTSTOP_ERROR,
}
impl MSTSTSTPERRR {
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
            MSTSTSTPERRR::NO_ERROR => false,
            MSTSTSTPERRR::STARTSTOP_ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTSTSTPERRR {
        match value {
            false => MSTSTSTPERRR::NO_ERROR,
            true => MSTSTSTPERRR::STARTSTOP_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == MSTSTSTPERRR::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `STARTSTOP_ERROR`"]
    #[inline]
    pub fn is_startstop_error(&self) -> bool {
        *self == MSTSTSTPERRR::STARTSTOP_ERROR
    }
}
#[doc = "Possible values of the field `SLVPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDINGR {
    #[doc = "In progress. The Slave function does not currently need service."]
    IN_PROGRESS,
    #[doc = "Pending. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    PENDING,
}
impl SLVPENDINGR {
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
            SLVPENDINGR::IN_PROGRESS => false,
            SLVPENDINGR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVPENDINGR {
        match value {
            false => SLVPENDINGR::IN_PROGRESS,
            true => SLVPENDINGR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline]
    pub fn is_in_progress(&self) -> bool {
        *self == SLVPENDINGR::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == SLVPENDINGR::PENDING
    }
}
#[doc = "Possible values of the field `SLVSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVSTATER {
    #[doc = "Slave address. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    SLAVE_ADDRESS,
    #[doc = "Slave receive. Received data is available (Slave Receiver mode)."]
    SLAVE_RECEIVE,
    #[doc = "Slave transmit. Data can be transmitted (Slave Transmitter mode)."]
    SLAVE_TRANSMIT,
}
impl SLVSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLVSTATER::SLAVE_ADDRESS => 0,
            SLVSTATER::SLAVE_RECEIVE => 1,
            SLVSTATER::SLAVE_TRANSMIT => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLVSTATER {
        match value {
            0 => SLVSTATER::SLAVE_ADDRESS,
            1 => SLVSTATER::SLAVE_RECEIVE,
            2 => SLVSTATER::SLAVE_TRANSMIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS`"]
    #[inline]
    pub fn is_slave_address(&self) -> bool {
        *self == SLVSTATER::SLAVE_ADDRESS
    }
    #[doc = "Checks if the value of the field is `SLAVE_RECEIVE`"]
    #[inline]
    pub fn is_slave_receive(&self) -> bool {
        *self == SLVSTATER::SLAVE_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SLAVE_TRANSMIT`"]
    #[inline]
    pub fn is_slave_transmit(&self) -> bool {
        *self == SLVSTATER::SLAVE_TRANSMIT
    }
}
#[doc = "Possible values of the field `SLVNOTSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTRR {
    #[doc = "Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    STRETCHING,
    #[doc = "Not stretching. The slave function is not currently stretching the I2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    NOT_STRETCHING,
}
impl SLVNOTSTRR {
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
            SLVNOTSTRR::STRETCHING => false,
            SLVNOTSTRR::NOT_STRETCHING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVNOTSTRR {
        match value {
            false => SLVNOTSTRR::STRETCHING,
            true => SLVNOTSTRR::NOT_STRETCHING,
        }
    }
    #[doc = "Checks if the value of the field is `STRETCHING`"]
    #[inline]
    pub fn is_stretching(&self) -> bool {
        *self == SLVNOTSTRR::STRETCHING
    }
    #[doc = "Checks if the value of the field is `NOT_STRETCHING`"]
    #[inline]
    pub fn is_not_stretching(&self) -> bool {
        *self == SLVNOTSTRR::NOT_STRETCHING
    }
}
#[doc = "Possible values of the field `SLVIDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVIDXR {
    #[doc = "Slave address 0 was matched."]
    SLAVE_ADDRESS_0_WAS,
    #[doc = "Slave address 1 was matched."]
    SLAVE_ADDRESS_1_WAS,
    #[doc = "Slave address 2 was matched."]
    SLAVE_ADDRESS_2_WAS,
    #[doc = "Slave address 3 was matched."]
    SLAVE_ADDRESS_3_WAS,
}
impl SLVIDXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLVIDXR::SLAVE_ADDRESS_0_WAS => 0,
            SLVIDXR::SLAVE_ADDRESS_1_WAS => 1,
            SLVIDXR::SLAVE_ADDRESS_2_WAS => 2,
            SLVIDXR::SLAVE_ADDRESS_3_WAS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLVIDXR {
        match value {
            0 => SLVIDXR::SLAVE_ADDRESS_0_WAS,
            1 => SLVIDXR::SLAVE_ADDRESS_1_WAS,
            2 => SLVIDXR::SLAVE_ADDRESS_2_WAS,
            3 => SLVIDXR::SLAVE_ADDRESS_3_WAS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS_0_WAS`"]
    #[inline]
    pub fn is_slave_address_0_was(&self) -> bool {
        *self == SLVIDXR::SLAVE_ADDRESS_0_WAS
    }
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS_1_WAS`"]
    #[inline]
    pub fn is_slave_address_1_was(&self) -> bool {
        *self == SLVIDXR::SLAVE_ADDRESS_1_WAS
    }
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS_2_WAS`"]
    #[inline]
    pub fn is_slave_address_2_was(&self) -> bool {
        *self == SLVIDXR::SLAVE_ADDRESS_2_WAS
    }
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS_3_WAS`"]
    #[inline]
    pub fn is_slave_address_3_was(&self) -> bool {
        *self == SLVIDXR::SLAVE_ADDRESS_3_WAS
    }
}
#[doc = "Possible values of the field `SLVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVSELR {
    #[doc = "Not selected. The Slave function is not currently selected."]
    NOT_SELECTED,
    #[doc = "Selected. The Slave function is currently selected."]
    SELECTED,
}
impl SLVSELR {
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
            SLVSELR::NOT_SELECTED => false,
            SLVSELR::SELECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVSELR {
        match value {
            false => SLVSELR::NOT_SELECTED,
            true => SLVSELR::SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SELECTED`"]
    #[inline]
    pub fn is_not_selected(&self) -> bool {
        *self == SLVSELR::NOT_SELECTED
    }
    #[doc = "Checks if the value of the field is `SELECTED`"]
    #[inline]
    pub fn is_selected(&self) -> bool {
        *self == SLVSELR::SELECTED
    }
}
#[doc = "Possible values of the field `SLVDESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELR {
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    NOT_DESELECTED,
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    DESELECTED,
}
impl SLVDESELR {
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
            SLVDESELR::NOT_DESELECTED => false,
            SLVDESELR::DESELECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVDESELR {
        match value {
            false => SLVDESELR::NOT_DESELECTED,
            true => SLVDESELR::DESELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DESELECTED`"]
    #[inline]
    pub fn is_not_deselected(&self) -> bool {
        *self == SLVDESELR::NOT_DESELECTED
    }
    #[doc = "Checks if the value of the field is `DESELECTED`"]
    #[inline]
    pub fn is_deselected(&self) -> bool {
        *self == SLVDESELR::DESELECTED
    }
}
#[doc = "Possible values of the field `MONRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYR {
    #[doc = "No data. The Monitor function does not currently have data available."]
    NO_DATA,
    #[doc = "Data waiting. The Monitor function has data waiting to be read."]
    DATA_WAITING,
}
impl MONRDYR {
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
            MONRDYR::NO_DATA => false,
            MONRDYR::DATA_WAITING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONRDYR {
        match value {
            false => MONRDYR::NO_DATA,
            true => MONRDYR::DATA_WAITING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline]
    pub fn is_no_data(&self) -> bool {
        *self == MONRDYR::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_WAITING`"]
    #[inline]
    pub fn is_data_waiting(&self) -> bool {
        *self == MONRDYR::DATA_WAITING
    }
}
#[doc = "Possible values of the field `MONOV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVR {
    #[doc = "No overrun. Monitor data has not overrun."]
    NO_OVERRUN,
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    OVERRUN,
}
impl MONOVR {
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
            MONOVR::NO_OVERRUN => false,
            MONOVR::OVERRUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONOVR {
        match value {
            false => MONOVR::NO_OVERRUN,
            true => MONOVR::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN`"]
    #[inline]
    pub fn is_no_overrun(&self) -> bool {
        *self == MONOVR::NO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline]
    pub fn is_overrun(&self) -> bool {
        *self == MONOVR::OVERRUN
    }
}
#[doc = "Possible values of the field `MONACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONACTIVER {
    #[doc = "Inactive. The Monitor function considers the I2C bus to be inactive."]
    INACTIVE,
    #[doc = "Active. The Monitor function considers the I2C bus to be active."]
    ACTIVE,
}
impl MONACTIVER {
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
            MONACTIVER::INACTIVE => false,
            MONACTIVER::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONACTIVER {
        match value {
            false => MONACTIVER::INACTIVE,
            true => MONACTIVER::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == MONACTIVER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == MONACTIVER::ACTIVE
    }
}
#[doc = "Possible values of the field `MONIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLER {
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    NOT_IDLE,
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    IDLE,
}
impl MONIDLER {
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
            MONIDLER::NOT_IDLE => false,
            MONIDLER::IDLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MONIDLER {
        match value {
            false => MONIDLER::NOT_IDLE,
            true => MONIDLER::IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_IDLE`"]
    #[inline]
    pub fn is_not_idle(&self) -> bool {
        *self == MONIDLER::NOT_IDLE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == MONIDLER::IDLE
    }
}
#[doc = "Possible values of the field `EVENTTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTR {
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    NO_TIME_OUT,
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the I2C TIMEOUT register."]
    EVENT_TIME_OUT,
}
impl EVENTTIMEOUTR {
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
            EVENTTIMEOUTR::NO_TIME_OUT => false,
            EVENTTIMEOUTR::EVENT_TIME_OUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EVENTTIMEOUTR {
        match value {
            false => EVENTTIMEOUTR::NO_TIME_OUT,
            true => EVENTTIMEOUTR::EVENT_TIME_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIME_OUT`"]
    #[inline]
    pub fn is_no_time_out(&self) -> bool {
        *self == EVENTTIMEOUTR::NO_TIME_OUT
    }
    #[doc = "Checks if the value of the field is `EVENT_TIME_OUT`"]
    #[inline]
    pub fn is_event_time_out(&self) -> bool {
        *self == EVENTTIMEOUTR::EVENT_TIME_OUT
    }
}
#[doc = "Possible values of the field `SCLTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTR {
    #[doc = "No time-out. SCL low time has not caused a time-out."]
    NO_TIME_OUT,
    #[doc = "Time-out. SCL low time has caused a time-out."]
    TIME_OUT,
}
impl SCLTIMEOUTR {
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
            SCLTIMEOUTR::NO_TIME_OUT => false,
            SCLTIMEOUTR::TIME_OUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCLTIMEOUTR {
        match value {
            false => SCLTIMEOUTR::NO_TIME_OUT,
            true => SCLTIMEOUTR::TIME_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIME_OUT`"]
    #[inline]
    pub fn is_no_time_out(&self) -> bool {
        *self == SCLTIMEOUTR::NO_TIME_OUT
    }
    #[doc = "Checks if the value of the field is `TIME_OUT`"]
    #[inline]
    pub fn is_time_out(&self) -> bool {
        *self == SCLTIMEOUTR::TIME_OUT
    }
}
#[doc = "Values that can be written to the field `MSTPENDING`"]
pub enum MSTPENDINGW {
    #[doc = "In progress. Communication is in progress and the Master function is busy and cannot currently accept a command."]
    IN_PROGRESS,
    #[doc = "Pending. The Master function needs software service or is in the idle state. If the master is not in the idle state, it is waiting to receive or transmit data or the NACK bit."]
    PENDING,
}
impl MSTPENDINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTPENDINGW::IN_PROGRESS => false,
            MSTPENDINGW::PENDING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTPENDINGW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTPENDINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTPENDINGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "In progress. Communication is in progress and the Master function is busy and cannot currently accept a command."]
    #[inline]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(MSTPENDINGW::IN_PROGRESS)
    }
    #[doc = "Pending. The Master function needs software service or is in the idle state. If the master is not in the idle state, it is waiting to receive or transmit data or the NACK bit."]
    #[inline]
    pub fn pending(self) -> &'a mut W {
        self.variant(MSTPENDINGW::PENDING)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSTSTATE`"]
pub enum MSTSTATEW {
    #[doc = "Idle. The Master function is available to be used for a new transaction."]
    IDLE,
    #[doc = "Receive ready. Received data  available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    RECEIVE_READY,
    #[doc = "Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    TRANSMIT_READY,
    #[doc = "NACK Address. Slave NACKed address."]
    NACK_ADDRESS,
    #[doc = "NACK Data. Slave NACKed transmitted data."]
    NACK_DATA,
}
impl MSTSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSTSTATEW::IDLE => 0,
            MSTSTATEW::RECEIVE_READY => 1,
            MSTSTATEW::TRANSMIT_READY => 2,
            MSTSTATEW::NACK_ADDRESS => 3,
            MSTSTATEW::NACK_DATA => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSTATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Idle. The Master function is available to be used for a new transaction."]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(MSTSTATEW::IDLE)
    }
    #[doc = "Receive ready. Received data available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    #[inline]
    pub fn receive_ready(self) -> &'a mut W {
        self.variant(MSTSTATEW::RECEIVE_READY)
    }
    #[doc = "Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    #[inline]
    pub fn transmit_ready(self) -> &'a mut W {
        self.variant(MSTSTATEW::TRANSMIT_READY)
    }
    #[doc = "NACK Address. Slave NACKed address."]
    #[inline]
    pub fn nack_address(self) -> &'a mut W {
        self.variant(MSTSTATEW::NACK_ADDRESS)
    }
    #[doc = "NACK Data. Slave NACKed transmitted data."]
    #[inline]
    pub fn nack_data(self) -> &'a mut W {
        self.variant(MSTSTATEW::NACK_DATA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSTARBLOSS`"]
pub enum MSTARBLOSSW {
    #[doc = "No loss. No Arbitration Loss has occurred."]
    NO_LOSS,
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    ARBITRATION_LOSS,
}
impl MSTARBLOSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTARBLOSSW::NO_LOSS => false,
            MSTARBLOSSW::ARBITRATION_LOSS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTARBLOSSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTARBLOSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTARBLOSSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No loss. No Arbitration Loss has occurred."]
    #[inline]
    pub fn no_loss(self) -> &'a mut W {
        self.variant(MSTARBLOSSW::NO_LOSS)
    }
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    #[inline]
    pub fn arbitration_loss(self) -> &'a mut W {
        self.variant(MSTARBLOSSW::ARBITRATION_LOSS)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSTSTSTPERR`"]
pub enum MSTSTSTPERRW {
    #[doc = "No Start/Stop Error has occurred."]
    NO_ERROR,
    #[doc = "Start/stop error has occurred. The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    STARTSTOP_ERROR,
}
impl MSTSTSTPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTSTPERRW::NO_ERROR => false,
            MSTSTSTPERRW::STARTSTOP_ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTSTSTPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTSTPERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTSTSTPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Start/Stop Error has occurred."]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(MSTSTSTPERRW::NO_ERROR)
    }
    #[doc = "Start/stop error has occurred. The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    #[inline]
    pub fn startstop_error(self) -> &'a mut W {
        self.variant(MSTSTSTPERRW::STARTSTOP_ERROR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLVPENDING`"]
pub enum SLVPENDINGW {
    #[doc = "In progress. The Slave function does not currently need service."]
    IN_PROGRESS,
    #[doc = "Pending. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    PENDING,
}
impl SLVPENDINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVPENDINGW::IN_PROGRESS => false,
            SLVPENDINGW::PENDING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVPENDINGW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVPENDINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVPENDINGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "In progress. The Slave function does not currently need service."]
    #[inline]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(SLVPENDINGW::IN_PROGRESS)
    }
    #[doc = "Pending. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    #[inline]
    pub fn pending(self) -> &'a mut W {
        self.variant(SLVPENDINGW::PENDING)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLVSTATE`"]
pub enum SLVSTATEW {
    #[doc = "Slave address. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    SLAVE_ADDRESS,
    #[doc = "Slave receive. Received data is available (Slave Receiver mode)."]
    SLAVE_RECEIVE,
    #[doc = "Slave transmit. Data can be transmitted (Slave Transmitter mode)."]
    SLAVE_TRANSMIT,
}
impl SLVSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLVSTATEW::SLAVE_ADDRESS => 0,
            SLVSTATEW::SLAVE_RECEIVE => 1,
            SLVSTATEW::SLAVE_TRANSMIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVSTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVSTATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Slave address. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    #[inline]
    pub fn slave_address(self) -> &'a mut W {
        self.variant(SLVSTATEW::SLAVE_ADDRESS)
    }
    #[doc = "Slave receive. Received data is available (Slave Receiver mode)."]
    #[inline]
    pub fn slave_receive(self) -> &'a mut W {
        self.variant(SLVSTATEW::SLAVE_RECEIVE)
    }
    #[doc = "Slave transmit. Data can be transmitted (Slave Transmitter mode)."]
    #[inline]
    pub fn slave_transmit(self) -> &'a mut W {
        self.variant(SLVSTATEW::SLAVE_TRANSMIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLVNOTSTR`"]
pub enum SLVNOTSTRW {
    #[doc = "Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    STRETCHING,
    #[doc = "Not stretching. The slave function is not currently stretching the I2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    NOT_STRETCHING,
}
impl SLVNOTSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVNOTSTRW::STRETCHING => false,
            SLVNOTSTRW::NOT_STRETCHING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVNOTSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVNOTSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVNOTSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    #[inline]
    pub fn stretching(self) -> &'a mut W {
        self.variant(SLVNOTSTRW::STRETCHING)
    }
    #[doc = "Not stretching. The slave function is not currently stretching the I2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    #[inline]
    pub fn not_stretching(self) -> &'a mut W {
        self.variant(SLVNOTSTRW::NOT_STRETCHING)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLVIDX`"]
pub enum SLVIDXW {
    #[doc = "Slave address 0 was matched."]
    SLAVE_ADDRESS_0_WAS,
    #[doc = "Slave address 1 was matched."]
    SLAVE_ADDRESS_1_WAS,
    #[doc = "Slave address 2 was matched."]
    SLAVE_ADDRESS_2_WAS,
    #[doc = "Slave address 3 was matched."]
    SLAVE_ADDRESS_3_WAS,
}
impl SLVIDXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLVIDXW::SLAVE_ADDRESS_0_WAS => 0,
            SLVIDXW::SLAVE_ADDRESS_1_WAS => 1,
            SLVIDXW::SLAVE_ADDRESS_2_WAS => 2,
            SLVIDXW::SLAVE_ADDRESS_3_WAS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVIDXW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVIDXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVIDXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Slave address 0 was matched."]
    #[inline]
    pub fn slave_address_0_was(self) -> &'a mut W {
        self.variant(SLVIDXW::SLAVE_ADDRESS_0_WAS)
    }
    #[doc = "Slave address 1 was matched."]
    #[inline]
    pub fn slave_address_1_was(self) -> &'a mut W {
        self.variant(SLVIDXW::SLAVE_ADDRESS_1_WAS)
    }
    #[doc = "Slave address 2 was matched."]
    #[inline]
    pub fn slave_address_2_was(self) -> &'a mut W {
        self.variant(SLVIDXW::SLAVE_ADDRESS_2_WAS)
    }
    #[doc = "Slave address 3 was matched."]
    #[inline]
    pub fn slave_address_3_was(self) -> &'a mut W {
        self.variant(SLVIDXW::SLAVE_ADDRESS_3_WAS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLVSEL`"]
pub enum SLVSELW {
    #[doc = "Not selected. The Slave function is not currently selected."]
    NOT_SELECTED,
    #[doc = "Selected. The Slave function is currently selected."]
    SELECTED,
}
impl SLVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVSELW::NOT_SELECTED => false,
            SLVSELW::SELECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected. The Slave function is not currently selected."]
    #[inline]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(SLVSELW::NOT_SELECTED)
    }
    #[doc = "Selected. The Slave function is currently selected."]
    #[inline]
    pub fn selected(self) -> &'a mut W {
        self.variant(SLVSELW::SELECTED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLVDESEL`"]
pub enum SLVDESELW {
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    NOT_DESELECTED,
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    DESELECTED,
}
impl SLVDESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVDESELW::NOT_DESELECTED => false,
            SLVDESELW::DESELECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVDESELW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVDESELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    #[inline]
    pub fn not_deselected(self) -> &'a mut W {
        self.variant(SLVDESELW::NOT_DESELECTED)
    }
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    #[inline]
    pub fn deselected(self) -> &'a mut W {
        self.variant(SLVDESELW::DESELECTED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONRDY`"]
pub enum MONRDYW {
    #[doc = "No data. The Monitor function does not currently have data available."]
    NO_DATA,
    #[doc = "Data waiting. The Monitor function has data waiting to be read."]
    DATA_WAITING,
}
impl MONRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONRDYW::NO_DATA => false,
            MONRDYW::DATA_WAITING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _MONRDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No data. The Monitor function does not currently have data available."]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(MONRDYW::NO_DATA)
    }
    #[doc = "Data waiting. The Monitor function has data waiting to be read."]
    #[inline]
    pub fn data_waiting(self) -> &'a mut W {
        self.variant(MONRDYW::DATA_WAITING)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONOV`"]
pub enum MONOVW {
    #[doc = "No overrun. Monitor data has not overrun."]
    NO_OVERRUN,
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    OVERRUN,
}
impl MONOVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONOVW::NO_OVERRUN => false,
            MONOVW::OVERRUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONOVW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONOVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overrun. Monitor data has not overrun."]
    #[inline]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(MONOVW::NO_OVERRUN)
    }
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    #[inline]
    pub fn overrun(self) -> &'a mut W {
        self.variant(MONOVW::OVERRUN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONACTIVE`"]
pub enum MONACTIVEW {
    #[doc = "Inactive. The Monitor function considers the I2C bus to be inactive."]
    INACTIVE,
    #[doc = "Active. The Monitor function considers the I2C bus to be active."]
    ACTIVE,
}
impl MONACTIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONACTIVEW::INACTIVE => false,
            MONACTIVEW::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _MONACTIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONACTIVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inactive. The Monitor function considers the I2C bus to be inactive."]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MONACTIVEW::INACTIVE)
    }
    #[doc = "Active. The Monitor function considers the I2C bus to be active."]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(MONACTIVEW::ACTIVE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MONIDLE`"]
pub enum MONIDLEW {
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    NOT_IDLE,
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    IDLE,
}
impl MONIDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MONIDLEW::NOT_IDLE => false,
            MONIDLEW::IDLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MONIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MONIDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MONIDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    #[inline]
    pub fn not_idle(self) -> &'a mut W {
        self.variant(MONIDLEW::NOT_IDLE)
    }
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(MONIDLEW::IDLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EVENTTIMEOUT`"]
pub enum EVENTTIMEOUTW {
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    NO_TIME_OUT,
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the I2C TIMEOUT register."]
    EVENT_TIME_OUT,
}
impl EVENTTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EVENTTIMEOUTW::NO_TIME_OUT => false,
            EVENTTIMEOUTW::EVENT_TIME_OUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVENTTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVENTTIMEOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    #[inline]
    pub fn no_time_out(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTW::NO_TIME_OUT)
    }
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the I2C TIMEOUT register."]
    #[inline]
    pub fn event_time_out(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTW::EVENT_TIME_OUT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCLTIMEOUT`"]
pub enum SCLTIMEOUTW {
    #[doc = "No time-out. SCL low time has not caused a time-out."]
    NO_TIME_OUT,
    #[doc = "Time-out. SCL low time has caused a time-out."]
    TIME_OUT,
}
impl SCLTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLTIMEOUTW::NO_TIME_OUT => false,
            SCLTIMEOUTW::TIME_OUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLTIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLTIMEOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No time-out. SCL low time has not caused a time-out."]
    #[inline]
    pub fn no_time_out(self) -> &'a mut W {
        self.variant(SCLTIMEOUTW::NO_TIME_OUT)
    }
    #[doc = "Time-out. SCL low time has caused a time-out."]
    #[inline]
    pub fn time_out(self) -> &'a mut W {
        self.variant(SCLTIMEOUTW::TIME_OUT)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt."]
    #[inline]
    pub fn mstpending(&self) -> MSTPENDINGR {
        MSTPENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved."]
    #[inline]
    pub fn mststate(&self) -> MSTSTATER {
        MSTSTATER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline]
    pub fn mstarbloss(&self) -> MSTARBLOSSR {
        MSTARBLOSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline]
    pub fn mstststperr(&self) -> MSTSTSTPERRR {
        MSTSTSTPERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the MSTCTL register."]
    #[inline]
    pub fn slvpending(&self) -> SLVPENDINGR {
        SLVPENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:10 - Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved."]
    #[inline]
    pub fn slvstate(&self) -> SLVSTATER {
        SLVSTATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[inline]
    pub fn slvnotstr(&self) -> SLVNOTSTRR {
        SLVNOTSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[inline]
    pub fn slvidx(&self) -> SLVIDXR {
        SLVIDXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, or when there is a Stop detected on the bus. SLVSEL is not cleared if software Nacks data."]
    #[inline]
    pub fn slvsel(&self) -> SLVSELR {
        SLVSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline]
    pub fn slvdesel(&self) -> SLVDESELR {
        SLVDESELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[inline]
    pub fn monrdy(&self) -> MONRDYR {
        MONRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline]
    pub fn monov(&self) -> MONOVR {
        MONOVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Monitor Active flag. This flag indicates when the Monitor function considers the I2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[inline]
    pub fn monactive(&self) -> MONACTIVER {
        MONACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register . The flag can be cleared by writing a 1 to this bit."]
    #[inline]
    pub fn monidle(&self) -> MONIDLER {
        MONIDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline]
    pub fn eventtimeout(&self) -> EVENTTIMEOUTR {
        EVENTTIMEOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline]
    pub fn scltimeout(&self) -> SCLTIMEOUTR {
        SCLTIMEOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2049 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt."]
    #[inline]
    pub fn mstpending(&mut self) -> _MSTPENDINGW {
        _MSTPENDINGW { w: self }
    }
    #[doc = "Bits 1:3 - Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved."]
    #[inline]
    pub fn mststate(&mut self) -> _MSTSTATEW {
        _MSTSTATEW { w: self }
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline]
    pub fn mstarbloss(&mut self) -> _MSTARBLOSSW {
        _MSTARBLOSSW { w: self }
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline]
    pub fn mstststperr(&mut self) -> _MSTSTSTPERRW {
        _MSTSTSTPERRW { w: self }
    }
    #[doc = "Bit 8 - Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the MSTCTL register."]
    #[inline]
    pub fn slvpending(&mut self) -> _SLVPENDINGW {
        _SLVPENDINGW { w: self }
    }
    #[doc = "Bits 9:10 - Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved."]
    #[inline]
    pub fn slvstate(&mut self) -> _SLVSTATEW {
        _SLVSTATEW { w: self }
    }
    #[doc = "Bit 11 - Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[inline]
    pub fn slvnotstr(&mut self) -> _SLVNOTSTRW {
        _SLVNOTSTRW { w: self }
    }
    #[doc = "Bits 12:13 - Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[inline]
    pub fn slvidx(&mut self) -> _SLVIDXW {
        _SLVIDXW { w: self }
    }
    #[doc = "Bit 14 - Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, or when there is a Stop detected on the bus. SLVSEL is not cleared if software Nacks data."]
    #[inline]
    pub fn slvsel(&mut self) -> _SLVSELW {
        _SLVSELW { w: self }
    }
    #[doc = "Bit 15 - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline]
    pub fn slvdesel(&mut self) -> _SLVDESELW {
        _SLVDESELW { w: self }
    }
    #[doc = "Bit 16 - Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[inline]
    pub fn monrdy(&mut self) -> _MONRDYW {
        _MONRDYW { w: self }
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline]
    pub fn monov(&mut self) -> _MONOVW {
        _MONOVW { w: self }
    }
    #[doc = "Bit 18 - Monitor Active flag. This flag indicates when the Monitor function considers the I2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[inline]
    pub fn monactive(&mut self) -> _MONACTIVEW {
        _MONACTIVEW { w: self }
    }
    #[doc = "Bit 19 - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register . The flag can be cleared by writing a 1 to this bit."]
    #[inline]
    pub fn monidle(&mut self) -> _MONIDLEW {
        _MONIDLEW { w: self }
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline]
    pub fn eventtimeout(&mut self) -> _EVENTTIMEOUTW {
        _EVENTTIMEOUTW { w: self }
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline]
    pub fn scltimeout(&mut self) -> _SCLTIMEOUTW {
        _SCLTIMEOUTW { w: self }
    }
}
