#[doc = "Reader of register TXCTL"]
pub type R = crate::R<u32, super::TXCTL>;
#[doc = "Writer for register TXCTL"]
pub type W = crate::W<u32, super::TXCTL>;
#[doc = "Register TXCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TXCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXSSEL0_N`"]
pub type TXSSEL0_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSSEL0_N`"]
pub struct TXSSEL0_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL0_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TXSSEL1_N`"]
pub type TXSSEL1_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSSEL1_N`"]
pub struct TXSSEL1_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL1_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TXSSEL2_N`"]
pub type TXSSEL2_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSSEL2_N`"]
pub struct TXSSEL2_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL2_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TXSSEL3_N`"]
pub type TXSSEL3_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSSEL3_N`"]
pub struct TXSSEL3_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL3_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `EOT`"]
pub type EOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOT`"]
pub struct EOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `EOF`"]
pub type EOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOF`"]
pub struct EOF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RXIGNORE`"]
pub type RXIGNORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIGNORE`"]
pub struct RXIGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIGNORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `LEN`"]
pub type LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEN`"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Transmit Slave Select 0."]
    #[inline(always)]
    pub fn txssel0_n(&self) -> TXSSEL0_N_R {
        TXSSEL0_N_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit Slave Select 1."]
    #[inline(always)]
    pub fn txssel1_n(&self) -> TXSSEL1_N_R {
        TXSSEL1_N_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit Slave Select 2."]
    #[inline(always)]
    pub fn txssel2_n(&self) -> TXSSEL2_N_R {
        TXSSEL2_N_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmit Slave Select 3."]
    #[inline(always)]
    pub fn txssel3_n(&self) -> TXSSEL3_N_R {
        TXSSEL3_N_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - End of Transfer."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - End of Frame."]
    #[inline(always)]
    pub fn eof(&self) -> EOF_R {
        EOF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Receive Ignore."]
    #[inline(always)]
    pub fn rxignore(&self) -> RXIGNORE_R {
        RXIGNORE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Data transfer Length."]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Transmit Slave Select 0."]
    #[inline(always)]
    pub fn txssel0_n(&mut self) -> TXSSEL0_N_W {
        TXSSEL0_N_W { w: self }
    }
    #[doc = "Bit 17 - Transmit Slave Select 1."]
    #[inline(always)]
    pub fn txssel1_n(&mut self) -> TXSSEL1_N_W {
        TXSSEL1_N_W { w: self }
    }
    #[doc = "Bit 18 - Transmit Slave Select 2."]
    #[inline(always)]
    pub fn txssel2_n(&mut self) -> TXSSEL2_N_W {
        TXSSEL2_N_W { w: self }
    }
    #[doc = "Bit 19 - Transmit Slave Select 3."]
    #[inline(always)]
    pub fn txssel3_n(&mut self) -> TXSSEL3_N_W {
        TXSSEL3_N_W { w: self }
    }
    #[doc = "Bit 20 - End of Transfer."]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W {
        EOT_W { w: self }
    }
    #[doc = "Bit 21 - End of Frame."]
    #[inline(always)]
    pub fn eof(&mut self) -> EOF_W {
        EOF_W { w: self }
    }
    #[doc = "Bit 22 - Receive Ignore."]
    #[inline(always)]
    pub fn rxignore(&mut self) -> RXIGNORE_W {
        RXIGNORE_W { w: self }
    }
    #[doc = "Bits 24:27 - Data transfer Length."]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
}
