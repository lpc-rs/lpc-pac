#[doc = "Reader of register EOTINTST"]
pub type R = crate::R<u32, super::EOTINTST>;
#[doc = "Reader of field `EPTXINTST0`"]
pub type EPTXINTST0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST1`"]
pub type EPTXINTST1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST2`"]
pub type EPTXINTST2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST3`"]
pub type EPTXINTST3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST4`"]
pub type EPTXINTST4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST5`"]
pub type EPTXINTST5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST6`"]
pub type EPTXINTST6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST7`"]
pub type EPTXINTST7_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST8`"]
pub type EPTXINTST8_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST9`"]
pub type EPTXINTST9_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST10`"]
pub type EPTXINTST10_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST11`"]
pub type EPTXINTST11_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST12`"]
pub type EPTXINTST12_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST13`"]
pub type EPTXINTST13_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST14`"]
pub type EPTXINTST14_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST15`"]
pub type EPTXINTST15_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST16`"]
pub type EPTXINTST16_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST17`"]
pub type EPTXINTST17_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST18`"]
pub type EPTXINTST18_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST19`"]
pub type EPTXINTST19_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST20`"]
pub type EPTXINTST20_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST21`"]
pub type EPTXINTST21_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST22`"]
pub type EPTXINTST22_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST23`"]
pub type EPTXINTST23_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST24`"]
pub type EPTXINTST24_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST25`"]
pub type EPTXINTST25_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST26`"]
pub type EPTXINTST26_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST27`"]
pub type EPTXINTST27_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST28`"]
pub type EPTXINTST28_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST29`"]
pub type EPTXINTST29_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST30`"]
pub type EPTXINTST30_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPTXINTST31`"]
pub type EPTXINTST31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst0(&self) -> EPTXINTST0_R {
        EPTXINTST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst1(&self) -> EPTXINTST1_R {
        EPTXINTST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst2(&self) -> EPTXINTST2_R {
        EPTXINTST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst3(&self) -> EPTXINTST3_R {
        EPTXINTST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst4(&self) -> EPTXINTST4_R {
        EPTXINTST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst5(&self) -> EPTXINTST5_R {
        EPTXINTST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst6(&self) -> EPTXINTST6_R {
        EPTXINTST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst7(&self) -> EPTXINTST7_R {
        EPTXINTST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst8(&self) -> EPTXINTST8_R {
        EPTXINTST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst9(&self) -> EPTXINTST9_R {
        EPTXINTST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst10(&self) -> EPTXINTST10_R {
        EPTXINTST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst11(&self) -> EPTXINTST11_R {
        EPTXINTST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst12(&self) -> EPTXINTST12_R {
        EPTXINTST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst13(&self) -> EPTXINTST13_R {
        EPTXINTST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst14(&self) -> EPTXINTST14_R {
        EPTXINTST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst15(&self) -> EPTXINTST15_R {
        EPTXINTST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst16(&self) -> EPTXINTST16_R {
        EPTXINTST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst17(&self) -> EPTXINTST17_R {
        EPTXINTST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst18(&self) -> EPTXINTST18_R {
        EPTXINTST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst19(&self) -> EPTXINTST19_R {
        EPTXINTST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst20(&self) -> EPTXINTST20_R {
        EPTXINTST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst21(&self) -> EPTXINTST21_R {
        EPTXINTST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst22(&self) -> EPTXINTST22_R {
        EPTXINTST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst23(&self) -> EPTXINTST23_R {
        EPTXINTST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst24(&self) -> EPTXINTST24_R {
        EPTXINTST24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst25(&self) -> EPTXINTST25_R {
        EPTXINTST25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst26(&self) -> EPTXINTST26_R {
        EPTXINTST26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst27(&self) -> EPTXINTST27_R {
        EPTXINTST27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst28(&self) -> EPTXINTST28_R {
        EPTXINTST28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst29(&self) -> EPTXINTST29_R {
        EPTXINTST29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst30(&self) -> EPTXINTST30_R {
        EPTXINTST30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint xx (2 <= xx <= 31) End of Transfer Interrupt request. 0 = There is no End of Transfer interrupt request for endpoint xx. 1 = There is an End of Transfer Interrupt request for endpoint xx."]
    #[inline(always)]
    pub fn eptxintst31(&self) -> EPTXINTST31_R {
        EPTXINTST31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
