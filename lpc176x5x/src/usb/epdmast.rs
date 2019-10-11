#[doc = "Reader of register EPDMAST"]
pub type R = crate::R<u32, super::EPDMAST>;
#[doc = "Reader of field `EP_DMA_ST0`"]
pub type EP_DMA_ST0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST1`"]
pub type EP_DMA_ST1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST2`"]
pub type EP_DMA_ST2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST3`"]
pub type EP_DMA_ST3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST4`"]
pub type EP_DMA_ST4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST5`"]
pub type EP_DMA_ST5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST6`"]
pub type EP_DMA_ST6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST7`"]
pub type EP_DMA_ST7_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST8`"]
pub type EP_DMA_ST8_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST9`"]
pub type EP_DMA_ST9_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST10`"]
pub type EP_DMA_ST10_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST11`"]
pub type EP_DMA_ST11_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST12`"]
pub type EP_DMA_ST12_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST13`"]
pub type EP_DMA_ST13_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST14`"]
pub type EP_DMA_ST14_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST15`"]
pub type EP_DMA_ST15_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST16`"]
pub type EP_DMA_ST16_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST17`"]
pub type EP_DMA_ST17_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST18`"]
pub type EP_DMA_ST18_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST19`"]
pub type EP_DMA_ST19_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST20`"]
pub type EP_DMA_ST20_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST21`"]
pub type EP_DMA_ST21_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST22`"]
pub type EP_DMA_ST22_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST23`"]
pub type EP_DMA_ST23_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST24`"]
pub type EP_DMA_ST24_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST25`"]
pub type EP_DMA_ST25_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST26`"]
pub type EP_DMA_ST26_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST27`"]
pub type EP_DMA_ST27_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST28`"]
pub type EP_DMA_ST28_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST29`"]
pub type EP_DMA_ST29_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST30`"]
pub type EP_DMA_ST30_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP_DMA_ST31`"]
pub type EP_DMA_ST31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_ENABLE bit must be 0)."]
    #[inline(always)]
    pub fn ep_dma_st0(&self) -> EP_DMA_ST0_R {
        EP_DMA_ST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_ENABLE bit must be 0)."]
    #[inline(always)]
    pub fn ep_dma_st1(&self) -> EP_DMA_ST1_R {
        EP_DMA_ST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st2(&self) -> EP_DMA_ST2_R {
        EP_DMA_ST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st3(&self) -> EP_DMA_ST3_R {
        EP_DMA_ST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st4(&self) -> EP_DMA_ST4_R {
        EP_DMA_ST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st5(&self) -> EP_DMA_ST5_R {
        EP_DMA_ST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st6(&self) -> EP_DMA_ST6_R {
        EP_DMA_ST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st7(&self) -> EP_DMA_ST7_R {
        EP_DMA_ST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st8(&self) -> EP_DMA_ST8_R {
        EP_DMA_ST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st9(&self) -> EP_DMA_ST9_R {
        EP_DMA_ST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st10(&self) -> EP_DMA_ST10_R {
        EP_DMA_ST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st11(&self) -> EP_DMA_ST11_R {
        EP_DMA_ST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st12(&self) -> EP_DMA_ST12_R {
        EP_DMA_ST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st13(&self) -> EP_DMA_ST13_R {
        EP_DMA_ST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st14(&self) -> EP_DMA_ST14_R {
        EP_DMA_ST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st15(&self) -> EP_DMA_ST15_R {
        EP_DMA_ST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st16(&self) -> EP_DMA_ST16_R {
        EP_DMA_ST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st17(&self) -> EP_DMA_ST17_R {
        EP_DMA_ST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st18(&self) -> EP_DMA_ST18_R {
        EP_DMA_ST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st19(&self) -> EP_DMA_ST19_R {
        EP_DMA_ST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st20(&self) -> EP_DMA_ST20_R {
        EP_DMA_ST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st21(&self) -> EP_DMA_ST21_R {
        EP_DMA_ST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st22(&self) -> EP_DMA_ST22_R {
        EP_DMA_ST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st23(&self) -> EP_DMA_ST23_R {
        EP_DMA_ST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st24(&self) -> EP_DMA_ST24_R {
        EP_DMA_ST24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st25(&self) -> EP_DMA_ST25_R {
        EP_DMA_ST25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st26(&self) -> EP_DMA_ST26_R {
        EP_DMA_ST26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st27(&self) -> EP_DMA_ST27_R {
        EP_DMA_ST27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st28(&self) -> EP_DMA_ST28_R {
        EP_DMA_ST28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st29(&self) -> EP_DMA_ST29_R {
        EP_DMA_ST29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st30(&self) -> EP_DMA_ST30_R {
        EP_DMA_ST30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) DMA enabled bit. 0 = The DMA for endpoint EPxx is disabled. 1 = The DMA for endpoint EPxx is enabled."]
    #[inline(always)]
    pub fn ep_dma_st31(&self) -> EP_DMA_ST31_R {
        EP_DMA_ST31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
