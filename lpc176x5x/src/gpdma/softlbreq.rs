#[doc = "Reader of register SOFTLBREQ"]
pub type R = crate::R<u32, super::SOFTLBREQ>;
#[doc = "Writer for register SOFTLBREQ"]
pub type W = crate::W<u32, super::SOFTLBREQ>;
#[doc = "Register SOFTLBREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SOFTLBREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFTLBREQ0`"]
pub type SOFTLBREQ0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ0`"]
pub struct SOFTLBREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ0_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ1`"]
pub type SOFTLBREQ1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ1`"]
pub struct SOFTLBREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ1_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ2`"]
pub type SOFTLBREQ2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ2`"]
pub struct SOFTLBREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ2_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ3`"]
pub type SOFTLBREQ3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ3`"]
pub struct SOFTLBREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ3_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ4`"]
pub type SOFTLBREQ4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ4`"]
pub struct SOFTLBREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ4_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ5`"]
pub type SOFTLBREQ5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ5`"]
pub struct SOFTLBREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ5_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ6`"]
pub type SOFTLBREQ6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ6`"]
pub struct SOFTLBREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ6_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ7`"]
pub type SOFTLBREQ7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ7`"]
pub struct SOFTLBREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ7_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ8`"]
pub type SOFTLBREQ8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ8`"]
pub struct SOFTLBREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ8_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ9`"]
pub type SOFTLBREQ9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ9`"]
pub struct SOFTLBREQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ9_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ10`"]
pub type SOFTLBREQ10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ10`"]
pub struct SOFTLBREQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ10_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ11`"]
pub type SOFTLBREQ11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ11`"]
pub struct SOFTLBREQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ11_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ12`"]
pub type SOFTLBREQ12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ12`"]
pub struct SOFTLBREQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ12_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ13`"]
pub type SOFTLBREQ13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ13`"]
pub struct SOFTLBREQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ13_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ14`"]
pub type SOFTLBREQ14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ14`"]
pub struct SOFTLBREQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ14_W<'a> {
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
#[doc = "Reader of field `SOFTLBREQ15`"]
pub type SOFTLBREQ15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLBREQ15`"]
pub struct SOFTLBREQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ15_W<'a> {
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
    #[doc = "Bit 0 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq0(&self) -> SOFTLBREQ0_R {
        SOFTLBREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq1(&self) -> SOFTLBREQ1_R {
        SOFTLBREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq2(&self) -> SOFTLBREQ2_R {
        SOFTLBREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq3(&self) -> SOFTLBREQ3_R {
        SOFTLBREQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq4(&self) -> SOFTLBREQ4_R {
        SOFTLBREQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq5(&self) -> SOFTLBREQ5_R {
        SOFTLBREQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq6(&self) -> SOFTLBREQ6_R {
        SOFTLBREQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq7(&self) -> SOFTLBREQ7_R {
        SOFTLBREQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq8(&self) -> SOFTLBREQ8_R {
        SOFTLBREQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq9(&self) -> SOFTLBREQ9_R {
        SOFTLBREQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq10(&self) -> SOFTLBREQ10_R {
        SOFTLBREQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq11(&self) -> SOFTLBREQ11_R {
        SOFTLBREQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq12(&self) -> SOFTLBREQ12_R {
        SOFTLBREQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq13(&self) -> SOFTLBREQ13_R {
        SOFTLBREQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq14(&self) -> SOFTLBREQ14_R {
        SOFTLBREQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq15(&self) -> SOFTLBREQ15_R {
        SOFTLBREQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq0(&mut self) -> SOFTLBREQ0_W {
        SOFTLBREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq1(&mut self) -> SOFTLBREQ1_W {
        SOFTLBREQ1_W { w: self }
    }
    #[doc = "Bit 2 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq2(&mut self) -> SOFTLBREQ2_W {
        SOFTLBREQ2_W { w: self }
    }
    #[doc = "Bit 3 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq3(&mut self) -> SOFTLBREQ3_W {
        SOFTLBREQ3_W { w: self }
    }
    #[doc = "Bit 4 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq4(&mut self) -> SOFTLBREQ4_W {
        SOFTLBREQ4_W { w: self }
    }
    #[doc = "Bit 5 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq5(&mut self) -> SOFTLBREQ5_W {
        SOFTLBREQ5_W { w: self }
    }
    #[doc = "Bit 6 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq6(&mut self) -> SOFTLBREQ6_W {
        SOFTLBREQ6_W { w: self }
    }
    #[doc = "Bit 7 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq7(&mut self) -> SOFTLBREQ7_W {
        SOFTLBREQ7_W { w: self }
    }
    #[doc = "Bit 8 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq8(&mut self) -> SOFTLBREQ8_W {
        SOFTLBREQ8_W { w: self }
    }
    #[doc = "Bit 9 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq9(&mut self) -> SOFTLBREQ9_W {
        SOFTLBREQ9_W { w: self }
    }
    #[doc = "Bit 10 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq10(&mut self) -> SOFTLBREQ10_W {
        SOFTLBREQ10_W { w: self }
    }
    #[doc = "Bit 11 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq11(&mut self) -> SOFTLBREQ11_W {
        SOFTLBREQ11_W { w: self }
    }
    #[doc = "Bit 12 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq12(&mut self) -> SOFTLBREQ12_W {
        SOFTLBREQ12_W { w: self }
    }
    #[doc = "Bit 13 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq13(&mut self) -> SOFTLBREQ13_W {
        SOFTLBREQ13_W { w: self }
    }
    #[doc = "Bit 14 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq14(&mut self) -> SOFTLBREQ14_W {
        SOFTLBREQ14_W { w: self }
    }
    #[doc = "Bit 15 - Software last burst request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last burst request for the corresponding request line."]
    #[inline(always)]
    pub fn softlbreq15(&mut self) -> SOFTLBREQ15_W {
        SOFTLBREQ15_W { w: self }
    }
}
