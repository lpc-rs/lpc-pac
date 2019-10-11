#[doc = "Reader of register RXRATE"]
pub type R = crate::R<u32, super::RXRATE>;
#[doc = "Writer for register RXRATE"]
pub type W = crate::W<u32, super::RXRATE>;
#[doc = "Register RXRATE `reset()`'s with value 0"]
impl crate::ResetValue for super::RXRATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Y_DIVIDER`"]
pub type Y_DIVIDER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Y_DIVIDER`"]
pub struct Y_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `X_DIVIDER`"]
pub type X_DIVIDER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `X_DIVIDER`"]
pub struct X_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> X_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
    #[inline(always)]
    pub fn y_divider(&self) -> Y_DIVIDER_R {
        Y_DIVIDER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
    #[inline(always)]
    pub fn x_divider(&self) -> X_DIVIDER_R {
        X_DIVIDER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S receive MCLK rate denominator. This value is used to divide PCLK to produce the receive MCLK. Eight bits of fractional divide supports a wide range of possibilities. A value of 0 stops the clock."]
    #[inline(always)]
    pub fn y_divider(&mut self) -> Y_DIVIDER_W {
        Y_DIVIDER_W { w: self }
    }
    #[doc = "Bits 8:15 - I2S receive MCLK rate numerator. This value is used to multiply PCLK by to produce the receive MCLK. A value of 0 stops the clock. Eight bits of fractional divide supports a wide range of possibilities. Note: the resulting ratio X/Y is divided by 2."]
    #[inline(always)]
    pub fn x_divider(&mut self) -> X_DIVIDER_W {
        X_DIVIDER_W { w: self }
    }
}
