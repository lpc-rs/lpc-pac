#[doc = "Register `SCICTRL` reader"]
pub struct R(crate::R<SCICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCICTRL` writer"]
pub struct W(crate::W<SCICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCICTRL_SPEC>;
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
impl From<crate::W<SCICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Smart Card Interface Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCIEN_A {
    #[doc = "0: Smart card interface disabled."]
    SMART_CARD_INTERFACE = 0,
    #[doc = "1: Asynchronous half duplex smart card interface is enabled."]
    ASYNCHRONOUS_HALF_DU = 1,
}
impl From<SCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCIEN` reader - Smart Card Interface Enable."]
pub struct SCIEN_R(crate::FieldReader<bool, SCIEN_A>);
impl SCIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCIEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCIEN_A {
        match self.bits {
            false => SCIEN_A::SMART_CARD_INTERFACE,
            true => SCIEN_A::ASYNCHRONOUS_HALF_DU,
        }
    }
    #[doc = "Checks if the value of the field is `SMART_CARD_INTERFACE`"]
    #[inline(always)]
    pub fn is_smart_card_interface(&self) -> bool {
        **self == SCIEN_A::SMART_CARD_INTERFACE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_HALF_DU`"]
    #[inline(always)]
    pub fn is_asynchronous_half_du(&self) -> bool {
        **self == SCIEN_A::ASYNCHRONOUS_HALF_DU
    }
}
impl core::ops::Deref for SCIEN_R {
    type Target = crate::FieldReader<bool, SCIEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCIEN` writer - Smart Card Interface Enable."]
pub struct SCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCIEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Smart card interface disabled."]
    #[inline(always)]
    pub fn smart_card_interface(self) -> &'a mut W {
        self.variant(SCIEN_A::SMART_CARD_INTERFACE)
    }
    #[doc = "Asynchronous half duplex smart card interface is enabled."]
    #[inline(always)]
    pub fn asynchronous_half_du(self) -> &'a mut W {
        self.variant(SCIEN_A::ASYNCHRONOUS_HALF_DU)
    }
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
#[doc = "NACK response disable. Only applicable in T=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKDIS_A {
    #[doc = "0: A NACK response is enabled."]
    ENABLED = 0,
    #[doc = "1: A NACK response is inhibited."]
    DISABLED = 1,
}
impl From<NACKDIS_A> for bool {
    #[inline(always)]
    fn from(variant: NACKDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKDIS` reader - NACK response disable. Only applicable in T=0."]
pub struct NACKDIS_R(crate::FieldReader<bool, NACKDIS_A>);
impl NACKDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACKDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKDIS_A {
        match self.bits {
            false => NACKDIS_A::ENABLED,
            true => NACKDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == NACKDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NACKDIS_A::DISABLED
    }
}
impl core::ops::Deref for NACKDIS_R {
    type Target = crate::FieldReader<bool, NACKDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACKDIS` writer - NACK response disable. Only applicable in T=0."]
pub struct NACKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACKDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A NACK response is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKDIS_A::ENABLED)
    }
    #[doc = "A NACK response is inhibited."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKDIS_A::DISABLED)
    }
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
#[doc = "Protocol selection as defined in the ISO7816-3 standard.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTSEL_A {
    #[doc = "0: T = 0"]
    T_EQ_0 = 0,
    #[doc = "1: T = 1"]
    T_EQ_1 = 1,
}
impl From<PROTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PROTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTSEL` reader - Protocol selection as defined in the ISO7816-3 standard."]
pub struct PROTSEL_R(crate::FieldReader<bool, PROTSEL_A>);
impl PROTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTSEL_A {
        match self.bits {
            false => PROTSEL_A::T_EQ_0,
            true => PROTSEL_A::T_EQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `T_EQ_0`"]
    #[inline(always)]
    pub fn is_t_eq_0(&self) -> bool {
        **self == PROTSEL_A::T_EQ_0
    }
    #[doc = "Checks if the value of the field is `T_EQ_1`"]
    #[inline(always)]
    pub fn is_t_eq_1(&self) -> bool {
        **self == PROTSEL_A::T_EQ_1
    }
}
impl core::ops::Deref for PROTSEL_R {
    type Target = crate::FieldReader<bool, PROTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROTSEL` writer - Protocol selection as defined in the ISO7816-3 standard."]
pub struct PROTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "T = 0"]
    #[inline(always)]
    pub fn t_eq_0(self) -> &'a mut W {
        self.variant(PROTSEL_A::T_EQ_0)
    }
    #[doc = "T = 1"]
    #[inline(always)]
    pub fn t_eq_1(self) -> &'a mut W {
        self.variant(PROTSEL_A::T_EQ_1)
    }
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
#[doc = "Field `TXRETRY` reader - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
pub struct TXRETRY_R(crate::FieldReader<u8, u8>);
impl TXRETRY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXRETRY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRETRY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRETRY` writer - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
pub struct TXRETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRETRY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `XTRAGUARD` reader - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
pub struct XTRAGUARD_R(crate::FieldReader<u8, u8>);
impl XTRAGUARD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTRAGUARD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTRAGUARD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTRAGUARD` writer - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
pub struct XTRAGUARD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTRAGUARD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&self) -> SCIEN_R {
        SCIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline(always)]
    pub fn nackdis(&self) -> NACKDIS_R {
        NACKDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&self) -> PROTSEL_R {
        PROTSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
    #[inline(always)]
    pub fn txretry(&self) -> TXRETRY_R {
        TXRETRY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
    #[inline(always)]
    pub fn xtraguard(&self) -> XTRAGUARD_R {
        XTRAGUARD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&mut self) -> SCIEN_W {
        SCIEN_W { w: self }
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline(always)]
    pub fn nackdis(&mut self) -> NACKDIS_W {
        NACKDIS_W { w: self }
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&mut self) -> PROTSEL_W {
        PROTSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
    #[inline(always)]
    pub fn txretry(&mut self) -> TXRETRY_W {
        TXRETRY_W { w: self }
    }
    #[doc = "Bits 8:15 - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
    #[inline(always)]
    pub fn xtraguard(&mut self) -> XTRAGUARD_W {
        XTRAGUARD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Smart Card Interface Control register. Enables and configures the Smart Card Interface feature.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scictrl](index.html) module"]
pub struct SCICTRL_SPEC;
impl crate::RegisterSpec for SCICTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scictrl::R](R) reader structure"]
impl crate::Readable for SCICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scictrl::W](W) writer structure"]
impl crate::Writable for SCICTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCICTRL to value 0"]
impl crate::Resettable for SCICTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
