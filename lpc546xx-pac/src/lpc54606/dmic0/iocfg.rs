///Register `IOCFG` reader
pub struct R(crate::R<IOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCFG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IOCFG` writer
pub struct W(crate::W<IOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCFG_SPEC>;
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
impl From<crate::W<IOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCFG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CLK_BYPASS0` reader - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus.
pub struct CLK_BYPASS0_R(crate::FieldReader<bool, bool>);
impl CLK_BYPASS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_BYPASS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_BYPASS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CLK_BYPASS0` writer - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus.
pub struct CLK_BYPASS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_BYPASS0_W<'a> {
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
///Field `CLK_BYPASS1` reader - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus.
pub struct CLK_BYPASS1_R(crate::FieldReader<bool, bool>);
impl CLK_BYPASS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_BYPASS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_BYPASS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CLK_BYPASS1` writer - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus.
pub struct CLK_BYPASS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_BYPASS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Field `STEREO_DATA0` reader - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone.
pub struct STEREO_DATA0_R(crate::FieldReader<bool, bool>);
impl STEREO_DATA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STEREO_DATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STEREO_DATA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `STEREO_DATA0` writer - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone.
pub struct STEREO_DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> STEREO_DATA0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    ///Bit 0 - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus.
    #[inline(always)]
    pub fn clk_bypass0(&self) -> CLK_BYPASS0_R {
        CLK_BYPASS0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus.
    #[inline(always)]
    pub fn clk_bypass1(&self) -> CLK_BYPASS1_R {
        CLK_BYPASS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone.
    #[inline(always)]
    pub fn stereo_data0(&self) -> STEREO_DATA0_R {
        STEREO_DATA0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus.
    #[inline(always)]
    pub fn clk_bypass0(&mut self) -> CLK_BYPASS0_W {
        CLK_BYPASS0_W { w: self }
    }
    ///Bit 1 - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus.
    #[inline(always)]
    pub fn clk_bypass1(&mut self) -> CLK_BYPASS1_W {
        CLK_BYPASS1_W { w: self }
    }
    ///Bit 2 - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone.
    #[inline(always)]
    pub fn stereo_data0(&mut self) -> STEREO_DATA0_W {
        STEREO_DATA0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I/O Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iocfg](index.html) module
pub struct IOCFG_SPEC;
impl crate::RegisterSpec for IOCFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [iocfg::R](R) reader structure
impl crate::Readable for IOCFG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [iocfg::W](W) writer structure
impl crate::Writable for IOCFG_SPEC {
    type Writer = W;
}
///`reset()` method sets IOCFG to value 0
impl crate::Resettable for IOCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
