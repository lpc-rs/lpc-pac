#[doc = "Writer for register INTSTATCLR"]
pub type W = crate::W<u32, super::INTSTATCLR>;
#[doc = "Register INTSTATCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTATCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PROG_CLR_ST`"]
pub struct PROG_CLR_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_CLR_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - Clear program operation finished interrupt status bit for EEPROM device."]
    #[inline(always)]
    pub fn prog_clr_st(&mut self) -> PROG_CLR_ST_W {
        PROG_CLR_ST_W { w: self }
    }
}
