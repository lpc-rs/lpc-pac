#[doc = "Reader of register INTTCSTAT"]
pub type R = crate::R<u32, super::INTTCSTAT>;
#[doc = "Reader of field `INTTCSTAT0`"]
pub type INTTCSTAT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTTCSTAT1`"]
pub type INTTCSTAT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTTCSTAT2`"]
pub type INTTCSTAT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTTCSTAT3`"]
pub type INTTCSTAT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTTCSTAT4`"]
pub type INTTCSTAT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTTCSTAT5`"]
pub type INTTCSTAT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTTCSTAT6`"]
pub type INTTCSTAT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTTCSTAT7`"]
pub type INTTCSTAT7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat0(&self) -> INTTCSTAT0_R {
        INTTCSTAT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat1(&self) -> INTTCSTAT1_R {
        INTTCSTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat2(&self) -> INTTCSTAT2_R {
        INTTCSTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat3(&self) -> INTTCSTAT3_R {
        INTTCSTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat4(&self) -> INTTCSTAT4_R {
        INTTCSTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat5(&self) -> INTTCSTAT5_R {
        INTTCSTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat6(&self) -> INTTCSTAT6_R {
        INTTCSTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat7(&self) -> INTTCSTAT7_R {
        INTTCSTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
