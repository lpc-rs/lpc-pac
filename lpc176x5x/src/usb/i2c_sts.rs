#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::I2C_STS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIR {
    #[doc = "Transaction has not completed."]
    NOT_COMPLETE,
    #[doc = "Transaction completed."]
    COMPLETE,
}
impl TDIR {
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
            TDIR::NOT_COMPLETE => false,
            TDIR::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDIR {
        match value {
            false => TDIR::NOT_COMPLETE,
            true => TDIR::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == TDIR::NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == TDIR::COMPLETE
    }
}
#[doc = "Possible values of the field `AFI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIR {
    #[doc = "No arbitration failure on last transmission."]
    NO_ARBITRATION_FAILU,
    #[doc = "Arbitration failure occurred on last transmission."]
    ARBITRATION_FAILURE_,
}
impl AFIR {
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
            AFIR::NO_ARBITRATION_FAILU => false,
            AFIR::ARBITRATION_FAILURE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AFIR {
        match value {
            false => AFIR::NO_ARBITRATION_FAILU,
            true => AFIR::ARBITRATION_FAILURE_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ARBITRATION_FAILU`"]
    #[inline]
    pub fn is_no_arbitration_failu(&self) -> bool {
        *self == AFIR::NO_ARBITRATION_FAILU
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_FAILURE_`"]
    #[inline]
    pub fn is_arbitration_failure_(&self) -> bool {
        *self == AFIR::ARBITRATION_FAILURE_
    }
}
#[doc = "Possible values of the field `NAI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAIR {
    #[doc = "Last transmission received an acknowledge."]
    ACKNOWLEDGE_RCVD,
    #[doc = "Last transmission did not receive an acknowledge."]
    NO_ACKNOWLEDGE_RCVD,
}
impl NAIR {
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
            NAIR::ACKNOWLEDGE_RCVD => false,
            NAIR::NO_ACKNOWLEDGE_RCVD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NAIR {
        match value {
            false => NAIR::ACKNOWLEDGE_RCVD,
            true => NAIR::NO_ACKNOWLEDGE_RCVD,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOWLEDGE_RCVD`"]
    #[inline]
    pub fn is_acknowledge_rcvd(&self) -> bool {
        *self == NAIR::ACKNOWLEDGE_RCVD
    }
    #[doc = "Checks if the value of the field is `NO_ACKNOWLEDGE_RCVD`"]
    #[inline]
    pub fn is_no_acknowledge_rcvd(&self) -> bool {
        *self == NAIR::NO_ACKNOWLEDGE_RCVD
    }
}
#[doc = "Possible values of the field `DRMI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMIR {
    #[doc = "Master transmitter does not need data."]
    BUSY,
    #[doc = "Master transmitter needs data."]
    NEED_DATA,
}
impl DRMIR {
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
            DRMIR::BUSY => false,
            DRMIR::NEED_DATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRMIR {
        match value {
            false => DRMIR::BUSY,
            true => DRMIR::NEED_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == DRMIR::BUSY
    }
    #[doc = "Checks if the value of the field is `NEED_DATA`"]
    #[inline]
    pub fn is_need_data(&self) -> bool {
        *self == DRMIR::NEED_DATA
    }
}
#[doc = "Possible values of the field `DRSI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRSIR {
    #[doc = "Slave transmitter does not need data."]
    BUSY,
    #[doc = "Slave transmitter needs data."]
    NEED_DATA,
}
impl DRSIR {
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
            DRSIR::BUSY => false,
            DRSIR::NEED_DATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DRSIR {
        match value {
            false => DRSIR::BUSY,
            true => DRSIR::NEED_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == DRSIR::BUSY
    }
    #[doc = "Checks if the value of the field is `NEED_DATA`"]
    #[inline]
    pub fn is_need_data(&self) -> bool {
        *self == DRSIR::NEED_DATA
    }
}
#[doc = r" Value of the field"]
pub struct ACTIVER {
    bits: bool,
}
impl ACTIVER {
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
pub struct SCLR {
    bits: bool,
}
impl SCLR {
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
pub struct SDAR {
    bits: bool,
}
impl SDAR {
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
#[doc = "Possible values of the field `RFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFFR {
    #[doc = "RX FIFO is not full"]
    RX_FIFO_IS_NOT_FULL,
    #[doc = "RX FIFO is full"]
    RX_FIFO_IS_FULL,
}
impl RFFR {
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
            RFFR::RX_FIFO_IS_NOT_FULL => false,
            RFFR::RX_FIFO_IS_FULL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFFR {
        match value {
            false => RFFR::RX_FIFO_IS_NOT_FULL,
            true => RFFR::RX_FIFO_IS_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_IS_NOT_FULL`"]
    #[inline]
    pub fn is_rx_fifo_is_not_full(&self) -> bool {
        *self == RFFR::RX_FIFO_IS_NOT_FULL
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_IS_FULL`"]
    #[inline]
    pub fn is_rx_fifo_is_full(&self) -> bool {
        *self == RFFR::RX_FIFO_IS_FULL
    }
}
#[doc = "Possible values of the field `RFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFER {
    #[doc = "RX FIFO contains data."]
    DATA,
    #[doc = "RX FIFO is empty"]
    EMPTY,
}
impl RFER {
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
            RFER::DATA => false,
            RFER::EMPTY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFER {
        match value {
            false => RFER::DATA,
            true => RFER::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline]
    pub fn is_data(&self) -> bool {
        *self == RFER::DATA
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == RFER::EMPTY
    }
}
#[doc = "Possible values of the field `TFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFR {
    #[doc = "TX FIFO is not full."]
    TX_FIFO_IS_NOT_FULL_,
    #[doc = "TX FIFO is full"]
    TX_FIFO_IS_FULL,
}
impl TFFR {
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
            TFFR::TX_FIFO_IS_NOT_FULL_ => false,
            TFFR::TX_FIFO_IS_FULL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFFR {
        match value {
            false => TFFR::TX_FIFO_IS_NOT_FULL_,
            true => TFFR::TX_FIFO_IS_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_IS_NOT_FULL_`"]
    #[inline]
    pub fn is_tx_fifo_is_not_full_(&self) -> bool {
        *self == TFFR::TX_FIFO_IS_NOT_FULL_
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_IS_FULL`"]
    #[inline]
    pub fn is_tx_fifo_is_full(&self) -> bool {
        *self == TFFR::TX_FIFO_IS_FULL
    }
}
#[doc = "Possible values of the field `TFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFER {
    #[doc = "TX FIFO contains valid data."]
    VALID_DATA,
    #[doc = "TX FIFO is empty"]
    EMPTY,
}
impl TFER {
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
            TFER::VALID_DATA => false,
            TFER::EMPTY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFER {
        match value {
            false => TFER::VALID_DATA,
            true => TFER::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_DATA`"]
    #[inline]
    pub fn is_valid_data(&self) -> bool {
        *self == TFER::VALID_DATA
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == TFER::EMPTY
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
    #[inline]
    pub fn tdi(&self) -> TDIR {
        TDIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
    #[inline]
    pub fn afi(&self) -> AFIR {
        AFIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
    #[inline]
    pub fn nai(&self) -> NAIR {
        NAIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
    #[inline]
    pub fn drmi(&self) -> DRMIR {
        DRMIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
    #[inline]
    pub fn drsi(&self) -> DRSIR {
        DRSIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
    #[inline]
    pub fn active(&self) -> ACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVER { bits }
    }
    #[doc = "Bit 6 - The current value of the SCL signal."]
    #[inline]
    pub fn scl(&self) -> SCLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLR { bits }
    }
    #[doc = "Bit 7 - The current value of the SDA signal."]
    #[inline]
    pub fn sda(&self) -> SDAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDAR { bits }
    }
    #[doc = "Bit 8 - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
    #[inline]
    pub fn rff(&self) -> RFFR {
        RFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
    #[inline]
    pub fn rfe(&self) -> RFER {
        RFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
    #[inline]
    pub fn tff(&self) -> TFFR {
        TFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
    #[inline]
    pub fn tfe(&self) -> TFER {
        TFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
