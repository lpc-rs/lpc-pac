///Register `DBTP` reader
pub struct R(crate::R<DBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBTP_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DBTP` writer
pub struct W(crate::W<DBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBTP_SPEC>;
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
impl From<crate::W<DBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBTP_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DSJW` reader - Data (re)synchronization jump width.
pub struct DSJW_R(crate::FieldReader<u8, u8>);
impl DSJW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DSJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DSJW` writer - Data (re)synchronization jump width.
pub struct DSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> DSJW_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
///Field `DTSEG2` reader - Data time segment after sample point.
pub struct DTSEG2_R(crate::FieldReader<u8, u8>);
impl DTSEG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DTSEG2` writer - Data time segment after sample point.
pub struct DTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///Field `DTSEG1` reader - Data time segment before sample point.
pub struct DTSEG1_R(crate::FieldReader<u8, u8>);
impl DTSEG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DTSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DTSEG1` writer - Data time segment before sample point.
pub struct DTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
///Field `DBRP` reader - Data bit rate prescaler.
pub struct DBRP_R(crate::FieldReader<u8, u8>);
impl DBRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBRP` writer - Data bit rate prescaler.
pub struct DBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBRP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
///Field `TDC` reader - Transmitter delay compensation.
pub struct TDC_R(crate::FieldReader<bool, bool>);
impl TDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TDC` writer - Transmitter delay compensation.
pub struct TDC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Data (re)synchronization jump width.
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Data time segment after sample point.
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:12 - Data time segment before sample point.
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Data bit rate prescaler.
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 23 - Transmitter delay compensation.
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:3 - Data (re)synchronization jump width.
    #[inline(always)]
    pub fn dsjw(&mut self) -> DSJW_W {
        DSJW_W { w: self }
    }
    ///Bits 4:7 - Data time segment after sample point.
    #[inline(always)]
    pub fn dtseg2(&mut self) -> DTSEG2_W {
        DTSEG2_W { w: self }
    }
    ///Bits 8:12 - Data time segment before sample point.
    #[inline(always)]
    pub fn dtseg1(&mut self) -> DTSEG1_W {
        DTSEG1_W { w: self }
    }
    ///Bits 16:20 - Data bit rate prescaler.
    #[inline(always)]
    pub fn dbrp(&mut self) -> DBRP_W {
        DBRP_W { w: self }
    }
    ///Bit 23 - Transmitter delay compensation.
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W {
        TDC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Data Bit Timing Prescaler Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dbtp](index.html) module
pub struct DBTP_SPEC;
impl crate::RegisterSpec for DBTP_SPEC {
    type Ux = u32;
}
///`read()` method returns [dbtp::R](R) reader structure
impl crate::Readable for DBTP_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dbtp::W](W) writer structure
impl crate::Writable for DBTP_SPEC {
    type Writer = W;
}
///`reset()` method sets DBTP to value 0x0a33
impl crate::Resettable for DBTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a33
    }
}
