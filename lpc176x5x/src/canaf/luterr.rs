#[doc = "Reader of register LUTERR"]
pub type R = crate::R<u32, super::LUTERR>;
#[doc = "Reader of field `LUTERR`"]
pub type LUTERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - This read-only bit is set to 1 if the Acceptance Filter encounters an error in the content of the tables in AF RAM. It is cleared when software reads the LUTerrAd register. This condition is ORed with the other CAN interrupts from the CAN controllers, to produce the request that is connected to the NVIC."]
    #[inline(always)]
    pub fn luterr(&self) -> LUTERR_R {
        LUTERR_R::new((self.bits & 0x01) != 0)
    }
}
