#[doc = "Reader of register SOFTLSREQ"]
pub type R = crate::R<u32, super::SOFTLSREQ>;
#[doc = "Writer for register SOFTLSREQ"]
pub type W = crate::W<u32, super::SOFTLSREQ>;
#[doc = "Register SOFTLSREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SOFTLSREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFTLSREQ0`"]
pub type SOFTLSREQ0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ0`"]
pub struct SOFTLSREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ0_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ1`"]
pub type SOFTLSREQ1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ1`"]
pub struct SOFTLSREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ1_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ2`"]
pub type SOFTLSREQ2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ2`"]
pub struct SOFTLSREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ2_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ3`"]
pub type SOFTLSREQ3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ3`"]
pub struct SOFTLSREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ3_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ4`"]
pub type SOFTLSREQ4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ4`"]
pub struct SOFTLSREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ4_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ5`"]
pub type SOFTLSREQ5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ5`"]
pub struct SOFTLSREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ5_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ6`"]
pub type SOFTLSREQ6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ6`"]
pub struct SOFTLSREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ6_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ7`"]
pub type SOFTLSREQ7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ7`"]
pub struct SOFTLSREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ7_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ8`"]
pub type SOFTLSREQ8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ8`"]
pub struct SOFTLSREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ8_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ9`"]
pub type SOFTLSREQ9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ9`"]
pub struct SOFTLSREQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ9_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ10`"]
pub type SOFTLSREQ10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ10`"]
pub struct SOFTLSREQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ10_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ11`"]
pub type SOFTLSREQ11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ11`"]
pub struct SOFTLSREQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ11_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ12`"]
pub type SOFTLSREQ12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ12`"]
pub struct SOFTLSREQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ12_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ13`"]
pub type SOFTLSREQ13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ13`"]
pub struct SOFTLSREQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ13_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ14`"]
pub type SOFTLSREQ14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ14`"]
pub struct SOFTLSREQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ14_W<'a> {
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
#[doc = "Reader of field `SOFTLSREQ15`"]
pub type SOFTLSREQ15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTLSREQ15`"]
pub struct SOFTLSREQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ15_W<'a> {
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
    #[doc = "Bit 0 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq0(&self) -> SOFTLSREQ0_R {
        SOFTLSREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq1(&self) -> SOFTLSREQ1_R {
        SOFTLSREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq2(&self) -> SOFTLSREQ2_R {
        SOFTLSREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq3(&self) -> SOFTLSREQ3_R {
        SOFTLSREQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq4(&self) -> SOFTLSREQ4_R {
        SOFTLSREQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq5(&self) -> SOFTLSREQ5_R {
        SOFTLSREQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq6(&self) -> SOFTLSREQ6_R {
        SOFTLSREQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq7(&self) -> SOFTLSREQ7_R {
        SOFTLSREQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq8(&self) -> SOFTLSREQ8_R {
        SOFTLSREQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq9(&self) -> SOFTLSREQ9_R {
        SOFTLSREQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq10(&self) -> SOFTLSREQ10_R {
        SOFTLSREQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq11(&self) -> SOFTLSREQ11_R {
        SOFTLSREQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq12(&self) -> SOFTLSREQ12_R {
        SOFTLSREQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq13(&self) -> SOFTLSREQ13_R {
        SOFTLSREQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq14(&self) -> SOFTLSREQ14_R {
        SOFTLSREQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq15(&self) -> SOFTLSREQ15_R {
        SOFTLSREQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq0(&mut self) -> SOFTLSREQ0_W {
        SOFTLSREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq1(&mut self) -> SOFTLSREQ1_W {
        SOFTLSREQ1_W { w: self }
    }
    #[doc = "Bit 2 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq2(&mut self) -> SOFTLSREQ2_W {
        SOFTLSREQ2_W { w: self }
    }
    #[doc = "Bit 3 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq3(&mut self) -> SOFTLSREQ3_W {
        SOFTLSREQ3_W { w: self }
    }
    #[doc = "Bit 4 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq4(&mut self) -> SOFTLSREQ4_W {
        SOFTLSREQ4_W { w: self }
    }
    #[doc = "Bit 5 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq5(&mut self) -> SOFTLSREQ5_W {
        SOFTLSREQ5_W { w: self }
    }
    #[doc = "Bit 6 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq6(&mut self) -> SOFTLSREQ6_W {
        SOFTLSREQ6_W { w: self }
    }
    #[doc = "Bit 7 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq7(&mut self) -> SOFTLSREQ7_W {
        SOFTLSREQ7_W { w: self }
    }
    #[doc = "Bit 8 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq8(&mut self) -> SOFTLSREQ8_W {
        SOFTLSREQ8_W { w: self }
    }
    #[doc = "Bit 9 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq9(&mut self) -> SOFTLSREQ9_W {
        SOFTLSREQ9_W { w: self }
    }
    #[doc = "Bit 10 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq10(&mut self) -> SOFTLSREQ10_W {
        SOFTLSREQ10_W { w: self }
    }
    #[doc = "Bit 11 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq11(&mut self) -> SOFTLSREQ11_W {
        SOFTLSREQ11_W { w: self }
    }
    #[doc = "Bit 12 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq12(&mut self) -> SOFTLSREQ12_W {
        SOFTLSREQ12_W { w: self }
    }
    #[doc = "Bit 13 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq13(&mut self) -> SOFTLSREQ13_W {
        SOFTLSREQ13_W { w: self }
    }
    #[doc = "Bit 14 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq14(&mut self) -> SOFTLSREQ14_W {
        SOFTLSREQ14_W { w: self }
    }
    #[doc = "Bit 15 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq15(&mut self) -> SOFTLSREQ15_W {
        SOFTLSREQ15_W { w: self }
    }
}
