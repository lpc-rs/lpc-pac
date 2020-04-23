#[doc = "Reader of register MTL_INTR_STAT"]
pub type R = crate::R<u32, super::MTL_INTR_STAT>;
#[doc = "Reader of field `Q0IS`"]
pub type Q0IS_R = crate::R<bool, bool>;
#[doc = "Reader of field `Q1IS`"]
pub type Q1IS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Queue 0 Interrupt status This bit indicates that there is an interrupt from Queue 0."]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Queue 1 Interrupt status This bit indicates that there is an interrupt from Queue 1."]
    #[inline(always)]
    pub fn q1is(&self) -> Q1IS_R {
        Q1IS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
