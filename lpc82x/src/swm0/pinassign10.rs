#[doc = "Reader of register PINASSIGN10"]
pub type R = crate::R<u32, super::PINASSIGN10>;
#[doc = "Writer for register PINASSIGN10"]
pub type W = crate::W<u32, super::PINASSIGN10>;
#[doc = "Register PINASSIGN10 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `I2C2_SCL_IO`"]
pub type I2C2_SCL_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C2_SCL_IO`"]
pub struct I2C2_SCL_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_SCL_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `I2C3_SDA_IO`"]
pub type I2C3_SDA_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C3_SDA_IO`"]
pub struct I2C3_SDA_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_SDA_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2C3_SCL_IO`"]
pub type I2C3_SCL_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C3_SCL_IO`"]
pub struct I2C3_SCL_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_SCL_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADC_PINTRIG0_I`"]
pub type ADC_PINTRIG0_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_PINTRIG0_I`"]
pub struct ADC_PINTRIG0_I_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PINTRIG0_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C1_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c2_scl_io(&self) -> I2C2_SCL_IO_R {
        I2C2_SCL_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2C3_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c3_sda_io(&self) -> I2C3_SDA_IO_R {
        I2C3_SDA_IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C3_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c3_scl_io(&self) -> I2C3_SCL_IO_R {
        I2C3_SCL_IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ADC_PINTRIG0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn adc_pintrig0_i(&self) -> ADC_PINTRIG0_I_R {
        ADC_PINTRIG0_I_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C1_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c2_scl_io(&mut self) -> I2C2_SCL_IO_W {
        I2C2_SCL_IO_W { w: self }
    }
    #[doc = "Bits 8:15 - I2C3_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c3_sda_io(&mut self) -> I2C3_SDA_IO_W {
        I2C3_SDA_IO_W { w: self }
    }
    #[doc = "Bits 16:23 - I2C3_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn i2c3_scl_io(&mut self) -> I2C3_SCL_IO_W {
        I2C3_SCL_IO_W { w: self }
    }
    #[doc = "Bits 24:31 - ADC_PINTRIG0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn adc_pintrig0_i(&mut self) -> ADC_PINTRIG0_I_W {
        ADC_PINTRIG0_I_W { w: self }
    }
}
