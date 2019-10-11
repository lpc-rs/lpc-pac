#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Direction bit. In combination with DIRINV bit indicates forward or reverse direction. See Table 597."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0x01) != 0)
    }
}
