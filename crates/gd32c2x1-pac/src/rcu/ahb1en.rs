#[doc = "Register `AHB1EN` reader"]
pub type R = crate::R<Ahb1enSpec>;
#[doc = "Register `AHB1EN` writer"]
pub type W = crate::W<Ahb1enSpec>;
#[doc = "Field `FMCEN` reader - FMC clock enable"]
pub type FmcenR = crate::BitReader;
#[doc = "Field `FMCEN` writer - FMC clock enable"]
pub type FmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA clock enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA clock enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUXEN` reader - DMAMUX clock enable"]
pub type DmamuxenR = crate::BitReader;
#[doc = "Field `DMAMUXEN` writer - DMAMUX clock enable"]
pub type DmamuxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - FMC clock enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FmcenR {
        FmcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA clock enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAMUX clock enable"]
    #[inline(always)]
    pub fn dmamuxen(&self) -> DmamuxenR {
        DmamuxenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - FMC clock enable"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FmcenW<'_, Ahb1enSpec> {
        FmcenW::new(self, 4)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, Ahb1enSpec> {
        CrcenW::new(self, 12)
    }
    #[doc = "Bit 21 - DMA clock enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, Ahb1enSpec> {
        DmaenW::new(self, 21)
    }
    #[doc = "Bit 23 - DMAMUX clock enable"]
    #[inline(always)]
    pub fn dmamuxen(&mut self) -> DmamuxenW<'_, Ahb1enSpec> {
        DmamuxenW::new(self, 23)
    }
}
#[doc = "AHB1 enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1enSpec;
impl crate::RegisterSpec for Ahb1enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1en::R`](R) reader structure"]
impl crate::Readable for Ahb1enSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1en::W`](W) writer structure"]
impl crate::Writable for Ahb1enSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1EN to value 0x10"]
impl crate::Resettable for Ahb1enSpec {
    const RESET_VALUE: u32 = 0x10;
}
