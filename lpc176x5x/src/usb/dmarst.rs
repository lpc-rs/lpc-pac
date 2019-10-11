#[doc = "Reader of register DMARST"]
pub type R = crate::R<u32, super::DMARST>;
#[doc = "Reader of field `EPRST0`"]
pub type EPRST0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST1`"]
pub type EPRST1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST2`"]
pub type EPRST2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST3`"]
pub type EPRST3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST4`"]
pub type EPRST4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST5`"]
pub type EPRST5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST6`"]
pub type EPRST6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST7`"]
pub type EPRST7_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST8`"]
pub type EPRST8_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST9`"]
pub type EPRST9_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST10`"]
pub type EPRST10_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST11`"]
pub type EPRST11_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST12`"]
pub type EPRST12_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST13`"]
pub type EPRST13_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST14`"]
pub type EPRST14_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST15`"]
pub type EPRST15_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST16`"]
pub type EPRST16_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST17`"]
pub type EPRST17_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST18`"]
pub type EPRST18_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST19`"]
pub type EPRST19_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST20`"]
pub type EPRST20_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST21`"]
pub type EPRST21_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST22`"]
pub type EPRST22_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST23`"]
pub type EPRST23_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST24`"]
pub type EPRST24_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST25`"]
pub type EPRST25_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST26`"]
pub type EPRST26_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST27`"]
pub type EPRST27_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST28`"]
pub type EPRST28_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST29`"]
pub type EPRST29_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST30`"]
pub type EPRST30_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPRST31`"]
pub type EPRST31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and EP0 bit must be 0)."]
    #[inline(always)]
    pub fn eprst0(&self) -> EPRST0_R {
        EPRST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and EP1 bit must be 0)."]
    #[inline(always)]
    pub fn eprst1(&self) -> EPRST1_R {
        EPRST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst2(&self) -> EPRST2_R {
        EPRST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst3(&self) -> EPRST3_R {
        EPRST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst4(&self) -> EPRST4_R {
        EPRST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst5(&self) -> EPRST5_R {
        EPRST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst6(&self) -> EPRST6_R {
        EPRST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst7(&self) -> EPRST7_R {
        EPRST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst8(&self) -> EPRST8_R {
        EPRST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst9(&self) -> EPRST9_R {
        EPRST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst10(&self) -> EPRST10_R {
        EPRST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst11(&self) -> EPRST11_R {
        EPRST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst12(&self) -> EPRST12_R {
        EPRST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst13(&self) -> EPRST13_R {
        EPRST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst14(&self) -> EPRST14_R {
        EPRST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst15(&self) -> EPRST15_R {
        EPRST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst16(&self) -> EPRST16_R {
        EPRST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst17(&self) -> EPRST17_R {
        EPRST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst18(&self) -> EPRST18_R {
        EPRST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst19(&self) -> EPRST19_R {
        EPRST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst20(&self) -> EPRST20_R {
        EPRST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst21(&self) -> EPRST21_R {
        EPRST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst22(&self) -> EPRST22_R {
        EPRST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst23(&self) -> EPRST23_R {
        EPRST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst24(&self) -> EPRST24_R {
        EPRST24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst25(&self) -> EPRST25_R {
        EPRST25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst26(&self) -> EPRST26_R {
        EPRST26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst27(&self) -> EPRST27_R {
        EPRST27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst28(&self) -> EPRST28_R {
        EPRST28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst29(&self) -> EPRST29_R {
        EPRST29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst30(&self) -> EPRST30_R {
        EPRST30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) DMA request. 0 = DMA not requested by endpoint xx. 1 = DMA requested by endpoint xx."]
    #[inline(always)]
    pub fn eprst31(&self) -> EPRST31_R {
        EPRST31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
