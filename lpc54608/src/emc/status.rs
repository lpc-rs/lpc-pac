#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `B`"]
pub type B_R = crate::R<bool, bool>;
#[doc = "Reader of field `S`"]
pub type S_R = crate::R<bool, bool>;
#[doc = "Reader of field `SA`"]
pub type SA_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Busy."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write buffer status."]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Self-refresh acknowledge."]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
