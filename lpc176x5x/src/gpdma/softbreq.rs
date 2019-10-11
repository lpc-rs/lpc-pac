#[doc = "Reader of register SOFTBREQ"]
pub type R = crate::R<u32, super::SOFTBREQ>;
#[doc = "Writer for register SOFTBREQ"]
pub type W = crate::W<u32, super::SOFTBREQ>;
#[doc = "Register SOFTBREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SOFTBREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFTBREQ0`"]
pub type SOFTBREQ0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ0`"]
pub struct SOFTBREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ0_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ1`"]
pub type SOFTBREQ1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ1`"]
pub struct SOFTBREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ1_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ2`"]
pub type SOFTBREQ2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ2`"]
pub struct SOFTBREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ2_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ3`"]
pub type SOFTBREQ3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ3`"]
pub struct SOFTBREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ3_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ4`"]
pub type SOFTBREQ4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ4`"]
pub struct SOFTBREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ4_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ5`"]
pub type SOFTBREQ5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ5`"]
pub struct SOFTBREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ5_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ6`"]
pub type SOFTBREQ6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ6`"]
pub struct SOFTBREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ6_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ7`"]
pub type SOFTBREQ7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ7`"]
pub struct SOFTBREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ7_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ8`"]
pub type SOFTBREQ8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ8`"]
pub struct SOFTBREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ8_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ9`"]
pub type SOFTBREQ9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ9`"]
pub struct SOFTBREQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ9_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ10`"]
pub type SOFTBREQ10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ10`"]
pub struct SOFTBREQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ10_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ11`"]
pub type SOFTBREQ11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ11`"]
pub struct SOFTBREQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ11_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ12`"]
pub type SOFTBREQ12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ12`"]
pub struct SOFTBREQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ12_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ13`"]
pub type SOFTBREQ13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ13`"]
pub struct SOFTBREQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ13_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ14`"]
pub type SOFTBREQ14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ14`"]
pub struct SOFTBREQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ14_W<'a> {
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
#[doc = "Reader of field `SOFTBREQ15`"]
pub type SOFTBREQ15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTBREQ15`"]
pub struct SOFTBREQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ15_W<'a> {
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
    #[doc = "Bit 0 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq0(&self) -> SOFTBREQ0_R {
        SOFTBREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq1(&self) -> SOFTBREQ1_R {
        SOFTBREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq2(&self) -> SOFTBREQ2_R {
        SOFTBREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq3(&self) -> SOFTBREQ3_R {
        SOFTBREQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq4(&self) -> SOFTBREQ4_R {
        SOFTBREQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq5(&self) -> SOFTBREQ5_R {
        SOFTBREQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq6(&self) -> SOFTBREQ6_R {
        SOFTBREQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq7(&self) -> SOFTBREQ7_R {
        SOFTBREQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq8(&self) -> SOFTBREQ8_R {
        SOFTBREQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq9(&self) -> SOFTBREQ9_R {
        SOFTBREQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq10(&self) -> SOFTBREQ10_R {
        SOFTBREQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq11(&self) -> SOFTBREQ11_R {
        SOFTBREQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq12(&self) -> SOFTBREQ12_R {
        SOFTBREQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq13(&self) -> SOFTBREQ13_R {
        SOFTBREQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq14(&self) -> SOFTBREQ14_R {
        SOFTBREQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq15(&self) -> SOFTBREQ15_R {
        SOFTBREQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq0(&mut self) -> SOFTBREQ0_W {
        SOFTBREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq1(&mut self) -> SOFTBREQ1_W {
        SOFTBREQ1_W { w: self }
    }
    #[doc = "Bit 2 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq2(&mut self) -> SOFTBREQ2_W {
        SOFTBREQ2_W { w: self }
    }
    #[doc = "Bit 3 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq3(&mut self) -> SOFTBREQ3_W {
        SOFTBREQ3_W { w: self }
    }
    #[doc = "Bit 4 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq4(&mut self) -> SOFTBREQ4_W {
        SOFTBREQ4_W { w: self }
    }
    #[doc = "Bit 5 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq5(&mut self) -> SOFTBREQ5_W {
        SOFTBREQ5_W { w: self }
    }
    #[doc = "Bit 6 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq6(&mut self) -> SOFTBREQ6_W {
        SOFTBREQ6_W { w: self }
    }
    #[doc = "Bit 7 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq7(&mut self) -> SOFTBREQ7_W {
        SOFTBREQ7_W { w: self }
    }
    #[doc = "Bit 8 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq8(&mut self) -> SOFTBREQ8_W {
        SOFTBREQ8_W { w: self }
    }
    #[doc = "Bit 9 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq9(&mut self) -> SOFTBREQ9_W {
        SOFTBREQ9_W { w: self }
    }
    #[doc = "Bit 10 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq10(&mut self) -> SOFTBREQ10_W {
        SOFTBREQ10_W { w: self }
    }
    #[doc = "Bit 11 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq11(&mut self) -> SOFTBREQ11_W {
        SOFTBREQ11_W { w: self }
    }
    #[doc = "Bit 12 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq12(&mut self) -> SOFTBREQ12_W {
        SOFTBREQ12_W { w: self }
    }
    #[doc = "Bit 13 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq13(&mut self) -> SOFTBREQ13_W {
        SOFTBREQ13_W { w: self }
    }
    #[doc = "Bit 14 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq14(&mut self) -> SOFTBREQ14_W {
        SOFTBREQ14_W { w: self }
    }
    #[doc = "Bit 15 - Software burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral Description (refer to Table 672 for peripheral hardware connections to the DMA controller): 0 - writing 0 has no effect. 1 - writing 1 generates a DMA burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softbreq15(&mut self) -> SOFTBREQ15_W {
        SOFTBREQ15_W { w: self }
    }
}
