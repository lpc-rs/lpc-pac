#[doc = "Reader of register CTIME1"]
pub type R = crate::R<u32, super::CTIME1>;
#[doc = "Reader of field `DOM`"]
pub type DOM_R = crate::R<u8, u8>;
#[doc = "Reader of field `MONTH`"]
pub type MONTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `YEAR`"]
pub type YEAR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&self) -> DOM_R {
        DOM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
