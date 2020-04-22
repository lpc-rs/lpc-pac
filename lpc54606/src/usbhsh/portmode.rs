#[doc = "Reader of register PORTMODE"]
pub type R = crate::R<u32, super::PORTMODE>;
#[doc = "Writer for register PORTMODE"]
pub type W = crate::W<u32, super::PORTMODE>;
#[doc = "Register PORTMODE `reset()`'s with value 0x0004_0000"]
impl crate::ResetValue for super::PORTMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0000
    }
}
#[doc = "Reader of field `ID0`"]
pub type ID0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ID0`"]
pub struct ID0_W<'a> {
    w: &'a mut W,
}
impl<'a> ID0_W<'a> {
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
#[doc = "Reader of field `ID0_EN`"]
pub type ID0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ID0_EN`"]
pub struct ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ID0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DEV_ENABLE`"]
pub type DEV_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_ENABLE`"]
pub struct DEV_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SW_CTRL_PDCOM`"]
pub type SW_CTRL_PDCOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_CTRL_PDCOM`"]
pub struct SW_CTRL_PDCOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CTRL_PDCOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SW_PDCOM`"]
pub type SW_PDCOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_PDCOM`"]
pub struct SW_PDCOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_PDCOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port 0 ID pin value."]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port 0 ID pin pull-up enable."]
    #[inline(always)]
    pub fn id0_en(&self) -> ID0_EN_R {
        ID0_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - If this bit is set to one, one of the ports will behave as a USB device."]
    #[inline(always)]
    pub fn dev_enable(&self) -> DEV_ENABLE_R {
        DEV_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[inline(always)]
    pub fn sw_ctrl_pdcom(&self) -> SW_CTRL_PDCOM_R {
        SW_CTRL_PDCOM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[inline(always)]
    pub fn sw_pdcom(&self) -> SW_PDCOM_R {
        SW_PDCOM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 0 ID pin value."]
    #[inline(always)]
    pub fn id0(&mut self) -> ID0_W {
        ID0_W { w: self }
    }
    #[doc = "Bit 8 - Port 0 ID pin pull-up enable."]
    #[inline(always)]
    pub fn id0_en(&mut self) -> ID0_EN_W {
        ID0_EN_W { w: self }
    }
    #[doc = "Bit 16 - If this bit is set to one, one of the ports will behave as a USB device."]
    #[inline(always)]
    pub fn dev_enable(&mut self) -> DEV_ENABLE_W {
        DEV_ENABLE_W { w: self }
    }
    #[doc = "Bit 18 - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[inline(always)]
    pub fn sw_ctrl_pdcom(&mut self) -> SW_CTRL_PDCOM_W {
        SW_CTRL_PDCOM_W { w: self }
    }
    #[doc = "Bit 19 - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[inline(always)]
    pub fn sw_pdcom(&mut self) -> SW_PDCOM_W {
        SW_PDCOM_W { w: self }
    }
}
