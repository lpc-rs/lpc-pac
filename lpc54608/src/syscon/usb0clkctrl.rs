#[doc = "Reader of register USB0CLKCTRL"]
pub type R = crate::R<u32, super::USB0CLKCTRL>;
#[doc = "Writer for register USB0CLKCTRL"]
pub type W = crate::W<u32, super::USB0CLKCTRL>;
#[doc = "Register USB0CLKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::USB0CLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AP_FS_DEV_CLK`"]
pub type AP_FS_DEV_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AP_FS_DEV_CLK`"]
pub struct AP_FS_DEV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_FS_DEV_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `POL_FS_DEV_CLK`"]
pub type POL_FS_DEV_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POL_FS_DEV_CLK`"]
pub struct POL_FS_DEV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_FS_DEV_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AP_FS_HOST_CLK`"]
pub type AP_FS_HOST_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AP_FS_HOST_CLK`"]
pub struct AP_FS_HOST_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_FS_HOST_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `POL_FS_HOST_CLK`"]
pub type POL_FS_HOST_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POL_FS_HOST_CLK`"]
pub struct POL_FS_HOST_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_FS_HOST_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PU_DISABLE`"]
pub type PU_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PU_DISABLE`"]
pub struct PU_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal control."]
    #[inline(always)]
    pub fn ap_fs_dev_clk(&self) -> AP_FS_DEV_CLK_R {
        AP_FS_DEV_CLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_dev_clk(&self) -> POL_FS_DEV_CLK_R {
        POL_FS_DEV_CLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB0 Host USB0_NEEDCLK signal control."]
    #[inline(always)]
    pub fn ap_fs_host_clk(&self) -> AP_FS_HOST_CLK_R {
        AP_FS_HOST_CLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_host_clk(&self) -> POL_FS_HOST_CLK_R {
        POL_FS_HOST_CLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Internal pull-up disable control."]
    #[inline(always)]
    pub fn pu_disable(&self) -> PU_DISABLE_R {
        PU_DISABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal control."]
    #[inline(always)]
    pub fn ap_fs_dev_clk(&mut self) -> AP_FS_DEV_CLK_W {
        AP_FS_DEV_CLK_W { w: self }
    }
    #[doc = "Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_dev_clk(&mut self) -> POL_FS_DEV_CLK_W {
        POL_FS_DEV_CLK_W { w: self }
    }
    #[doc = "Bit 2 - USB0 Host USB0_NEEDCLK signal control."]
    #[inline(always)]
    pub fn ap_fs_host_clk(&mut self) -> AP_FS_HOST_CLK_W {
        AP_FS_HOST_CLK_W { w: self }
    }
    #[doc = "Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_host_clk(&mut self) -> POL_FS_HOST_CLK_W {
        POL_FS_HOST_CLK_W { w: self }
    }
    #[doc = "Bit 4 - Internal pull-up disable control."]
    #[inline(always)]
    pub fn pu_disable(&mut self) -> PU_DISABLE_W {
        PU_DISABLE_W { w: self }
    }
}
