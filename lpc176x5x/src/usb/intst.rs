#[doc = "Reader of register INTST"]
pub type R = crate::R<u32, super::INTST>;
#[doc = "Reader of field `TMR`"]
pub type TMR_R = crate::R<bool, bool>;
#[doc = "Reader of field `REMOVE_PU`"]
pub type REMOVE_PU_R = crate::R<bool, bool>;
#[doc = "Reader of field `HNP_FAILURE`"]
pub type HNP_FAILURE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HNP_SUCCESS`"]
pub type HNP_SUCCESS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Timer time-out."]
    #[inline(always)]
    pub fn tmr(&self) -> TMR_R {
        TMR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Remove pull-up. This bit is set by hardware to indicate that software needs to disable the D+ pull-up resistor."]
    #[inline(always)]
    pub fn remove_pu(&self) -> REMOVE_PU_R {
        REMOVE_PU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HNP failed. This bit is set by hardware to indicate that the HNP switching has failed."]
    #[inline(always)]
    pub fn hnp_failure(&self) -> HNP_FAILURE_R {
        HNP_FAILURE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HNP succeeded. This bit is set by hardware to indicate that the HNP switching has succeeded."]
    #[inline(always)]
    pub fn hnp_success(&self) -> HNP_SUCCESS_R {
        HNP_SUCCESS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
