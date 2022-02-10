///Register `CTRL` reader
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CTRL` writer
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LCDEN` reader - LCD enable control bit.
pub struct LCDEN_R(crate::FieldReader<bool, bool>);
impl LCDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCDEN` writer - LCD enable control bit.
pub struct LCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Field `LCDBPP` reader - LCD bits per pixel.
pub struct LCDBPP_R(crate::FieldReader<u8, u8>);
impl LCDBPP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCDBPP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDBPP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCDBPP` writer - LCD bits per pixel.
pub struct LCDBPP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBPP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
///Field `LCDBW` reader - STN LCD monochrome/color selection.
pub struct LCDBW_R(crate::FieldReader<bool, bool>);
impl LCDBW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCDBW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDBW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCDBW` writer - STN LCD monochrome/color selection.
pub struct LCDBW_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDBW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Field `LCDTFT` reader - LCD panel TFT type selection.
pub struct LCDTFT_R(crate::FieldReader<bool, bool>);
impl LCDTFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCDTFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDTFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCDTFT` writer - LCD panel TFT type selection.
pub struct LCDTFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDTFT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Field `LCDMONO8` reader - Monochrome LCD interface width.
pub struct LCDMONO8_R(crate::FieldReader<bool, bool>);
impl LCDMONO8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCDMONO8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDMONO8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCDMONO8` writer - Monochrome LCD interface width.
pub struct LCDMONO8_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDMONO8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Field `LCDDUAL` reader - Single or Dual LCD panel selection.
pub struct LCDDUAL_R(crate::FieldReader<bool, bool>);
impl LCDDUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCDDUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDDUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCDDUAL` writer - Single or Dual LCD panel selection.
pub struct LCDDUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDDUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///Field `BGR` reader - Color format selection.
pub struct BGR_R(crate::FieldReader<bool, bool>);
impl BGR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BGR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BGR` writer - Color format selection.
pub struct BGR_W<'a> {
    w: &'a mut W,
}
impl<'a> BGR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Field `BEBO` reader - Big-endian Byte Order.
pub struct BEBO_R(crate::FieldReader<bool, bool>);
impl BEBO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEBO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEBO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BEBO` writer - Big-endian Byte Order.
pub struct BEBO_W<'a> {
    w: &'a mut W,
}
impl<'a> BEBO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Field `BEPO` reader - Big-Endian Pixel Ordering.
pub struct BEPO_R(crate::FieldReader<bool, bool>);
impl BEPO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEPO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEPO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BEPO` writer - Big-Endian Pixel Ordering.
pub struct BEPO_W<'a> {
    w: &'a mut W,
}
impl<'a> BEPO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Field `LCDPWR` reader - LCD power enable.
pub struct LCDPWR_R(crate::FieldReader<bool, bool>);
impl LCDPWR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCDPWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDPWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCDPWR` writer - LCD power enable.
pub struct LCDPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDPWR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Field `LCDVCOMP` reader - LCD Vertical Compare Interrupt.
pub struct LCDVCOMP_R(crate::FieldReader<u8, u8>);
impl LCDVCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LCDVCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDVCOMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCDVCOMP` writer - LCD Vertical Compare Interrupt.
pub struct LCDVCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDVCOMP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
///Field `WATERMARK` reader - LCD DMA FIFO watermark level.
pub struct WATERMARK_R(crate::FieldReader<bool, bool>);
impl WATERMARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WATERMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WATERMARK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WATERMARK` writer - LCD DMA FIFO watermark level.
pub struct WATERMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> WATERMARK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - LCD enable control bit.
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:3 - LCD bits per pixel.
    #[inline(always)]
    pub fn lcdbpp(&self) -> LCDBPP_R {
        LCDBPP_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    ///Bit 4 - STN LCD monochrome/color selection.
    #[inline(always)]
    pub fn lcdbw(&self) -> LCDBW_R {
        LCDBW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - LCD panel TFT type selection.
    #[inline(always)]
    pub fn lcdtft(&self) -> LCDTFT_R {
        LCDTFT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Monochrome LCD interface width.
    #[inline(always)]
    pub fn lcdmono8(&self) -> LCDMONO8_R {
        LCDMONO8_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Single or Dual LCD panel selection.
    #[inline(always)]
    pub fn lcddual(&self) -> LCDDUAL_R {
        LCDDUAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Color format selection.
    #[inline(always)]
    pub fn bgr(&self) -> BGR_R {
        BGR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Big-endian Byte Order.
    #[inline(always)]
    pub fn bebo(&self) -> BEBO_R {
        BEBO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Big-Endian Pixel Ordering.
    #[inline(always)]
    pub fn bepo(&self) -> BEPO_R {
        BEPO_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - LCD power enable.
    #[inline(always)]
    pub fn lcdpwr(&self) -> LCDPWR_R {
        LCDPWR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bits 12:13 - LCD Vertical Compare Interrupt.
    #[inline(always)]
    pub fn lcdvcomp(&self) -> LCDVCOMP_R {
        LCDVCOMP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bit 16 - LCD DMA FIFO watermark level.
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - LCD enable control bit.
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W {
        LCDEN_W { w: self }
    }
    ///Bits 1:3 - LCD bits per pixel.
    #[inline(always)]
    pub fn lcdbpp(&mut self) -> LCDBPP_W {
        LCDBPP_W { w: self }
    }
    ///Bit 4 - STN LCD monochrome/color selection.
    #[inline(always)]
    pub fn lcdbw(&mut self) -> LCDBW_W {
        LCDBW_W { w: self }
    }
    ///Bit 5 - LCD panel TFT type selection.
    #[inline(always)]
    pub fn lcdtft(&mut self) -> LCDTFT_W {
        LCDTFT_W { w: self }
    }
    ///Bit 6 - Monochrome LCD interface width.
    #[inline(always)]
    pub fn lcdmono8(&mut self) -> LCDMONO8_W {
        LCDMONO8_W { w: self }
    }
    ///Bit 7 - Single or Dual LCD panel selection.
    #[inline(always)]
    pub fn lcddual(&mut self) -> LCDDUAL_W {
        LCDDUAL_W { w: self }
    }
    ///Bit 8 - Color format selection.
    #[inline(always)]
    pub fn bgr(&mut self) -> BGR_W {
        BGR_W { w: self }
    }
    ///Bit 9 - Big-endian Byte Order.
    #[inline(always)]
    pub fn bebo(&mut self) -> BEBO_W {
        BEBO_W { w: self }
    }
    ///Bit 10 - Big-Endian Pixel Ordering.
    #[inline(always)]
    pub fn bepo(&mut self) -> BEPO_W {
        BEPO_W { w: self }
    }
    ///Bit 11 - LCD power enable.
    #[inline(always)]
    pub fn lcdpwr(&mut self) -> LCDPWR_W {
        LCDPWR_W { w: self }
    }
    ///Bits 12:13 - LCD Vertical Compare Interrupt.
    #[inline(always)]
    pub fn lcdvcomp(&mut self) -> LCDVCOMP_W {
        LCDVCOMP_W { w: self }
    }
    ///Bit 16 - LCD DMA FIFO watermark level.
    #[inline(always)]
    pub fn watermark(&mut self) -> WATERMARK_W {
        WATERMARK_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LCD Control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ctrl](index.html) module
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [ctrl::R](R) reader structure
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ctrl::W](W) writer structure
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
