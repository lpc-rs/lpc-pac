#[doc = "Reader of register DMAREQ1"]
pub type R = crate::R<u32, super::DMAREQ1>;
#[doc = "Writer for register DMAREQ1"]
pub type W = crate::W<u32, super::DMAREQ1>;
#[doc = "Register DMAREQ1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAREQ1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEV_1`"]
pub type DEV_1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEV_1`"]
pub struct DEV_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DRL1`"]
pub type DRL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRL1`"]
pub struct DRL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DRL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DRQ1`"]
pub type DRQ1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRQ1`"]
pub struct DRQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> DRQ1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_1(&self) -> DEV_1_R {
        DEV_1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    pub fn drl1(&self) -> DRL1_R {
        DRL1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq1(&self) -> DRQ1_R {
        DRQ1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_1(&mut self) -> DEV_1_W {
        DEV_1_W { w: self }
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    pub fn drl1(&mut self) -> DRL1_W {
        DRL1_W { w: self }
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq1(&mut self) -> DRQ1_W {
        DRQ1_W { w: self }
    }
}
