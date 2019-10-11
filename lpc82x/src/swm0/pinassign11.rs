#[doc = "Reader of register PINASSIGN11"]
pub type R = crate::R<u32, super::PINASSIGN11>;
#[doc = "Writer for register PINASSIGN11"]
pub type W = crate::W<u32, super::PINASSIGN11>;
#[doc = "Register PINASSIGN11 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ADC_PINTRIG1_I`"]
pub type ADC_PINTRIG1_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_PINTRIG1_I`"]
pub struct ADC_PINTRIG1_I_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PINTRIG1_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ACMP_O_O`"]
pub type ACMP_O_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACMP_O_O`"]
pub struct ACMP_O_O_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_O_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLKOUT_O`"]
pub type CLKOUT_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKOUT_O`"]
pub struct CLKOUT_O_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `GPIO_INT_BMAT_O`"]
pub type GPIO_INT_BMAT_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_INT_BMAT_O`"]
pub struct GPIO_INT_BMAT_O_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_BMAT_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADC_PINTRIG1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn adc_pintrig1_i(&self) -> ADC_PINTRIG1_I_R {
        ADC_PINTRIG1_I_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ACMP_O function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn acmp_o_o(&self) -> ACMP_O_O_R {
        ACMP_O_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn clkout_o(&self) -> CLKOUT_O_R {
        CLKOUT_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn gpio_int_bmat_o(&self) -> GPIO_INT_BMAT_O_R {
        GPIO_INT_BMAT_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC_PINTRIG1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn adc_pintrig1_i(&mut self) -> ADC_PINTRIG1_I_W {
        ADC_PINTRIG1_I_W { w: self }
    }
    #[doc = "Bits 8:15 - ACMP_O function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn acmp_o_o(&mut self) -> ACMP_O_O_W {
        ACMP_O_O_W { w: self }
    }
    #[doc = "Bits 16:23 - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn clkout_o(&mut self) -> CLKOUT_O_W {
        CLKOUT_O_W { w: self }
    }
    #[doc = "Bits 24:31 - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn gpio_int_bmat_o(&mut self) -> GPIO_INT_BMAT_O_W {
        GPIO_INT_BMAT_O_W { w: self }
    }
}
