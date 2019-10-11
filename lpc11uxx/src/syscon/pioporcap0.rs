#[doc = "Reader of register PIOPORCAP0"]
pub type R = crate::R<u32, super::PIOPORCAP0>;
#[doc = "Reader of field `PIOSTAT`"]
pub type PIOSTAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - State of PIO0_23 through PIO0_0 at power-on reset"]
    #[inline(always)]
    pub fn piostat(&self) -> PIOSTAT_R {
        PIOSTAT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
