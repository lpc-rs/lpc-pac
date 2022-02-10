///Register `USB1CLKCTRL` reader
pub struct R(crate::R<USB1CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `USB1CLKCTRL` writer
pub struct W(crate::W<USB1CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1CLKCTRL_SPEC>;
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
impl From<crate::W<USB1CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AP_FS_DEV_CLK` reader - USB1 Device need_clock signal control.
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
///Field `AP_FS_DEV_CLK` writer - USB1 Device need_clock signal control.
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
///Field `POL_FS_DEV_CLK` reader - USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt.
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
///Field `POL_FS_DEV_CLK` writer - USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt.
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
///Field `AP_FS_HOST_CLK` reader - USB1 Host need_clock signal control.
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
///Field `AP_FS_HOST_CLK` writer - USB1 Host need_clock signal control.
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
///Field `POL_FS_HOST_CLK` reader - USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt.
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
///Field `POL_FS_HOST_CLK` writer - USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt.
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
///Field `HS_DEV_WAKEUP_N` reader - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic.
pub struct HS_DEV_WAKEUP_N_R(crate::FieldReader<bool, bool>);
impl HS_DEV_WAKEUP_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HS_DEV_WAKEUP_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS_DEV_WAKEUP_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HS_DEV_WAKEUP_N` writer - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic.
pub struct HS_DEV_WAKEUP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_DEV_WAKEUP_N_W<'a> {
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
    ///Bit 0 - USB1 Device need_clock signal control.
    #[inline(always)]
    pub fn ap_fs_dev_clk(&self) -> AP_FS_DEV_CLK_R {
        AP_FS_DEV_CLK_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt.
    #[inline(always)]
    pub fn pol_fs_dev_clk(&self) -> POL_FS_DEV_CLK_R {
        POL_FS_DEV_CLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - USB1 Host need_clock signal control.
    #[inline(always)]
    pub fn ap_fs_host_clk(&self) -> AP_FS_HOST_CLK_R {
        AP_FS_HOST_CLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt.
    #[inline(always)]
    pub fn pol_fs_host_clk(&self) -> POL_FS_HOST_CLK_R {
        POL_FS_HOST_CLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic.
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&self) -> HS_DEV_WAKEUP_N_R {
        HS_DEV_WAKEUP_N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - USB1 Device need_clock signal control.
    #[inline(always)]
    pub fn ap_fs_dev_clk(&mut self) -> AP_FS_DEV_CLK_W {
        AP_FS_DEV_CLK_W { w: self }
    }
    ///Bit 1 - USB1 Device need_clock polarity for triggering the USB1 wake-up interrupt.
    #[inline(always)]
    pub fn pol_fs_dev_clk(&mut self) -> POL_FS_DEV_CLK_W {
        POL_FS_DEV_CLK_W { w: self }
    }
    ///Bit 2 - USB1 Host need_clock signal control.
    #[inline(always)]
    pub fn ap_fs_host_clk(&mut self) -> AP_FS_HOST_CLK_W {
        AP_FS_HOST_CLK_W { w: self }
    }
    ///Bit 3 - USB1 Host need_clock polarity for triggering the USB1 wake-up interrupt.
    #[inline(always)]
    pub fn pol_fs_host_clk(&mut self) -> POL_FS_HOST_CLK_W {
        POL_FS_HOST_CLK_W { w: self }
    }
    ///Bit 4 - External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic.
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&mut self) -> HS_DEV_WAKEUP_N_W {
        HS_DEV_WAKEUP_N_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///USB1 clock control
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [usb1clkctrl](index.html) module
pub struct USB1CLKCTRL_SPEC;
impl crate::RegisterSpec for USB1CLKCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [usb1clkctrl::R](R) reader structure
impl crate::Readable for USB1CLKCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [usb1clkctrl::W](W) writer structure
impl crate::Writable for USB1CLKCTRL_SPEC {
    type Writer = W;
}
///`reset()` method sets USB1CLKCTRL to value 0x10
impl crate::Resettable for USB1CLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
