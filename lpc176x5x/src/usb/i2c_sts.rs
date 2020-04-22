#[doc = "Reader of register I2C_STS"]
pub type R = crate::R<u32, super::I2C_STS>;
#[doc = "Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDI_A {
    #[doc = "0: Transaction has not completed."]
    NOT_COMPLETE = 0,
    #[doc = "1: Transaction completed."]
    COMPLETE = 1,
}
impl From<TDI_A> for bool {
    #[inline(always)]
    fn from(variant: TDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDI`"]
pub type TDI_R = crate::R<bool, TDI_A>;
impl TDI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDI_A {
        match self.bits {
            false => TDI_A::NOT_COMPLETE,
            true => TDI_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TDI_A::NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TDI_A::COMPLETE
    }
}
#[doc = "Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFI_A {
    #[doc = "0: No arbitration failure on last transmission."]
    NO_ARBITRATION_FAILU = 0,
    #[doc = "1: Arbitration failure occurred on last transmission."]
    ARBITRATION_FAILURE_ = 1,
}
impl From<AFI_A> for bool {
    #[inline(always)]
    fn from(variant: AFI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AFI`"]
pub type AFI_R = crate::R<bool, AFI_A>;
impl AFI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFI_A {
        match self.bits {
            false => AFI_A::NO_ARBITRATION_FAILU,
            true => AFI_A::ARBITRATION_FAILURE_,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ARBITRATION_FAILU`"]
    #[inline(always)]
    pub fn is_no_arbitration_failu(&self) -> bool {
        *self == AFI_A::NO_ARBITRATION_FAILU
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_FAILURE_`"]
    #[inline(always)]
    pub fn is_arbitration_failure_(&self) -> bool {
        *self == AFI_A::ARBITRATION_FAILURE_
    }
}
#[doc = "No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAI_A {
    #[doc = "0: Last transmission received an acknowledge."]
    ACKNOWLEDGE_RCVD = 0,
    #[doc = "1: Last transmission did not receive an acknowledge."]
    NO_ACKNOWLEDGE_RCVD = 1,
}
impl From<NAI_A> for bool {
    #[inline(always)]
    fn from(variant: NAI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NAI`"]
pub type NAI_R = crate::R<bool, NAI_A>;
impl NAI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NAI_A {
        match self.bits {
            false => NAI_A::ACKNOWLEDGE_RCVD,
            true => NAI_A::NO_ACKNOWLEDGE_RCVD,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOWLEDGE_RCVD`"]
    #[inline(always)]
    pub fn is_acknowledge_rcvd(&self) -> bool {
        *self == NAI_A::ACKNOWLEDGE_RCVD
    }
    #[doc = "Checks if the value of the field is `NO_ACKNOWLEDGE_RCVD`"]
    #[inline(always)]
    pub fn is_no_acknowledge_rcvd(&self) -> bool {
        *self == NAI_A::NO_ACKNOWLEDGE_RCVD
    }
}
#[doc = "Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRMI_A {
    #[doc = "0: Master transmitter does not need data."]
    BUSY = 0,
    #[doc = "1: Master transmitter needs data."]
    NEED_DATA = 1,
}
impl From<DRMI_A> for bool {
    #[inline(always)]
    fn from(variant: DRMI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRMI`"]
pub type DRMI_R = crate::R<bool, DRMI_A>;
impl DRMI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRMI_A {
        match self.bits {
            false => DRMI_A::BUSY,
            true => DRMI_A::NEED_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == DRMI_A::BUSY
    }
    #[doc = "Checks if the value of the field is `NEED_DATA`"]
    #[inline(always)]
    pub fn is_need_data(&self) -> bool {
        *self == DRMI_A::NEED_DATA
    }
}
#[doc = "Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRSI_A {
    #[doc = "0: Slave transmitter does not need data."]
    BUSY = 0,
    #[doc = "1: Slave transmitter needs data."]
    NEED_DATA = 1,
}
impl From<DRSI_A> for bool {
    #[inline(always)]
    fn from(variant: DRSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRSI`"]
pub type DRSI_R = crate::R<bool, DRSI_A>;
impl DRSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRSI_A {
        match self.bits {
            false => DRSI_A::BUSY,
            true => DRSI_A::NEED_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == DRSI_A::BUSY
    }
    #[doc = "Checks if the value of the field is `NEED_DATA`"]
    #[inline(always)]
    pub fn is_need_data(&self) -> bool {
        *self == DRSI_A::NEED_DATA
    }
}
#[doc = "Reader of field `Active`"]
pub type ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCL`"]
pub type SCL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDA`"]
pub type SDA_R = crate::R<bool, bool>;
#[doc = "Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFF_A {
    #[doc = "0: RX FIFO is not full"]
    RX_FIFO_IS_NOT_FULL = 0,
    #[doc = "1: RX FIFO is full"]
    RX_FIFO_IS_FULL = 1,
}
impl From<RFF_A> for bool {
    #[inline(always)]
    fn from(variant: RFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RFF`"]
pub type RFF_R = crate::R<bool, RFF_A>;
impl RFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFF_A {
        match self.bits {
            false => RFF_A::RX_FIFO_IS_NOT_FULL,
            true => RFF_A::RX_FIFO_IS_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_IS_NOT_FULL`"]
    #[inline(always)]
    pub fn is_rx_fifo_is_not_full(&self) -> bool {
        *self == RFF_A::RX_FIFO_IS_NOT_FULL
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_IS_FULL`"]
    #[inline(always)]
    pub fn is_rx_fifo_is_full(&self) -> bool {
        *self == RFF_A::RX_FIFO_IS_FULL
    }
}
#[doc = "Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFE_A {
    #[doc = "0: RX FIFO contains data."]
    DATA = 0,
    #[doc = "1: RX FIFO is empty"]
    EMPTY = 1,
}
impl From<RFE_A> for bool {
    #[inline(always)]
    fn from(variant: RFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RFE`"]
pub type RFE_R = crate::R<bool, RFE_A>;
impl RFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFE_A {
        match self.bits {
            false => RFE_A::DATA,
            true => RFE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RFE_A::DATA
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RFE_A::EMPTY
    }
}
#[doc = "Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFF_A {
    #[doc = "0: TX FIFO is not full."]
    TX_FIFO_IS_NOT_FULL_ = 0,
    #[doc = "1: TX FIFO is full"]
    TX_FIFO_IS_FULL = 1,
}
impl From<TFF_A> for bool {
    #[inline(always)]
    fn from(variant: TFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TFF`"]
pub type TFF_R = crate::R<bool, TFF_A>;
impl TFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFF_A {
        match self.bits {
            false => TFF_A::TX_FIFO_IS_NOT_FULL_,
            true => TFF_A::TX_FIFO_IS_FULL,
        }
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_IS_NOT_FULL_`"]
    #[inline(always)]
    pub fn is_tx_fifo_is_not_full_(&self) -> bool {
        *self == TFF_A::TX_FIFO_IS_NOT_FULL_
    }
    #[doc = "Checks if the value of the field is `TX_FIFO_IS_FULL`"]
    #[inline(always)]
    pub fn is_tx_fifo_is_full(&self) -> bool {
        *self == TFF_A::TX_FIFO_IS_FULL
    }
}
#[doc = "Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFE_A {
    #[doc = "0: TX FIFO contains valid data."]
    VALID_DATA = 0,
    #[doc = "1: TX FIFO is empty"]
    EMPTY = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, TFE_A>;
impl TFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::VALID_DATA,
            true => TFE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_DATA`"]
    #[inline(always)]
    pub fn is_valid_data(&self) -> bool {
        *self == TFE_A::VALID_DATA
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TFE_A::EMPTY
    }
}
impl R {
    #[doc = "Bit 0 - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
    #[inline(always)]
    pub fn tdi(&self) -> TDI_R {
        TDI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
    #[inline(always)]
    pub fn afi(&self) -> AFI_R {
        AFI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn nai(&self) -> NAI_R {
        NAI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn drmi(&self) -> DRMI_R {
        DRMI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
    #[inline(always)]
    pub fn drsi(&self) -> DRSI_R {
        DRSI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The current value of the SCL signal."]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The current value of the SDA signal."]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
