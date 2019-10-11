#[doc = "Reader of register PINASSIGN9"]
pub type R = crate::R<u32, super::PINASSIGN9>;
#[doc = "Writer for register PINASSIGN9"]
pub type W = crate::W<u32, super::PINASSIGN9>;
#[doc = "Register PINASSIGN9 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `SCT_OUT5_O`"]
pub type SCT_OUT5_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT_OUT5_O`"]
pub struct SCT_OUT5_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT5_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SCT_OUT6_O`"]
pub type SCT_OUT6_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT_OUT6_O`"]
pub struct SCT_OUT6_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT6_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2C1_SDA_IO`"]
pub type I2C1_SDA_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C1_SDA_IO`"]
pub struct I2C1_SDA_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_SDA_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `I2C1_SCL_IO`"]
pub type I2C1_SCL_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C1_SCL_IO`"]
pub struct I2C1_SCL_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_SCL_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SCT_OUT5 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out5_o(&self) -> SCT_OUT5_O_R {
        SCT_OUT5_O_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCT_OUT6 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out6_o(&self) -> SCT_OUT6_O_R {
        SCT_OUT6_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C1_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn i2c1_sda_io(&self) -> I2C1_SDA_IO_R {
        I2C1_SDA_IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - I2C1_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn i2c1_scl_io(&self) -> I2C1_SCL_IO_R {
        I2C1_SCL_IO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCT_OUT5 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out5_o(&mut self) -> SCT_OUT5_O_W {
        SCT_OUT5_O_W { w: self }
    }
    #[doc = "Bits 8:15 - SCT_OUT6 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out6_o(&mut self) -> SCT_OUT6_O_W {
        SCT_OUT6_O_W { w: self }
    }
    #[doc = "Bits 16:23 - I2C1_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn i2c1_sda_io(&mut self) -> I2C1_SDA_IO_W {
        I2C1_SDA_IO_W { w: self }
    }
    #[doc = "Bits 24:31 - I2C1_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn i2c1_scl_io(&mut self) -> I2C1_SCL_IO_W {
        I2C1_SCL_IO_W { w: self }
    }
}
