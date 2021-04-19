#[doc = "Register `DATA_BUFFER` reader"]
pub struct R(crate::R<DATA_BUFFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_BUFFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DATA_BUFFER_SPEC>> for R {
    fn from(reader: crate::R<DATA_BUFFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Data` reader - This register holds contents of the 8 MSBs of the DAT shift register."]
pub struct DATA_R(crate::FieldReader<u8, u8>);
impl DATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - This register holds contents of the 8 MSBs of the DAT shift register."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Data buffer register. The contents of the 8 MSBs of the I2DAT shift register will be transferred to the DATA_BUFFER automatically after every nine bits (8 bits of data plus ACK or NACK) has been received on the bus.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_buffer](index.html) module"]
pub struct DATA_BUFFER_SPEC;
impl crate::RegisterSpec for DATA_BUFFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_buffer::R](R) reader structure"]
impl crate::Readable for DATA_BUFFER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA_BUFFER to value 0"]
impl crate::Resettable for DATA_BUFFER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
