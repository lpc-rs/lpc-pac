#[doc = "Reader of register MCMD"]
pub type R = crate::R<u32, super::MCMD>;
#[doc = "Writer for register MCMD"]
pub type W = crate::W<u32, super::MCMD>;
#[doc = "Register MCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::MCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READ`"]
pub type READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ`"]
pub struct READ_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_W<'a> {
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
#[doc = "Reader of field `SCAN`"]
pub type SCAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN`"]
pub struct SCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit causes the MII Management hardware to perform a single Read cycle. The Read data is returned in Register MRDD (MII Mgmt Read Data)."]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
    #[doc = "Bit 1 - This bit causes the MII Management hardware to perform Read cycles continuously. This is useful for monitoring Link Fail for example."]
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W {
        SCAN_W { w: self }
    }
}
