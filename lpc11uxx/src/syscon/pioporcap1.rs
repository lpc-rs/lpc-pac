#[doc = "Reader of register PIOPORCAP1"]
pub type R = crate::R<u32, super::PIOPORCAP1>;
#[doc = "Reader of field `PIOSTAT`"]
pub type PIOSTAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - State of PIO1_31 through PIO1_0 at power-on reset"]
    #[inline(always)]
    pub fn piostat(&self) -> PIOSTAT_R {
        PIOSTAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
