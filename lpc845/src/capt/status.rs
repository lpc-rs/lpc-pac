#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0x0008_0000"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0000
    }
}
#[doc = "Reader of field `YESTOUCH`"]
pub type YESTOUCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `YESTOUCH`"]
pub struct YESTOUCH_W<'a> {
    w: &'a mut W,
}
impl<'a> YESTOUCH_W<'a> {
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
#[doc = "Reader of field `NOTOUCH`"]
pub type NOTOUCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOTOUCH`"]
pub struct NOTOUCH_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTOUCH_W<'a> {
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
#[doc = "Reader of field `POLLDONE`"]
pub type POLLDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLLDONE`"]
pub struct POLLDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLDONE_W<'a> {
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
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMEOUT`"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
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
#[doc = "Reader of field `OVERUN`"]
pub type OVERUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERUN`"]
pub struct OVERUN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERUN_W<'a> {
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
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `XMAX`"]
pub type XMAX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Is 1 if a touch has been detected, including a wakeup from low-power mode."]
    #[inline(always)]
    pub fn yestouch(&self) -> YESTOUCH_R {
        YESTOUCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Is 1 if a no-touch has been detected (ie. completed an integration cycle and found no-touch). This is not set when in low-power mode."]
    #[inline(always)]
    pub fn notouch(&self) -> NOTOUCH_R {
        NOTOUCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Is 1 if a poll or POLLNOW is complete."]
    #[inline(always)]
    pub fn polldone(&self) -> POLLDONE_R {
        POLLDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Is 1 if an integration cycle ended with a timeout (should not happen)."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Is 1 if new data was collected before application read out previous ISTOUCH. No-touch (ISTOUCH==0) data will be silently overrun. Is not possible if WAIT=1."]
    #[inline(always)]
    pub fn overun(&self) -> OVERUN_R {
        OVERUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - In a poll now."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Indicates the maximum number of X pins allowed 0-relative. So, 15 means there are pins 0 to 15, or 16 total X pins. INTERNAL note: this may be setup to be written by ROM boot."]
    #[inline(always)]
    pub fn xmax(&self) -> XMAX_R {
        XMAX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Is 1 if a touch has been detected, including a wakeup from low-power mode."]
    #[inline(always)]
    pub fn yestouch(&mut self) -> YESTOUCH_W {
        YESTOUCH_W { w: self }
    }
    #[doc = "Bit 1 - Is 1 if a no-touch has been detected (ie. completed an integration cycle and found no-touch). This is not set when in low-power mode."]
    #[inline(always)]
    pub fn notouch(&mut self) -> NOTOUCH_W {
        NOTOUCH_W { w: self }
    }
    #[doc = "Bit 2 - Is 1 if a poll or POLLNOW is complete."]
    #[inline(always)]
    pub fn polldone(&mut self) -> POLLDONE_W {
        POLLDONE_W { w: self }
    }
    #[doc = "Bit 3 - Is 1 if an integration cycle ended with a timeout (should not happen)."]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 4 - Is 1 if new data was collected before application read out previous ISTOUCH. No-touch (ISTOUCH==0) data will be silently overrun. Is not possible if WAIT=1."]
    #[inline(always)]
    pub fn overun(&mut self) -> OVERUN_W {
        OVERUN_W { w: self }
    }
}
