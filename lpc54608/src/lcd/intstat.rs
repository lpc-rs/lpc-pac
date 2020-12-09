#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Reader of field `FUFMIS`"]
pub type FUFMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LNBUMIS`"]
pub type LNBUMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `VCOMPMIS`"]
pub type VCOMPMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BERMIS`"]
pub type BERMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - FIFO underflow masked interrupt status."]
    #[inline(always)]
    pub fn fufmis(&self) -> FUFMIS_R {
        FUFMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD next address base update masked interrupt status."]
    #[inline(always)]
    pub fn lnbumis(&self) -> LNBUMIS_R {
        LNBUMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical compare masked interrupt status."]
    #[inline(always)]
    pub fn vcompmis(&self) -> VCOMPMIS_R {
        VCOMPMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master bus error masked interrupt status."]
    #[inline(always)]
    pub fn bermis(&self) -> BERMIS_R {
        BERMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
