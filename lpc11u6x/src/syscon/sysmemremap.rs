#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSMEMREMAP {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `MAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAPR {
    #[doc = "Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    BOOT_LOADER_MODE,
    #[doc = "User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    USER_RAM_MODE,
    #[doc = "User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    USER_FLASH_MODE,
}
impl crate::ToBits<u8> for MAPR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MAPR::BOOT_LOADER_MODE => 0,
            MAPR::USER_RAM_MODE => 1,
            MAPR::USER_FLASH_MODE => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MAP_R = crate::FR<u8, MAPR>;
impl MAP_R {
    #[doc = "Checks if the value of the field is `BOOT_LOADER_MODE`"]
    #[inline(always)]
    pub fn is_boot_loader_mode(&self) -> bool {
        *self == MAPR::BOOT_LOADER_MODE
    }
    #[doc = "Checks if the value of the field is `USER_RAM_MODE`"]
    #[inline(always)]
    pub fn is_user_ram_mode(&self) -> bool {
        *self == MAPR::USER_RAM_MODE
    }
    #[doc = "Checks if the value of the field is `USER_FLASH_MODE`"]
    #[inline(always)]
    pub fn is_user_flash_mode(&self) -> bool {
        *self == MAPR::USER_FLASH_MODE
    }
}
#[doc = "Values that can be written to the field `MAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAPW {
    #[doc = "Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    BOOT_LOADER_MODE,
    #[doc = "User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    USER_RAM_MODE,
    #[doc = "User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    USER_FLASH_MODE,
}
impl MAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MAPW::BOOT_LOADER_MODE => 0,
            MAPW::USER_RAM_MODE => 1,
            MAPW::USER_FLASH_MODE => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MAPW<'a> {
    w: &'a mut W,
}
impl<'a> _MAPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    #[inline(always)]
    pub fn boot_loader_mode(self) -> &'a mut W {
        self.variant(MAPW::BOOT_LOADER_MODE)
    }
    #[doc = "User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    #[inline(always)]
    pub fn user_ram_mode(self) -> &'a mut W {
        self.variant(MAPW::USER_RAM_MODE)
    }
    #[doc = "User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    #[inline(always)]
    pub fn user_flash_mode(self) -> &'a mut W {
        self.variant(MAPW::USER_FLASH_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - System memory remap. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new((self.bits() & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - System memory remap. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn map(&mut self) -> _MAPW {
        _MAPW { w: self }
    }
}
