#[doc = "Register `SYSMEMREMAP` reader"]
pub struct R(crate::R<SYSMEMREMAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSMEMREMAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSMEMREMAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSMEMREMAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSMEMREMAP` writer"]
pub struct W(crate::W<SYSMEMREMAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSMEMREMAP_SPEC>;
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
impl From<crate::W<SYSMEMREMAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSMEMREMAP_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `MAP` reader - System memory remap. Value 0x3 is reserved."]
pub struct MAP_R(crate::FieldReader<u8, MAP_A>);
impl MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAP_A> {
        match self.bits {
            0 => Some(MAP_A::BOOT_LOADER_MODE),
            1 => Some(MAP_A::USER_RAM_MODE),
            2 => Some(MAP_A::USER_FLASH_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_LOADER_MODE`"]
    #[inline(always)]
    pub fn is_boot_loader_mode(&self) -> bool {
        **self == MAP_A::BOOT_LOADER_MODE
    }
    #[doc = "Checks if the value of the field is `USER_RAM_MODE`"]
    #[inline(always)]
    pub fn is_user_ram_mode(&self) -> bool {
        **self == MAP_A::USER_RAM_MODE
    }
    #[doc = "Checks if the value of the field is `USER_FLASH_MODE`"]
    #[inline(always)]
    pub fn is_user_flash_mode(&self) -> bool {
        **self == MAP_A::USER_FLASH_MODE
    }
}
impl core::ops::Deref for MAP_R {
    type Target = crate::FieldReader<u8, MAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAP` writer - System memory remap. Value 0x3 is reserved."]
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysmemremap](index.html) module"]
pub struct SYSMEMREMAP_SPEC;
impl crate::RegisterSpec for SYSMEMREMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysmemremap::R](R) reader structure"]
impl crate::Readable for SYSMEMREMAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysmemremap::W](W) writer structure"]
impl crate::Writable for SYSMEMREMAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSMEMREMAP to value 0"]
impl crate::Resettable for SYSMEMREMAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
