#[doc = "Register `HWWAKE` reader"]
pub struct R(crate::R<HWWAKE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWWAKE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWWAKE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWWAKE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWWAKE` writer"]
pub struct W(crate::W<HWWAKE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWWAKE_SPEC>;
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
impl From<crate::W<HWWAKE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWWAKE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCEWAKE` reader - Force peripheral clocking to stay on during Deep Sleep and Power-down modes. When 1, clocking to peripherals is prevented from being shut down when the CPU enters Deep Sleep and Power-down modes. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
pub struct FORCEWAKE_R(crate::FieldReader<bool, bool>);
impl FORCEWAKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCEWAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCEWAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCEWAKE` writer - Force peripheral clocking to stay on during Deep Sleep and Power-down modes. When 1, clocking to peripherals is prevented from being shut down when the CPU enters Deep Sleep and Power-down modes. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
pub struct FORCEWAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEWAKE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `FCWAKE` reader - Wake for Flexcomms. When 1, any Flexcomm FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
pub struct FCWAKE_R(crate::FieldReader<bool, bool>);
impl FCWAKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCWAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCWAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCWAKE` writer - Wake for Flexcomms. When 1, any Flexcomm FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
pub struct FCWAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCWAKE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `WAKEDMIC` reader - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
pub struct WAKEDMIC_R(crate::FieldReader<bool, bool>);
impl WAKEDMIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEDMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEDMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEDMIC` writer - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
pub struct WAKEDMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEDMIC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `WAKEDMA` reader - Wake for DMA. When 1, DMA being busy will cause peripheral clocking to remain running until DMA completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMA has completed its related activity."]
pub struct WAKEDMA_R(crate::FieldReader<bool, bool>);
impl WAKEDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEDMA` writer - Wake for DMA. When 1, DMA being busy will cause peripheral clocking to remain running until DMA completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMA has completed its related activity."]
pub struct WAKEDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEDMA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Force peripheral clocking to stay on during Deep Sleep and Power-down modes. When 1, clocking to peripherals is prevented from being shut down when the CPU enters Deep Sleep and Power-down modes. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
    #[inline(always)]
    pub fn forcewake(&self) -> FORCEWAKE_R {
        FORCEWAKE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake for Flexcomms. When 1, any Flexcomm FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn fcwake(&self) -> FCWAKE_R {
        FCWAKE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn wakedmic(&self) -> WAKEDMIC_R {
        WAKEDMIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wake for DMA. When 1, DMA being busy will cause peripheral clocking to remain running until DMA completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMA has completed its related activity."]
    #[inline(always)]
    pub fn wakedma(&self) -> WAKEDMA_R {
        WAKEDMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force peripheral clocking to stay on during Deep Sleep and Power-down modes. When 1, clocking to peripherals is prevented from being shut down when the CPU enters Deep Sleep and Power-down modes. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
    #[inline(always)]
    pub fn forcewake(&mut self) -> FORCEWAKE_W {
        FORCEWAKE_W { w: self }
    }
    #[doc = "Bit 1 - Wake for Flexcomms. When 1, any Flexcomm FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn fcwake(&mut self) -> FCWAKE_W {
        FCWAKE_W { w: self }
    }
    #[doc = "Bit 2 - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn wakedmic(&mut self) -> WAKEDMIC_W {
        WAKEDMIC_W { w: self }
    }
    #[doc = "Bit 3 - Wake for DMA. When 1, DMA being busy will cause peripheral clocking to remain running until DMA completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMA has completed its related activity."]
    #[inline(always)]
    pub fn wakedma(&mut self) -> WAKEDMA_W {
        WAKEDMA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures special cases of hardware wake-up\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwwake](index.html) module"]
pub struct HWWAKE_SPEC;
impl crate::RegisterSpec for HWWAKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwwake::R](R) reader structure"]
impl crate::Readable for HWWAKE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwwake::W](W) writer structure"]
impl crate::Writable for HWWAKE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWWAKE to value 0"]
impl crate::Resettable for HWWAKE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
