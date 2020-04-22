#[doc = "Reader of register PRESETCTRL2"]
pub type R = crate::R<u32, super::PRESETCTRL2>;
#[doc = "Writer for register PRESETCTRL2"]
pub type W = crate::W<u32, super::PRESETCTRL2>;
#[doc = "Register PRESETCTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESETCTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCD_RST`"]
pub type LCD_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCD_RST`"]
pub struct LCD_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_RST_W<'a> {
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
#[doc = "Reader of field `SDIO_RST`"]
pub type SDIO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIO_RST`"]
pub struct SDIO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_RST_W<'a> {
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
#[doc = "Reader of field `USB1H_RST`"]
pub type USB1H_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB1H_RST`"]
pub struct USB1H_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1H_RST_W<'a> {
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
#[doc = "Reader of field `USB1D_RST`"]
pub type USB1D_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB1D_RST`"]
pub struct USB1D_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1D_RST_W<'a> {
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
#[doc = "Reader of field `USB1RAM_RST`"]
pub type USB1RAM_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB1RAM_RST`"]
pub struct USB1RAM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1RAM_RST_W<'a> {
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
#[doc = "Reader of field `EMC_RESET`"]
pub type EMC_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMC_RESET`"]
pub struct EMC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC_RESET_W<'a> {
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
#[doc = "Reader of field `ETH_RST`"]
pub type ETH_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETH_RST`"]
pub struct ETH_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_RST_W<'a> {
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
#[doc = "Reader of field `GPIO4_RST`"]
pub type GPIO4_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO4_RST`"]
pub struct GPIO4_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4_RST_W<'a> {
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
#[doc = "Reader of field `GPIO5_RST`"]
pub type GPIO5_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO5_RST`"]
pub struct GPIO5_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5_RST_W<'a> {
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
#[doc = "Reader of field `AES_RST`"]
pub type AES_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES_RST`"]
pub struct AES_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_RST_W<'a> {
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
#[doc = "Reader of field `OTP_RST`"]
pub type OTP_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTP_RST`"]
pub struct OTP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_RST_W<'a> {
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
#[doc = "Reader of field `RNG_RST`"]
pub type RNG_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNG_RST`"]
pub struct RNG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_RST_W<'a> {
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
#[doc = "Reader of field `FC8_RST`"]
pub type FC8_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FC8_RST`"]
pub struct FC8_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC8_RST_W<'a> {
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
#[doc = "Reader of field `FC9_RST`"]
pub type FC9_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FC9_RST`"]
pub struct FC9_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC9_RST_W<'a> {
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
#[doc = "Reader of field `USB0HMR_RST`"]
pub type USB0HMR_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB0HMR_RST`"]
pub struct USB0HMR_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0HMR_RST_W<'a> {
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
#[doc = "Reader of field `USB0HSL_RST`"]
pub type USB0HSL_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB0HSL_RST`"]
pub struct USB0HSL_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0HSL_RST_W<'a> {
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
#[doc = "Reader of field `SHA_RST`"]
pub type SHA_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHA_RST`"]
pub struct SHA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_RST_W<'a> {
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
#[doc = "Reader of field `SC0_RST`"]
pub type SC0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SC0_RST`"]
pub struct SC0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0_RST_W<'a> {
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
#[doc = "Reader of field `SC1_RST`"]
pub type SC1_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SC1_RST`"]
pub struct SC1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SC1_RST_W<'a> {
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
    #[doc = "Bit 2 - LCD reset control."]
    #[inline(always)]
    pub fn lcd_rst(&self) -> LCD_RST_R {
        LCD_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1h_rst(&self) -> USB1H_RST_R {
        USB1H_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB1 Device reset control."]
    #[inline(always)]
    pub fn usb1d_rst(&self) -> USB1D_RST_R {
        USB1D_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1ram_rst(&self) -> USB1RAM_RST_R {
        USB1RAM_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EMC reset control."]
    #[inline(always)]
    pub fn emc_reset(&self) -> EMC_RESET_R {
        EMC_RESET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Ethernet reset control."]
    #[inline(always)]
    pub fn eth_rst(&self) -> ETH_RST_R {
        ETH_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO4 reset control."]
    #[inline(always)]
    pub fn gpio4_rst(&self) -> GPIO4_RST_R {
        GPIO4_RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO5 reset control."]
    #[inline(always)]
    pub fn gpio5_rst(&self) -> GPIO5_RST_R {
        GPIO5_RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AES reset control."]
    #[inline(always)]
    pub fn aes_rst(&self) -> AES_RST_R {
        AES_RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - OTP reset control."]
    #[inline(always)]
    pub fn otp_rst(&self) -> OTP_RST_R {
        OTP_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&self) -> RNG_RST_R {
        RNG_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 8 reset control."]
    #[inline(always)]
    pub fn fc8_rst(&self) -> FC8_RST_R {
        FC8_RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 9 reset control."]
    #[inline(always)]
    pub fn fc9_rst(&self) -> FC9_RST_R {
        FC9_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB0 HOST master reset control."]
    #[inline(always)]
    pub fn usb0hmr_rst(&self) -> USB0HMR_RST_R {
        USB0HMR_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USB0 HOST slave reset control."]
    #[inline(always)]
    pub fn usb0hsl_rst(&self) -> USB0HSL_RST_R {
        USB0HSL_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SHA reset control."]
    #[inline(always)]
    pub fn sha_rst(&self) -> SHA_RST_R {
        SHA_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Smart card 0 reset control."]
    #[inline(always)]
    pub fn sc0_rst(&self) -> SC0_RST_R {
        SC0_RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Smart card 1 reset control."]
    #[inline(always)]
    pub fn sc1_rst(&self) -> SC1_RST_R {
        SC1_RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - LCD reset control."]
    #[inline(always)]
    pub fn lcd_rst(&mut self) -> LCD_RST_W {
        LCD_RST_W { w: self }
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W {
        SDIO_RST_W { w: self }
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1h_rst(&mut self) -> USB1H_RST_W {
        USB1H_RST_W { w: self }
    }
    #[doc = "Bit 5 - USB1 Device reset control."]
    #[inline(always)]
    pub fn usb1d_rst(&mut self) -> USB1D_RST_W {
        USB1D_RST_W { w: self }
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1ram_rst(&mut self) -> USB1RAM_RST_W {
        USB1RAM_RST_W { w: self }
    }
    #[doc = "Bit 7 - EMC reset control."]
    #[inline(always)]
    pub fn emc_reset(&mut self) -> EMC_RESET_W {
        EMC_RESET_W { w: self }
    }
    #[doc = "Bit 8 - Ethernet reset control."]
    #[inline(always)]
    pub fn eth_rst(&mut self) -> ETH_RST_W {
        ETH_RST_W { w: self }
    }
    #[doc = "Bit 9 - GPIO4 reset control."]
    #[inline(always)]
    pub fn gpio4_rst(&mut self) -> GPIO4_RST_W {
        GPIO4_RST_W { w: self }
    }
    #[doc = "Bit 10 - GPIO5 reset control."]
    #[inline(always)]
    pub fn gpio5_rst(&mut self) -> GPIO5_RST_W {
        GPIO5_RST_W { w: self }
    }
    #[doc = "Bit 11 - AES reset control."]
    #[inline(always)]
    pub fn aes_rst(&mut self) -> AES_RST_W {
        AES_RST_W { w: self }
    }
    #[doc = "Bit 12 - OTP reset control."]
    #[inline(always)]
    pub fn otp_rst(&mut self) -> OTP_RST_W {
        OTP_RST_W { w: self }
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&mut self) -> RNG_RST_W {
        RNG_RST_W { w: self }
    }
    #[doc = "Bit 14 - Flexcomm 8 reset control."]
    #[inline(always)]
    pub fn fc8_rst(&mut self) -> FC8_RST_W {
        FC8_RST_W { w: self }
    }
    #[doc = "Bit 15 - Flexcomm 9 reset control."]
    #[inline(always)]
    pub fn fc9_rst(&mut self) -> FC9_RST_W {
        FC9_RST_W { w: self }
    }
    #[doc = "Bit 16 - USB0 HOST master reset control."]
    #[inline(always)]
    pub fn usb0hmr_rst(&mut self) -> USB0HMR_RST_W {
        USB0HMR_RST_W { w: self }
    }
    #[doc = "Bit 17 - USB0 HOST slave reset control."]
    #[inline(always)]
    pub fn usb0hsl_rst(&mut self) -> USB0HSL_RST_W {
        USB0HSL_RST_W { w: self }
    }
    #[doc = "Bit 18 - SHA reset control."]
    #[inline(always)]
    pub fn sha_rst(&mut self) -> SHA_RST_W {
        SHA_RST_W { w: self }
    }
    #[doc = "Bit 19 - Smart card 0 reset control."]
    #[inline(always)]
    pub fn sc0_rst(&mut self) -> SC0_RST_W {
        SC0_RST_W { w: self }
    }
    #[doc = "Bit 20 - Smart card 1 reset control."]
    #[inline(always)]
    pub fn sc1_rst(&mut self) -> SC1_RST_W {
        SC1_RST_W { w: self }
    }
}
