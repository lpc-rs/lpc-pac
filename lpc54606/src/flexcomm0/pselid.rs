#[doc = "Reader of register PSELID"]
pub type R = crate::R<u32, super::PSELID>;
#[doc = "Writer for register PSELID"]
pub type W = crate::W<u32, super::PSELID>;
#[doc = "Register PSELID `reset()`'s with value 0x0010_1000"]
impl crate::ResetValue for super::PSELID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_1000
    }
}
#[doc = "Peripheral Select. This field is writable by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERSEL_A {
    #[doc = "0: No peripheral selected."]
    NO_PERIPH_SELECTED = 0,
    #[doc = "1: USART function selected."]
    USART = 1,
    #[doc = "2: SPI function selected."]
    SPI = 2,
    #[doc = "3: I2C function selected."]
    I2C = 3,
    #[doc = "4: I2S transmit function selected."]
    I2S_TRANSMIT = 4,
    #[doc = "5: I2S receive function selected."]
    I2S_RECEIVE = 5,
}
impl From<PERSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PERSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PERSEL`"]
pub type PERSEL_R = crate::R<u8, PERSEL_A>;
impl PERSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PERSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PERSEL_A::NO_PERIPH_SELECTED),
            1 => Val(PERSEL_A::USART),
            2 => Val(PERSEL_A::SPI),
            3 => Val(PERSEL_A::I2C),
            4 => Val(PERSEL_A::I2S_TRANSMIT),
            5 => Val(PERSEL_A::I2S_RECEIVE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_PERIPH_SELECTED`"]
    #[inline(always)]
    pub fn is_no_periph_selected(&self) -> bool {
        *self == PERSEL_A::NO_PERIPH_SELECTED
    }
    #[doc = "Checks if the value of the field is `USART`"]
    #[inline(always)]
    pub fn is_usart(&self) -> bool {
        *self == PERSEL_A::USART
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == PERSEL_A::SPI
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == PERSEL_A::I2C
    }
    #[doc = "Checks if the value of the field is `I2S_TRANSMIT`"]
    #[inline(always)]
    pub fn is_i2s_transmit(&self) -> bool {
        *self == PERSEL_A::I2S_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `I2S_RECEIVE`"]
    #[inline(always)]
    pub fn is_i2s_receive(&self) -> bool {
        *self == PERSEL_A::I2S_RECEIVE
    }
}
#[doc = "Write proxy for field `PERSEL`"]
pub struct PERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No peripheral selected."]
    #[inline(always)]
    pub fn no_periph_selected(self) -> &'a mut W {
        self.variant(PERSEL_A::NO_PERIPH_SELECTED)
    }
    #[doc = "USART function selected."]
    #[inline(always)]
    pub fn usart(self) -> &'a mut W {
        self.variant(PERSEL_A::USART)
    }
    #[doc = "SPI function selected."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(PERSEL_A::SPI)
    }
    #[doc = "I2C function selected."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(PERSEL_A::I2C)
    }
    #[doc = "I2S transmit function selected."]
    #[inline(always)]
    pub fn i2s_transmit(self) -> &'a mut W {
        self.variant(PERSEL_A::I2S_TRANSMIT)
    }
    #[doc = "I2S receive function selected."]
    #[inline(always)]
    pub fn i2s_receive(self) -> &'a mut W {
        self.variant(PERSEL_A::I2S_RECEIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Lock the peripheral select. This field is writable by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: Peripheral select can be changed by software."]
    UNLOCKED = 0,
    #[doc = "1: Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripheral select can be changed by software."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "USART present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USARTPRESENT_A {
    #[doc = "0: This Flexcomm does not include the USART function."]
    NOT_PRESENT = 0,
    #[doc = "1: This Flexcomm includes the USART function."]
    PRESENT = 1,
}
impl From<USARTPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: USARTPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USARTPRESENT`"]
pub type USARTPRESENT_R = crate::R<bool, USARTPRESENT_A>;
impl USARTPRESENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USARTPRESENT_A {
        match self.bits {
            false => USARTPRESENT_A::NOT_PRESENT,
            true => USARTPRESENT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == USARTPRESENT_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == USARTPRESENT_A::PRESENT
    }
}
#[doc = "SPI present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIPRESENT_A {
    #[doc = "0: This Flexcomm does not include the SPI function."]
    NOT_PRESENT = 0,
    #[doc = "1: This Flexcomm includes the SPI function."]
    PRESENT = 1,
}
impl From<SPIPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: SPIPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIPRESENT`"]
pub type SPIPRESENT_R = crate::R<bool, SPIPRESENT_A>;
impl SPIPRESENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIPRESENT_A {
        match self.bits {
            false => SPIPRESENT_A::NOT_PRESENT,
            true => SPIPRESENT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == SPIPRESENT_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == SPIPRESENT_A::PRESENT
    }
}
#[doc = "I2C present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CPRESENT_A {
    #[doc = "0: This Flexcomm does not include the I2C function."]
    NOT_PRESENT = 0,
    #[doc = "1: This Flexcomm includes the I2C function."]
    PRESENT = 1,
}
impl From<I2CPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: I2CPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2CPRESENT`"]
pub type I2CPRESENT_R = crate::R<bool, I2CPRESENT_A>;
impl I2CPRESENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CPRESENT_A {
        match self.bits {
            false => I2CPRESENT_A::NOT_PRESENT,
            true => I2CPRESENT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == I2CPRESENT_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == I2CPRESENT_A::PRESENT
    }
}
#[doc = "I 2S present indicator. This field is Read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SPRESENT_A {
    #[doc = "0: This Flexcomm does not include the I2S function."]
    NOT_PRESENT = 0,
    #[doc = "1: This Flexcomm includes the I2S function."]
    PRESENT = 1,
}
impl From<I2SPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: I2SPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2SPRESENT`"]
pub type I2SPRESENT_R = crate::R<bool, I2SPRESENT_A>;
impl I2SPRESENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SPRESENT_A {
        match self.bits {
            false => I2SPRESENT_A::NOT_PRESENT,
            true => I2SPRESENT_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == I2SPRESENT_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == I2SPRESENT_A::PRESENT
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub fn persel(&self) -> PERSEL_R {
        PERSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn usartpresent(&self) -> USARTPRESENT_R {
        USARTPRESENT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn spipresent(&self) -> SPIPRESENT_R {
        SPIPRESENT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn i2cpresent(&self) -> I2CPRESENT_R {
        I2CPRESENT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I 2S present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn i2spresent(&self) -> I2SPRESENT_R {
        I2SPRESENT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 12:31 - Flexcomm ID."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:2 - Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub fn persel(&mut self) -> PERSEL_W {
        PERSEL_W { w: self }
    }
    #[doc = "Bit 3 - Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
