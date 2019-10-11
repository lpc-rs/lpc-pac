#[doc = "Reader of register RXFILTERCTRL"]
pub type R = crate::R<u32, super::RXFILTERCTRL>;
#[doc = "Writer for register RXFILTERCTRL"]
pub type W = crate::W<u32, super::RXFILTERCTRL>;
#[doc = "Register RXFILTERCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFILTERCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUE`"]
pub type AUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUE`"]
pub struct AUE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUE_W<'a> {
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
#[doc = "Reader of field `ABE`"]
pub type ABE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABE`"]
pub struct ABE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABE_W<'a> {
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
#[doc = "Reader of field `AME`"]
pub type AME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AME`"]
pub struct AME_W<'a> {
    w: &'a mut W,
}
impl<'a> AME_W<'a> {
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
#[doc = "Reader of field `AUHE`"]
pub type AUHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUHE`"]
pub struct AUHE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUHE_W<'a> {
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
#[doc = "Reader of field `AMHE`"]
pub type AMHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMHE`"]
pub struct AMHE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMHE_W<'a> {
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
#[doc = "Reader of field `APE`"]
pub type APE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APE`"]
pub struct APE_W<'a> {
    w: &'a mut W,
}
impl<'a> APE_W<'a> {
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
#[doc = "Reader of field `MPEW`"]
pub type MPEW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPEW`"]
pub struct MPEW_W<'a> {
    w: &'a mut W,
}
impl<'a> MPEW_W<'a> {
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
#[doc = "Reader of field `RFEW`"]
pub type RFEW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFEW`"]
pub struct RFEW_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEW_W<'a> {
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
impl R {
    #[doc = "Bit 0 - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
    #[inline(always)]
    pub fn aue(&self) -> AUE_R {
        AUE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
    #[inline(always)]
    pub fn abe(&self) -> ABE_R {
        ABE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
    #[inline(always)]
    pub fn ame(&self) -> AME_R {
        AME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn auhe(&self) -> AUHE_R {
        AUHE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn amhe(&self) -> AMHE_R {
        AMHE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
    #[inline(always)]
    pub fn ape(&self) -> APE_R {
        APE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn mpew(&self) -> MPEW_R {
        MPEW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn rfew(&self) -> RFEW_R {
        RFEW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
    #[inline(always)]
    pub fn aue(&mut self) -> AUE_W {
        AUE_W { w: self }
    }
    #[doc = "Bit 1 - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
    #[inline(always)]
    pub fn abe(&mut self) -> ABE_W {
        ABE_W { w: self }
    }
    #[doc = "Bit 2 - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
    #[inline(always)]
    pub fn ame(&mut self) -> AME_W {
        AME_W { w: self }
    }
    #[doc = "Bit 3 - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn auhe(&mut self) -> AUHE_W {
        AUHE_W { w: self }
    }
    #[doc = "Bit 4 - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn amhe(&mut self) -> AMHE_W {
        AMHE_W { w: self }
    }
    #[doc = "Bit 5 - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
    #[inline(always)]
    pub fn ape(&mut self) -> APE_W {
        APE_W { w: self }
    }
    #[doc = "Bit 12 - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn mpew(&mut self) -> MPEW_W {
        MPEW_W { w: self }
    }
    #[doc = "Bit 13 - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn rfew(&mut self) -> RFEW_W {
        RFEW_W { w: self }
    }
}
