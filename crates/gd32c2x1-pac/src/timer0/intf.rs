#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Register `INTF` writer"]
pub type W = crate::W<IntfSpec>;
#[doc = "Field `UPIF` reader - Update interrupt flag"]
pub type UpifR = crate::BitReader;
#[doc = "Field `UPIF` writer - Update interrupt flag"]
pub type UpifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0IF` reader - Channel 0 capture/compare interrupt flag"]
pub type Ch0ifR = crate::BitReader;
#[doc = "Field `CH0IF` writer - Channel 0 capture/compare interrupt flag"]
pub type Ch0ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1IF` reader - Channel 1 capture/compare interrupt flag"]
pub type Ch1ifR = crate::BitReader;
#[doc = "Field `CH1IF` writer - Channel 1 capture/compare interrupt flag"]
pub type Ch1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2IF` reader - Channel 2 capture/compare interrupt flag"]
pub type Ch2ifR = crate::BitReader;
#[doc = "Field `CH2IF` writer - Channel 2 capture/compare interrupt flag"]
pub type Ch2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3IF` reader - Channel 3 capture/compare interrupt flag"]
pub type Ch3ifR = crate::BitReader;
#[doc = "Field `CH3IF` writer - Channel 3 capture/compare interrupt flag"]
pub type Ch3ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMTIF` reader - Channel commutation interrupt flag"]
pub type CmtifR = crate::BitReader;
#[doc = "Field `CMTIF` writer - Channel commutation interrupt flag"]
pub type CmtifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TrgifR = crate::BitReader;
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TrgifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK0IF` reader - BREAK0 interrupt flag"]
pub type Brk0ifR = crate::BitReader;
#[doc = "Field `BRK0IF` writer - BREAK0 interrupt flag"]
pub type Brk0ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK1IF` reader - BREAK1 interrupt flag"]
pub type Brk1ifR = crate::BitReader;
#[doc = "Field `BRK1IF` writer - BREAK1 interrupt flag"]
pub type Brk1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OF` reader - Channel 0 over capture flag"]
pub type Ch0ofR = crate::BitReader;
#[doc = "Field `CH0OF` writer - Channel 0 over capture flag"]
pub type Ch0ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OF` reader - Channel 1 over capture flag"]
pub type Ch1ofR = crate::BitReader;
#[doc = "Field `CH1OF` writer - Channel 1 over capture flag"]
pub type Ch1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2OF` reader - Channel 2 over capture flag"]
pub type Ch2ofR = crate::BitReader;
#[doc = "Field `CH2OF` writer - Channel 2 over capture flag"]
pub type Ch2ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3OF` reader - Channel 3 over capture flag"]
pub type Ch3ofR = crate::BitReader;
#[doc = "Field `CH3OF` writer - Channel 3 over capture flag"]
pub type Ch3ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBIF` reader - System source break interrupt flag"]
pub type SysbifR = crate::BitReader;
#[doc = "Field `SYSBIF` writer - System source break interrupt flag"]
pub type SysbifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4IF` reader - Channel 4 capture/compare interrupt flag"]
pub type Ch4ifR = crate::BitReader;
#[doc = "Field `CH4IF` writer - Channel 4 capture/compare interrupt flag"]
pub type Ch4ifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&self) -> UpifR {
        UpifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&self) -> Ch0ifR {
        Ch0ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch1if(&self) -> Ch1ifR {
        Ch1ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch2if(&self) -> Ch2ifR {
        Ch2ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch3if(&self) -> Ch3ifR {
        Ch3ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&self) -> CmtifR {
        CmtifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TrgifR {
        TrgifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BREAK0 interrupt flag"]
    #[inline(always)]
    pub fn brk0if(&self) -> Brk0ifR {
        Brk0ifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BREAK1 interrupt flag"]
    #[inline(always)]
    pub fn brk1if(&self) -> Brk1ifR {
        Brk1ifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 over capture flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> Ch0ofR {
        Ch0ofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 over capture flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> Ch1ofR {
        Ch1ofR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 over capture flag"]
    #[inline(always)]
    pub fn ch2of(&self) -> Ch2ofR {
        Ch2ofR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 over capture flag"]
    #[inline(always)]
    pub fn ch3of(&self) -> Ch3ofR {
        Ch3ofR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - System source break interrupt flag"]
    #[inline(always)]
    pub fn sysbif(&self) -> SysbifR {
        SysbifR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 4 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch4if(&self) -> Ch4ifR {
        Ch4ifR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn upif(&mut self) -> UpifW<'_, IntfSpec> {
        UpifW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch0if(&mut self) -> Ch0ifW<'_, IntfSpec> {
        Ch0ifW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch1if(&mut self) -> Ch1ifW<'_, IntfSpec> {
        Ch1ifW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch2if(&mut self) -> Ch2ifW<'_, IntfSpec> {
        Ch2ifW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch3if(&mut self) -> Ch3ifW<'_, IntfSpec> {
        Ch3ifW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel commutation interrupt flag"]
    #[inline(always)]
    pub fn cmtif(&mut self) -> CmtifW<'_, IntfSpec> {
        CmtifW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&mut self) -> TrgifW<'_, IntfSpec> {
        TrgifW::new(self, 6)
    }
    #[doc = "Bit 7 - BREAK0 interrupt flag"]
    #[inline(always)]
    pub fn brk0if(&mut self) -> Brk0ifW<'_, IntfSpec> {
        Brk0ifW::new(self, 7)
    }
    #[doc = "Bit 8 - BREAK1 interrupt flag"]
    #[inline(always)]
    pub fn brk1if(&mut self) -> Brk1ifW<'_, IntfSpec> {
        Brk1ifW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 0 over capture flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> Ch0ofW<'_, IntfSpec> {
        Ch0ofW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 1 over capture flag"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> Ch1ofW<'_, IntfSpec> {
        Ch1ofW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 2 over capture flag"]
    #[inline(always)]
    pub fn ch2of(&mut self) -> Ch2ofW<'_, IntfSpec> {
        Ch2ofW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 3 over capture flag"]
    #[inline(always)]
    pub fn ch3of(&mut self) -> Ch3ofW<'_, IntfSpec> {
        Ch3ofW::new(self, 12)
    }
    #[doc = "Bit 13 - System source break interrupt flag"]
    #[inline(always)]
    pub fn sysbif(&mut self) -> SysbifW<'_, IntfSpec> {
        SysbifW::new(self, 13)
    }
    #[doc = "Bit 16 - Channel 4 capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ch4if(&mut self) -> Ch4ifW<'_, IntfSpec> {
        Ch4ifW::new(self, 16)
    }
}
#[doc = "Interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`write(|w| ..)` method takes [`intf::W`](W) writer structure"]
impl crate::Writable for IntfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {}
