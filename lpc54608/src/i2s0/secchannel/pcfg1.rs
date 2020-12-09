#[doc = "Reader of register PCFG1"]
pub type R = crate::R<u32, super::PCFG1>;
#[doc = "Writer for register PCFG1"]
pub type W = crate::W<u32, super::PCFG1>;
#[doc = "Register PCFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PAIRENABLE`"]
pub type PAIRENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAIRENABLE`"]
pub struct PAIRENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAIRENABLE_W<'a> {
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
#[doc = "Reader of field `ONECHANNEL`"]
pub type ONECHANNEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONECHANNEL`"]
pub struct ONECHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ONECHANNEL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable for this channel pair.."]
    #[inline(always)]
    pub fn pairenable(&self) -> PAIRENABLE_R {
        PAIRENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 10 - Single channel mode."]
    #[inline(always)]
    pub fn onechannel(&self) -> ONECHANNEL_R {
        ONECHANNEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for this channel pair.."]
    #[inline(always)]
    pub fn pairenable(&mut self) -> PAIRENABLE_W {
        PAIRENABLE_W { w: self }
    }
    #[doc = "Bit 10 - Single channel mode."]
    #[inline(always)]
    pub fn onechannel(&mut self) -> ONECHANNEL_W {
        ONECHANNEL_W { w: self }
    }
}
