#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0x0801"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0801
    }
}
#[doc = "Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDING_A {
    #[doc = "0: In progress. Communication is in progress and the Master function is busy and cannot currently accept a command."]
    IN_PROGRESS = 0,
    #[doc = "1: Pending. The Master function needs software service or is in the idle state. If the master is not in the idle state, it is waiting to receive or transmit data or the NACK bit."]
    PENDING = 1,
}
impl From<MSTPENDING_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPENDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTPENDING`"]
pub type MSTPENDING_R = crate::R<bool, MSTPENDING_A>;
impl MSTPENDING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPENDING_A {
        match self.bits {
            false => MSTPENDING_A::IN_PROGRESS,
            true => MSTPENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == MSTPENDING_A::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == MSTPENDING_A::PENDING
    }
}
#[doc = "Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved. See Table 400 for details of state values and appropriate responses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSTSTATE_A {
    #[doc = "0: Idle. The Master function is available to be used for a new transaction."]
    IDLE = 0,
    #[doc = "1: Receive ready. Received data available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    RECEIVE_READY = 1,
    #[doc = "2: Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    TRANSMIT_READY = 2,
    #[doc = "3: NACK Address. Slave NACKed address."]
    NACK_ADDRESS = 3,
    #[doc = "4: NACK Data. Slave NACKed transmitted data."]
    NACK_DATA = 4,
}
impl From<MSTSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSTSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSTSTATE`"]
pub type MSTSTATE_R = crate::R<u8, MSTSTATE_A>;
impl MSTSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MSTSTATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MSTSTATE_A::IDLE),
            1 => Val(MSTSTATE_A::RECEIVE_READY),
            2 => Val(MSTSTATE_A::TRANSMIT_READY),
            3 => Val(MSTSTATE_A::NACK_ADDRESS),
            4 => Val(MSTSTATE_A::NACK_DATA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == MSTSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `RECEIVE_READY`"]
    #[inline(always)]
    pub fn is_receive_ready(&self) -> bool {
        *self == MSTSTATE_A::RECEIVE_READY
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_READY`"]
    #[inline(always)]
    pub fn is_transmit_ready(&self) -> bool {
        *self == MSTSTATE_A::TRANSMIT_READY
    }
    #[doc = "Checks if the value of the field is `NACK_ADDRESS`"]
    #[inline(always)]
    pub fn is_nack_address(&self) -> bool {
        *self == MSTSTATE_A::NACK_ADDRESS
    }
    #[doc = "Checks if the value of the field is `NACK_DATA`"]
    #[inline(always)]
    pub fn is_nack_data(&self) -> bool {
        *self == MSTSTATE_A::NACK_DATA
    }
}
#[doc = "Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSS_A {
    #[doc = "0: No Arbitration Loss has occurred."]
    NO_LOSS = 0,
    #[doc = "1: Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    ARBITRATION_LOSS = 1,
}
impl From<MSTARBLOSS_A> for bool {
    #[inline(always)]
    fn from(variant: MSTARBLOSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTARBLOSS`"]
pub type MSTARBLOSS_R = crate::R<bool, MSTARBLOSS_A>;
impl MSTARBLOSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTARBLOSS_A {
        match self.bits {
            false => MSTARBLOSS_A::NO_LOSS,
            true => MSTARBLOSS_A::ARBITRATION_LOSS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_LOSS`"]
    #[inline(always)]
    pub fn is_no_loss(&self) -> bool {
        *self == MSTARBLOSS_A::NO_LOSS
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOSS`"]
    #[inline(always)]
    pub fn is_arbitration_loss(&self) -> bool {
        *self == MSTARBLOSS_A::ARBITRATION_LOSS
    }
}
#[doc = "Write proxy for field `MSTARBLOSS`"]
pub struct MSTARBLOSS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTARBLOSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTARBLOSS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Arbitration Loss has occurred."]
    #[inline(always)]
    pub fn no_loss(self) -> &'a mut W {
        self.variant(MSTARBLOSS_A::NO_LOSS)
    }
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    #[inline(always)]
    pub fn arbitration_loss(self) -> &'a mut W {
        self.variant(MSTARBLOSS_A::ARBITRATION_LOSS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERR_A {
    #[doc = "0: No Start/Stop Error has occurred."]
    NO_ERROR = 0,
    #[doc = "1: The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    ERROR = 1,
}
impl From<MSTSTSTPERR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSTSTPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTSTSTPERR`"]
pub type MSTSTSTPERR_R = crate::R<bool, MSTSTSTPERR_A>;
impl MSTSTSTPERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSTSTPERR_A {
        match self.bits {
            false => MSTSTSTPERR_A::NO_ERROR,
            true => MSTSTSTPERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MSTSTSTPERR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MSTSTSTPERR_A::ERROR
    }
}
#[doc = "Write proxy for field `MSTSTSTPERR`"]
pub struct MSTSTSTPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSTSTPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSTSTPERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Start/Stop Error has occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(MSTSTSTPERR_A::NO_ERROR)
    }
    #[doc = "The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(MSTSTSTPERR_A::ERROR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the SLVCTL register. The point in time when SlvPending is set depends on whether the I2C interface is in HSCAPABLE mode. See Section 25.7.2.2.2. When the I2C interface is configured to be HSCAPABLE, HS master codes are detected automatically. Due to the requirements of the HS I2C specification, slave addresses must also be detected automatically, since the address must be acknowledged before the clock can be stretched.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDING_A {
    #[doc = "0: In progress. The Slave function does not currently need service."]
    IN_PROGRESS = 0,
    #[doc = "1: Pending. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    PENDING = 1,
}
impl From<SLVPENDING_A> for bool {
    #[inline(always)]
    fn from(variant: SLVPENDING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVPENDING`"]
pub type SLVPENDING_R = crate::R<bool, SLVPENDING_A>;
impl SLVPENDING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVPENDING_A {
        match self.bits {
            false => SLVPENDING_A::IN_PROGRESS,
            true => SLVPENDING_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == SLVPENDING_A::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SLVPENDING_A::PENDING
    }
}
#[doc = "Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved. See Table 401 for state values and actions. note that the occurrence of some states and how they are handled are affected by DMA mode and Automatic Operation modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLVSTATE_A {
    #[doc = "0: Slave address. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    SLAVE_ADDRESS = 0,
    #[doc = "1: Slave receive. Received data is available (Slave Receiver mode)."]
    SLAVE_RECEIVE = 1,
    #[doc = "2: Slave transmit. Data can be transmitted (Slave Transmitter mode)."]
    SLAVE_TRANSMIT = 2,
}
impl From<SLVSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: SLVSTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLVSTATE`"]
pub type SLVSTATE_R = crate::R<u8, SLVSTATE_A>;
impl SLVSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLVSTATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLVSTATE_A::SLAVE_ADDRESS),
            1 => Val(SLVSTATE_A::SLAVE_RECEIVE),
            2 => Val(SLVSTATE_A::SLAVE_TRANSMIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS`"]
    #[inline(always)]
    pub fn is_slave_address(&self) -> bool {
        *self == SLVSTATE_A::SLAVE_ADDRESS
    }
    #[doc = "Checks if the value of the field is `SLAVE_RECEIVE`"]
    #[inline(always)]
    pub fn is_slave_receive(&self) -> bool {
        *self == SLVSTATE_A::SLAVE_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SLAVE_TRANSMIT`"]
    #[inline(always)]
    pub fn is_slave_transmit(&self) -> bool {
        *self == SLVSTATE_A::SLAVE_TRANSMIT
    }
}
#[doc = "Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTR_A {
    #[doc = "0: Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    STRETCHING = 0,
    #[doc = "1: Not stretching. The slave function is not currently stretching the I 2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    NOT_STRETCHING = 1,
}
impl From<SLVNOTSTR_A> for bool {
    #[inline(always)]
    fn from(variant: SLVNOTSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVNOTSTR`"]
pub type SLVNOTSTR_R = crate::R<bool, SLVNOTSTR_A>;
impl SLVNOTSTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVNOTSTR_A {
        match self.bits {
            false => SLVNOTSTR_A::STRETCHING,
            true => SLVNOTSTR_A::NOT_STRETCHING,
        }
    }
    #[doc = "Checks if the value of the field is `STRETCHING`"]
    #[inline(always)]
    pub fn is_stretching(&self) -> bool {
        *self == SLVNOTSTR_A::STRETCHING
    }
    #[doc = "Checks if the value of the field is `NOT_STRETCHING`"]
    #[inline(always)]
    pub fn is_not_stretching(&self) -> bool {
        *self == SLVNOTSTR_A::NOT_STRETCHING
    }
}
#[doc = "Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLVIDX_A {
    #[doc = "0: Address 0. Slave address 0 was matched."]
    ADDRESS0 = 0,
    #[doc = "1: Address 1. Slave address 1 was matched."]
    ADDRESS1 = 1,
    #[doc = "2: Address 2. Slave address 2 was matched."]
    ADDRESS2 = 2,
    #[doc = "3: Address 3. Slave address 3 was matched."]
    ADDRESS3 = 3,
}
impl From<SLVIDX_A> for u8 {
    #[inline(always)]
    fn from(variant: SLVIDX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLVIDX`"]
pub type SLVIDX_R = crate::R<u8, SLVIDX_A>;
impl SLVIDX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVIDX_A {
        match self.bits {
            0 => SLVIDX_A::ADDRESS0,
            1 => SLVIDX_A::ADDRESS1,
            2 => SLVIDX_A::ADDRESS2,
            3 => SLVIDX_A::ADDRESS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS0`"]
    #[inline(always)]
    pub fn is_address0(&self) -> bool {
        *self == SLVIDX_A::ADDRESS0
    }
    #[doc = "Checks if the value of the field is `ADDRESS1`"]
    #[inline(always)]
    pub fn is_address1(&self) -> bool {
        *self == SLVIDX_A::ADDRESS1
    }
    #[doc = "Checks if the value of the field is `ADDRESS2`"]
    #[inline(always)]
    pub fn is_address2(&self) -> bool {
        *self == SLVIDX_A::ADDRESS2
    }
    #[doc = "Checks if the value of the field is `ADDRESS3`"]
    #[inline(always)]
    pub fn is_address3(&self) -> bool {
        *self == SLVIDX_A::ADDRESS3
    }
}
#[doc = "Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address, or when the address has been automatically acknowledged. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, when there is a Stop detected on the bus, when the master NACKs slave data, and in some combinations of Automatic Operation. SLVSEL is not cleared if software NACKs data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVSEL_A {
    #[doc = "0: Not selected. The Slave function is not currently selected."]
    NOT_SELECTED = 0,
    #[doc = "1: Selected. The Slave function is currently selected."]
    SELECTED = 1,
}
impl From<SLVSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SLVSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVSEL`"]
pub type SLVSEL_R = crate::R<bool, SLVSEL_A>;
impl SLVSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVSEL_A {
        match self.bits {
            false => SLVSEL_A::NOT_SELECTED,
            true => SLVSEL_A::SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == SLVSEL_A::NOT_SELECTED
    }
    #[doc = "Checks if the value of the field is `SELECTED`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == SLVSEL_A::SELECTED
    }
}
#[doc = "Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESEL_A {
    #[doc = "0: Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    NOT_DESELECTED = 0,
    #[doc = "1: Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    DESELECTED = 1,
}
impl From<SLVDESEL_A> for bool {
    #[inline(always)]
    fn from(variant: SLVDESEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVDESEL`"]
pub type SLVDESEL_R = crate::R<bool, SLVDESEL_A>;
impl SLVDESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVDESEL_A {
        match self.bits {
            false => SLVDESEL_A::NOT_DESELECTED,
            true => SLVDESEL_A::DESELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DESELECTED`"]
    #[inline(always)]
    pub fn is_not_deselected(&self) -> bool {
        *self == SLVDESEL_A::NOT_DESELECTED
    }
    #[doc = "Checks if the value of the field is `DESELECTED`"]
    #[inline(always)]
    pub fn is_deselected(&self) -> bool {
        *self == SLVDESEL_A::DESELECTED
    }
}
#[doc = "Write proxy for field `SLVDESEL`"]
pub struct SLVDESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVDESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVDESEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    #[inline(always)]
    pub fn not_deselected(self) -> &'a mut W {
        self.variant(SLVDESEL_A::NOT_DESELECTED)
    }
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    #[inline(always)]
    pub fn deselected(self) -> &'a mut W {
        self.variant(SLVDESEL_A::DESELECTED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Monitor Ready. This flag is cleared when the MONRXDAT register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDY_A {
    #[doc = "0: No data. The Monitor function does not currently have data available."]
    NO_DATA = 0,
    #[doc = "1: Data waiting. The Monitor function has data waiting to be read."]
    DATA_WAITING = 1,
}
impl From<MONRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MONRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONRDY`"]
pub type MONRDY_R = crate::R<bool, MONRDY_A>;
impl MONRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONRDY_A {
        match self.bits {
            false => MONRDY_A::NO_DATA,
            true => MONRDY_A::DATA_WAITING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == MONRDY_A::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_WAITING`"]
    #[inline(always)]
    pub fn is_data_waiting(&self) -> bool {
        *self == MONRDY_A::DATA_WAITING
    }
}
#[doc = "Monitor Overflow flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOV_A {
    #[doc = "0: No overrun. Monitor data has not overrun."]
    NO_OVERRUN = 0,
    #[doc = "1: Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    OVERRUN = 1,
}
impl From<MONOV_A> for bool {
    #[inline(always)]
    fn from(variant: MONOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONOV`"]
pub type MONOV_R = crate::R<bool, MONOV_A>;
impl MONOV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONOV_A {
        match self.bits {
            false => MONOV_A::NO_OVERRUN,
            true => MONOV_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == MONOV_A::NO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == MONOV_A::OVERRUN
    }
}
#[doc = "Write proxy for field `MONOV`"]
pub struct MONOV_W<'a> {
    w: &'a mut W,
}
impl<'a> MONOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONOV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No overrun. Monitor data has not overrun."]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(MONOV_A::NO_OVERRUN)
    }
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(MONOV_A::OVERRUN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Monitor Active flag. Indicates when the Monitor function considers the I 2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONACTIVE_A {
    #[doc = "0: Inactive. The Monitor function considers the I2C bus to be inactive."]
    INACTIVE = 0,
    #[doc = "1: Active. The Monitor function considers the I2C bus to be active."]
    ACTIVE = 1,
}
impl From<MONACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: MONACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONACTIVE`"]
pub type MONACTIVE_R = crate::R<bool, MONACTIVE_A>;
impl MONACTIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONACTIVE_A {
        match self.bits {
            false => MONACTIVE_A::INACTIVE,
            true => MONACTIVE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == MONACTIVE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MONACTIVE_A::ACTIVE
    }
}
#[doc = "Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLE_A {
    #[doc = "0: Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    NOT_IDLE = 0,
    #[doc = "1: Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    IDLE = 1,
}
impl From<MONIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: MONIDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONIDLE`"]
pub type MONIDLE_R = crate::R<bool, MONIDLE_A>;
impl MONIDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONIDLE_A {
        match self.bits {
            false => MONIDLE_A::NOT_IDLE,
            true => MONIDLE_A::IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_IDLE`"]
    #[inline(always)]
    pub fn is_not_idle(&self) -> bool {
        *self == MONIDLE_A::NOT_IDLE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == MONIDLE_A::IDLE
    }
}
#[doc = "Write proxy for field `MONIDLE`"]
pub struct MONIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MONIDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONIDLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    #[inline(always)]
    pub fn not_idle(self) -> &'a mut W {
        self.variant(MONIDLE_A::NOT_IDLE)
    }
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(MONIDLE_A::IDLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUT_A {
    #[doc = "0: No time-out. I2C bus events have not caused a time-out."]
    NO_TIMEOUT = 0,
    #[doc = "1: Event time-out. The time between I2C bus events has been longer than the time specified by the TIMEOUT register."]
    EVEN_TIMEOUT = 1,
}
impl From<EVENTTIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTTIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTTIMEOUT`"]
pub type EVENTTIMEOUT_R = crate::R<bool, EVENTTIMEOUT_A>;
impl EVENTTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTTIMEOUT_A {
        match self.bits {
            false => EVENTTIMEOUT_A::NO_TIMEOUT,
            true => EVENTTIMEOUT_A::EVEN_TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == EVENTTIMEOUT_A::NO_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `EVEN_TIMEOUT`"]
    #[inline(always)]
    pub fn is_even_timeout(&self) -> bool {
        *self == EVENTTIMEOUT_A::EVEN_TIMEOUT
    }
}
#[doc = "Write proxy for field `EVENTTIMEOUT`"]
pub struct EVENTTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTTIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTTIMEOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut W {
        self.variant(EVENTTIMEOUT_A::NO_TIMEOUT)
    }
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the TIMEOUT register."]
    #[inline(always)]
    pub fn even_timeout(self) -> &'a mut W {
        self.variant(EVENTTIMEOUT_A::EVEN_TIMEOUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUT_A {
    #[doc = "0: No time-out. SCL low time has not caused a time-out."]
    NO_TIMEOUT = 0,
    #[doc = "1: Time-out. SCL low time has caused a time-out."]
    TIMEOUT = 1,
}
impl From<SCLTIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: SCLTIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLTIMEOUT`"]
pub type SCLTIMEOUT_R = crate::R<bool, SCLTIMEOUT_A>;
impl SCLTIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLTIMEOUT_A {
        match self.bits {
            false => SCLTIMEOUT_A::NO_TIMEOUT,
            true => SCLTIMEOUT_A::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == SCLTIMEOUT_A::NO_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == SCLTIMEOUT_A::TIMEOUT
    }
}
#[doc = "Write proxy for field `SCLTIMEOUT`"]
pub struct SCLTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLTIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLTIMEOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No time-out. SCL low time has not caused a time-out."]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut W {
        self.variant(SCLTIMEOUT_A::NO_TIMEOUT)
    }
    #[doc = "Time-out. SCL low time has caused a time-out."]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(SCLTIMEOUT_A::TIMEOUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt."]
    #[inline(always)]
    pub fn mstpending(&self) -> MSTPENDING_R {
        MSTPENDING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved. See Table 400 for details of state values and appropriate responses."]
    #[inline(always)]
    pub fn mststate(&self) -> MSTSTATE_R {
        MSTSTATE_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn mstarbloss(&self) -> MSTARBLOSS_R {
        MSTARBLOSS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn mstststperr(&self) -> MSTSTSTPERR_R {
        MSTSTSTPERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the SLVCTL register. The point in time when SlvPending is set depends on whether the I2C interface is in HSCAPABLE mode. See Section 25.7.2.2.2. When the I2C interface is configured to be HSCAPABLE, HS master codes are detected automatically. Due to the requirements of the HS I2C specification, slave addresses must also be detected automatically, since the address must be acknowledged before the clock can be stretched."]
    #[inline(always)]
    pub fn slvpending(&self) -> SLVPENDING_R {
        SLVPENDING_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved. See Table 401 for state values and actions. note that the occurrence of some states and how they are handled are affected by DMA mode and Automatic Operation modes."]
    #[inline(always)]
    pub fn slvstate(&self) -> SLVSTATE_R {
        SLVSTATE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[inline(always)]
    pub fn slvnotstr(&self) -> SLVNOTSTR_R {
        SLVNOTSTR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[inline(always)]
    pub fn slvidx(&self) -> SLVIDX_R {
        SLVIDX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address, or when the address has been automatically acknowledged. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, when there is a Stop detected on the bus, when the master NACKs slave data, and in some combinations of Automatic Operation. SLVSEL is not cleared if software NACKs data."]
    #[inline(always)]
    pub fn slvsel(&self) -> SLVSEL_R {
        SLVSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn slvdesel(&self) -> SLVDESEL_R {
        SLVDESEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    pub fn monov(&self) -> MONOV_R {
        MONOV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Monitor Active flag. Indicates when the Monitor function considers the I 2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[inline(always)]
    pub fn monactive(&self) -> MONACTIVE_R {
        MONACTIVE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn monidle(&self) -> MONIDLE_R {
        MONIDLE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline(always)]
    pub fn eventtimeout(&self) -> EVENTTIMEOUT_R {
        EVENTTIMEOUT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn scltimeout(&self) -> SCLTIMEOUT_R {
        SCLTIMEOUT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn mstarbloss(&mut self) -> MSTARBLOSS_W {
        MSTARBLOSS_W { w: self }
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn mstststperr(&mut self) -> MSTSTSTPERR_W {
        MSTSTSTPERR_W { w: self }
    }
    #[doc = "Bit 15 - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn slvdesel(&mut self) -> SLVDESEL_W {
        SLVDESEL_W { w: self }
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    pub fn monov(&mut self) -> MONOV_W {
        MONOV_W { w: self }
    }
    #[doc = "Bit 19 - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn monidle(&mut self) -> MONIDLE_W {
        MONIDLE_W { w: self }
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline(always)]
    pub fn eventtimeout(&mut self) -> EVENTTIMEOUT_W {
        EVENTTIMEOUT_W { w: self }
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn scltimeout(&mut self) -> SCLTIMEOUT_W {
        SCLTIMEOUT_W { w: self }
    }
}
