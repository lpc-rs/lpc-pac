///Register `USB0CLKCTRL` reader
pub struct R(crate::R<USB0CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `USB0CLKCTRL` writer
pub struct W(crate::W<USB0CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0CLKCTRL_SPEC>;
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
impl From<crate::W<USB0CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AP_FS_DEV_CLK` reader - USB0 Device USB0_NEEDCLK signal control.
pub struct AP_FS_DEV_CLK_R(crate::FieldReader<bool, bool>);
impl AP_FS_DEV_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AP_FS_DEV_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP_FS_DEV_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AP_FS_DEV_CLK` writer - USB0 Device USB0_NEEDCLK signal control.
pub struct AP_FS_DEV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_FS_DEV_CLK_W<'a> {
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
///Field `POL_FS_DEV_CLK` reader - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt.
pub struct POL_FS_DEV_CLK_R(crate::FieldReader<bool, bool>);
impl POL_FS_DEV_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_FS_DEV_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_FS_DEV_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `POL_FS_DEV_CLK` writer - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt.
pub struct POL_FS_DEV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_FS_DEV_CLK_W<'a> {
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
///Field `AP_FS_HOST_CLK` reader - USB0 Host USB0_NEEDCLK signal control.
pub struct AP_FS_HOST_CLK_R(crate::FieldReader<bool, bool>);
impl AP_FS_HOST_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AP_FS_HOST_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP_FS_HOST_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AP_FS_HOST_CLK` writer - USB0 Host USB0_NEEDCLK signal control.
pub struct AP_FS_HOST_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_FS_HOST_CLK_W<'a> {
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
///Field `POL_FS_HOST_CLK` reader - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt.
pub struct POL_FS_HOST_CLK_R(crate::FieldReader<bool, bool>);
impl POL_FS_HOST_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_FS_HOST_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_FS_HOST_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `POL_FS_HOST_CLK` writer - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt.
pub struct POL_FS_HOST_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_FS_HOST_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `PU_DISABLE` reader - Internal pull-up disable control.
pub struct PU_DISABLE_R(crate::FieldReader<bool, bool>);
impl PU_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PU_DISABLE` writer - Internal pull-up disable control.
pub struct PU_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DISABLE_W<'a> {
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
impl R {
    ///Bit 0 - USB0 Device USB0_NEEDCLK signal control.
    #[inline(always)]
    pub fn ap_fs_dev_clk(&self) -> AP_FS_DEV_CLK_R {
        AP_FS_DEV_CLK_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt.
    #[inline(always)]
    pub fn pol_fs_dev_clk(&self) -> POL_FS_DEV_CLK_R {
        POL_FS_DEV_CLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - USB0 Host USB0_NEEDCLK signal control.
    #[inline(always)]
    pub fn ap_fs_host_clk(&self) -> AP_FS_HOST_CLK_R {
        AP_FS_HOST_CLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt.
    #[inline(always)]
    pub fn pol_fs_host_clk(&self) -> POL_FS_HOST_CLK_R {
        POL_FS_HOST_CLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Internal pull-up disable control.
    #[inline(always)]
    pub fn pu_disable(&self) -> PU_DISABLE_R {
        PU_DISABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - USB0 Device USB0_NEEDCLK signal control.
    #[inline(always)]
    pub fn ap_fs_dev_clk(&mut self) -> AP_FS_DEV_CLK_W {
        AP_FS_DEV_CLK_W { w: self }
    }
    ///Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt.
    #[inline(always)]
    pub fn pol_fs_dev_clk(&mut self) -> POL_FS_DEV_CLK_W {
        POL_FS_DEV_CLK_W { w: self }
    }
    ///Bit 2 - USB0 Host USB0_NEEDCLK signal control.
    #[inline(always)]
    pub fn ap_fs_host_clk(&mut self) -> AP_FS_HOST_CLK_W {
        AP_FS_HOST_CLK_W { w: self }
    }
    ///Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt.
    #[inline(always)]
    pub fn pol_fs_host_clk(&mut self) -> POL_FS_HOST_CLK_W {
        POL_FS_HOST_CLK_W { w: self }
    }
    ///Bit 4 - Internal pull-up disable control.
    #[inline(always)]
    pub fn pu_disable(&mut self) -> PU_DISABLE_W {
        PU_DISABLE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///USB0 clock control
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [usb0clkctrl](index.html) module
pub struct USB0CLKCTRL_SPEC;
impl crate::RegisterSpec for USB0CLKCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [usb0clkctrl::R](R) reader structure
impl crate::Readable for USB0CLKCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [usb0clkctrl::W](W) writer structure
impl crate::Writable for USB0CLKCTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets USB0CLKCTRL to value 0
impl crate::Resettable for USB0CLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
