#[doc = "Reader of register I2C_CLKHI"]
pub type R = crate::R<u32, super::I2C_CLKHI>;
#[doc = "Writer for register I2C_CLKHI"]
pub type W = crate::W<u32, super::I2C_CLKHI>;
#[doc = "Register I2C_CLKHI `reset()`'s with value 0xb9"]
impl crate::ResetValue for super::I2C_CLKHI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb9
    }
}
#[doc = "Reader of field `CDHI`"]
pub type CDHI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CDHI`"]
pub struct CDHI_W<'a> {
    w: &'a mut W,
}
impl<'a> CDHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
    #[inline(always)]
    pub fn cdhi(&self) -> CDHI_R {
        CDHI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
    #[inline(always)]
    pub fn cdhi(&mut self) -> CDHI_W {
        CDHI_W { w: self }
    }
}
