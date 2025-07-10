#include <stdint.h>

int main(void);

extern uint32_t _sidata; // Source address of .data in Flash
extern uint32_t _sdata;  // Start address of .data in RAM
extern uint32_t _edata;  // End address of .data
extern uint32_t _sbss;   // Start address of .bss
extern uint32_t _ebss;   // End address of .bss

extern void tim7_interrupt_handler();

void default_handler(void) {
  while (1)
    ;
}

void unhandled_interrupt(void) __attribute__((weak, alias("default_handler")));

// Macro for creating weak aliases
#define WEAK_ALIAS(fn)                                                         \
  void fn(void) __attribute__((weak, alias("default_handler")));

// List of all required handlers
WEAK_ALIAS(WWDG);
WEAK_ALIAS(PVD);
WEAK_ALIAS(TAMP_STAMP);
WEAK_ALIAS(RTC_WKUP);
WEAK_ALIAS(FLASH);
WEAK_ALIAS(RCC);
WEAK_ALIAS(EXTI0);
WEAK_ALIAS(EXTI1);
WEAK_ALIAS(EXTI2);
WEAK_ALIAS(EXTI3);
WEAK_ALIAS(EXTI4);
WEAK_ALIAS(DMA1_STREAM0);
WEAK_ALIAS(DMA1_STREAM1);
WEAK_ALIAS(DMA1_STREAM2);
WEAK_ALIAS(DMA1_STREAM3);
WEAK_ALIAS(DMA1_STREAM4);
WEAK_ALIAS(DMA1_STREAM5);
WEAK_ALIAS(DMA1_STREAM6);
WEAK_ALIAS(ADC);
WEAK_ALIAS(CAN1_TX);
WEAK_ALIAS(CAN1_RX0);
WEAK_ALIAS(CAN1_RX1);
WEAK_ALIAS(CAN1_SCE);
WEAK_ALIAS(EXTI9_5);
WEAK_ALIAS(TIM1_BRK_TIM9);
WEAK_ALIAS(TIM1_UP_TIM10);
WEAK_ALIAS(TIM1_TRG_COM_TIM11);
WEAK_ALIAS(TIM1_CC);
WEAK_ALIAS(TIM2);
WEAK_ALIAS(TIM3);
WEAK_ALIAS(TIM4);
WEAK_ALIAS(I2C1_EV);
WEAK_ALIAS(I2C1_ER);
WEAK_ALIAS(I2C2_EV);
WEAK_ALIAS(I2C2_ER);
WEAK_ALIAS(SPI1);
WEAK_ALIAS(SPI2);
WEAK_ALIAS(USART1);
WEAK_ALIAS(USART2);
WEAK_ALIAS(USART3);
WEAK_ALIAS(EXTI15_10);
WEAK_ALIAS(RTC_ALARM);
WEAK_ALIAS(OTG_FS_WKUP);
WEAK_ALIAS(TIM8_BRK_TIM12);
WEAK_ALIAS(TIM8_UP_TIM13);
WEAK_ALIAS(TIM8_TRG_COM_TIM14);
WEAK_ALIAS(TIM8_CC);
WEAK_ALIAS(DMA1_STREAM7);
WEAK_ALIAS(FMC);
WEAK_ALIAS(SDIO);
WEAK_ALIAS(TIM5);
WEAK_ALIAS(SPI3);
WEAK_ALIAS(UART4);
WEAK_ALIAS(UART5);
WEAK_ALIAS(TIM6_DAC);
WEAK_ALIAS(TIM7);
WEAK_ALIAS(DMA2_STREAM0);
WEAK_ALIAS(DMA2_STREAM1);
WEAK_ALIAS(DMA2_STREAM2);
WEAK_ALIAS(DMA2_STREAM3);
WEAK_ALIAS(DMA2_STREAM4);
WEAK_ALIAS(ETH);
WEAK_ALIAS(ETH_WKUP);
WEAK_ALIAS(CAN2_TX);
WEAK_ALIAS(CAN2_RX0);
WEAK_ALIAS(CAN2_RX1);
WEAK_ALIAS(CAN2_SCE);
WEAK_ALIAS(OTG_FS);
WEAK_ALIAS(DMA2_STREAM5);
WEAK_ALIAS(DMA2_STREAM6);
WEAK_ALIAS(DMA2_STREAM7);
WEAK_ALIAS(USART6);
WEAK_ALIAS(I2C3_EV);
WEAK_ALIAS(I2C3_ER);
WEAK_ALIAS(OTG_HS_EP1_OUT);
WEAK_ALIAS(OTG_HS_EP1_IN);
WEAK_ALIAS(OTG_HS_WKUP);
WEAK_ALIAS(OTG_HS);
WEAK_ALIAS(DCMI);
WEAK_ALIAS(CRYP);
WEAK_ALIAS(HASH_RNG);
WEAK_ALIAS(FPU);
WEAK_ALIAS(UART7);
WEAK_ALIAS(UART8);
WEAK_ALIAS(SPI4);
WEAK_ALIAS(SPI5);
WEAK_ALIAS(SPI6);
WEAK_ALIAS(SAI1);
WEAK_ALIAS(LCD_TFT);
WEAK_ALIAS(LCD_TFT_1);
WEAK_ALIAS(DMA2D);

// Main entry
// It's where the whole system starts.
void __attribute__((naked)) reset_handler(void) {
  // Copy .data from Flash to RAM (init data)
  uint32_t *idata_begin = &_sidata;
  uint32_t *data_begin = &_sdata;
  uint32_t *data_end = &_edata;
  while (data_begin < data_end) {
    *data_begin++ = *idata_begin++;
  }

  // Reset .bss (zero fill the bss segment)
  uint32_t *bss_begin = &_sbss;
  uint32_t *bss_end = &_ebss;
  while (bss_begin < bss_end) {
    *bss_begin++ = 0;
  }

  // Conditionally enable the FPU
  __asm__("ldr r0, =0xE000ED88\n" // Address of SCB.CPACR

          // Enable access to CP10 and CP11 from both privileged and
          // unprivileged mode
          "ldr r1, =(0b1111 << 20)\n"
          // RMW
          "ldr r2, [r0]\n"
          "orr r2, r2, r1\n"
          "str r2, [r0]\n"

          // Barrier is required on some processors
          "dsb sy\n"
          "isb sy\n");

  main();

  while (1)
    ; // Should not be achieved
}

// interrupt vector that will be linked to the very start of FLASH
void *const g_pfnVectors[] __attribute__((section(".isr_vector"), used)) = {
    reset_handler,       // Reset Handler
    unhandled_interrupt, // NMI Handler
    unhandled_interrupt, // Hard Fault Handler
    unhandled_interrupt, // Memory Manage Handler
    unhandled_interrupt, // Bus Fault Handler
    unhandled_interrupt, // Usage Fault Handler
    0,
    0,
    0,
    0,                   // Reserved
    unhandled_interrupt, // SVCall Handler
    0,                   // Debug Monitor Handler
    0,                   // Reserved
    unhandled_interrupt, // PendSV Handler
    unhandled_interrupt  // SysTick Handler
};
