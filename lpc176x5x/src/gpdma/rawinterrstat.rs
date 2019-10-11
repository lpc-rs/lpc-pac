#[doc = "Reader of register RAWINTERRSTAT"]
pub type R = crate::R<u32, super::RAWINTERRSTAT>;
#[doc = "Reader of field `RAWINTERRSTAT0`"]
pub type RAWINTERRSTAT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWINTERRSTAT1`"]
pub type RAWINTERRSTAT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWINTERRSTAT2`"]
pub type RAWINTERRSTAT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWINTERRSTAT3`"]
pub type RAWINTERRSTAT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWINTERRSTAT4`"]
pub type RAWINTERRSTAT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWINTERRSTAT5`"]
pub type RAWINTERRSTAT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWINTERRSTAT6`"]
pub type RAWINTERRSTAT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAWINTERRSTAT7`"]
pub type RAWINTERRSTAT7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat0(&self) -> RAWINTERRSTAT0_R {
        RAWINTERRSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat1(&self) -> RAWINTERRSTAT1_R {
        RAWINTERRSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat2(&self) -> RAWINTERRSTAT2_R {
        RAWINTERRSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat3(&self) -> RAWINTERRSTAT3_R {
        RAWINTERRSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat4(&self) -> RAWINTERRSTAT4_R {
        RAWINTERRSTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat5(&self) -> RAWINTERRSTAT5_R {
        RAWINTERRSTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat6(&self) -> RAWINTERRSTAT6_R {
        RAWINTERRSTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat7(&self) -> RAWINTERRSTAT7_R {
        RAWINTERRSTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
