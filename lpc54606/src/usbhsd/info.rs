#[doc = "Reader of register INFO"]
pub type R = crate::R<u32, super::INFO>;
#[doc = "Reader of field `FRAME_NR`"]
pub type FRAME_NR_R = crate::R<u16, u16>;
#[doc = "Reader of field `ERR_CODE`"]
pub type ERR_CODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `Minrev`"]
pub type MINREV_R = crate::R<u8, u8>;
#[doc = "Reader of field `Majrev`"]
pub type MAJREV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:10 - Frame number."]
    #[inline(always)]
    pub fn frame_nr(&self) -> FRAME_NR_R {
        FRAME_NR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - The error code which last occurred:."]
    #[inline(always)]
    pub fn err_code(&self) -> ERR_CODE_R {
        ERR_CODE_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Minor revision."]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major revision."]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
