#[doc = "Reader of register INTERRSTAT"]
pub type R = crate::R<u32, super::INTERRSTAT>;
#[doc = "Reader of field `INTERRSTAT0`"]
pub type INTERRSTAT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTERRSTAT1`"]
pub type INTERRSTAT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTERRSTAT2`"]
pub type INTERRSTAT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTERRSTAT3`"]
pub type INTERRSTAT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTERRSTAT4`"]
pub type INTERRSTAT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTERRSTAT5`"]
pub type INTERRSTAT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTERRSTAT6`"]
pub type INTERRSTAT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTERRSTAT7`"]
pub type INTERRSTAT7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat0(&self) -> INTERRSTAT0_R {
        INTERRSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat1(&self) -> INTERRSTAT1_R {
        INTERRSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat2(&self) -> INTERRSTAT2_R {
        INTERRSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat3(&self) -> INTERRSTAT3_R {
        INTERRSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat4(&self) -> INTERRSTAT4_R {
        INTERRSTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat5(&self) -> INTERRSTAT5_R {
        INTERRSTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat6(&self) -> INTERRSTAT6_R {
        INTERRSTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat7(&self) -> INTERRSTAT7_R {
        INTERRSTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
