#[doc = "Reader of register INTSTATUS"]
pub type R = crate::R<u32, super::INTSTATUS>;
#[doc = "Reader of field `RXOVERRUNINT`"]
pub type RXOVERRUNINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXERRORINT`"]
pub type RXERRORINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFINISHEDINT`"]
pub type RXFINISHEDINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDONEINT`"]
pub type RXDONEINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUNDERRUNINT`"]
pub type TXUNDERRUNINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXERRORINT`"]
pub type TXERRORINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFINISHEDINT`"]
pub type TXFINISHEDINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXDONEINT`"]
pub type TXDONEINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOFTINT`"]
pub type SOFTINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WAKEUPINT`"]
pub type WAKEUPINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt set on a fatal overrun error in the receive queue. The fatal interrupt should be resolved by a Rx soft-reset. The bit is not set when there is a nonfatal overrun error."]
    #[inline(always)]
    pub fn rxoverrunint(&self) -> RXOVERRUNINT_R {
        RXOVERRUNINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt trigger on receive errors: AlignmentError, RangeError, LengthError, SymbolError, CRCError or NoDescriptor or Overrun."]
    #[inline(always)]
    pub fn rxerrorint(&self) -> RXERRORINT_R {
        RXERRORINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedint(&self) -> RXFINISHEDINT_R {
        RXFINISHEDINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneint(&self) -> RXDONEINT_R {
        RXDONEINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt set on a fatal underrun error in the transmit queue. The fatal interrupt should be resolved by a Tx soft-reset. The bit is not set when there is a nonfatal underrun error."]
    #[inline(always)]
    pub fn txunderrunint(&self) -> TXUNDERRUNINT_R {
        TXUNDERRUNINT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt trigger on transmit errors: LateCollision, ExcessiveCollision and ExcessiveDefer, NoDescriptor or Underrun."]
    #[inline(always)]
    pub fn txerrorint(&self) -> TXERRORINT_R {
        TXERRORINT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedint(&self) -> TXFINISHEDINT_R {
        TXFINISHEDINT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneint(&self) -> TXDONEINT_R {
        TXDONEINT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt triggered by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softint(&self) -> SOFTINT_R {
        SOFTINT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupint(&self) -> WAKEUPINT_R {
        WAKEUPINT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
