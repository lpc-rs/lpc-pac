#[doc = "Writer for register INTSTATSET"]
pub type W = crate::W<u32, super::INTSTATSET>;
#[doc = "Register INTSTATSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTATSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PROG_SET_ST`"]
pub struct PROG_SET_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_SET_ST_W<'a> {
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
    #[doc = "Bit 2 - Set program operation finished interrupt status bit for EEPROM device."]
    #[inline(always)]
    pub fn prog_set_st(&mut self) -> PROG_SET_ST_W {
        PROG_SET_ST_W { w: self }
    }
}
