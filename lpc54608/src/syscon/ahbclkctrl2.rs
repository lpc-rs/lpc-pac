#[doc = "Reader of register AHBCLKCTRL2"]
pub type R = crate::R<u32, super::AHBCLKCTRL2>;
#[doc = "Writer for register AHBCLKCTRL2"]
pub type W = crate::W<u32, super::AHBCLKCTRL2>;
#[doc = "Register AHBCLKCTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::AHBCLKCTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCD`"]
pub type LCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCD`"]
pub struct LCD_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_W<'a> {
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
#[doc = "Reader of field `SDIO`"]
pub type SDIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO`"]
pub struct SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_W<'a> {
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
#[doc = "Reader of field `USB1H`"]
pub type USB1H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB1H`"]
pub struct USB1H_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1H_W<'a> {
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
#[doc = "Reader of field `USB1D`"]
pub type USB1D_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB1D`"]
pub struct USB1D_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1D_W<'a> {
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
#[doc = "Reader of field `USB1RAM`"]
pub type USB1RAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB1RAM`"]
pub struct USB1RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1RAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EMC`"]
pub type EMC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMC`"]
pub struct EMC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC_W<'a> {
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
#[doc = "Reader of field `ETH`"]
pub type ETH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETH`"]
pub struct ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_W<'a> {
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
#[doc = "Reader of field `GPIO4`"]
pub type GPIO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO4`"]
pub struct GPIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4_W<'a> {
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
#[doc = "Reader of field `GPIO5`"]
pub type GPIO5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO5`"]
pub struct GPIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5_W<'a> {
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
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES`"]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `OTP`"]
pub type OTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTP`"]
pub struct OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_W<'a> {
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
#[doc = "Reader of field `RNG`"]
pub type RNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNG`"]
pub struct RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `USB0HMR`"]
pub type USB0HMR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB0HMR`"]
pub struct USB0HMR_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0HMR_W<'a> {
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
#[doc = "Reader of field `USB0HSL`"]
pub type USB0HSL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB0HSL`"]
pub struct USB0HSL_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0HSL_W<'a> {
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
#[doc = "Reader of field `SHA0`"]
pub type SHA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHA0`"]
pub struct SHA0_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA0_W<'a> {
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
#[doc = "Reader of field `SC0`"]
pub type SC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SC0`"]
pub struct SC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0_W<'a> {
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
#[doc = "Reader of field `SC1`"]
pub type SC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SC1`"]
pub struct SC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SC1_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Enables the clock for the LCD interface."]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO interface."]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 host interface."]
    #[inline(always)]
    pub fn usb1h(&self) -> USB1H_R {
        USB1H_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 device interface."]
    #[inline(always)]
    pub fn usb1d(&self) -> USB1D_R {
        USB1D_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM interface."]
    #[inline(always)]
    pub fn usb1ram(&self) -> USB1RAM_R {
        USB1RAM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the EMC interface."]
    #[inline(always)]
    pub fn emc(&self) -> EMC_R {
        EMC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the ethernet interface."]
    #[inline(always)]
    pub fn eth(&self) -> ETH_R {
        ETH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4 interface."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5 interface."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the AES interface."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for the OTP interface."]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG interface."]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the Flexcomm8 interface."]
    #[inline(always)]
    pub fn flexcomm8(&self) -> FLEXCOMM8_R {
        FLEXCOMM8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the Flexcomm9 interface."]
    #[inline(always)]
    pub fn flexcomm9(&self) -> FLEXCOMM9_R {
        FLEXCOMM9_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the USB host master interface."]
    #[inline(always)]
    pub fn usb0hmr(&self) -> USB0HMR_R {
        USB0HMR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the USB host slave interface."]
    #[inline(always)]
    pub fn usb0hsl(&self) -> USB0HSL_R {
        USB0HSL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the SHA interface."]
    #[inline(always)]
    pub fn sha0(&self) -> SHA0_R {
        SHA0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the Smart card0 interface."]
    #[inline(always)]
    pub fn sc0(&self) -> SC0_R {
        SC0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the Smart card1 interface."]
    #[inline(always)]
    pub fn sc1(&self) -> SC1_R {
        SC1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enables the clock for the LCD interface."]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W {
        LCD_W { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO interface."]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 host interface."]
    #[inline(always)]
    pub fn usb1h(&mut self) -> USB1H_W {
        USB1H_W { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 device interface."]
    #[inline(always)]
    pub fn usb1d(&mut self) -> USB1D_W {
        USB1D_W { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM interface."]
    #[inline(always)]
    pub fn usb1ram(&mut self) -> USB1RAM_W {
        USB1RAM_W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the EMC interface."]
    #[inline(always)]
    pub fn emc(&mut self) -> EMC_W {
        EMC_W { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the ethernet interface."]
    #[inline(always)]
    pub fn eth(&mut self) -> ETH_W {
        ETH_W { w: self }
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4 interface."]
    #[inline(always)]
    pub fn gpio4(&mut self) -> GPIO4_W {
        GPIO4_W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5 interface."]
    #[inline(always)]
    pub fn gpio5(&mut self) -> GPIO5_W {
        GPIO5_W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the AES interface."]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for the OTP interface."]
    #[inline(always)]
    pub fn otp(&mut self) -> OTP_W {
        OTP_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the RNG interface."]
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W {
        RNG_W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the Flexcomm8 interface."]
    #[inline(always)]
    pub fn flexcomm8(&mut self) -> FLEXCOMM8_W {
        FLEXCOMM8_W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the Flexcomm9 interface."]
    #[inline(always)]
    pub fn flexcomm9(&mut self) -> FLEXCOMM9_W {
        FLEXCOMM9_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the USB host master interface."]
    #[inline(always)]
    pub fn usb0hmr(&mut self) -> USB0HMR_W {
        USB0HMR_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the USB host slave interface."]
    #[inline(always)]
    pub fn usb0hsl(&mut self) -> USB0HSL_W {
        USB0HSL_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the SHA interface."]
    #[inline(always)]
    pub fn sha0(&mut self) -> SHA0_W {
        SHA0_W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the Smart card0 interface."]
    #[inline(always)]
    pub fn sc0(&mut self) -> SC0_W {
        SC0_W { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the Smart card1 interface."]
    #[inline(always)]
    pub fn sc1(&mut self) -> SC1_W {
        SC1_W { w: self }
    }
}
