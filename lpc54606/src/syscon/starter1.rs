#[doc = "Reader of register STARTER1"]
pub type R = crate::R<u32, super::STARTER1>;
#[doc = "Writer for register STARTER1"]
pub type W = crate::W<u32, super::STARTER1>;
#[doc = "Register STARTER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PINT4`"]
pub type PINT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINT4`"]
pub struct PINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT4_W<'a> {
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
#[doc = "Reader of field `PINT5`"]
pub type PINT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINT5`"]
pub struct PINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT5_W<'a> {
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
#[doc = "Reader of field `PINT6`"]
pub type PINT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINT6`"]
pub struct PINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT6_W<'a> {
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
#[doc = "Reader of field `PINT7`"]
pub type PINT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PINT7`"]
pub struct PINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT7_W<'a> {
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
#[doc = "Reader of field `CTIMER2`"]
pub type CTIMER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTIMER2`"]
pub struct CTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER2_W<'a> {
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
#[doc = "Reader of field `CTIMER4`"]
pub type CTIMER4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTIMER4`"]
pub struct CTIMER4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER4_W<'a> {
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
#[doc = "Reader of field `SPIFI`"]
pub type SPIFI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIFI`"]
pub struct SPIFI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIFI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FLEXCOMM8`"]
pub type FLEXCOMM8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLEXCOMM8`"]
pub struct FLEXCOMM8_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM8_W<'a> {
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
#[doc = "Reader of field `FLEXCOMM9`"]
pub type FLEXCOMM9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLEXCOMM9`"]
pub struct FLEXCOMM9_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `USB1`"]
pub type USB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB1`"]
pub struct USB1_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `USB1_ACT`"]
pub type USB1_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB1_ACT`"]
pub struct USB1_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1_ACT_W<'a> {
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
#[doc = "Reader of field `ENET_INT1`"]
pub type ENET_INT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENET_INT1`"]
pub struct ENET_INT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET_INT1_W<'a> {
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
#[doc = "Reader of field `ENET_INT2`"]
pub type ENET_INT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENET_INT2`"]
pub struct ENET_INT2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET_INT2_W<'a> {
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
#[doc = "Reader of field `ENET_INT0`"]
pub type ENET_INT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENET_INT0`"]
pub struct ENET_INT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET_INT0_W<'a> {
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
#[doc = "Reader of field `SMARTCARD0`"]
pub type SMARTCARD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMARTCARD0`"]
pub struct SMARTCARD0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMARTCARD0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SMARTCARD1`"]
pub type SMARTCARD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMARTCARD1`"]
pub struct SMARTCARD1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMARTCARD1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GPIO pin interrupt 4 wake-up."]
    #[inline(always)]
    pub fn pint4(&self) -> PINT4_R {
        PINT4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO pin interrupt 5 wake-up."]
    #[inline(always)]
    pub fn pint5(&self) -> PINT5_R {
        PINT5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO pin interrupt 6 wake-up."]
    #[inline(always)]
    pub fn pint6(&self) -> PINT6_R {
        PINT6_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO pin interrupt 7 wake-up."]
    #[inline(always)]
    pub fn pint7(&self) -> PINT7_R {
        PINT7_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Standard counter/timer CTIMER2 wake-up."]
    #[inline(always)]
    pub fn ctimer2(&self) -> CTIMER2_R {
        CTIMER2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Standard counter/timer CTIMER4 wake-up."]
    #[inline(always)]
    pub fn ctimer4(&self) -> CTIMER4_R {
        CTIMER4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPIFI interrupt wake-up. 0 = Wake-up disabled. 1 = Wake-up enabled."]
    #[inline(always)]
    pub fn spifi(&self) -> SPIFI_R {
        SPIFI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flexcomm Interface 8 wake-up."]
    #[inline(always)]
    pub fn flexcomm8(&self) -> FLEXCOMM8_R {
        FLEXCOMM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Flexcomm Interface 9 wake-up."]
    #[inline(always)]
    pub fn flexcomm9(&self) -> FLEXCOMM9_R {
        FLEXCOMM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USB 1 wake-up."]
    #[inline(always)]
    pub fn usb1(&self) -> USB1_R {
        USB1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB 1 activity wake-up."]
    #[inline(always)]
    pub fn usb1_act(&self) -> USB1_ACT_R {
        USB1_ACT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Ethernet."]
    #[inline(always)]
    pub fn enet_int1(&self) -> ENET_INT1_R {
        ENET_INT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Ethernet."]
    #[inline(always)]
    pub fn enet_int2(&self) -> ENET_INT2_R {
        ENET_INT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Ethernet."]
    #[inline(always)]
    pub fn enet_int0(&self) -> ENET_INT0_R {
        ENET_INT0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Smart card 0 wake-up."]
    #[inline(always)]
    pub fn smartcard0(&self) -> SMARTCARD0_R {
        SMARTCARD0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Smart card 1 wake-up."]
    #[inline(always)]
    pub fn smartcard1(&self) -> SMARTCARD1_R {
        SMARTCARD1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO pin interrupt 4 wake-up."]
    #[inline(always)]
    pub fn pint4(&mut self) -> PINT4_W {
        PINT4_W { w: self }
    }
    #[doc = "Bit 1 - GPIO pin interrupt 5 wake-up."]
    #[inline(always)]
    pub fn pint5(&mut self) -> PINT5_W {
        PINT5_W { w: self }
    }
    #[doc = "Bit 2 - GPIO pin interrupt 6 wake-up."]
    #[inline(always)]
    pub fn pint6(&mut self) -> PINT6_W {
        PINT6_W { w: self }
    }
    #[doc = "Bit 3 - GPIO pin interrupt 7 wake-up."]
    #[inline(always)]
    pub fn pint7(&mut self) -> PINT7_W {
        PINT7_W { w: self }
    }
    #[doc = "Bit 4 - Standard counter/timer CTIMER2 wake-up."]
    #[inline(always)]
    pub fn ctimer2(&mut self) -> CTIMER2_W {
        CTIMER2_W { w: self }
    }
    #[doc = "Bit 5 - Standard counter/timer CTIMER4 wake-up."]
    #[inline(always)]
    pub fn ctimer4(&mut self) -> CTIMER4_W {
        CTIMER4_W { w: self }
    }
    #[doc = "Bit 7 - SPIFI interrupt wake-up. 0 = Wake-up disabled. 1 = Wake-up enabled."]
    #[inline(always)]
    pub fn spifi(&mut self) -> SPIFI_W {
        SPIFI_W { w: self }
    }
    #[doc = "Bit 8 - Flexcomm Interface 8 wake-up."]
    #[inline(always)]
    pub fn flexcomm8(&mut self) -> FLEXCOMM8_W {
        FLEXCOMM8_W { w: self }
    }
    #[doc = "Bit 9 - Flexcomm Interface 9 wake-up."]
    #[inline(always)]
    pub fn flexcomm9(&mut self) -> FLEXCOMM9_W {
        FLEXCOMM9_W { w: self }
    }
    #[doc = "Bit 15 - USB 1 wake-up."]
    #[inline(always)]
    pub fn usb1(&mut self) -> USB1_W {
        USB1_W { w: self }
    }
    #[doc = "Bit 16 - USB 1 activity wake-up."]
    #[inline(always)]
    pub fn usb1_act(&mut self) -> USB1_ACT_W {
        USB1_ACT_W { w: self }
    }
    #[doc = "Bit 17 - Ethernet."]
    #[inline(always)]
    pub fn enet_int1(&mut self) -> ENET_INT1_W {
        ENET_INT1_W { w: self }
    }
    #[doc = "Bit 18 - Ethernet."]
    #[inline(always)]
    pub fn enet_int2(&mut self) -> ENET_INT2_W {
        ENET_INT2_W { w: self }
    }
    #[doc = "Bit 19 - Ethernet."]
    #[inline(always)]
    pub fn enet_int0(&mut self) -> ENET_INT0_W {
        ENET_INT0_W { w: self }
    }
    #[doc = "Bit 23 - Smart card 0 wake-up."]
    #[inline(always)]
    pub fn smartcard0(&mut self) -> SMARTCARD0_W {
        SMARTCARD0_W { w: self }
    }
    #[doc = "Bit 24 - Smart card 1 wake-up."]
    #[inline(always)]
    pub fn smartcard1(&mut self) -> SMARTCARD1_W {
        SMARTCARD1_W { w: self }
    }
}
