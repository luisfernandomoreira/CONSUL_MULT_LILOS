#[doc = "Register `AHB2RST` reader"]
pub type R = crate::R<Ahb2rstSpec>;
#[doc = "Register `AHB2RST` writer"]
pub type W = crate::W<Ahb2rstSpec>;
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub type ParstR = crate::BitReader;
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub type ParstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub type PbrstR = crate::BitReader;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub type PbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub type PcrstR = crate::BitReader;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub type PcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRST` reader - GPIO port D reset"]
pub type PdrstR = crate::BitReader;
#[doc = "Field `PDRST` writer - GPIO port D reset"]
pub type PdrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFRST` reader - GPIO port F reset"]
pub type PfrstR = crate::BitReader;
#[doc = "Field `PFRST` writer - GPIO port F reset"]
pub type PfrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&self) -> ParstR {
        ParstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&self) -> PbrstR {
        PbrstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&self) -> PcrstR {
        PcrstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO port D reset"]
    #[inline(always)]
    pub fn pdrst(&self) -> PdrstR {
        PdrstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO port F reset"]
    #[inline(always)]
    pub fn pfrst(&self) -> PfrstR {
        PfrstR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&mut self) -> ParstW<'_, Ahb2rstSpec> {
        ParstW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&mut self) -> PbrstW<'_, Ahb2rstSpec> {
        PbrstW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&mut self) -> PcrstW<'_, Ahb2rstSpec> {
        PcrstW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO port D reset"]
    #[inline(always)]
    pub fn pdrst(&mut self) -> PdrstW<'_, Ahb2rstSpec> {
        PdrstW::new(self, 20)
    }
    #[doc = "Bit 22 - GPIO port F reset"]
    #[inline(always)]
    pub fn pfrst(&mut self) -> PfrstW<'_, Ahb2rstSpec> {
        PfrstW::new(self, 22)
    }
}
#[doc = "AHB2 reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2rstSpec;
impl crate::RegisterSpec for Ahb2rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rst::R`](R) reader structure"]
impl crate::Readable for Ahb2rstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2rst::W`](W) writer structure"]
impl crate::Writable for Ahb2rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2RST to value 0"]
impl crate::Resettable for Ahb2rstSpec {}
