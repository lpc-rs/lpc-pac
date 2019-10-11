#[doc = "Reader of register ENBLDCHNS"]
pub type R = crate::R<u32, super::ENBLDCHNS>;
#[doc = "Reader of field `ENABLEDCHANNELS0`"]
pub type ENABLEDCHANNELS0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLEDCHANNELS1`"]
pub type ENABLEDCHANNELS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLEDCHANNELS2`"]
pub type ENABLEDCHANNELS2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLEDCHANNELS3`"]
pub type ENABLEDCHANNELS3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLEDCHANNELS4`"]
pub type ENABLEDCHANNELS4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLEDCHANNELS5`"]
pub type ENABLEDCHANNELS5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLEDCHANNELS6`"]
pub type ENABLEDCHANNELS6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLEDCHANNELS7`"]
pub type ENABLEDCHANNELS7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels0(&self) -> ENABLEDCHANNELS0_R {
        ENABLEDCHANNELS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels1(&self) -> ENABLEDCHANNELS1_R {
        ENABLEDCHANNELS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels2(&self) -> ENABLEDCHANNELS2_R {
        ENABLEDCHANNELS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels3(&self) -> ENABLEDCHANNELS3_R {
        ENABLEDCHANNELS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels4(&self) -> ENABLEDCHANNELS4_R {
        ENABLEDCHANNELS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels5(&self) -> ENABLEDCHANNELS5_R {
        ENABLEDCHANNELS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels6(&self) -> ENABLEDCHANNELS6_R {
        ENABLEDCHANNELS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels7(&self) -> ENABLEDCHANNELS7_R {
        ENABLEDCHANNELS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
