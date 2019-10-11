#[doc = "Writer for register I2C_CLKLO"]
pub type W = crate::W<u32, super::I2C_CLKLO>;
#[doc = "Register I2C_CLKLO `reset()`'s with value 0xb9"]
impl crate::ResetValue for super::I2C_CLKLO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb9
    }
}
#[doc = "Write proxy for field `CDLO`"]
pub struct CDLO_W<'a> {
    w: &'a mut W,
}
impl<'a> CDLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divisor low. This value is the number of 48 MHz clocks the serial clock (SCL) will be low."]
    #[inline(always)]
    pub fn cdlo(&mut self) -> CDLO_W {
        CDLO_W { w: self }
    }
}
