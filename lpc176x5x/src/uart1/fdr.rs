#[doc = "Reader of register FDR"]
pub type R = crate::R<u32, super::FDR>;
#[doc = "Writer for register FDR"]
pub type W = crate::W<u32, super::FDR>;
#[doc = "Register FDR `reset()`'s with value 0x10"]
impl crate::ResetValue for super::FDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `DIVADDVAL`"]
pub type DIVADDVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVADDVAL`"]
pub struct DIVADDVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVADDVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `MULVAL`"]
pub type MULVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MULVAL`"]
pub struct MULVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MULVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Baud rate generation pre-scaler divisor value. If this field is 0, fractional baud rate generator will not impact the UART1 baud rate."]
    #[inline(always)]
    pub fn divaddval(&self) -> DIVADDVAL_R {
        DIVADDVAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Baud rate pre-scaler multiplier value. This field must be greater or equal 1 for UART1 to operate properly, regardless of whether the fractional baud rate generator is used or not."]
    #[inline(always)]
    pub fn mulval(&self) -> MULVAL_R {
        MULVAL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud rate generation pre-scaler divisor value. If this field is 0, fractional baud rate generator will not impact the UART1 baud rate."]
    #[inline(always)]
    pub fn divaddval(&mut self) -> DIVADDVAL_W {
        DIVADDVAL_W { w: self }
    }
    #[doc = "Bits 4:7 - Baud rate pre-scaler multiplier value. This field must be greater or equal 1 for UART1 to operate properly, regardless of whether the fractional baud rate generator is used or not."]
    #[inline(always)]
    pub fn mulval(&mut self) -> MULVAL_W {
        MULVAL_W { w: self }
    }
}
