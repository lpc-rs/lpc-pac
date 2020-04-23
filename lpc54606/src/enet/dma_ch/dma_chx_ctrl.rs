#[doc = "Reader of register DMA_CHx_CTRL"]
pub type R = crate::R<u32, super::DMA_CHX_CTRL>;
#[doc = "Writer for register DMA_CHx_CTRL"]
pub type W = crate::W<u32, super::DMA_CHX_CTRL>;
#[doc = "Register DMA_CHx_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CHX_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PBLx8`"]
pub type PBLX8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBLx8`"]
pub struct PBLX8_W<'a> {
    w: &'a mut W,
}
impl<'a> PBLX8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DSL`"]
pub type DSL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSL`"]
pub struct DSL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\]
in DMA Channel Transmit Control Table 780 is multiplied eight times."]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\]
in DMA Channel Transmit Control Table 780 is multiplied eight times."]
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W {
        PBLX8_W { w: self }
    }
    #[doc = "Bits 18:20 - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W {
        DSL_W { w: self }
    }
}
