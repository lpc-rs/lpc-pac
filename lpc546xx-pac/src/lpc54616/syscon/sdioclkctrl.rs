///Register `SDIOCLKCTRL` reader
pub struct R(crate::R<SDIOCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIOCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIOCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIOCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SDIOCLKCTRL` writer
pub struct W(crate::W<SDIOCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIOCLKCTRL_SPEC>;
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
impl From<crate::W<SDIOCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIOCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCLK_DRV_PHASE` reader - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in.
pub struct CCLK_DRV_PHASE_R(crate::FieldReader<u8, u8>);
impl CCLK_DRV_PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLK_DRV_PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_DRV_PHASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCLK_DRV_PHASE` writer - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in.
pub struct CCLK_DRV_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_PHASE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///Field `CCLK_SAMPLE_PHASE` reader - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.
pub struct CCLK_SAMPLE_PHASE_R(crate::FieldReader<u8, u8>);
impl CCLK_SAMPLE_PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLK_SAMPLE_PHASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_SAMPLE_PHASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCLK_SAMPLE_PHASE` writer - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.
pub struct CCLK_SAMPLE_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_PHASE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Field `PHASE_ACTIVE` reader - sdio_clk by 2, before feeding into ccl_in, cclk_in_sample, and cclk_in_drv.
pub struct PHASE_ACTIVE_R(crate::FieldReader<bool, bool>);
impl PHASE_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHASE_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHASE_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PHASE_ACTIVE` writer - sdio_clk by 2, before feeding into ccl_in, cclk_in_sample, and cclk_in_drv.
pub struct PHASE_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_ACTIVE_W<'a> {
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
///Field `CCLK_DRV_DELAY` reader - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in.
pub struct CCLK_DRV_DELAY_R(crate::FieldReader<u8, u8>);
impl CCLK_DRV_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLK_DRV_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_DRV_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCLK_DRV_DELAY` writer - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in.
pub struct CCLK_DRV_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_DELAY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
///Field `CCLK_DRV_DELAY_ACTIVE` reader - Enables drive delay, as controlled by the CCLK_DRV_DELAY field.
pub struct CCLK_DRV_DELAY_ACTIVE_R(crate::FieldReader<bool, bool>);
impl CCLK_DRV_DELAY_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK_DRV_DELAY_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_DRV_DELAY_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCLK_DRV_DELAY_ACTIVE` writer - Enables drive delay, as controlled by the CCLK_DRV_DELAY field.
pub struct CCLK_DRV_DELAY_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_DELAY_ACTIVE_W<'a> {
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
///Field `CCLK_SAMPLE_DELAY` reader - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.
pub struct CCLK_SAMPLE_DELAY_R(crate::FieldReader<u8, u8>);
impl CCLK_SAMPLE_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLK_SAMPLE_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_SAMPLE_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCLK_SAMPLE_DELAY` writer - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.
pub struct CCLK_SAMPLE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_DELAY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
///Field `CCLK_SAMPLE_DELAY_ACTIVE` reader - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field.
pub struct CCLK_SAMPLE_DELAY_ACTIVE_R(crate::FieldReader<bool, bool>);
impl CCLK_SAMPLE_DELAY_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK_SAMPLE_DELAY_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_SAMPLE_DELAY_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCLK_SAMPLE_DELAY_ACTIVE` writer - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field.
pub struct CCLK_SAMPLE_DELAY_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_DELAY_ACTIVE_W<'a> {
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
    ///Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in.
    #[inline(always)]
    pub fn cclk_drv_phase(&self) -> CCLK_DRV_PHASE_R {
        CCLK_DRV_PHASE_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.
    #[inline(always)]
    pub fn cclk_sample_phase(&self) -> CCLK_SAMPLE_PHASE_R {
        CCLK_SAMPLE_PHASE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bit 7 - sdio_clk by 2, before feeding into ccl_in, cclk_in_sample, and cclk_in_drv.
    #[inline(always)]
    pub fn phase_active(&self) -> PHASE_ACTIVE_R {
        PHASE_ACTIVE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in.
    #[inline(always)]
    pub fn cclk_drv_delay(&self) -> CCLK_DRV_DELAY_R {
        CCLK_DRV_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field.
    #[inline(always)]
    pub fn cclk_drv_delay_active(&self) -> CCLK_DRV_DELAY_ACTIVE_R {
        CCLK_DRV_DELAY_ACTIVE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.
    #[inline(always)]
    pub fn cclk_sample_delay(&self) -> CCLK_SAMPLE_DELAY_R {
        CCLK_SAMPLE_DELAY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field.
    #[inline(always)]
    pub fn cclk_sample_delay_active(&self) -> CCLK_SAMPLE_DELAY_ACTIVE_R {
        CCLK_SAMPLE_DELAY_ACTIVE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in.
    #[inline(always)]
    pub fn cclk_drv_phase(&mut self) -> CCLK_DRV_PHASE_W {
        CCLK_DRV_PHASE_W { w: self }
    }
    ///Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.
    #[inline(always)]
    pub fn cclk_sample_phase(&mut self) -> CCLK_SAMPLE_PHASE_W {
        CCLK_SAMPLE_PHASE_W { w: self }
    }
    ///Bit 7 - sdio_clk by 2, before feeding into ccl_in, cclk_in_sample, and cclk_in_drv.
    #[inline(always)]
    pub fn phase_active(&mut self) -> PHASE_ACTIVE_W {
        PHASE_ACTIVE_W { w: self }
    }
    ///Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in.
    #[inline(always)]
    pub fn cclk_drv_delay(&mut self) -> CCLK_DRV_DELAY_W {
        CCLK_DRV_DELAY_W { w: self }
    }
    ///Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field.
    #[inline(always)]
    pub fn cclk_drv_delay_active(&mut self) -> CCLK_DRV_DELAY_ACTIVE_W {
        CCLK_DRV_DELAY_ACTIVE_W { w: self }
    }
    ///Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.
    #[inline(always)]
    pub fn cclk_sample_delay(&mut self) -> CCLK_SAMPLE_DELAY_W {
        CCLK_SAMPLE_DELAY_W { w: self }
    }
    ///Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field.
    #[inline(always)]
    pub fn cclk_sample_delay_active(&mut self) -> CCLK_SAMPLE_DELAY_ACTIVE_W {
        CCLK_SAMPLE_DELAY_ACTIVE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SDIO CCLKIN phase and delay control
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdioclkctrl](index.html) module
pub struct SDIOCLKCTRL_SPEC;
impl crate::RegisterSpec for SDIOCLKCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdioclkctrl::R](R) reader structure
impl crate::Readable for SDIOCLKCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sdioclkctrl::W](W) writer structure
impl crate::Writable for SDIOCLKCTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets SDIOCLKCTRL to value 0
impl crate::Resettable for SDIOCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
