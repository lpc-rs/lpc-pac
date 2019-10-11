#[doc = "Reader of register TEST"]
pub type R = crate::R<u32, super::TEST>;
#[doc = "Writer for register TEST"]
pub type W = crate::W<u32, super::TEST>;
#[doc = "Register TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCPQ`"]
pub type SCPQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCPQ`"]
pub struct SCPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SCPQ_W<'a> {
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
#[doc = "Reader of field `TESTPAUSE`"]
pub type TESTPAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTPAUSE`"]
pub struct TESTPAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTPAUSE_W<'a> {
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
#[doc = "Reader of field `TESTBP`"]
pub type TESTBP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTBP`"]
pub struct TESTBP_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTBP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SHORTCUT PAUSE QUANTA. This bit reduces the effective PAUSE quanta from 64 byte-times to 1 byte-time."]
    #[inline(always)]
    pub fn scpq(&self) -> SCPQ_R {
        SCPQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit causes the MAC Control sublayer to inhibit transmissions, just as if a PAUSE Receive Control frame with a nonzero pause time parameter was received."]
    #[inline(always)]
    pub fn testpause(&self) -> TESTPAUSE_R {
        TESTPAUSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TEST BACKPRESSURE. Setting this bit will cause the MAC to assert backpressure on the link. Backpressure causes preamble to be transmitted, raising carrier sense. A transmit packet from the system will be sent during backpressure."]
    #[inline(always)]
    pub fn testbp(&self) -> TESTBP_R {
        TESTBP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHORTCUT PAUSE QUANTA. This bit reduces the effective PAUSE quanta from 64 byte-times to 1 byte-time."]
    #[inline(always)]
    pub fn scpq(&mut self) -> SCPQ_W {
        SCPQ_W { w: self }
    }
    #[doc = "Bit 1 - This bit causes the MAC Control sublayer to inhibit transmissions, just as if a PAUSE Receive Control frame with a nonzero pause time parameter was received."]
    #[inline(always)]
    pub fn testpause(&mut self) -> TESTPAUSE_W {
        TESTPAUSE_W { w: self }
    }
    #[doc = "Bit 2 - TEST BACKPRESSURE. Setting this bit will cause the MAC to assert backpressure on the link. Backpressure causes preamble to be transmitted, raising carrier sense. A transmit packet from the system will be sent during backpressure."]
    #[inline(always)]
    pub fn testbp(&mut self) -> TESTBP_W {
        TESTBP_W { w: self }
    }
}
