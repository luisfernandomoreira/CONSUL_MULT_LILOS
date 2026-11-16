#[doc = "Register `AHB2EN` reader"]
pub type R = crate::R<Ahb2enSpec>;
#[doc = "Register `AHB2EN` writer"]
pub type W = crate::W<Ahb2enSpec>;
#[doc = "Field `PAEN` reader - GPIO port A clock enable"]
pub type PaenR = crate::BitReader;
#[doc = "Field `PAEN` writer - GPIO port A clock enable"]
pub type PaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBEN` reader - GPIO port B clock enable"]
pub type PbenR = crate::BitReader;
#[doc = "Field `PBEN` writer - GPIO port B clock enable"]
pub type PbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCEN` reader - GPIO port C clock enable"]
pub type PcenR = crate::BitReader;
#[doc = "Field `PCEN` writer - GPIO port C clock enable"]
pub type PcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - GPIO port D clock enable"]
pub type PdenR = crate::BitReader;
#[doc = "Field `PDEN` writer - GPIO port D clock enable"]
pub type PdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFEN` reader - GPIO port F clock enable"]
pub type PfenR = crate::BitReader;
#[doc = "Field `PFEN` writer - GPIO port F clock enable"]
pub type PfenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&self) -> PaenR {
        PaenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&self) -> PbenR {
        PbenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PcenR {
        PcenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&self) -> PdenR {
        PdenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PfenR {
        PfenR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&mut self) -> PaenW<'_, Ahb2enSpec> {
        PaenW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&mut self) -> PbenW<'_, Ahb2enSpec> {
        PbenW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PcenW<'_, Ahb2enSpec> {
        PcenW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PdenW<'_, Ahb2enSpec> {
        PdenW::new(self, 20)
    }
    #[doc = "Bit 22 - GPIO port F clock enable"]
    #[inline(always)]
    pub fn pfen(&mut self) -> PfenW<'_, Ahb2enSpec> {
        PfenW::new(self, 22)
    }
}
#[doc = "AHB2 enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2enSpec;
impl crate::RegisterSpec for Ahb2enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2en::R`](R) reader structure"]
impl crate::Readable for Ahb2enSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2en::W`](W) writer structure"]
impl crate::Writable for Ahb2enSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2EN to value 0"]
impl crate::Resettable for Ahb2enSpec {}
