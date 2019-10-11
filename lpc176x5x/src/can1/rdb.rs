#[doc = "Reader of register RDB"]
pub type R = crate::R<u32, super::RDB>;
#[doc = "Writer for register RDB"]
pub type W = crate::W<u32, super::RDB>;
#[doc = "Register RDB `reset()`'s with value 0"]
impl crate::ResetValue for super::RDB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA5`"]
pub type DATA5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA5`"]
pub struct DATA5_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DATA6`"]
pub type DATA6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA6`"]
pub struct DATA6_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DATA7`"]
pub type DATA7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA7`"]
pub struct DATA7_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATA8`"]
pub type DATA8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA8`"]
pub struct DATA8_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data 5. If the DLC field in CANRFS >= 0101, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data 6. If the DLC field in CANRFS >= 0110, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data 7. If the DLC field in CANRFS >= 0111, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data 8. If the DLC field in CANRFS >= 1000, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 5. If the DLC field in CANRFS >= 0101, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W {
        DATA5_W { w: self }
    }
    #[doc = "Bits 8:15 - Data 6. If the DLC field in CANRFS >= 0110, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W {
        DATA6_W { w: self }
    }
    #[doc = "Bits 16:23 - Data 7. If the DLC field in CANRFS >= 0111, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W {
        DATA7_W { w: self }
    }
    #[doc = "Bits 24:31 - Data 8. If the DLC field in CANRFS >= 1000, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data8(&mut self) -> DATA8_W {
        DATA8_W { w: self }
    }
}
