#[doc = "Reader of register CLRT"]
pub type R = crate::R<u32, super::CLRT>;
#[doc = "Writer for register CLRT"]
pub type W = crate::W<u32, super::CLRT>;
#[doc = "Register CLRT `reset()`'s with value 0x370f"]
impl crate::ResetValue for super::CLRT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x370f
    }
}
#[doc = "Reader of field `RETRANSMAX`"]
pub type RETRANSMAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RETRANSMAX`"]
pub struct RETRANSMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRANSMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `COLLWIN`"]
pub type COLLWIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COLLWIN`"]
pub struct COLLWIN_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLWIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
    #[inline(always)]
    pub fn retransmax(&self) -> RETRANSMAX_R {
        RETRANSMAX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
    #[inline(always)]
    pub fn collwin(&self) -> COLLWIN_R {
        COLLWIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RETRANSMISSION MAXIMUM.This is a programmable field specifying the number of retransmission attempts following a collision before aborting the packet due to excessive collisions. The Standard specifies the attemptLimit to be 0xF (15d). See IEEE 802.3/4.2.3.2.5."]
    #[inline(always)]
    pub fn retransmax(&mut self) -> RETRANSMAX_W {
        RETRANSMAX_W { w: self }
    }
    #[doc = "Bits 8:13 - COLLISION WINDOW. This is a programmable field representing the slot time or collision window during which collisions occur in properly configured networks. The default value of 0x37 (55d) represents a 56 byte window following the preamble and SFD."]
    #[inline(always)]
    pub fn collwin(&mut self) -> COLLWIN_W {
        COLLWIN_W { w: self }
    }
}
