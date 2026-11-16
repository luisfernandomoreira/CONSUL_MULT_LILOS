#[doc = "Register `AHB1SPDPEN` reader"]
pub type R = crate::R<Ahb1spdpenSpec>;
#[doc = "Register `AHB1SPDPEN` writer"]
pub type W = crate::W<Ahb1spdpenSpec>;
#[doc = "Field `SRAMSPDPEN` reader - SRAM interface clock enable when sleep and deep sleep mode"]
pub type SramspdpenR = crate::BitReader;
#[doc = "Field `SRAMSPDPEN` writer - SRAM interface clock enable when sleep and deep sleep mode"]
pub type SramspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCSPDPEN` reader - FMC clock enable when sleep and deep sleep mode"]
pub type FmcspdpenR = crate::BitReader;
#[doc = "Field `FMCSPDPEN` writer - FMC clock enable when sleep and deep sleep mode"]
pub type FmcspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSPDPEN` reader - CRC clock enable when sleep and deep sleep mode"]
pub type CrcspdpenR = crate::BitReader;
#[doc = "Field `CRCSPDPEN` writer - CRC clock enable when sleep and deep sleep mode"]
pub type CrcspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASPDPEN` reader - DMA clock enable when sleep and deep sleep mode"]
pub type DmaspdpenR = crate::BitReader;
#[doc = "Field `DMASPDPEN` writer - DMA clock enable when sleep and deep sleep mode"]
pub type DmaspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUXSPDPEN` reader - DMAMUX clock enable when sleep and deep sleep mode"]
pub type DmamuxspdpenR = crate::BitReader;
#[doc = "Field `DMAMUXSPDPEN` writer - DMAMUX clock enable when sleep and deep sleep mode"]
pub type DmamuxspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - SRAM interface clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn sramspdpen(&self) -> SramspdpenR {
        SramspdpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FMC clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn fmcspdpen(&self) -> FmcspdpenR {
        FmcspdpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn crcspdpen(&self) -> CrcspdpenR {
        CrcspdpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn dmaspdpen(&self) -> DmaspdpenR {
        DmaspdpenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAMUX clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn dmamuxspdpen(&self) -> DmamuxspdpenR {
        DmamuxspdpenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SRAM interface clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn sramspdpen(&mut self) -> SramspdpenW<'_, Ahb1spdpenSpec> {
        SramspdpenW::new(self, 2)
    }
    #[doc = "Bit 4 - FMC clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn fmcspdpen(&mut self) -> FmcspdpenW<'_, Ahb1spdpenSpec> {
        FmcspdpenW::new(self, 4)
    }
    #[doc = "Bit 12 - CRC clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn crcspdpen(&mut self) -> CrcspdpenW<'_, Ahb1spdpenSpec> {
        CrcspdpenW::new(self, 12)
    }
    #[doc = "Bit 21 - DMA clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn dmaspdpen(&mut self) -> DmaspdpenW<'_, Ahb1spdpenSpec> {
        DmaspdpenW::new(self, 21)
    }
    #[doc = "Bit 23 - DMAMUX clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn dmamuxspdpen(&mut self) -> DmamuxspdpenW<'_, Ahb1spdpenSpec> {
        DmamuxspdpenW::new(self, 23)
    }
}
#[doc = "AHB1 sleep and deep sleep mode enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1spdpen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1spdpen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1spdpenSpec;
impl crate::RegisterSpec for Ahb1spdpenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1spdpen::R`](R) reader structure"]
impl crate::Readable for Ahb1spdpenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1spdpen::W`](W) writer structure"]
impl crate::Writable for Ahb1spdpenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1SPDPEN to value 0x00a0_1014"]
impl crate::Resettable for Ahb1spdpenSpec {
    const RESET_VALUE: u32 = 0x00a0_1014;
}
