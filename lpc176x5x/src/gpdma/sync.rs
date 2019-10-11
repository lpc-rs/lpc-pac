#[doc = "Reader of register SYNC"]
pub type R = crate::R<u32, super::SYNC>;
#[doc = "Writer for register SYNC"]
pub type W = crate::W<u32, super::SYNC>;
#[doc = "Register SYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMACSYNC0`"]
pub type DMACSYNC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC0`"]
pub struct DMACSYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC0_W<'a> {
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
#[doc = "Reader of field `DMACSYNC1`"]
pub type DMACSYNC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC1`"]
pub struct DMACSYNC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC1_W<'a> {
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
#[doc = "Reader of field `DMACSYNC2`"]
pub type DMACSYNC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC2`"]
pub struct DMACSYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC2_W<'a> {
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
#[doc = "Reader of field `DMACSYNC3`"]
pub type DMACSYNC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC3`"]
pub struct DMACSYNC3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC3_W<'a> {
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
#[doc = "Reader of field `DMACSYNC4`"]
pub type DMACSYNC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC4`"]
pub struct DMACSYNC4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC4_W<'a> {
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
#[doc = "Reader of field `DMACSYNC5`"]
pub type DMACSYNC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC5`"]
pub struct DMACSYNC5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DMACSYNC6`"]
pub type DMACSYNC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC6`"]
pub struct DMACSYNC6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DMACSYNC7`"]
pub type DMACSYNC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC7`"]
pub struct DMACSYNC7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DMACSYNC8`"]
pub type DMACSYNC8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC8`"]
pub struct DMACSYNC8_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC8_W<'a> {
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
#[doc = "Reader of field `DMACSYNC9`"]
pub type DMACSYNC9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC9`"]
pub struct DMACSYNC9_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DMACSYNC10`"]
pub type DMACSYNC10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC10`"]
pub struct DMACSYNC10_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DMACSYNC11`"]
pub type DMACSYNC11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC11`"]
pub struct DMACSYNC11_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DMACSYNC12`"]
pub type DMACSYNC12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC12`"]
pub struct DMACSYNC12_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DMACSYNC13`"]
pub type DMACSYNC13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC13`"]
pub struct DMACSYNC13_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `DMACSYNC14`"]
pub type DMACSYNC14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC14`"]
pub struct DMACSYNC14_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DMACSYNC15`"]
pub type DMACSYNC15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMACSYNC15`"]
pub struct DMACSYNC15_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACSYNC15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync0(&self) -> DMACSYNC0_R {
        DMACSYNC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync1(&self) -> DMACSYNC1_R {
        DMACSYNC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync2(&self) -> DMACSYNC2_R {
        DMACSYNC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync3(&self) -> DMACSYNC3_R {
        DMACSYNC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync4(&self) -> DMACSYNC4_R {
        DMACSYNC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync5(&self) -> DMACSYNC5_R {
        DMACSYNC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync6(&self) -> DMACSYNC6_R {
        DMACSYNC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync7(&self) -> DMACSYNC7_R {
        DMACSYNC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync8(&self) -> DMACSYNC8_R {
        DMACSYNC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync9(&self) -> DMACSYNC9_R {
        DMACSYNC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync10(&self) -> DMACSYNC10_R {
        DMACSYNC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync11(&self) -> DMACSYNC11_R {
        DMACSYNC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync12(&self) -> DMACSYNC12_R {
        DMACSYNC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync13(&self) -> DMACSYNC13_R {
        DMACSYNC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync14(&self) -> DMACSYNC14_R {
        DMACSYNC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync15(&self) -> DMACSYNC15_R {
        DMACSYNC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync0(&mut self) -> DMACSYNC0_W {
        DMACSYNC0_W { w: self }
    }
    #[doc = "Bit 1 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync1(&mut self) -> DMACSYNC1_W {
        DMACSYNC1_W { w: self }
    }
    #[doc = "Bit 2 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync2(&mut self) -> DMACSYNC2_W {
        DMACSYNC2_W { w: self }
    }
    #[doc = "Bit 3 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync3(&mut self) -> DMACSYNC3_W {
        DMACSYNC3_W { w: self }
    }
    #[doc = "Bit 4 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync4(&mut self) -> DMACSYNC4_W {
        DMACSYNC4_W { w: self }
    }
    #[doc = "Bit 5 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync5(&mut self) -> DMACSYNC5_W {
        DMACSYNC5_W { w: self }
    }
    #[doc = "Bit 6 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync6(&mut self) -> DMACSYNC6_W {
        DMACSYNC6_W { w: self }
    }
    #[doc = "Bit 7 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync7(&mut self) -> DMACSYNC7_W {
        DMACSYNC7_W { w: self }
    }
    #[doc = "Bit 8 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync8(&mut self) -> DMACSYNC8_W {
        DMACSYNC8_W { w: self }
    }
    #[doc = "Bit 9 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync9(&mut self) -> DMACSYNC9_W {
        DMACSYNC9_W { w: self }
    }
    #[doc = "Bit 10 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync10(&mut self) -> DMACSYNC10_W {
        DMACSYNC10_W { w: self }
    }
    #[doc = "Bit 11 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync11(&mut self) -> DMACSYNC11_W {
        DMACSYNC11_W { w: self }
    }
    #[doc = "Bit 12 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync12(&mut self) -> DMACSYNC12_W {
        DMACSYNC12_W { w: self }
    }
    #[doc = "Bit 13 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync13(&mut self) -> DMACSYNC13_W {
        DMACSYNC13_W { w: self }
    }
    #[doc = "Bit 14 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync14(&mut self) -> DMACSYNC14_W {
        DMACSYNC14_W { w: self }
    }
    #[doc = "Bit 15 - Controls the synchronization logic for DMA request signals. Each bit represents one set of DMA request lines as described in the preceding text: 0 - synchronization logic for the corresponding DMA request signals are enabled. 1 - synchronization logic for the corresponding DMA request signals are disabled."]
    #[inline(always)]
    pub fn dmacsync15(&mut self) -> DMACSYNC15_W {
        DMACSYNC15_W { w: self }
    }
}
