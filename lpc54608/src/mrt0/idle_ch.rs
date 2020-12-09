#[doc = "Reader of register IDLE_CH"]
pub type R = crate::R<u32, super::IDLE_CH>;
#[doc = "Reader of field `CHAN`"]
pub type CHAN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 4:7 - Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. The number is positioned such that it can be used as an offset from the MRT base address in order to access the registers for the allocated channel. If all timer channels are running, CHAN = 0xF. See text above for more details."]
    #[inline(always)]
    pub fn chan(&self) -> CHAN_R {
        CHAN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
