#[doc = "Reader of register USBSTS"]
pub type R = crate::R<u32, super::USBSTS>;
#[doc = "Writer for register USBSTS"]
pub type W = crate::W<u32, super::USBSTS>;
#[doc = "Register USBSTS `reset()`'s with value 0"]
impl crate::ResetValue for super::USBSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCD`"]
pub type PCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCD`"]
pub struct PCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PCD_W<'a> {
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
#[doc = "Reader of field `FLR`"]
pub type FLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLR`"]
pub struct FLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLR_W<'a> {
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
#[doc = "Reader of field `ATL_IRQ`"]
pub type ATL_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATL_IRQ`"]
pub struct ATL_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_IRQ_W<'a> {
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
#[doc = "Reader of field `ISO_IRQ`"]
pub type ISO_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISO_IRQ`"]
pub struct ISO_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_IRQ_W<'a> {
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
#[doc = "Reader of field `INT_IRQ`"]
pub type INT_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_IRQ`"]
pub struct INT_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_IRQ_W<'a> {
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
#[doc = "Reader of field `SOF_IRQ`"]
pub type SOF_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF_IRQ`"]
pub struct SOF_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_IRQ_W<'a> {
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
    #[doc = "Bit 2 - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[inline(always)]
    pub fn pcd(&self) -> PCD_R {
        PCD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub fn flr(&self) -> FLR_R {
        FLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn atl_irq(&self) -> ATL_IRQ_R {
        ATL_IRQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn iso_irq(&self) -> ISO_IRQ_R {
        ISO_IRQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn int_irq(&self) -> INT_IRQ_R {
        INT_IRQ_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub fn sof_irq(&self) -> SOF_IRQ_R {
        SOF_IRQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[inline(always)]
    pub fn pcd(&mut self) -> PCD_W {
        PCD_W { w: self }
    }
    #[doc = "Bit 3 - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub fn flr(&mut self) -> FLR_W {
        FLR_W { w: self }
    }
    #[doc = "Bit 16 - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn atl_irq(&mut self) -> ATL_IRQ_W {
        ATL_IRQ_W { w: self }
    }
    #[doc = "Bit 17 - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn iso_irq(&mut self) -> ISO_IRQ_W {
        ISO_IRQ_W { w: self }
    }
    #[doc = "Bit 18 - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn int_irq(&mut self) -> INT_IRQ_W {
        INT_IRQ_W { w: self }
    }
    #[doc = "Bit 19 - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub fn sof_irq(&mut self) -> SOF_IRQ_W {
        SOF_IRQ_W { w: self }
    }
}
