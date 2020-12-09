#[doc = "Reader of register PIDR0"]
pub type R = crate::R<u32, super::PIDR0>;
#[doc = "Reader of field `PartNumber`"]
pub type PARTNUMBER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Part Number \\[7:0\\]"]
    #[inline(always)]
    pub fn part_number(&self) -> PARTNUMBER_R {
        PARTNUMBER_R::new((self.bits & 0xff) as u8)
    }
}
