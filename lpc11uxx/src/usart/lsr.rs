#[doc = "Register `LSR` reader"]
pub struct R(crate::R<LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LSR_SPEC>> for R {
    fn from(reader: crate::R<LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Receiver Data Ready:LSR\\[0\\]
is set when the RBR holds an unread character and is cleared when the USART RBR FIFO is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDR_A {
    #[doc = "0: RBR is empty."]
    EMPTY = 0,
    #[doc = "1: RBR contains valid data."]
    VALID = 1,
}
impl From<RDR_A> for bool {
    #[inline(always)]
    fn from(variant: RDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDR` reader - Receiver Data Ready:LSR\\[0\\]
is set when the RBR holds an unread character and is cleared when the USART RBR FIFO is empty."]
pub struct RDR_R(crate::FieldReader<bool, RDR_A>);
impl RDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDR_A {
        match self.bits {
            false => RDR_A::EMPTY,
            true => RDR_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == RDR_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == RDR_A::VALID
    }
}
impl core::ops::Deref for RDR_R {
    type Target = crate::FieldReader<bool, RDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Overrun Error. The overrun error condition is set as soon as it occurs. A LSR read clears LSR\\[1\\]. LSR\\[1\\]
is set when USART RSR has a new character assembled and the USART RBR FIFO is full. In this case, the USART RBR FIFO will not be overwritten and the character in the USART RSR will be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OE_A {
    #[doc = "0: Overrun error status is inactive."]
    INACTIVE = 0,
    #[doc = "1: Overrun error status is active."]
    ACTIVE = 1,
}
impl From<OE_A> for bool {
    #[inline(always)]
    fn from(variant: OE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OE` reader - Overrun Error. The overrun error condition is set as soon as it occurs. A LSR read clears LSR\\[1\\]. LSR\\[1\\]
is set when USART RSR has a new character assembled and the USART RBR FIFO is full. In this case, the USART RBR FIFO will not be overwritten and the character in the USART RSR will be lost."]
pub struct OE_R(crate::FieldReader<bool, OE_A>);
impl OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_R(crate::FieldReader::new(bits))
    }
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
        **self == OE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == OE_A::ACTIVE
    }
}
impl core::ops::Deref for OE_R {
    type Target = crate::FieldReader<bool, OE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. A LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the USART RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: Parity error status is inactive."]
    INACTIVE = 0,
    #[doc = "1: Parity error status is active."]
    ACTIVE = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. A LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the USART RBR FIFO."]
pub struct PE_R(crate::FieldReader<bool, PE_A>);
impl PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE_R(crate::FieldReader::new(bits))
    }
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
        **self == PE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == PE_A::ACTIVE
    }
}
impl core::ops::Deref for PE_R {
    type Target = crate::FieldReader<bool, PE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. A LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to re-synchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the USART RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FE_A {
    #[doc = "0: Framing error status is inactive."]
    INACTIVE = 0,
    #[doc = "1: Framing error status is active."]
    ACTIVE = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. A LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to re-synchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the USART RBR FIFO."]
pub struct FE_R(crate::FieldReader<bool, FE_A>);
impl FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FE_R(crate::FieldReader::new(bits))
    }
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
        **self == FE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == FE_A::ACTIVE
    }
}
impl core::ops::Deref for FE_R {
    type Target = crate::FieldReader<bool, FE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Break Interrupt. When RXD1 is held in the spacing state (all zeros) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). A LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the USART RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BI_A {
    #[doc = "0: Break interrupt status is inactive."]
    INACTIVE = 0,
    #[doc = "1: Break interrupt status is active."]
    ACTIVE = 1,
}
impl From<BI_A> for bool {
    #[inline(always)]
    fn from(variant: BI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BI` reader - Break Interrupt. When RXD1 is held in the spacing state (all zeros) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). A LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the USART RBR FIFO."]
pub struct BI_R(crate::FieldReader<bool, BI_A>);
impl BI_R {
    pub(crate) fn new(bits: bool) -> Self {
        BI_R(crate::FieldReader::new(bits))
    }
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
        **self == BI_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == BI_A::ACTIVE
    }
}
impl core::ops::Deref for BI_R {
    type Target = crate::FieldReader<bool, BI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty USART THR and is cleared on a THR write.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRE_A {
    #[doc = "0: THR contains valid data."]
    VALID = 0,
    #[doc = "1: THR is empty."]
    EMPTY = 1,
}
impl From<THRE_A> for bool {
    #[inline(always)]
    fn from(variant: THRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THRE` reader - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty USART THR and is cleared on a THR write."]
pub struct THRE_R(crate::FieldReader<bool, THRE_A>);
impl THRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        THRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRE_A {
        match self.bits {
            false => THRE_A::VALID,
            true => THRE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == THRE_A::VALID
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == THRE_A::EMPTY
    }
}
impl core::ops::Deref for THRE_R {
    type Target = crate::FieldReader<bool, THRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMT_A {
    #[doc = "0: THR and/or the TSR contains valid data."]
    VALID = 0,
    #[doc = "1: THR and the TSR are empty."]
    EMPTY = 1,
}
impl From<TEMT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMT` reader - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
pub struct TEMT_R(crate::FieldReader<bool, TEMT_A>);
impl TEMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEMT_R(crate::FieldReader::new(bits))
    }
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
        **self == TEMT_A::VALID
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == TEMT_A::EMPTY
    }
}
impl core::ops::Deref for TEMT_R {
    type Target = crate::FieldReader<bool, TEMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Error in RX FIFO. LSR\\[7\\]
is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the USART FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFE_A {
    #[doc = "0: RBR contains no USART RX errors or FCR\\[0\\]=0."]
    NO_ERROR = 0,
    #[doc = "1: USART RBR contains at least one USART RX error."]
    ERRO = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - Error in RX FIFO. LSR\\[7\\]
is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the USART FIFO."]
pub struct RXFE_R(crate::FieldReader<bool, RXFE_A>);
impl RXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFE_A {
        match self.bits {
            false => RXFE_A::NO_ERROR,
            true => RXFE_A::ERRO,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == RXFE_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERRO`"]
    #[inline(always)]
    pub fn is_erro(&self) -> bool {
        **self == RXFE_A::ERRO
    }
}
impl core::ops::Deref for RXFE_R {
    type Target = crate::FieldReader<bool, RXFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXERR` reader - Tx Error. In smart card T=0 operation, this bit is set when the smart card has NACKed a transmitted character, one more than the number of times indicated by the TXRETRY field."]
pub struct TXERR_R(crate::FieldReader<bool, bool>);
impl TXERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Data Ready:LSR\\[0\\]
is set when the RBR holds an unread character and is cleared when the USART RBR FIFO is empty."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. A LSR read clears LSR\\[1\\]. LSR\\[1\\]
is set when USART RSR has a new character assembled and the USART RBR FIFO is full. In this case, the USART RBR FIFO will not be overwritten and the character in the USART RSR will be lost."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. A LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the USART RBR FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. A LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to re-synchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the USART RBR FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt. When RXD1 is held in the spacing state (all zeros) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). A LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the USART RBR FIFO."]
    #[inline(always)]
    pub fn bi(&self) -> BI_R {
        BI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty USART THR and is cleared on a THR write."]
    #[inline(always)]
    pub fn thre(&self) -> THRE_R {
        THRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
    #[inline(always)]
    pub fn temt(&self) -> TEMT_R {
        TEMT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO. LSR\\[7\\]
is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the USART FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Tx Error. In smart card T=0 operation, this bit is set when the smart card has NACKed a transmitted character, one more than the number of times indicated by the TXRETRY field."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](index.html) module"]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsr::R](R) reader structure"]
impl crate::Readable for LSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSR to value 0x60"]
impl crate::Resettable for LSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x60
    }
}
