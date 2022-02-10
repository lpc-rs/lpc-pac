///Register `RXF0C` reader
pub struct R(crate::R<RXF0C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0C_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RXF0C` writer
pub struct W(crate::W<RXF0C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF0C_SPEC>;
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
impl From<crate::W<RXF0C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF0C_SPEC>) -> Self {
        W(writer)
    }
}
///Field `F0SA` reader - Rx FIFO 0 start address.
pub struct F0SA_R(crate::FieldReader<u16, u16>);
impl F0SA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        F0SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0SA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F0SA` writer - Rx FIFO 0 start address.
pub struct F0SA_W<'a> {
    w: &'a mut W,
}
impl<'a> F0SA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
///Field `F0S` reader - Rx FIFO 0 size.
pub struct F0S_R(crate::FieldReader<u8, u8>);
impl F0S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F0S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F0S` writer - Rx FIFO 0 size.
pub struct F0S_W<'a> {
    w: &'a mut W,
}
impl<'a> F0S_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
///Field `F0WM` reader - Rx FIFO 0 watermark 0 = Watermark interrupt disabled.
pub struct F0WM_R(crate::FieldReader<u8, u8>);
impl F0WM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F0WM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0WM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F0WM` writer - Rx FIFO 0 watermark 0 = Watermark interrupt disabled.
pub struct F0WM_W<'a> {
    w: &'a mut W,
}
impl<'a> F0WM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
///Field `F0OM` reader - FIFO 0 operation mode.
pub struct F0OM_R(crate::FieldReader<bool, bool>);
impl F0OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F0OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F0OM` writer - FIFO 0 operation mode.
pub struct F0OM_W<'a> {
    w: &'a mut W,
}
impl<'a> F0OM_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 2:15 - Rx FIFO 0 start address.
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:22 - Rx FIFO 0 size.
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - Rx FIFO 0 watermark 0 = Watermark interrupt disabled.
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    ///Bit 31 - FIFO 0 operation mode.
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 2:15 - Rx FIFO 0 start address.
    #[inline(always)]
    pub fn f0sa(&mut self) -> F0SA_W {
        F0SA_W { w: self }
    }
    ///Bits 16:22 - Rx FIFO 0 size.
    #[inline(always)]
    pub fn f0s(&mut self) -> F0S_W {
        F0S_W { w: self }
    }
    ///Bits 24:30 - Rx FIFO 0 watermark 0 = Watermark interrupt disabled.
    #[inline(always)]
    pub fn f0wm(&mut self) -> F0WM_W {
        F0WM_W { w: self }
    }
    ///Bit 31 - FIFO 0 operation mode.
    #[inline(always)]
    pub fn f0om(&mut self) -> F0OM_W {
        F0OM_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Rx FIFO 0 Configuration
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxf0c](index.html) module
pub struct RXF0C_SPEC;
impl crate::RegisterSpec for RXF0C_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxf0c::R](R) reader structure
impl crate::Readable for RXF0C_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rxf0c::W](W) writer structure
impl crate::Writable for RXF0C_SPEC {
    type Writer = W;
}
///`reset()` method sets RXF0C to value 0
impl crate::Resettable for RXF0C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
