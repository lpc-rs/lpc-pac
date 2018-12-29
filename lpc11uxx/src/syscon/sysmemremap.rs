#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSMEMREMAP {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `MAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAPR {
    #[doc = "Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    BOOT_LOADER_MODE_IN,
    #[doc = "User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    USER_RAM_MODE_INTER,
    #[doc = "User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    USER_FLASH_MODE_INT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MAPR::BOOT_LOADER_MODE_IN => 0,
            MAPR::USER_RAM_MODE_INTER => 1,
            MAPR::USER_FLASH_MODE_INT => 2,
            MAPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MAPR {
        match value {
            0 => MAPR::BOOT_LOADER_MODE_IN,
            1 => MAPR::USER_RAM_MODE_INTER,
            2 => MAPR::USER_FLASH_MODE_INT,
            i => MAPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_LOADER_MODE_IN`"]
    #[inline]
    pub fn is_boot_loader_mode_in(&self) -> bool {
        *self == MAPR::BOOT_LOADER_MODE_IN
    }
    #[doc = "Checks if the value of the field is `USER_RAM_MODE_INTER`"]
    #[inline]
    pub fn is_user_ram_mode_inter(&self) -> bool {
        *self == MAPR::USER_RAM_MODE_INTER
    }
    #[doc = "Checks if the value of the field is `USER_FLASH_MODE_INT`"]
    #[inline]
    pub fn is_user_flash_mode_int(&self) -> bool {
        *self == MAPR::USER_FLASH_MODE_INT
    }
}
#[doc = "Values that can be written to the field `MAP`"]
pub enum MAPW {
    #[doc = "Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    BOOT_LOADER_MODE_IN,
    #[doc = "User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    USER_RAM_MODE_INTER,
    #[doc = "User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    USER_FLASH_MODE_INT,
}
impl MAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MAPW::BOOT_LOADER_MODE_IN => 0,
            MAPW::USER_RAM_MODE_INTER => 1,
            MAPW::USER_FLASH_MODE_INT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAPW<'a> {
    w: &'a mut W,
}
impl<'a> _MAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Boot Loader Mode. Interrupt vectors are re-mapped to Boot ROM."]
    #[inline]
    pub fn boot_loader_mode_in(self) -> &'a mut W {
        self.variant(MAPW::BOOT_LOADER_MODE_IN)
    }
    #[doc = "User RAM Mode. Interrupt vectors are re-mapped to Static RAM."]
    #[inline]
    pub fn user_ram_mode_inter(self) -> &'a mut W {
        self.variant(MAPW::USER_RAM_MODE_INTER)
    }
    #[doc = "User Flash Mode. Interrupt vectors are not re-mapped and reside in Flash."]
    #[inline]
    pub fn user_flash_mode_int(self) -> &'a mut W {
        self.variant(MAPW::USER_FLASH_MODE_INT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - System memory remap. Value 0x3 is reserved."]
    #[inline]
    pub fn map(&self) -> MAPR {
        MAPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - System memory remap. Value 0x3 is reserved."]
    #[inline]
    pub fn map(&mut self) -> _MAPW {
        _MAPW { w: self }
    }
}
