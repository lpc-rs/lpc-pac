#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UOF`"]
pub type UOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `Progbit`"]
pub type PROGBIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `Status`"]
pub type STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Status`"]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
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
#[doc = "Reader of field `Trigger`"]
pub type TRIGGER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Trigger`"]
pub struct TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Untraced overflow flag. If set to 1, there is an overflow that has not yet been traced. This bit is cleared to 0 when either: - trace is restarted - the ETM Power Down bit, bit \\[0\\]
of the ETM Control Register, 0x00, is set to 1. Note: Setting or clearing the ETM programming bit does not cause this bit to be cleared to 0."]
    #[inline(always)]
    pub fn uof(&self) -> UOF_R {
        UOF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ETM programming bit value (Progbit). The current effective value of the ETM Programming bit (ETM Control Register bit \\[10\\]). Tou must wait for this bit to go to 1 before you start to program the ETM."]
    #[inline(always)]
    pub fn progbit(&self) -> PROGBIT_R {
        PROGBIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Holds the current status of the trace start/stop resource. If set to 1, it indicates that a trace on address has been matched, without a corresponding trace off address match."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Trigger bit. Set when the trigger occurs, and prevents the trigger from being output until the ETM is next programmed."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Holds the current status of the trace start/stop resource. If set to 1, it indicates that a trace on address has been matched, without a corresponding trace off address match."]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
    #[doc = "Bit 3 - Trigger bit. Set when the trigger occurs, and prevents the trigger from being output until the ETM is next programmed."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
}
