///Register `PIN[%s]` reader
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PIN[%s]` writer
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PORT` reader - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit.
pub struct PORT_R(crate::FieldReader<u32, u32>);
impl PORT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PORT` writer - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit.
pub struct PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit.
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit.
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Port pin register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pin](index.html) module
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
///`read()` method returns [pin::R](R) reader structure
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pin::W](W) writer structure
impl crate::Writable for PIN_SPEC {
    type Writer = W;
}
///`reset()` method sets PIN[%s]
///to value 0
impl crate::Resettable for PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
