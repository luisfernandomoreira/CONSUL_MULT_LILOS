#[doc = "Register `AHB2SPDPEN` reader"]
pub type R = crate::R<Ahb2spdpenSpec>;
#[doc = "Register `AHB2SPDPEN` writer"]
pub type W = crate::W<Ahb2spdpenSpec>;
#[doc = "Field `PASPDPEN` reader - GPIO port A clock enable when sleep and deep sleep mode"]
pub type PaspdpenR = crate::BitReader;
#[doc = "Field `PASPDPEN` writer - GPIO port A clock enable when sleep and deep sleep mode"]
pub type PaspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBSPDPEN` reader - GPIO port B clock enable when sleep and deep sleep mode"]
pub type PbspdpenR = crate::BitReader;
#[doc = "Field `PBSPDPEN` writer - GPIO port B clock enable when sleep and deep sleep mode"]
pub type PbspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSPDPEN` reader - GPIO port C clock enable when sleep and deep sleep mode"]
pub type PcspdpenR = crate::BitReader;
#[doc = "Field `PCSPDPEN` writer - GPIO port C clock enable when sleep and deep sleep mode"]
pub type PcspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDSPDPEN` reader - GPIO port D clock enable when sleep and deep sleep mode"]
pub type PdspdpenR = crate::BitReader;
#[doc = "Field `PDSPDPEN` writer - GPIO port D clock enable when sleep and deep sleep mode"]
pub type PdspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFSPDPEN` reader - GPIO port F clock enable when sleep and deep sleep mode"]
pub type PfspdpenR = crate::BitReader;
#[doc = "Field `PFSPDPEN` writer - GPIO port F clock enable when sleep and deep sleep mode"]
pub type PfspdpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - GPIO port A clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn paspdpen(&self) -> PaspdpenR {
        PaspdpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO port B clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn pbspdpen(&self) -> PbspdpenR {
        PbspdpenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO port C clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn pcspdpen(&self) -> PcspdpenR {
        PcspdpenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO port D clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn pdspdpen(&self) -> PdspdpenR {
        PdspdpenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO port F clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn pfspdpen(&self) -> PfspdpenR {
        PfspdpenR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - GPIO port A clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn paspdpen(&mut self) -> PaspdpenW<'_, Ahb2spdpenSpec> {
        PaspdpenW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO port B clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn pbspdpen(&mut self) -> PbspdpenW<'_, Ahb2spdpenSpec> {
        PbspdpenW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO port C clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn pcspdpen(&mut self) -> PcspdpenW<'_, Ahb2spdpenSpec> {
        PcspdpenW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO port D clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn pdspdpen(&mut self) -> PdspdpenW<'_, Ahb2spdpenSpec> {
        PdspdpenW::new(self, 20)
    }
    #[doc = "Bit 22 - GPIO port F clock enable when sleep and deep sleep mode"]
    #[inline(always)]
    pub fn pfspdpen(&mut self) -> PfspdpenW<'_, Ahb2spdpenSpec> {
        PfspdpenW::new(self, 22)
    }
}
#[doc = "AHB2 sleep and deep sleep mode enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2spdpen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2spdpen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2spdpenSpec;
impl crate::RegisterSpec for Ahb2spdpenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2spdpen::R`](R) reader structure"]
impl crate::Readable for Ahb2spdpenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2spdpen::W`](W) writer structure"]
impl crate::Writable for Ahb2spdpenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2SPDPEN to value 0x005e_0000"]
impl crate::Resettable for Ahb2spdpenSpec {
    const RESET_VALUE: u32 = 0x005e_0000;
}
