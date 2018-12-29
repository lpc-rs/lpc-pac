#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRR {
    #[doc = "The UART1 receiver FIFO is empty."]
    EMPTY,
    #[doc = "The UART1 receiver FIFO is not empty."]
    NOTEMPTY,
}
impl RDRR {
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
            RDRR::EMPTY => false,
            RDRR::NOTEMPTY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDRR {
        match value {
            false => RDRR::EMPTY,
            true => RDRR::NOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == RDRR::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline]
    pub fn is_notempty(&self) -> bool {
        *self == RDRR::NOTEMPTY
    }
}
#[doc = "Possible values of the field `OE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OER {
    #[doc = "Overrun error status is inactive."]
    INACTIVE,
    #[doc = "Overrun error status is active."]
    ACTIVE,
}
impl OER {
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
            OER::INACTIVE => false,
            OER::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OER {
        match value {
            false => OER::INACTIVE,
            true => OER::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == OER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == OER::ACTIVE
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Parity error status is inactive."]
    INACTIVE,
    #[doc = "Parity error status is active."]
    ACTIVE,
}
impl PER {
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
            PER::INACTIVE => false,
            PER::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::INACTIVE,
            true => PER::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == PER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == PER::ACTIVE
    }
}
#[doc = "Possible values of the field `FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER {
    #[doc = "Framing error status is inactive."]
    INACTIVE,
    #[doc = "Framing error status is active."]
    ACTIVE,
}
impl FER {
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
            FER::INACTIVE => false,
            FER::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FER {
        match value {
            false => FER::INACTIVE,
            true => FER::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == FER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == FER::ACTIVE
    }
}
#[doc = "Possible values of the field `BI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIR {
    #[doc = "Break interrupt status is inactive."]
    INACTIVE,
    #[doc = "Break interrupt status is active."]
    ACTIVE,
}
impl BIR {
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
            BIR::INACTIVE => false,
            BIR::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIR {
        match value {
            false => BIR::INACTIVE,
            true => BIR::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == BIR::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == BIR::ACTIVE
    }
}
#[doc = "Possible values of the field `THRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRER {
    #[doc = "THR contains valid data."]
    VALID,
    #[doc = "THR is empty."]
    THR_IS_EMPTY_,
}
impl THRER {
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
            THRER::VALID => false,
            THRER::THR_IS_EMPTY_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> THRER {
        match value {
            false => THRER::VALID,
            true => THRER::THR_IS_EMPTY_,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == THRER::VALID
    }
    #[doc = "Checks if the value of the field is `THR_IS_EMPTY_`"]
    #[inline]
    pub fn is_thr_is_empty_(&self) -> bool {
        *self == THRER::THR_IS_EMPTY_
    }
}
#[doc = "Possible values of the field `TEMT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMTR {
    #[doc = "THR and/or the TSR contains valid data."]
    VALID,
    #[doc = "THR and the TSR are empty."]
    EMPTY,
}
impl TEMTR {
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
            TEMTR::VALID => false,
            TEMTR::EMPTY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEMTR {
        match value {
            false => TEMTR::VALID,
            true => TEMTR::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == TEMTR::VALID
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == TEMTR::EMPTY
    }
}
#[doc = "Possible values of the field `RXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFER {
    #[doc = "RBR contains no UART1 RX errors or FCR\\[0\\]=0."]
    NOERROR,
    #[doc = "UART1 RBR contains at least one UART1 RX error."]
    ERRORS,
}
impl RXFER {
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
            RXFER::NOERROR => false,
            RXFER::ERRORS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFER {
        match value {
            false => RXFER::NOERROR,
            true => RXFER::ERRORS,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_noerror(&self) -> bool {
        *self == RXFER::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERRORS`"]
    #[inline]
    pub fn is_errors(&self) -> bool {
        *self == RXFER::ERRORS
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receiver Data Ready. LSR\\[0\\] is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty."]
    #[inline]
    pub fn rdr(&self) -> RDRR {
        RDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\] is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost."]
    #[inline]
    pub fn oe(&self) -> OER {
        OER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline]
    pub fn fe(&self) -> FER {
        FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline]
    pub fn bi(&self) -> BIR {
        BIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write."]
    #[inline]
    pub fn thre(&self) -> THRER {
        THRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
    #[inline]
    pub fn temt(&self) -> TEMTR {
        TEMTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Error in RX FIFO. LSR\\[7\\] is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO."]
    #[inline]
    pub fn rxfe(&self) -> RXFER {
        RXFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
