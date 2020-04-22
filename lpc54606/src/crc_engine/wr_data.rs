#[doc = "Writer for register WR_DATA"]
pub type W = crate::W<u32, super::WR_DATA>;
#[doc = "Register WR_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::WR_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CRC_WR_DATA`"]
pub struct CRC_WR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_WR_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[inline(always)]
    pub fn crc_wr_data(&mut self) -> CRC_WR_DATA_W {
        CRC_WR_DATA_W { w: self }
    }
}
