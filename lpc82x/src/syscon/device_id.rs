#[doc = "Reader of register DEVICE_ID"]
pub type R = crate::R<u32, super::DEVICE_ID>;
#[doc = "Reader of field `DEVICEID`"]
pub type DEVICEID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Part ID"]
    #[inline(always)]
    pub fn deviceid(&self) -> DEVICEID_R {
        DEVICEID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
