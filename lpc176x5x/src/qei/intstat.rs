#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
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
    #[doc = "Bit 0 - Indicates that an index pulse was detected."]
    #[inline(always)]
    pub fn inx_int(&self) -> INX_INT_R {
        INX_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates that a velocity timer overflow occurred"]
    #[inline(always)]
    pub fn tim_int(&self) -> TIM_INT_R {
        TIM_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates that captured velocity is less than compare velocity."]
    #[inline(always)]
    pub fn velc_int(&self) -> VELC_INT_R {
        VELC_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates that a change of direction was detected."]
    #[inline(always)]
    pub fn dir_int(&self) -> DIR_INT_R {
        DIR_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates that an encoder phase error was detected."]
    #[inline(always)]
    pub fn err_int(&self) -> ERR_INT_R {
        ERR_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates that and encoder clock pulse was detected."]
    #[inline(always)]
    pub fn enclk_int(&self) -> ENCLK_INT_R {
        ENCLK_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates that the position 0 compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos0_int(&self) -> POS0_INT_R {
        POS0_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates that the position 1compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos1_int(&self) -> POS1_INT_R {
        POS1_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Indicates that the position 2 compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos2_int(&self) -> POS2_INT_R {
        POS2_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates that the index compare 0 value is equal to the current index count."]
    #[inline(always)]
    pub fn rev0_int(&self) -> REV0_INT_R {
        REV0_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Combined position 0 and revolution count interrupt. Set when both the POS0_Int bit is set and the REV0_Int is set."]
    #[inline(always)]
    pub fn pos0rev_int(&self) -> POS0REV_INT_R {
        POS0REV_INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Combined position 1 and revolution count interrupt. Set when both the POS1_Int bit is set and the REV1_Int is set."]
    #[inline(always)]
    pub fn pos1rev_int(&self) -> POS1REV_INT_R {
        POS1REV_INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Combined position 2 and revolution count interrupt. Set when both the POS2_Int bit is set and the REV2_Int is set."]
    #[inline(always)]
    pub fn pos2rev_int(&self) -> POS2REV_INT_R {
        POS2REV_INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Indicates that the index compare 1value is equal to the current index count."]
    #[inline(always)]
    pub fn rev1_int(&self) -> REV1_INT_R {
        REV1_INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Indicates that the index compare 2 value is equal to the current index count."]
    #[inline(always)]
    pub fn rev2_int(&self) -> REV2_INT_R {
        REV2_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Indicates that the current position count goes through the MAXPOS value to zero in the forward direction, or through zero to MAXPOS in the reverse direction."]
    #[inline(always)]
    pub fn maxpos_int(&self) -> MAXPOS_INT_R {
        MAXPOS_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
