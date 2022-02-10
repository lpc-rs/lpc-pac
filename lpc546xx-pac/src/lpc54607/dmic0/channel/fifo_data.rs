///Register `FIFO_DATA` reader
pub struct R(crate::R<FIFO_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_DATA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FIFO_DATA` writer
pub struct W(crate::W<FIFO_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_DATA_SPEC>;
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
impl From<crate::W<FIFO_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_DATA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATA` reader - Data from the top of the input filter FIFO.
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DATA` writer - Data from the top of the input filter FIFO.
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:23 - Data from the top of the input filter FIFO.
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:23 - Data from the top of the input filter FIFO.
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FIFO Data Register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fifo_data](index.html) module
pub struct FIFO_DATA_SPEC;
impl crate::RegisterSpec for FIFO_DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [fifo_data::R](R) reader structure
impl crate::Readable for FIFO_DATA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fifo_data::W](W) writer structure
impl crate::Writable for FIFO_DATA_SPEC {
    type Writer = W;
}
///`reset()` method sets FIFO_DATA to value 0
impl crate::Resettable for FIFO_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
