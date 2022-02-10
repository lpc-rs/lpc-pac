///Register `CLKENA` reader
pub struct R(crate::R<CLKENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKENA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CLKENA` writer
pub struct W(crate::W<CLKENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKENA_SPEC>;
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
impl From<crate::W<CLKENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKENA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCLK_ENABLE` reader - Clock-enable control for SD card clock.
pub struct CCLK_ENABLE_R(crate::FieldReader<bool, bool>);
impl CCLK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCLK_ENABLE` writer - Clock-enable control for SD card clock.
pub struct CCLK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_ENABLE_W<'a> {
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
///Field `CCLK_LOW_POWER` reader - Low-power control for SD card clock.
pub struct CCLK_LOW_POWER_R(crate::FieldReader<bool, bool>);
impl CCLK_LOW_POWER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK_LOW_POWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_LOW_POWER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCLK_LOW_POWER` writer - Low-power control for SD card clock.
pub struct CCLK_LOW_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_LOW_POWER_W<'a> {
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
    ///Bit 0 - Clock-enable control for SD card clock.
    #[inline(always)]
    pub fn cclk_enable(&self) -> CCLK_ENABLE_R {
        CCLK_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 16 - Low-power control for SD card clock.
    #[inline(always)]
    pub fn cclk_low_power(&self) -> CCLK_LOW_POWER_R {
        CCLK_LOW_POWER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Clock-enable control for SD card clock.
    #[inline(always)]
    pub fn cclk_enable(&mut self) -> CCLK_ENABLE_W {
        CCLK_ENABLE_W { w: self }
    }
    ///Bit 16 - Low-power control for SD card clock.
    #[inline(always)]
    pub fn cclk_low_power(&mut self) -> CCLK_LOW_POWER_W {
        CCLK_LOW_POWER_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock Enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clkena](index.html) module
pub struct CLKENA_SPEC;
impl crate::RegisterSpec for CLKENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [clkena::R](R) reader structure
impl crate::Readable for CLKENA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [clkena::W](W) writer structure
impl crate::Writable for CLKENA_SPEC {
    type Writer = W;
}
///`reset()` method sets CLKENA to value 0
impl crate::Resettable for CLKENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
