#[doc = "Reader of register LPM"]
pub type R = crate::R<u32, super::LPM>;
#[doc = "Writer for register LPM"]
pub type W = crate::W<u32, super::LPM>;
#[doc = "Register LPM `reset()`'s with value 0"]
impl crate::ResetValue for super::LPM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIRD_HW`"]
pub type HIRD_HW_R = crate::R<u8, u8>;
#[doc = "Reader of field `HIRD_SW`"]
pub type HIRD_SW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HIRD_SW`"]
pub struct HIRD_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRD_SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DATA_PENDING`"]
pub type DATA_PENDING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_PENDING`"]
pub struct DATA_PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_PENDING_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Host Initiated Resume Duration - HW."]
    #[inline(always)]
    pub fn hird_hw(&self) -> HIRD_HW_R {
        HIRD_HW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW."]
    #[inline(always)]
    pub fn hird_sw(&self) -> HIRD_SW_R {
        HIRD_SW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[inline(always)]
    pub fn data_pending(&self) -> DATA_PENDING_R {
        DATA_PENDING_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Host Initiated Resume Duration - SW."]
    #[inline(always)]
    pub fn hird_sw(&mut self) -> HIRD_SW_W {
        HIRD_SW_W { w: self }
    }
    #[doc = "Bit 8 - As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[inline(always)]
    pub fn data_pending(&mut self) -> DATA_PENDING_W {
        DATA_PENDING_W { w: self }
    }
}
