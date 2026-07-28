#[doc = "Register `TASK_CON` reader"]
pub type R = crate::R<TaskConSpec>;
#[doc = "Register `TASK_CON` writer"]
pub type W = crate::W<TaskConSpec>;
#[doc = "Field `TASK_NUMBER` reader - 要执行的 task 总数"]
pub type TaskNumberR = crate::FieldReader<u16>;
#[doc = "Field `TASK_NUMBER` writer - 要执行的 task 总数"]
pub type TaskNumberW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TASK_PP_EN` reader - Ping-pong 模式使能。0：关闭，第二组寄存器在第一组 task 完成后才取；1：开启，第二组寄存器在第一组取完后立即开始取"]
pub type TaskPpEnR = crate::BitReader;
#[doc = "Field `TASK_PP_EN` writer - Ping-pong 模式使能。0：关闭，第二组寄存器在第一组 task 完成后才取；1：开启，第二组寄存器在第一组取完后立即开始取"]
pub type TaskPpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_COUNT_CLEAR` reader - 任务计数器清除。清除当前 task 计数器，建议在 task 启动前清除"]
pub type TaskCountClearR = crate::BitReader;
#[doc = "Field `TASK_COUNT_CLEAR` writer - 任务计数器清除。清除当前 task 计数器，建议在 task 启动前清除"]
pub type TaskCountClearW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 要执行的 task 总数"]
    #[inline(always)]
    pub fn task_number(&self) -> TaskNumberR {
        TaskNumberR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Ping-pong 模式使能。0：关闭，第二组寄存器在第一组 task 完成后才取；1：开启，第二组寄存器在第一组取完后立即开始取"]
    #[inline(always)]
    pub fn task_pp_en(&self) -> TaskPpEnR {
        TaskPpEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 任务计数器清除。清除当前 task 计数器，建议在 task 启动前清除"]
    #[inline(always)]
    pub fn task_count_clear(&self) -> TaskCountClearR {
        TaskCountClearR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 要执行的 task 总数"]
    #[inline(always)]
    pub fn task_number(&mut self) -> TaskNumberW<'_, TaskConSpec> {
        TaskNumberW::new(self, 0)
    }
    #[doc = "Bit 12 - Ping-pong 模式使能。0：关闭，第二组寄存器在第一组 task 完成后才取；1：开启，第二组寄存器在第一组取完后立即开始取"]
    #[inline(always)]
    pub fn task_pp_en(&mut self) -> TaskPpEnW<'_, TaskConSpec> {
        TaskPpEnW::new(self, 12)
    }
    #[doc = "Bit 13 - 任务计数器清除。清除当前 task 计数器，建议在 task 启动前清除"]
    #[inline(always)]
    pub fn task_count_clear(&mut self) -> TaskCountClearW<'_, TaskConSpec> {
        TaskCountClearW::new(self, 13)
    }
}
#[doc = "task_con\n\nYou can [`read`](crate::Reg::read) this register and get [`task_con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaskConSpec;
impl crate::RegisterSpec for TaskConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_con::R`](R) reader structure"]
impl crate::Readable for TaskConSpec {}
#[doc = "`write(|w| ..)` method takes [`task_con::W`](W) writer structure"]
impl crate::Writable for TaskConSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x2000;
}
#[doc = "`reset()` method sets TASK_CON to value 0"]
impl crate::Resettable for TaskConSpec {}
