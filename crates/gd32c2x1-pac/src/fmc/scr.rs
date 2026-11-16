#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `SCR_PAGE_CNT` reader - Configure the number of pages in secure user area."]
pub type ScrPageCntR = crate::FieldReader;
#[doc = "Field `SCR_PAGE_CNT` writer - Configure the number of pages in secure user area."]
pub type ScrPageCntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BOOTLK` reader - This bit is set to force boot from user Flash area."]
pub type BootlkR = crate::BitReader;
#[doc = "Field `BOOTLK` writer - This bit is set to force boot from user Flash area."]
pub type BootlkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Configure the number of pages in secure user area."]
    #[inline(always)]
    pub fn scr_page_cnt(&self) -> ScrPageCntR {
        ScrPageCntR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - This bit is set to force boot from user Flash area."]
    #[inline(always)]
    pub fn bootlk(&self) -> BootlkR {
        BootlkR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Configure the number of pages in secure user area."]
    #[inline(always)]
    pub fn scr_page_cnt(&mut self) -> ScrPageCntW<'_, ScrSpec> {
        ScrPageCntW::new(self, 0)
    }
    #[doc = "Bit 16 - This bit is set to force boot from user Flash area."]
    #[inline(always)]
    pub fn bootlk(&mut self) -> BootlkW<'_, ScrSpec> {
        BootlkW::new(self, 16)
    }
}
#[doc = "Secure user area register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {}
