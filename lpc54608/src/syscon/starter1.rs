#[doc = "Register `STARTER1` reader"]
pub struct R(crate::R<STARTER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STARTER1_SPEC>> for R {
    fn from(reader: crate::R<STARTER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTER1` writer"]
pub struct W(crate::W<STARTER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<STARTER1_SPEC>> for W {
    fn from(writer: crate::W<STARTER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINT4` reader - GPIO pin interrupt 4 wake-up."]
pub struct PINT4_R(crate::FieldReader<bool, bool>);
impl PINT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINT4` writer - GPIO pin interrupt 4 wake-up."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PINT5` reader - GPIO pin interrupt 5 wake-up."]
pub struct PINT5_R(crate::FieldReader<bool, bool>);
impl PINT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINT5` writer - GPIO pin interrupt 5 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PINT6` reader - GPIO pin interrupt 6 wake-up."]
pub struct PINT6_R(crate::FieldReader<bool, bool>);
impl PINT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINT6` writer - GPIO pin interrupt 6 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PINT7` reader - GPIO pin interrupt 7 wake-up."]
pub struct PINT7_R(crate::FieldReader<bool, bool>);
impl PINT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINT7` writer - GPIO pin interrupt 7 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CTIMER2` reader - Standard counter/timer CTIMER2 wake-up."]
pub struct CTIMER2_R(crate::FieldReader<bool, bool>);
impl CTIMER2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER2` writer - Standard counter/timer CTIMER2 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CTIMER4` reader - Standard counter/timer CTIMER4 wake-up."]
pub struct CTIMER4_R(crate::FieldReader<bool, bool>);
impl CTIMER4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER4` writer - Standard counter/timer CTIMER4 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SPIFI` reader - SPIFI interrupt wake-up. 0 = Wake-up disabled. 1 = Wake-up enabled."]
pub struct SPIFI_R(crate::FieldReader<bool, bool>);
impl SPIFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIFI` writer - SPIFI interrupt wake-up. 0 = Wake-up disabled. 1 = Wake-up enabled."]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `FLEXCOMM8` reader - Flexcomm Interface 8 wake-up."]
pub struct FLEXCOMM8_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM8` writer - Flexcomm Interface 8 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FLEXCOMM9` reader - Flexcomm Interface 9 wake-up."]
pub struct FLEXCOMM9_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM9_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM9` writer - Flexcomm Interface 9 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `USB1` reader - USB 1 wake-up."]
pub struct USB1_R(crate::FieldReader<bool, bool>);
impl USB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1` writer - USB 1 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `USB1_ACT` reader - USB 1 activity wake-up."]
pub struct USB1_ACT_R(crate::FieldReader<bool, bool>);
impl USB1_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB1_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1_ACT` writer - USB 1 activity wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ENET_INT1` reader - Ethernet."]
pub struct ENET_INT1_R(crate::FieldReader<bool, bool>);
impl ENET_INT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENET_INT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENET_INT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENET_INT1` writer - Ethernet."]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ENET_INT2` reader - Ethernet."]
pub struct ENET_INT2_R(crate::FieldReader<bool, bool>);
impl ENET_INT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENET_INT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENET_INT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENET_INT2` writer - Ethernet."]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `ENET_INT0` reader - Ethernet."]
pub struct ENET_INT0_R(crate::FieldReader<bool, bool>);
impl ENET_INT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENET_INT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENET_INT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENET_INT0` writer - Ethernet."]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SMARTCARD0` reader - Smart card 0 wake-up."]
pub struct SMARTCARD0_R(crate::FieldReader<bool, bool>);
impl SMARTCARD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMARTCARD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMARTCARD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMARTCARD0` writer - Smart card 0 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `SMARTCARD1` reader - Smart card 1 wake-up."]
pub struct SMARTCARD1_R(crate::FieldReader<bool, bool>);
impl SMARTCARD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMARTCARD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMARTCARD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMARTCARD1` writer - Smart card 1 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic 0 wake-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starter1](index.html) module"]
pub struct STARTER1_SPEC;
impl crate::RegisterSpec for STARTER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starter1::R](R) reader structure"]
impl crate::Readable for STARTER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starter1::W](W) writer structure"]
impl crate::Writable for STARTER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTER1 to value 0"]
impl crate::Resettable for STARTER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
