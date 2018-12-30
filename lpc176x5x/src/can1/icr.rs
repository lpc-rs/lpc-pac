#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ICR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl RIR {
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
            RIR::RESET => false,
            RIR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIR {
        match value {
            false => RIR::RESET,
            true => RIR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == RIR::SET
    }
}
#[doc = "Possible values of the field `TI1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI1R {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl TI1R {
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
            TI1R::RESET => false,
            TI1R::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TI1R {
        match value {
            false => TI1R::RESET,
            true => TI1R::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TI1R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == TI1R::SET
    }
}
#[doc = "Possible values of the field `EI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl EIR {
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
            EIR::RESET => false,
            EIR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EIR {
        match value {
            false => EIR::RESET,
            true => EIR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == EIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == EIR::SET
    }
}
#[doc = "Possible values of the field `DOI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl DOIR {
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
            DOIR::RESET => false,
            DOIR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOIR {
        match value {
            false => DOIR::RESET,
            true => DOIR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == DOIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == DOIR::SET
    }
}
#[doc = "Possible values of the field `WUI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl WUIR {
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
            WUIR::RESET => false,
            WUIR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUIR {
        match value {
            false => WUIR::RESET,
            true => WUIR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == WUIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == WUIR::SET
    }
}
#[doc = "Possible values of the field `EPI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl EPIR {
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
            EPIR::RESET => false,
            EPIR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPIR {
        match value {
            false => EPIR::RESET,
            true => EPIR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == EPIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == EPIR::SET
    }
}
#[doc = "Possible values of the field `ALI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl ALIR {
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
            ALIR::RESET => false,
            ALIR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALIR {
        match value {
            false => ALIR::RESET,
            true => ALIR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == ALIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == ALIR::SET
    }
}
#[doc = "Possible values of the field `BEI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl BEIR {
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
            BEIR::RESET => false,
            BEIR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEIR {
        match value {
            false => BEIR::RESET,
            true => BEIR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == BEIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == BEIR::SET
    }
}
#[doc = "Possible values of the field `IDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDIR {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl IDIR {
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
            IDIR::RESET => false,
            IDIR::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDIR {
        match value {
            false => IDIR::RESET,
            true => IDIR::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == IDIR::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == IDIR::SET
    }
}
#[doc = "Possible values of the field `TI2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI2R {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl TI2R {
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
            TI2R::RESET => false,
            TI2R::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TI2R {
        match value {
            false => TI2R::RESET,
            true => TI2R::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TI2R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == TI2R::SET
    }
}
#[doc = "Possible values of the field `TI3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI3R {
    #[doc = "Reset"]
    RESET,
    #[doc = "Set"]
    SET,
}
impl TI3R {
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
            TI3R::RESET => false,
            TI3R::SET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TI3R {
        match value {
            false => TI3R::RESET,
            true => TI3R::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == TI3R::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == TI3R::SET
    }
}
#[doc = r" Value of the field"]
pub struct ERRBIT4_0R {
    bits: u8,
}
impl ERRBIT4_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ERRDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRDIRR {
    #[doc = "Error occurred during transmitting."]
    TX,
    #[doc = "Error occurred during receiving."]
    RX,
}
impl ERRDIRR {
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
            ERRDIRR::TX => false,
            ERRDIRR::RX => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRDIRR {
        match value {
            false => ERRDIRR::TX,
            true => ERRDIRR::RX,
        }
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline]
    pub fn is_tx(&self) -> bool {
        *self == ERRDIRR::TX
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline]
    pub fn is_rx(&self) -> bool {
        *self == ERRDIRR::RX
    }
}
#[doc = "Possible values of the field `ERRC1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRC1_0R {
    #[doc = "Bit error"]
    BIT_ERROR,
    #[doc = "Form error"]
    FORM_ERROR,
    #[doc = "Stuff error"]
    STUFF_ERROR,
    #[doc = "Other error"]
    OTHER_ERROR,
}
impl ERRC1_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ERRC1_0R::BIT_ERROR => 0,
            ERRC1_0R::FORM_ERROR => 1,
            ERRC1_0R::STUFF_ERROR => 2,
            ERRC1_0R::OTHER_ERROR => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ERRC1_0R {
        match value {
            0 => ERRC1_0R::BIT_ERROR,
            1 => ERRC1_0R::FORM_ERROR,
            2 => ERRC1_0R::STUFF_ERROR,
            3 => ERRC1_0R::OTHER_ERROR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIT_ERROR`"]
    #[inline]
    pub fn is_bit_error(&self) -> bool {
        *self == ERRC1_0R::BIT_ERROR
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR`"]
    #[inline]
    pub fn is_form_error(&self) -> bool {
        *self == ERRC1_0R::FORM_ERROR
    }
    #[doc = "Checks if the value of the field is `STUFF_ERROR`"]
    #[inline]
    pub fn is_stuff_error(&self) -> bool {
        *self == ERRC1_0R::STUFF_ERROR
    }
    #[doc = "Checks if the value of the field is `OTHER_ERROR`"]
    #[inline]
    pub fn is_other_error(&self) -> bool {
        *self == ERRC1_0R::OTHER_ERROR
    }
}
#[doc = r" Value of the field"]
pub struct ALCBITR {
    bits: u8,
}
impl ALCBITR {
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
    #[doc = "Bit 0 - Receive Interrupt. This bit is set whenever the RBS bit in CANxSR and the RIE bit in CANxIER are both 1, indicating that a new message was received and stored in the Receive Buffer. The Receive Interrupt Bit is not cleared upon a read access to the Interrupt Register. Giving the Command Release Receive Buffer will clear RI temporarily. If there is another message available within the Receive Buffer after the release command, RI is set again. Otherwise RI remains cleared."]
    #[inline]
    pub fn ri(&self) -> RIR {
        RIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transmit Interrupt 1. This bit is set when the TBS1 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB1 was successfully transmitted or aborted), indicating that Transmit buffer 1 is available, and the TIE1 bit in CANxIER is 1."]
    #[inline]
    pub fn ti1(&self) -> TI1R {
        TI1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Error Warning Interrupt. This bit is set on every change (set or clear) of either the Error Status or Bus Status bit in CANxSR and the EIE bit bit is set within the Interrupt Enable Register at the time of the change."]
    #[inline]
    pub fn ei(&self) -> EIR {
        EIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Data Overrun Interrupt. This bit is set when the DOS bit in CANxSR goes from 0 to 1 and the DOIE bit in CANxIER is 1."]
    #[inline]
    pub fn doi(&self) -> DOIR {
        DOIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Wake-Up Interrupt. This bit is set if the CAN controller is sleeping and bus activity is detected and the WUIE bit in CANxIER is 1. A Wake-Up Interrupt is also generated if the CPU tries to set the Sleep bit while the CAN controller is involved in bus activities or a CAN Interrupt is pending. The WUI flag can also get asserted when the according enable bit WUIE is not set. In this case a Wake-Up Interrupt does not get asserted."]
    #[inline]
    pub fn wui(&self) -> WUIR {
        WUIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Error Passive Interrupt. This bit is set if the EPIE bit in CANxIER is 1, and the CAN controller switches between Error Passive and Error Active mode in either direction. This is the case when the CAN Controller has reached the Error Passive Status (at least one error counter exceeds the CAN protocol defined level of 127) or if the CAN Controller is in Error Passive Status and enters the Error Active Status again."]
    #[inline]
    pub fn epi(&self) -> EPIR {
        EPIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Arbitration Lost Interrupt. This bit is set if the ALIE bit in CANxIER is 1, and the CAN controller loses arbitration while attempting to transmit. In this case the CAN node becomes a receiver."]
    #[inline]
    pub fn ali(&self) -> ALIR {
        ALIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Bus Error Interrupt -- this bit is set if the BEIE bit in CANxIER is 1, and the CAN controller detects an error on the bus."]
    #[inline]
    pub fn bei(&self) -> BEIR {
        BEIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - ID Ready Interrupt -- this bit is set if the IDIE bit in CANxIER is 1, and a CAN Identifier has been received (a message was successfully transmitted or aborted). This bit is set whenever a message was successfully transmitted or aborted and the IDIE bit is set in the IER register."]
    #[inline]
    pub fn idi(&self) -> IDIR {
        IDIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Transmit Interrupt 2. This bit is set when the TBS2 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB2 was successfully transmitted or aborted), indicating that Transmit buffer 2 is available, and the TIE2 bit in CANxIER is 1."]
    #[inline]
    pub fn ti2(&self) -> TI2R {
        TI2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transmit Interrupt 3. This bit is set when the TBS3 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB3 was successfully transmitted or aborted), indicating that Transmit buffer 3 is available, and the TIE3 bit in CANxIER is 1."]
    #[inline]
    pub fn ti3(&self) -> TI3R {
        TI3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Error Code Capture: when the CAN controller detects a bus error, the location of the error within the frame is captured in this field. The value reflects an internal state variable, and as a result is not very linear: 00011 = Start of Frame 00010 = ID28 ... ID21 00110 = ID20 ... ID18 00100 = SRTR Bit 00101 = IDE bit 00111 = ID17 ... 13 01111 = ID12 ... ID5 01110 = ID4 ... ID0 01100 = RTR Bit 01101 = Reserved Bit 1 01001 = Reserved Bit 0 01011 = Data Length Code 01010 = Data Field 01000 = CRC Sequence 11000 = CRC Delimiter 11001 = Acknowledge Slot 11011 = Acknowledge Delimiter 11010 = End of Frame 10010 = Intermission Whenever a bus error occurs, the corresponding bus error interrupt is forced, if enabled. At the same time, the current position of the Bit Stream Processor is captured into the Error Code Capture Register. The content within this register is fixed until the user software has read out its content once. From now on, the capture mechanism is activated again, i.e. reading the CANxICR enables another Bus Error Interrupt."]
    #[inline]
    pub fn errbit4_0(&self) -> ERRBIT4_0R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ERRBIT4_0R { bits }
    }
    #[doc = "Bit 21 - When the CAN controller detects a bus error, the direction of the current bit is captured in this bit."]
    #[inline]
    pub fn errdir(&self) -> ERRDIRR {
        ERRDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:23 - When the CAN controller detects a bus error, the type of error is captured in this field:"]
    #[inline]
    pub fn errc1_0(&self) -> ERRC1_0R {
        ERRC1_0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - Each time arbitration is lost while trying to send on the CAN, the bit number within the frame is captured into this field. After the content of ALCBIT is read, the ALI bit is cleared and a new Arbitration Lost interrupt can occur. 00 = arbitration lost in the first bit (MS) of identifier ... 11 = arbitration lost in SRTS bit (RTR bit for standard frame messages) 12 = arbitration lost in IDE bit 13 = arbitration lost in 12th bit of identifier (extended frame only) ... 30 = arbitration lost in last bit of identifier (extended frame only) 31 = arbitration lost in RTR bit (extended frame only) On arbitration lost, the corresponding arbitration lost interrupt is forced, if enabled. At that time, the current bit position of the Bit Stream Processor is captured into the Arbitration Lost Capture Register. The content within this register is fixed until the user application has read out its contents once. From now on, the capture mechanism is activated again."]
    #[inline]
    pub fn alcbit(&self) -> ALCBITR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ALCBITR { bits }
    }
}
