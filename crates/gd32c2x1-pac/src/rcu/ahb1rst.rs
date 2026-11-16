#[doc = "Register `AHB1RST` reader"]
pub type R = crate::R<Ahb1rstSpec>;
#[doc = "Register `AHB1RST` writer"]
pub type W = crate::W<Ahb1rstSpec>;
#[doc = "Field `CRCRST` reader - CRC Reset"]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC Reset"]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMARST` reader - DMA reset"]
pub type DmarstR = crate::BitReader;
#[doc = "Field `DMARST` writer - DMA reset"]
pub type DmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAMUXRST` reader - DMAMUX reset"]
pub type DmamuxrstR = crate::BitReader;
#[doc = "Field `DMAMUXRST` writer - DMAMUX reset"]
pub type DmamuxrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - CRC Reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA reset"]
    #[inline(always)]
    pub fn dmarst(&self) -> DmarstR {
        DmarstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAMUX reset"]
    #[inline(always)]
    pub fn dmamuxrst(&self) -> DmamuxrstR {
        DmamuxrstR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - CRC Reset"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CrcrstW<'_, Ahb1rstSpec> {
        CrcrstW::new(self, 12)
    }
    #[doc = "Bit 21 - DMA reset"]
    #[inline(always)]
    pub fn dmarst(&mut self) -> DmarstW<'_, Ahb1rstSpec> {
        DmarstW::new(self, 21)
    }
    #[doc = "Bit 23 - DMAMUX reset"]
    #[inline(always)]
    pub fn dmamuxrst(&mut self) -> DmamuxrstW<'_, Ahb1rstSpec> {
        DmamuxrstW::new(self, 23)
    }
}
#[doc = "AHB1 reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1rstSpec;
impl crate::RegisterSpec for Ahb1rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rst::R`](R) reader structure"]
impl crate::Readable for Ahb1rstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1rst::W`](W) writer structure"]
impl crate::Writable for Ahb1rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1RST to value 0"]
impl crate::Resettable for Ahb1rstSpec {}
