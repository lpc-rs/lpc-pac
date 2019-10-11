#[doc = "Reader of register PIOPORCAP[%s]"]
pub type R = crate::R<u32, super::PIOPORCAP>;
#[doc = "Reader of field `PIOSTAT`"]
pub type PIOSTAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - State of PION_31 through PION_0 at power-on reset"]
    #[inline(always)]
    pub fn piostat(&self) -> PIOSTAT_R {
        PIOSTAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
