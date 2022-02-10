///Register `MRBA` reader
pub struct R(crate::R<MRBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRBA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRBA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MRBA` writer
pub struct W(crate::W<MRBA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRBA_SPEC>;
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
impl From<crate::W<MRBA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRBA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BA` reader - Base address for the message RAM in the chip memory map.
pub struct BA_R(crate::FieldReader<u16, u16>);
impl BA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BA` writer - Base address for the message RAM in the chip memory map.
pub struct BA_W<'a> {
    w: &'a mut W,
}
impl<'a> BA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 16:31 - Base address for the message RAM in the chip memory map.
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:31 - Base address for the message RAM in the chip memory map.
    #[inline(always)]
    pub fn ba(&mut self) -> BA_W {
        BA_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CAN Message RAM Base Address
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mrba](index.html) module
pub struct MRBA_SPEC;
impl crate::RegisterSpec for MRBA_SPEC {
    type Ux = u32;
}
///`read()` method returns [mrba::R](R) reader structure
impl crate::Readable for MRBA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mrba::W](W) writer structure
impl crate::Writable for MRBA_SPEC {
    type Writer = W;
}
///`reset()` method sets MRBA to value 0
impl crate::Resettable for MRBA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
