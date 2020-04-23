#[doc = "Reader of register DMAREQ0"]
pub type R = crate::R<u32, super::DMAREQ0>;
#[doc = "Writer for register DMAREQ0"]
pub type W = crate::W<u32, super::DMAREQ0>;
#[doc = "Register DMAREQ0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAREQ0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEV_0`"]
pub type DEV_0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEV_0`"]
pub struct DEV_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DRL0`"]
pub type DRL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRL0`"]
pub struct DRL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRL0_W<'a> {
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
#[doc = "Reader of field `DRQ0`"]
pub type DRQ0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRQ0`"]
pub struct DRQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRQ0_W<'a> {
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
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_0(&self) -> DEV_0_R {
        DEV_0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    pub fn drl0(&self) -> DRL0_R {
        DRL0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq0(&self) -> DRQ0_R {
        DRQ0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_0(&mut self) -> DEV_0_W {
        DEV_0_W { w: self }
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    pub fn drl0(&mut self) -> DRL0_W {
        DRL0_W { w: self }
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq0(&mut self) -> DRQ0_W {
        DRQ0_W { w: self }
    }
}
