#[doc = "Reader of register SYSMEMREMAP"]
pub type R = crate::R<u32, super::SYSMEMREMAP>;
#[doc = "Writer for register SYSMEMREMAP"]
pub type W = crate::W<u32, super::SYSMEMREMAP>;
#[doc = "Register SYSMEMREMAP `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSMEMREMAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System memory remap. Value 0x3 is reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAP_A {
    #[doc = "0: Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    BOOT_LOADER_MODE = 0,
    #[doc = "1: User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    USER_RAM_MODE = 1,
    #[doc = "2: User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    USER_FLASH_MODE = 2,
}
impl From<MAP_A> for u8 {
    #[inline(always)]
    fn from(variant: MAP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MAP`"]
pub type MAP_R = crate::R<u8, MAP_A>;
impl MAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MAP_A::BOOT_LOADER_MODE),
            1 => Val(MAP_A::USER_RAM_MODE),
            2 => Val(MAP_A::USER_FLASH_MODE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_LOADER_MODE`"]
    #[inline(always)]
    pub fn is_boot_loader_mode(&self) -> bool {
        *self == MAP_A::BOOT_LOADER_MODE
    }
    #[doc = "Checks if the value of the field is `USER_RAM_MODE`"]
    #[inline(always)]
    pub fn is_user_ram_mode(&self) -> bool {
        *self == MAP_A::USER_RAM_MODE
    }
    #[doc = "Checks if the value of the field is `USER_FLASH_MODE`"]
    #[inline(always)]
    pub fn is_user_flash_mode(&self) -> bool {
        *self == MAP_A::USER_FLASH_MODE
    }
}
#[doc = "Write proxy for field `MAP`"]
pub struct MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    #[inline(always)]
    pub fn boot_loader_mode(self) -> &'a mut W {
        self.variant(MAP_A::BOOT_LOADER_MODE)
    }
    #[doc = "User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    #[inline(always)]
    pub fn user_ram_mode(self) -> &'a mut W {
        self.variant(MAP_A::USER_RAM_MODE)
    }
    #[doc = "User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    #[inline(always)]
    pub fn user_flash_mode(self) -> &'a mut W {
        self.variant(MAP_A::USER_FLASH_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - System memory remap. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System memory remap. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn map(&mut self) -> MAP_W {
        MAP_W { w: self }
    }
}
