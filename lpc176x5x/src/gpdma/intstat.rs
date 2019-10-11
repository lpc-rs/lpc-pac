#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Reader of field `INTSTAT0`"]
pub type INTSTAT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT1`"]
pub type INTSTAT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT2`"]
pub type INTSTAT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT3`"]
pub type INTSTAT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT4`"]
pub type INTSTAT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT5`"]
pub type INTSTAT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT6`"]
pub type INTSTAT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTSTAT7`"]
pub type INTSTAT7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat0(&self) -> INTSTAT0_R {
        INTSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat1(&self) -> INTSTAT1_R {
        INTSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat2(&self) -> INTSTAT2_R {
        INTSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat3(&self) -> INTSTAT3_R {
        INTSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat4(&self) -> INTSTAT4_R {
        INTSTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat5(&self) -> INTSTAT5_R {
        INTSTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat6(&self) -> INTSTAT6_R {
        INTSTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat7(&self) -> INTSTAT7_R {
        INTSTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
