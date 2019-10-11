#[doc = "Reader of register CTIME2"]
pub type R = crate::R<u32, super::CTIME2>;
#[doc = "Reader of field `DOY`"]
pub type DOY_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&self) -> DOY_R {
        DOY_R::new((self.bits & 0x0fff) as u16)
    }
}
