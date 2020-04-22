#[doc = "Reader of register I2CPADCFG"]
pub type R = crate::R<u32, super::I2CPADCFG>;
#[doc = "Writer for register I2CPADCFG"]
pub type W = crate::W<u32, super::I2CPADCFG>;
#[doc = "Register I2CPADCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::I2CPADCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Drive mode control for the SDA0 pin, P0.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDADRV0_A {
    #[doc = "0: Standard. The SDA0 pin is in the standard drive mode."]
    STANDARD = 0,
    #[doc = "1: Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    FAST_MODE_PLUS = 1,
}
impl From<SDADRV0_A> for bool {
    #[inline(always)]
    fn from(variant: SDADRV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDADRV0`"]
pub type SDADRV0_R = crate::R<bool, SDADRV0_A>;
impl SDADRV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDADRV0_A {
        match self.bits {
            false => SDADRV0_A::STANDARD,
            true => SDADRV0_A::FAST_MODE_PLUS,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SDADRV0_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS`"]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == SDADRV0_A::FAST_MODE_PLUS
    }
}
#[doc = "Write proxy for field `SDADRV0`"]
pub struct SDADRV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDADRV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDADRV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard. The SDA0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SDADRV0_A::STANDARD)
    }
    #[doc = "Fast-mode plus. The SDA0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut W {
        self.variant(SDADRV0_A::FAST_MODE_PLUS)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "I 2C filter mode control for the SDA0 pin, P0.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDAI2C0_A {
    #[doc = "0: Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    ENABLED = 0,
    #[doc = "1: Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    DISABLED = 1,
}
impl From<SDAI2C0_A> for bool {
    #[inline(always)]
    fn from(variant: SDAI2C0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDAI2C0`"]
pub type SDAI2C0_R = crate::R<bool, SDAI2C0_A>;
impl SDAI2C0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDAI2C0_A {
        match self.bits {
            false => SDAI2C0_A::ENABLED,
            true => SDAI2C0_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDAI2C0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDAI2C0_A::DISABLED
    }
}
#[doc = "Write proxy for field `SDAI2C0`"]
pub struct SDAI2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SDAI2C0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDAI2C0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enabled. The SDA0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDAI2C0_A::ENABLED)
    }
    #[doc = "Disabled. The SDA0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDAI2C0_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Drive mode control for the SCL0 pin, P0.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLDRV0_A {
    #[doc = "0: Standard. The SCL0 pin is in the standard drive mode."]
    STANDARD = 0,
    #[doc = "1: Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    FAST_MODE_PLUS = 1,
}
impl From<SCLDRV0_A> for bool {
    #[inline(always)]
    fn from(variant: SCLDRV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLDRV0`"]
pub type SCLDRV0_R = crate::R<bool, SCLDRV0_A>;
impl SCLDRV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLDRV0_A {
        match self.bits {
            false => SCLDRV0_A::STANDARD,
            true => SCLDRV0_A::FAST_MODE_PLUS,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SCLDRV0_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS`"]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == SCLDRV0_A::FAST_MODE_PLUS
    }
}
#[doc = "Write proxy for field `SCLDRV0`"]
pub struct SCLDRV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLDRV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLDRV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard. The SCL0 pin is in the standard drive mode."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SCLDRV0_A::STANDARD)
    }
    #[doc = "Fast-mode plus. The SCL0 pin is in Fast Mode Plus drive mode."]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut W {
        self.variant(SCLDRV0_A::FAST_MODE_PLUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "I 2C filter mode control for the SCL0 pin, P0.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLI2C0_A {
    #[doc = "0: Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    ENABLED = 0,
    #[doc = "1: Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    DISABLED = 1,
}
impl From<SCLI2C0_A> for bool {
    #[inline(always)]
    fn from(variant: SCLI2C0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCLI2C0`"]
pub type SCLI2C0_R = crate::R<bool, SCLI2C0_A>;
impl SCLI2C0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLI2C0_A {
        match self.bits {
            false => SCLI2C0_A::ENABLED,
            true => SCLI2C0_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCLI2C0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCLI2C0_A::DISABLED
    }
}
#[doc = "Write proxy for field `SCLI2C0`"]
pub struct SCLI2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLI2C0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLI2C0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enabled. The SCL0 pin has I2C glitch filtering and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCLI2C0_A::ENABLED)
    }
    #[doc = "Disabled. The SCL0 pin has I2C glitch filtering and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCLI2C0_A::DISABLED)
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
impl R {
    #[doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdadrv0(&self) -> SDADRV0_R {
        SDADRV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdai2c0(&self) -> SDAI2C0_R {
        SDAI2C0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scldrv0(&self) -> SCLDRV0_R {
        SCLDRV0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scli2c0(&self) -> SCLI2C0_R {
        SCLI2C0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdadrv0(&mut self) -> SDADRV0_W {
        SDADRV0_W { w: self }
    }
    #[doc = "Bit 1 - I 2C filter mode control for the SDA0 pin, P0.27."]
    #[inline(always)]
    pub fn sdai2c0(&mut self) -> SDAI2C0_W {
        SDAI2C0_W { w: self }
    }
    #[doc = "Bit 2 - Drive mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scldrv0(&mut self) -> SCLDRV0_W {
        SCLDRV0_W { w: self }
    }
    #[doc = "Bit 3 - I 2C filter mode control for the SCL0 pin, P0.28."]
    #[inline(always)]
    pub fn scli2c0(&mut self) -> SCLI2C0_W {
        SCLI2C0_W { w: self }
    }
}
