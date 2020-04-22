#[doc = "Reader of register USBINTR"]
pub type R = crate::R<u32, super::USBINTR>;
#[doc = "Writer for register USBINTR"]
pub type W = crate::W<u32, super::USBINTR>;
#[doc = "Register USBINTR `reset()`'s with value 0"]
impl crate::ResetValue for super::USBINTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCDE`"]
pub type PCDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCDE`"]
pub struct PCDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCDE_W<'a> {
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
#[doc = "Reader of field `FLRE`"]
pub type FLRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLRE`"]
pub struct FLRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLRE_W<'a> {
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
#[doc = "Reader of field `ATL_IRQ_E`"]
pub type ATL_IRQ_E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATL_IRQ_E`"]
pub struct ATL_IRQ_E_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_IRQ_E_W<'a> {
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
#[doc = "Reader of field `ISO_IRQ_E`"]
pub type ISO_IRQ_E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISO_IRQ_E`"]
pub struct ISO_IRQ_E_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_IRQ_E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `INT_IRQ_E`"]
pub type INT_IRQ_E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_IRQ_E`"]
pub struct INT_IRQ_E_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_IRQ_E_W<'a> {
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
#[doc = "Reader of field `SOF_E`"]
pub type SOF_E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF_E`"]
pub struct SOF_E_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_E_W<'a> {
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
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn pcde(&self) -> PCDE_R {
        PCDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn flre(&self) -> FLRE_R {
        FLRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ATL IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn atl_irq_e(&self) -> ATL_IRQ_E_R {
        ATL_IRQ_E_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ISO IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn iso_irq_e(&self) -> ISO_IRQ_E_R {
        ISO_IRQ_E_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - INT IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn int_irq_e(&self) -> INT_IRQ_E_R {
        INT_IRQ_E_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn sof_e(&self) -> SOF_E_R {
        SOF_E_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn pcde(&mut self) -> PCDE_W {
        PCDE_W { w: self }
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn flre(&mut self) -> FLRE_W {
        FLRE_W { w: self }
    }
    #[doc = "Bit 16 - ATL IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn atl_irq_e(&mut self) -> ATL_IRQ_E_W {
        ATL_IRQ_E_W { w: self }
    }
    #[doc = "Bit 17 - ISO IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn iso_irq_e(&mut self) -> ISO_IRQ_E_W {
        ISO_IRQ_E_W { w: self }
    }
    #[doc = "Bit 18 - INT IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn int_irq_e(&mut self) -> INT_IRQ_E_W {
        INT_IRQ_E_W { w: self }
    }
    #[doc = "Bit 19 - SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn sof_e(&mut self) -> SOF_E_W {
        SOF_E_W { w: self }
    }
}
