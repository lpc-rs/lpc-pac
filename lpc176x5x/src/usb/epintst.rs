#[doc = "Reader of register EPINTST"]
pub type R = crate::R<u32, super::EPINTST>;
#[doc = "Reader of field `EPST0`"]
pub type EPST0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST1`"]
pub type EPST1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST2`"]
pub type EPST2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST3`"]
pub type EPST3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST4`"]
pub type EPST4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST5`"]
pub type EPST5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST6`"]
pub type EPST6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST7`"]
pub type EPST7_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST8`"]
pub type EPST8_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST9`"]
pub type EPST9_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST10`"]
pub type EPST10_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST11`"]
pub type EPST11_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST12`"]
pub type EPST12_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST13`"]
pub type EPST13_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST14`"]
pub type EPST14_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST15`"]
pub type EPST15_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST16`"]
pub type EPST16_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST17`"]
pub type EPST17_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST18`"]
pub type EPST18_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST19`"]
pub type EPST19_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST20`"]
pub type EPST20_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST21`"]
pub type EPST21_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST22`"]
pub type EPST22_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST23`"]
pub type EPST23_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST24`"]
pub type EPST24_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST25`"]
pub type EPST25_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST26`"]
pub type EPST26_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST27`"]
pub type EPST27_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST28`"]
pub type EPST28_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST29`"]
pub type EPST29_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST30`"]
pub type EPST30_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPST31`"]
pub type EPST31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst0(&self) -> EPST0_R {
        EPST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst1(&self) -> EPST1_R {
        EPST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst2(&self) -> EPST2_R {
        EPST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst3(&self) -> EPST3_R {
        EPST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst4(&self) -> EPST4_R {
        EPST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst5(&self) -> EPST5_R {
        EPST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst6(&self) -> EPST6_R {
        EPST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst7(&self) -> EPST7_R {
        EPST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst8(&self) -> EPST8_R {
        EPST8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst9(&self) -> EPST9_R {
        EPST9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst10(&self) -> EPST10_R {
        EPST10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst11(&self) -> EPST11_R {
        EPST11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst12(&self) -> EPST12_R {
        EPST12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst13(&self) -> EPST13_R {
        EPST13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst14(&self) -> EPST14_R {
        EPST14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst15(&self) -> EPST15_R {
        EPST15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst16(&self) -> EPST16_R {
        EPST16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst17(&self) -> EPST17_R {
        EPST17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst18(&self) -> EPST18_R {
        EPST18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst19(&self) -> EPST19_R {
        EPST19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst20(&self) -> EPST20_R {
        EPST20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst21(&self) -> EPST21_R {
        EPST21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst22(&self) -> EPST22_R {
        EPST22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst23(&self) -> EPST23_R {
        EPST23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst24(&self) -> EPST24_R {
        EPST24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst25(&self) -> EPST25_R {
        EPST25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst26(&self) -> EPST26_R {
        EPST26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst27(&self) -> EPST27_R {
        EPST27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst28(&self) -> EPST28_R {
        EPST28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst29(&self) -> EPST29_R {
        EPST29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst30(&self) -> EPST30_R {
        EPST30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 1 = Endpoint Data Received (bits 0, 2, 4, ..., 30) or Transmitted (bits 1, 3, 5, ..., 31) Interrupt received."]
    #[inline(always)]
    pub fn epst31(&self) -> EPST31_R {
        EPST31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
