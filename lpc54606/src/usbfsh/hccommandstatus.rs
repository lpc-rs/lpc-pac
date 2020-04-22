#[doc = "Reader of register HCCOMMANDSTATUS"]
pub type R = crate::R<u32, super::HCCOMMANDSTATUS>;
#[doc = "Writer for register HCCOMMANDSTATUS"]
pub type W = crate::W<u32, super::HCCOMMANDSTATUS>;
#[doc = "Register HCCOMMANDSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::HCCOMMANDSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HCR`"]
pub type HCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCR`"]
pub struct HCR_W<'a> {
    w: &'a mut W,
}
impl<'a> HCR_W<'a> {
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
#[doc = "Reader of field `CLF`"]
pub type CLF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLF`"]
pub struct CLF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLF_W<'a> {
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
#[doc = "Reader of field `BLF`"]
pub type BLF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLF`"]
pub struct BLF_W<'a> {
    w: &'a mut W,
}
impl<'a> BLF_W<'a> {
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
#[doc = "Reader of field `OCR`"]
pub type OCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCR`"]
pub struct OCR_W<'a> {
    w: &'a mut W,
}
impl<'a> OCR_W<'a> {
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
#[doc = "Reader of field `SOC`"]
pub type SOC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOC`"]
pub struct SOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[inline(always)]
    pub fn hcr(&self) -> HCR_R {
        HCR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[inline(always)]
    pub fn clf(&self) -> CLF_R {
        CLF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[inline(always)]
    pub fn blf(&self) -> BLF_R {
        BLF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[inline(always)]
    pub fn ocr(&self) -> OCR_R {
        OCR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[inline(always)]
    pub fn soc(&self) -> SOC_R {
        SOC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[inline(always)]
    pub fn hcr(&mut self) -> HCR_W {
        HCR_W { w: self }
    }
    #[doc = "Bit 1 - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[inline(always)]
    pub fn clf(&mut self) -> CLF_W {
        CLF_W { w: self }
    }
    #[doc = "Bit 2 - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[inline(always)]
    pub fn blf(&mut self) -> BLF_W {
        BLF_W { w: self }
    }
    #[doc = "Bit 3 - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[inline(always)]
    pub fn ocr(&mut self) -> OCR_W {
        OCR_W { w: self }
    }
    #[doc = "Bits 6:7 - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[inline(always)]
    pub fn soc(&mut self) -> SOC_W {
        SOC_W { w: self }
    }
}
