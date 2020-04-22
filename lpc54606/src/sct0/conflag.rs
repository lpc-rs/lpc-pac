#[doc = "Reader of register CONFLAG"]
pub type R = crate::R<u32, super::CONFLAG>;
#[doc = "Writer for register CONFLAG"]
pub type W = crate::W<u32, super::CONFLAG>;
#[doc = "Register CONFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NCFLAG`"]
pub type NCFLAG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NCFLAG`"]
pub struct NCFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> NCFLAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `BUSERRL`"]
pub type BUSERRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSERRL`"]
pub struct BUSERRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `BUSERRH`"]
pub type BUSERRH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSERRH`"]
pub struct BUSERRH_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSERRH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncflag(&self) -> NCFLAG_R {
        NCFLAG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    pub fn buserrl(&self) -> BUSERRL_R {
        BUSERRL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    pub fn buserrh(&self) -> BUSERRH_R {
        BUSERRH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncflag(&mut self) -> NCFLAG_W {
        NCFLAG_W { w: self }
    }
    #[doc = "Bit 30 - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    pub fn buserrl(&mut self) -> BUSERRL_W {
        BUSERRL_W { w: self }
    }
    #[doc = "Bit 31 - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    pub fn buserrh(&mut self) -> BUSERRH_W {
        BUSERRH_W { w: self }
    }
}
