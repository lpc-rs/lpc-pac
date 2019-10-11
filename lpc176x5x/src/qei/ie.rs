#[doc = "Reader of register IE"]
pub type R = crate::R<u32, super::IE>;
#[doc = "Reader of field `INX_INT`"]
pub type INX_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM_INT`"]
pub type TIM_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `VELC_INT`"]
pub type VELC_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIR_INT`"]
pub type DIR_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR_INT`"]
pub type ERR_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENCLK_INT`"]
pub type ENCLK_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `POS0_INT`"]
pub type POS0_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `POS1_INT`"]
pub type POS1_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `POS2_INT`"]
pub type POS2_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `REV0_INT`"]
pub type REV0_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `POS0REV_INT`"]
pub type POS0REV_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `POS1REV_INT`"]
pub type POS1REV_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `POS2REV_INT`"]
pub type POS2REV_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `REV1_INT`"]
pub type REV1_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `REV2_INT`"]
pub type REV2_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAXPOS_INT`"]
pub type MAXPOS_INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - When 1, the INX_Int interrupt is enabled."]
    #[inline(always)]
    pub fn inx_int(&self) -> INX_INT_R {
        INX_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, the TIN_Int interrupt is enabled."]
    #[inline(always)]
    pub fn tim_int(&self) -> TIM_INT_R {
        TIM_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, the VELC_Int interrupt is enabled."]
    #[inline(always)]
    pub fn velc_int(&self) -> VELC_INT_R {
        VELC_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, the DIR_Int interrupt is enabled."]
    #[inline(always)]
    pub fn dir_int(&self) -> DIR_INT_R {
        DIR_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, the ERR_Int interrupt is enabled."]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 1, the ENCLK_Int interrupt is enabled."]
    #[inline(always)]
    pub fn enclk_int(&self) -> ENCLK_INT_R {
        ENCLK_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 1, the POS0_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos0_int(&self) -> POS0_INT_R {
        POS0_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When 1, the POS1_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos1_int(&self) -> POS1_INT_R {
        POS1_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When 1, the POS2_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos2_int(&self) -> POS2_INT_R {
        POS2_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When 1, the REV0_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev0_int(&self) -> REV0_INT_R {
        REV0_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - When 1, the POS0REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos0rev_int(&self) -> POS0REV_INT_R {
        POS0REV_INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When 1, the POS1REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos1rev_int(&self) -> POS1REV_INT_R {
        POS1REV_INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - When 1, the POS2REV_Int interrupt is enabled."]
    #[inline(always)]
    pub fn pos2rev_int(&self) -> POS2REV_INT_R {
        POS2REV_INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - When 1, the REV1_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev1_int(&self) -> REV1_INT_R {
        REV1_INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - When 1, the REV2_Int interrupt is enabled."]
    #[inline(always)]
    pub fn rev2_int(&self) -> REV2_INT_R {
        REV2_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - When 1, the MAXPOS_Int interrupt is enabled."]
    #[inline(always)]
    pub fn maxpos_int(&self) -> MAXPOS_INT_R {
        MAXPOS_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
