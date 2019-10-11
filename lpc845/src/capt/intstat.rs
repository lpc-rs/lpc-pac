#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Writer for register INTSTAT"]
pub type W = crate::W<u32, super::INTSTAT>;
#[doc = "Register INTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `YESTOUCH`"]
pub type YESTOUCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `NOTOUCH`"]
pub type NOTOUCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `POLLDONE`"]
pub type POLLDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERUN`"]
pub type OVERUN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - the status of touch interrrupt"]
    #[inline(always)]
    pub fn yestouch(&self) -> YESTOUCH_R {
        YESTOUCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - the status of no-touch interrrupt"]
    #[inline(always)]
    pub fn notouch(&self) -> NOTOUCH_R {
        NOTOUCH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - the status of poll or pollnow completing interrupt"]
    #[inline(always)]
    pub fn polldone(&self) -> POLLDONE_R {
        POLLDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - the status of timeout interrupt"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - the status of overrun interrupt"]
    #[inline(always)]
    pub fn overun(&self) -> OVERUN_R {
        OVERUN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {}
