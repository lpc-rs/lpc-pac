#[doc = "Reader of register INTRAW"]
pub type R = crate::R<u32, super::INTRAW>;
#[doc = "Reader of field `FUFRIS`"]
pub type FUFRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LNBURIS`"]
pub type LNBURIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `VCOMPRIS`"]
pub type VCOMPRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BERRAW`"]
pub type BERRAW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - FIFO underflow raw interrupt status."]
    #[inline(always)]
    pub fn fufris(&self) -> FUFRIS_R {
        FUFRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD next address base update raw interrupt status."]
    #[inline(always)]
    pub fn lnburis(&self) -> LNBURIS_R {
        LNBURIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical compare raw interrupt status."]
    #[inline(always)]
    pub fn vcompris(&self) -> VCOMPRIS_R {
        VCOMPRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master bus error raw interrupt status."]
    #[inline(always)]
    pub fn berraw(&self) -> BERRAW_R {
        BERRAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
