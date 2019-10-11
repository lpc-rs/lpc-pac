#[doc = "Reader of register SOFTSREQ"]
pub type R = crate::R<u32, super::SOFTSREQ>;
#[doc = "Writer for register SOFTSREQ"]
pub type W = crate::W<u32, super::SOFTSREQ>;
#[doc = "Register SOFTSREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SOFTSREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFTSREQ0`"]
pub type SOFTSREQ0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ0`"]
pub struct SOFTSREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ0_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ1`"]
pub type SOFTSREQ1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ1`"]
pub struct SOFTSREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ1_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ2`"]
pub type SOFTSREQ2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ2`"]
pub struct SOFTSREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ2_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ3`"]
pub type SOFTSREQ3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ3`"]
pub struct SOFTSREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ3_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ4`"]
pub type SOFTSREQ4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ4`"]
pub struct SOFTSREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ4_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ5`"]
pub type SOFTSREQ5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ5`"]
pub struct SOFTSREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ5_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ6`"]
pub type SOFTSREQ6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ6`"]
pub struct SOFTSREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ6_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ7`"]
pub type SOFTSREQ7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ7`"]
pub struct SOFTSREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ7_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ8`"]
pub type SOFTSREQ8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ8`"]
pub struct SOFTSREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ8_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ9`"]
pub type SOFTSREQ9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ9`"]
pub struct SOFTSREQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ9_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ10`"]
pub type SOFTSREQ10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ10`"]
pub struct SOFTSREQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ10_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ11`"]
pub type SOFTSREQ11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ11`"]
pub struct SOFTSREQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ11_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ12`"]
pub type SOFTSREQ12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ12`"]
pub struct SOFTSREQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ12_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ13`"]
pub type SOFTSREQ13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ13`"]
pub struct SOFTSREQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ13_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ14`"]
pub type SOFTSREQ14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ14`"]
pub struct SOFTSREQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ14_W<'a> {
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
#[doc = "Reader of field `SOFTSREQ15`"]
pub type SOFTSREQ15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTSREQ15`"]
pub struct SOFTSREQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ15_W<'a> {
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
    #[doc = "Bit 0 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq0(&self) -> SOFTSREQ0_R {
        SOFTSREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq1(&self) -> SOFTSREQ1_R {
        SOFTSREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq2(&self) -> SOFTSREQ2_R {
        SOFTSREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq3(&self) -> SOFTSREQ3_R {
        SOFTSREQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq4(&self) -> SOFTSREQ4_R {
        SOFTSREQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq5(&self) -> SOFTSREQ5_R {
        SOFTSREQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq6(&self) -> SOFTSREQ6_R {
        SOFTSREQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq7(&self) -> SOFTSREQ7_R {
        SOFTSREQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq8(&self) -> SOFTSREQ8_R {
        SOFTSREQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq9(&self) -> SOFTSREQ9_R {
        SOFTSREQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq10(&self) -> SOFTSREQ10_R {
        SOFTSREQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq11(&self) -> SOFTSREQ11_R {
        SOFTSREQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq12(&self) -> SOFTSREQ12_R {
        SOFTSREQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq13(&self) -> SOFTSREQ13_R {
        SOFTSREQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq14(&self) -> SOFTSREQ14_R {
        SOFTSREQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq15(&self) -> SOFTSREQ15_R {
        SOFTSREQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq0(&mut self) -> SOFTSREQ0_W {
        SOFTSREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq1(&mut self) -> SOFTSREQ1_W {
        SOFTSREQ1_W { w: self }
    }
    #[doc = "Bit 2 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq2(&mut self) -> SOFTSREQ2_W {
        SOFTSREQ2_W { w: self }
    }
    #[doc = "Bit 3 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq3(&mut self) -> SOFTSREQ3_W {
        SOFTSREQ3_W { w: self }
    }
    #[doc = "Bit 4 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq4(&mut self) -> SOFTSREQ4_W {
        SOFTSREQ4_W { w: self }
    }
    #[doc = "Bit 5 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq5(&mut self) -> SOFTSREQ5_W {
        SOFTSREQ5_W { w: self }
    }
    #[doc = "Bit 6 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq6(&mut self) -> SOFTSREQ6_W {
        SOFTSREQ6_W { w: self }
    }
    #[doc = "Bit 7 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq7(&mut self) -> SOFTSREQ7_W {
        SOFTSREQ7_W { w: self }
    }
    #[doc = "Bit 8 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq8(&mut self) -> SOFTSREQ8_W {
        SOFTSREQ8_W { w: self }
    }
    #[doc = "Bit 9 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq9(&mut self) -> SOFTSREQ9_W {
        SOFTSREQ9_W { w: self }
    }
    #[doc = "Bit 10 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq10(&mut self) -> SOFTSREQ10_W {
        SOFTSREQ10_W { w: self }
    }
    #[doc = "Bit 11 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq11(&mut self) -> SOFTSREQ11_W {
        SOFTSREQ11_W { w: self }
    }
    #[doc = "Bit 12 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq12(&mut self) -> SOFTSREQ12_W {
        SOFTSREQ12_W { w: self }
    }
    #[doc = "Bit 13 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq13(&mut self) -> SOFTSREQ13_W {
        SOFTSREQ13_W { w: self }
    }
    #[doc = "Bit 14 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq14(&mut self) -> SOFTSREQ14_W {
        SOFTSREQ14_W { w: self }
    }
    #[doc = "Bit 15 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq15(&mut self) -> SOFTSREQ15_W {
        SOFTSREQ15_W { w: self }
    }
}
