#[doc = "Reader of register SYNCFR"]
pub type R = crate::R<u32, super::SYNCFR>;
#[doc = "Reader of field `SyncFrequency`"]
pub type SYNCFREQUENCY_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Synchronization frequency. Default value is 1024."]
    #[inline(always)]
    pub fn sync_frequency(&self) -> SYNCFREQUENCY_R {
        SYNCFREQUENCY_R::new((self.bits & 0x0fff) as u16)
    }
}
