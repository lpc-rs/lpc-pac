#[doc = "Reader of register LSR"]
pub type R = crate::R<u32, super::LSR>;
#[doc = "Receiver Data Ready. LSR\\[0\\] is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDR_A {
    #[doc = "0: The UART1 receiver FIFO is empty."]
    EMPTY,
    #[doc = "1: The UART1 receiver FIFO is not empty."]
    NOTEMPTY,
}
impl From<RDR_A> for bool {
    #[inline(always)]
    fn from(variant: RDR_A) -> Self {
        match variant {
            RDR_A::EMPTY => false,
            RDR_A::NOTEMPTY => true,
        }
    }
}
#[doc = "Reader of field `RDR`"]
pub type RDR_R = crate::R<bool, RDR_A>;
impl RDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDR_A {
        match self.bits {
            false => RDR_A::EMPTY,
            true => RDR_A::NOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RDR_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_notempty(&self) -> bool {
        *self == RDR_A::NOTEMPTY
    }
}
#[doc = "Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\] is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OE_A {
    #[doc = "0: Overrun error status is inactive."]
    INACTIVE,
    #[doc = "1: Overrun error status is active."]
    ACTIVE,
}
impl From<OE_A> for bool {
    #[inline(always)]
    fn from(variant: OE_A) -> Self {
        match variant {
            OE_A::INACTIVE => false,
            OE_A::ACTIVE => true,
        }
    }
}
#[doc = "Reader of field `OE`"]
pub type OE_R = crate::R<bool, OE_A>;
impl OE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OE_A {
        match self.bits {
            false => OE_A::INACTIVE,
            true => OE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == OE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == OE_A::ACTIVE
    }
}
#[doc = "Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: Parity error status is inactive."]
    INACTIVE,
    #[doc = "1: Parity error status is active."]
    ACTIVE,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        match variant {
            PE_A::INACTIVE => false,
            PE_A::ACTIVE => true,
        }
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::INACTIVE,
            true => PE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == PE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == PE_A::ACTIVE
    }
}
#[doc = "Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_A {
    #[doc = "0: Framing error status is inactive."]
    INACTIVE,
    #[doc = "1: Framing error status is active."]
    ACTIVE,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        match variant {
            FE_A::INACTIVE => false,
            FE_A::ACTIVE => true,
        }
    }
}
#[doc = "Reader of field `FE`"]
pub type FE_R = crate::R<bool, FE_A>;
impl FE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::INACTIVE,
            true => FE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == FE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == FE_A::ACTIVE
    }
}
#[doc = "Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BI_A {
    #[doc = "0: Break interrupt status is inactive."]
    INACTIVE,
    #[doc = "1: Break interrupt status is active."]
    ACTIVE,
}
impl From<BI_A> for bool {
    #[inline(always)]
    fn from(variant: BI_A) -> Self {
        match variant {
            BI_A::INACTIVE => false,
            BI_A::ACTIVE => true,
        }
    }
}
#[doc = "Reader of field `BI`"]
pub type BI_R = crate::R<bool, BI_A>;
impl BI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BI_A {
        match self.bits {
            false => BI_A::INACTIVE,
            true => BI_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BI_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BI_A::ACTIVE
    }
}
#[doc = "Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRE_A {
    #[doc = "0: THR contains valid data."]
    VALID,
    #[doc = "1: THR is empty."]
    THR_IS_EMPTY_,
}
impl From<THRE_A> for bool {
    #[inline(always)]
    fn from(variant: THRE_A) -> Self {
        match variant {
            THRE_A::VALID => false,
            THRE_A::THR_IS_EMPTY_ => true,
        }
    }
}
#[doc = "Reader of field `THRE`"]
pub type THRE_R = crate::R<bool, THRE_A>;
impl THRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRE_A {
        match self.bits {
            false => THRE_A::VALID,
            true => THRE_A::THR_IS_EMPTY_,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == THRE_A::VALID
    }
    #[doc = "Checks if the value of the field is `THR_IS_EMPTY_`"]
    #[inline(always)]
    pub fn is_thr_is_empty_(&self) -> bool {
        *self == THRE_A::THR_IS_EMPTY_
    }
}
#[doc = "Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMT_A {
    #[doc = "0: THR and/or the TSR contains valid data."]
    VALID,
    #[doc = "1: THR and the TSR are empty."]
    EMPTY,
}
impl From<TEMT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMT_A) -> Self {
        match variant {
            TEMT_A::VALID => false,
            TEMT_A::EMPTY => true,
        }
    }
}
#[doc = "Reader of field `TEMT`"]
pub type TEMT_R = crate::R<bool, TEMT_A>;
impl TEMT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMT_A {
        match self.bits {
            false => TEMT_A::VALID,
            true => TEMT_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == TEMT_A::VALID
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TEMT_A::EMPTY
    }
}
#[doc = "Error in RX FIFO. LSR\\[7\\] is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFE_A {
    #[doc = "0: RBR contains no UART1 RX errors or FCR\\[0\\]=0."]
    NOERROR,
    #[doc = "1: UART1 RBR contains at least one UART1 RX error."]
    ERRORS,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        match variant {
            RXFE_A::NOERROR => false,
            RXFE_A::ERRORS => true,
        }
    }
}
#[doc = "Reader of field `RXFE`"]
pub type RXFE_R = crate::R<bool, RXFE_A>;
impl RXFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFE_A {
        match self.bits {
            false => RXFE_A::NOERROR,
            true => RXFE_A::ERRORS,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == RXFE_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERRORS`"]
    #[inline(always)]
    pub fn is_errors(&self) -> bool {
        *self == RXFE_A::ERRORS
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Data Ready. LSR\\[0\\] is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\] is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn bi(&self) -> BI_R {
        BI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write."]
    #[inline(always)]
    pub fn thre(&self) -> THRE_R {
        THRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
    #[inline(always)]
    pub fn temt(&self) -> TEMT_R {
        TEMT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO. LSR\\[7\\] is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
