#[doc = "Reader of register CONF"]
pub type R = crate::R<u32, super::CONF>;
#[doc = "Writer for register CONF"]
pub type W = crate::W<u32, super::CONF>;
#[doc = "Register CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIRINV`"]
pub type DIRINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRINV`"]
pub struct DIRINV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRINV_W<'a> {
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
#[doc = "Reader of field `SIGMODE`"]
pub type SIGMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIGMODE`"]
pub struct SIGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGMODE_W<'a> {
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
#[doc = "Reader of field `CAPMODE`"]
pub type CAPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPMODE`"]
pub struct CAPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPMODE_W<'a> {
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
#[doc = "Reader of field `INVINX`"]
pub type INVINX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVINX`"]
pub struct INVINX_W<'a> {
    w: &'a mut W,
}
impl<'a> INVINX_W<'a> {
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
#[doc = "Reader of field `CRESPI`"]
pub type CRESPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRESPI`"]
pub struct CRESPI_W<'a> {
    w: &'a mut W,
}
impl<'a> CRESPI_W<'a> {
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
#[doc = "Reader of field `INXGATE`"]
pub type INXGATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INXGATE`"]
pub struct INXGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INXGATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
    #[inline(always)]
    pub fn dirinv(&self) -> DIRINV_R {
        DIRINV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
    #[inline(always)]
    pub fn sigmode(&self) -> SIGMODE_R {
        SIGMODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
    #[inline(always)]
    pub fn capmode(&self) -> CAPMODE_R {
        CAPMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
    #[inline(always)]
    pub fn invinx(&self) -> INVINX_R {
        INVINX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
    #[inline(always)]
    pub fn crespi(&self) -> CRESPI_R {
        CRESPI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
    #[inline(always)]
    pub fn inxgate(&self) -> INXGATE_R {
        INXGATE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Direction invert. When 1, complements the DIR bit."]
    #[inline(always)]
    pub fn dirinv(&mut self) -> DIRINV_W {
        DIRINV_W { w: self }
    }
    #[doc = "Bit 1 - Signal Mode. When 0, PhA and PhB function as quadrature encoder inputs. When 1, PhA functions as the direction signal and PhB functions as the clock signal."]
    #[inline(always)]
    pub fn sigmode(&mut self) -> SIGMODE_W {
        SIGMODE_W { w: self }
    }
    #[doc = "Bit 2 - Capture Mode. When 0, only PhA edges are counted (2X). When 1, BOTH PhA and PhB edges are counted (4X), increasing resolution but decreasing range."]
    #[inline(always)]
    pub fn capmode(&mut self) -> CAPMODE_W {
        CAPMODE_W { w: self }
    }
    #[doc = "Bit 3 - Invert Index. When 1, inverts the sense of the index input."]
    #[inline(always)]
    pub fn invinx(&mut self) -> INVINX_W {
        INVINX_W { w: self }
    }
    #[doc = "Bit 4 - Continuously reset the position counter on index. When 1, resets the position counter to all zeros whenever an index pulse occurs after the next position increase (recalibration)."]
    #[inline(always)]
    pub fn crespi(&mut self) -> CRESPI_W {
        CRESPI_W { w: self }
    }
    #[doc = "Bits 16:19 - Index gating configuration: When INXGATE\\[16\\]
= 1, pass the index when PHA = 1 and PHB = 0, otherwise block index. When INXGATE\\[17\\]
= 1, pass the index when PHA = 1 and PHB = 1, otherwise block index. When INXGATE\\[18\\]
= 1, pass the index when PHA = 0 and PHB = 1, otherwise block index. When INXGATE\\[19\\]
= 1, pass the index when PHA = 0 and PHB = 0, otherwise block index."]
    #[inline(always)]
    pub fn inxgate(&mut self) -> INXGATE_W {
        INXGATE_W { w: self }
    }
}
