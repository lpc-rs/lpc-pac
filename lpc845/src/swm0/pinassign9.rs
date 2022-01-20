#[doc = "Register `PINASSIGN9` reader"]
pub struct R(crate::R<PINASSIGN9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINASSIGN9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINASSIGN9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINASSIGN9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINASSIGN9` writer"]
pub struct W(crate::W<PINASSIGN9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINASSIGN9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PINASSIGN9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINASSIGN9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCT_OUT5_O` reader - SCT_OUT5 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT_OUT5_O_R(crate::FieldReader<u8, u8>);
impl SCT_OUT5_O_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCT_OUT5_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT_OUT5_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_OUT5_O` writer - SCT_OUT5 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT_OUT5_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT5_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SCT_OUT6_O` reader - SCT_OUT6 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT_OUT6_O_R(crate::FieldReader<u8, u8>);
impl SCT_OUT6_O_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCT_OUT6_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT_OUT6_O_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT_OUT6_O` writer - SCT_OUT6 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct SCT_OUT6_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT6_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `I2C1_SDA_IO` reader - I2C1_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct I2C1_SDA_IO_R(crate::FieldReader<u8, u8>);
impl I2C1_SDA_IO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2C1_SDA_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_SDA_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_SDA_IO` writer - I2C1_SDA function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct I2C1_SDA_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_SDA_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `I2C1_SCL_IO` reader - I2C1_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct I2C1_SCL_IO_R(crate::FieldReader<u8, u8>);
impl I2C1_SCL_IO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2C1_SCL_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_SCL_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_SCL_IO` writer - I2C1_SCL function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
pub struct I2C1_SCL_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_SCL_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin assign register 9. Assign movable functions SCT_OUT5, SCT_OUT6, I2C1_SDA, I2C1_SCL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign9](index.html) module"]
pub struct PINASSIGN9_SPEC;
impl crate::RegisterSpec for PINASSIGN9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinassign9::R](R) reader structure"]
impl crate::Readable for PINASSIGN9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinassign9::W](W) writer structure"]
impl crate::Writable for PINASSIGN9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINASSIGN9 to value 0xffff_ffff"]
impl crate::Resettable for PINASSIGN9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
