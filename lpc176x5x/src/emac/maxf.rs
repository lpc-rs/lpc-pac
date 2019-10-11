#[doc = "Reader of register MAXF"]
pub type R = crate::R<u32, super::MAXF>;
#[doc = "Writer for register MAXF"]
pub type W = crate::W<u32, super::MAXF>;
#[doc = "Register MAXF `reset()`'s with value 0x0600"]
impl crate::ResetValue for super::MAXF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0600
    }
}
#[doc = "Reader of field `MAXFLEN`"]
pub type MAXFLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAXFLEN`"]
pub struct MAXFLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXFLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
    #[inline(always)]
    pub fn maxflen(&self) -> MAXFLEN_R {
        MAXFLEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
    #[inline(always)]
    pub fn maxflen(&mut self) -> MAXFLEN_W {
        MAXFLEN_W { w: self }
    }
}
