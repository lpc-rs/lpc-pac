#[doc = "Reader of register LSR"]
pub type R = crate::R<u32, super::LSR>;
#[doc = "Reader of field `RDR`"]
pub type RDR_R = crate::R<bool, bool>;
#[doc = "Reader of field `OE`"]
pub type OE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FE`"]
pub type FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `THRE`"]
pub type THRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMT`"]
pub type TEMT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFE`"]
pub type RXFE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Receiver Data Ready."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Error."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity Error."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Framing Error."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty."]
    #[inline(always)]
    pub fn thre(&self) -> THRE_R {
        THRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty."]
    #[inline(always)]
    pub fn temt(&self) -> TEMT_R {
        TEMT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
