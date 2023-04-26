# svd2cpp

## 介绍
### SVD

CMSIS 系统视图描述格式 (CMSIS-SVD) 形式化了基于 Arm Cortex-M 处理器的微控制器中包含的系统的描述，特别是外围设备的内存映射寄存器。系统视图描述中包含的详细信息与设备参考手册中的数据相当。信息范围从外设的高级功能描述一直到内存映射寄存器中单个位字段的定义和用途。

CMSIS-SVD 文件由芯片供应商开发和维护。芯片供应商将他们的描述作为 CMSIS 设备系列包的一部分分发。工具供应商使用 CMSIS-SVD 文件在其调试器中提供特定于设备的外围设备调试视图。最后但同样重要的是，CMSIS 兼容的设备头文件是从 CMSIS-SVD 文件生成的。
### PAC
PAC: Peripheral access control, 用于抽象芯片底层寄存器的操作接口，根据外设寄存器的读写权限、位域的宽度和读写权限生成相应的接口，提供给 HAL 层调用。

## 使用

### 编译并执行
```bash
# 编译rust代码，然后执行转换芯片 svd 文件到 pac 接口
cargo run test/EFR32MG1B.svd
```

### ubuntu/deepin上执行执行
```bash
# 转换芯片 EFR32MG1B
./bin/svd2cpp test/EFR32MG1B.svd
# 转换芯片 HDSC_HC32F46x
./bin/svd2cpp test/HDSC_HC32F46x.svd

```

## 生成样例
### 外设地址描述
```c++
#ifndef _PAC_EFR32MG1B_H
#define _PAC_EFR32MG1B_H
#include <stdint.h>
#include "pac_msc.h"
volatile MSC *PAC_MSC = (MSC *)0x400e0000;
#include "pac_emu.h"
volatile EMU *PAC_EMU = (EMU *)0x400e3000;
#include "pac_rmu.h"
volatile RMU *PAC_RMU = (RMU *)0x400e5000;
#include "pac_cmu.h"
volatile CMU *PAC_CMU = (CMU *)0x400e4000;
#include "pac_crypto.h"
volatile CRYPTO *PAC_CRYPTO = (CRYPTO *)0x400f0000;
#include "pac_gpio.h"
volatile GPIO *PAC_GPIO = (GPIO *)0x4000a000;
#include "pac_prs.h"
volatile PRS *PAC_PRS = (PRS *)0x400e6000;
#include "pac_ldma.h"
volatile LDMA *PAC_LDMA = (LDMA *)0x400e2000;
#include "pac_fpueh.h"
volatile FPUEH *PAC_FPUEH = (FPUEH *)0x400e1000;
#include "pac_gpcrc.h"
volatile GPCRC *PAC_GPCRC = (GPCRC *)0x4001c000;
#include "pac_timer0.h"
volatile TIMER0 *PAC_TIMER0 = (TIMER0 *)0x40018000;
#include "pac_timer1.h"
volatile TIMER1 *PAC_TIMER1 = (TIMER1 *)0x40018400;
#include "pac_usart0.h"
volatile USART0 *PAC_USART0 = (USART0 *)0x40010000;
#include "pac_usart1.h"
volatile USART1 *PAC_USART1 = (USART1 *)0x40010400;
#include "pac_leuart0.h"
volatile LEUART0 *PAC_LEUART0 = (LEUART0 *)0x4004a000;
#include "pac_letimer0.h"
volatile LETIMER0 *PAC_LETIMER0 = (LETIMER0 *)0x40046000;
#include "pac_cryotimer.h"
volatile CRYOTIMER *PAC_CRYOTIMER = (CRYOTIMER *)0x4001e000;
#include "pac_pcnt0.h"
volatile PCNT0 *PAC_PCNT0 = (PCNT0 *)0x4004e000;
#include "pac_i2c0.h"
volatile I2C0 *PAC_I2C0 = (I2C0 *)0x4000c000;
#include "pac_adc0.h"
volatile ADC0 *PAC_ADC0 = (ADC0 *)0x40002000;
#include "pac_acmp0.h"
volatile ACMP0 *PAC_ACMP0 = (ACMP0 *)0x40000000;
#include "pac_acmp1.h"
volatile ACMP1 *PAC_ACMP1 = (ACMP1 *)0x40000400;
#include "pac_idac0.h"
volatile IDAC0 *PAC_IDAC0 = (IDAC0 *)0x40006000;
#include "pac_rtcc.h"
volatile RTCC *PAC_RTCC = (RTCC *)0x40042000;
#include "pac_wdog0.h"
volatile WDOG0 *PAC_WDOG0 = (WDOG0 *)0x40052000;
#endif
```
### 外设寄存器描述
```c++
#ifndef _PAC_GPIO_H
#define _PAC_GPIO_H
#include "register.h"
#include <stdint.h>
/* GPIO */
struct GPIO {
	/* Port Control Register */
    public: union PA_CTRL{ /* addr: 0x0 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DRIVESTRENGTH = 0x00000001,
            MASK_SLEWRATE   = 0x00000070,
            MASK_DINDIS     = 0x00001000,
            MASK_DRIVESTRENGTHALT = 0x00010000,
            MASK_SLEWRATEALT = 0x00700000,
            MASK_DINDISALT  = 0x10000000,
        };
        public: struct fields{
			/* Drive Strength for Port */
            private: uint32_t   _drivestrength : 1;
            /* read fields PA_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline uint32_t r_drivestrength(void){return (uint32_t)_drivestrength;}
            /* write fields PA_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline void w_drivestrength(uint32_t var){_drivestrength = var & 0x1;}
            private: uint32_t   : 3;
			/* Slewrate Limit for Port */
            private: uint32_t   _slewrate : 3;
            /* read fields PA_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline uint32_t r_slewrate(void){return (uint32_t)_slewrate;}
            /* write fields PA_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline void w_slewrate(uint32_t var){_slewrate = var & 0x7;}
            private: uint32_t   : 5;
			/* Data in Disable */
            private: uint32_t   _dindis : 1;
            /* read fields PA_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline uint32_t r_dindis(void){return (uint32_t)_dindis;}
            /* write fields PA_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline void w_dindis(uint32_t var){_dindis = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Drive Strength for Port */
            private: uint32_t   _drivestrengthalt : 1;
            /* read fields PA_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline uint32_t r_drivestrengthalt(void){return (uint32_t)_drivestrengthalt;}
            /* write fields PA_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline void w_drivestrengthalt(uint32_t var){_drivestrengthalt = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Slewrate Limit for Port */
            private: uint32_t   _slewratealt : 3;
            /* read fields PA_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline uint32_t r_slewratealt(void){return (uint32_t)_slewratealt;}
            /* write fields PA_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline void w_slewratealt(uint32_t var){_slewratealt = var & 0x7;}
            private: uint32_t   : 5;
			/* Alternate Data in Disable */
            private: uint32_t   _dindisalt : 1;
            /* read fields PA_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline uint32_t r_dindisalt(void){return (uint32_t)_dindisalt;}
            /* write fields PA_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline void w_dindisalt(uint32_t var){_dindisalt = var & 0x1;}
            private: uint32_t : 3;
        }fields;
    }PA_CTRL;
	/* Port Pin Mode Low Register */
    public: union PA_MODEL{ /* addr: 0x4 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE0      = 0x0000000f,
            MASK_MODE1      = 0x000000f0,
            MASK_MODE2      = 0x00000f00,
            MASK_MODE3      = 0x0000f000,
            MASK_MODE4      = 0x000f0000,
            MASK_MODE5      = 0x00f00000,
            MASK_MODE6      = 0x0f000000,
            MASK_MODE7      = 0xf0000000,
        };
		/* Pin 0 Mode */
        public: enum mode0_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE0_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE0_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE0_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE0_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE0_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE0_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE0_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE0_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE0_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE0_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE0_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE0_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE0_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE0_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE0_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE0_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 1 Mode */
        public: enum mode1_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE1_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE1_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE1_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE1_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE1_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE1_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE1_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE1_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE1_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE1_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE1_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE1_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE1_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE1_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE1_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE1_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 2 Mode */
        public: enum mode2_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE2_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE2_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE2_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE2_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE2_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE2_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE2_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE2_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE2_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE2_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE2_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE2_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE2_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE2_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE2_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE2_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 3 Mode */
        public: enum mode3_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE3_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE3_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE3_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE3_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE3_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE3_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE3_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE3_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE3_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE3_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE3_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE3_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE3_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE3_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE3_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE3_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 4 Mode */
        public: enum mode4_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE4_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE4_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE4_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE4_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE4_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE4_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE4_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE4_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE4_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE4_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE4_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE4_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE4_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE4_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE4_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE4_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 5 Mode */
        public: enum mode5_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE5_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE5_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE5_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE5_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE5_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE5_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE5_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE5_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE5_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE5_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE5_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE5_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE5_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE5_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE5_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE5_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 6 Mode */
        public: enum mode6_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE6_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE6_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE6_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE6_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE6_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE6_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE6_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE6_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE6_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE6_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE6_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE6_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE6_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE6_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE6_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE6_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 7 Mode */
        public: enum mode7_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE7_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE7_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE7_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE7_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE7_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE7_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE7_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE7_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE7_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE7_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE7_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE7_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE7_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE7_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE7_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE7_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 0 Mode */
            private: uint32_t   _mode0 : 4;
            /* read fields PA_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline mode0_t r_mode0(void){return (mode0_t)_mode0;}
            /* write fields PA_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline void w_mode0(mode0_t var){_mode0 = var & 0xf;}
			/* Pin 1 Mode */
            private: uint32_t   _mode1 : 4;
            /* read fields PA_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline mode1_t r_mode1(void){return (mode1_t)_mode1;}
            /* write fields PA_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline void w_mode1(mode1_t var){_mode1 = var & 0xf;}
			/* Pin 2 Mode */
            private: uint32_t   _mode2 : 4;
            /* read fields PA_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline mode2_t r_mode2(void){return (mode2_t)_mode2;}
            /* write fields PA_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline void w_mode2(mode2_t var){_mode2 = var & 0xf;}
			/* Pin 3 Mode */
            private: uint32_t   _mode3 : 4;
            /* read fields PA_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline mode3_t r_mode3(void){return (mode3_t)_mode3;}
            /* write fields PA_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline void w_mode3(mode3_t var){_mode3 = var & 0xf;}
			/* Pin 4 Mode */
            private: uint32_t   _mode4 : 4;
            /* read fields PA_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline mode4_t r_mode4(void){return (mode4_t)_mode4;}
            /* write fields PA_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline void w_mode4(mode4_t var){_mode4 = var & 0xf;}
			/* Pin 5 Mode */
            private: uint32_t   _mode5 : 4;
            /* read fields PA_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline mode5_t r_mode5(void){return (mode5_t)_mode5;}
            /* write fields PA_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline void w_mode5(mode5_t var){_mode5 = var & 0xf;}
			/* Pin 6 Mode */
            private: uint32_t   _mode6 : 4;
            /* read fields PA_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline mode6_t r_mode6(void){return (mode6_t)_mode6;}
            /* write fields PA_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline void w_mode6(mode6_t var){_mode6 = var & 0xf;}
			/* Pin 7 Mode */
            private: uint32_t   _mode7 : 4;
            /* read fields PA_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline mode7_t r_mode7(void){return (mode7_t)_mode7;}
            /* write fields PA_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline void w_mode7(mode7_t var){_mode7 = var & 0xf;}
        }fields;
    }PA_MODEL;
	/* Port Pin Mode High Register */
    public: union PA_MODEH{ /* addr: 0x8 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE8      = 0x0000000f,
            MASK_MODE9      = 0x000000f0,
            MASK_MODE10     = 0x00000f00,
            MASK_MODE11     = 0x0000f000,
            MASK_MODE12     = 0x000f0000,
            MASK_MODE13     = 0x00f00000,
            MASK_MODE14     = 0x0f000000,
            MASK_MODE15     = 0xf0000000,
        };
		/* Pin 8 Mode */
        public: enum mode8_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE8_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE8_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE8_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE8_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE8_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE8_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE8_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE8_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE8_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE8_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE8_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE8_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE8_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE8_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE8_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE8_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 9 Mode */
        public: enum mode9_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE9_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE9_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE9_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE9_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE9_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE9_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE9_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE9_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE9_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE9_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE9_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE9_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE9_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE9_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE9_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE9_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 10 Mode */
        public: enum mode10_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE10_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE10_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE10_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE10_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE10_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE10_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE10_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE10_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE10_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE10_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE10_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE10_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE10_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE10_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE10_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE10_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 11 Mode */
        public: enum mode11_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE11_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE11_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE11_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE11_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE11_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE11_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE11_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE11_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE11_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE11_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE11_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE11_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE11_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE11_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE11_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE11_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 12 Mode */
        public: enum mode12_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE12_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE12_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE12_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE12_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE12_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE12_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE12_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE12_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE12_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE12_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE12_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE12_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE12_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE12_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE12_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE12_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 13 Mode */
        public: enum mode13_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE13_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE13_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE13_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE13_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE13_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE13_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE13_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE13_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE13_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE13_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE13_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE13_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE13_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE13_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE13_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE13_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 14 Mode */
        public: enum mode14_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE14_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE14_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE14_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE14_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE14_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE14_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE14_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE14_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE14_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE14_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE14_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE14_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE14_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE14_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE14_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE14_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 15 Mode */
        public: enum mode15_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE15_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE15_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE15_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE15_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE15_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE15_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE15_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE15_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE15_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE15_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE15_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE15_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE15_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE15_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE15_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE15_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 8 Mode */
            private: uint32_t   _mode8 : 4;
            /* read fields PA_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline mode8_t r_mode8(void){return (mode8_t)_mode8;}
            /* write fields PA_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline void w_mode8(mode8_t var){_mode8 = var & 0xf;}
			/* Pin 9 Mode */
            private: uint32_t   _mode9 : 4;
            /* read fields PA_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline mode9_t r_mode9(void){return (mode9_t)_mode9;}
            /* write fields PA_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline void w_mode9(mode9_t var){_mode9 = var & 0xf;}
			/* Pin 10 Mode */
            private: uint32_t   _mode10 : 4;
            /* read fields PA_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline mode10_t r_mode10(void){return (mode10_t)_mode10;}
            /* write fields PA_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline void w_mode10(mode10_t var){_mode10 = var & 0xf;}
			/* Pin 11 Mode */
            private: uint32_t   _mode11 : 4;
            /* read fields PA_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline mode11_t r_mode11(void){return (mode11_t)_mode11;}
            /* write fields PA_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline void w_mode11(mode11_t var){_mode11 = var & 0xf;}
			/* Pin 12 Mode */
            private: uint32_t   _mode12 : 4;
            /* read fields PA_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline mode12_t r_mode12(void){return (mode12_t)_mode12;}
            /* write fields PA_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline void w_mode12(mode12_t var){_mode12 = var & 0xf;}
			/* Pin 13 Mode */
            private: uint32_t   _mode13 : 4;
            /* read fields PA_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline mode13_t r_mode13(void){return (mode13_t)_mode13;}
            /* write fields PA_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline void w_mode13(mode13_t var){_mode13 = var & 0xf;}
			/* Pin 14 Mode */
            private: uint32_t   _mode14 : 4;
            /* read fields PA_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline mode14_t r_mode14(void){return (mode14_t)_mode14;}
            /* write fields PA_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline void w_mode14(mode14_t var){_mode14 = var & 0xf;}
			/* Pin 15 Mode */
            private: uint32_t   _mode15 : 4;
            /* read fields PA_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline mode15_t r_mode15(void){return (mode15_t)_mode15;}
            /* write fields PA_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline void w_mode15(mode15_t var){_mode15 = var & 0xf;}
        }fields;
    }PA_MODEH;
	/* Port Data Out Register */
    public: union PA_DOUT{ /* addr: 0xc */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DOUT       = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out */
            private: uint32_t   _dout : 16;
            /* read fields PA_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline uint32_t r_dout(void){return (uint32_t)_dout;}
            /* write fields PA_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline void w_dout(uint32_t var){_dout = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PA_DOUT;
    private: uint8_t rsv_0[8];
	/* Port Data Out Toggle Register */
    public: union PA_DOUTTGL{ /* addr: 0x18 */ 
        WriteOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DOUTTGL    = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out Toggle */
            private: uint32_t   _douttgl : 16;
            /* write fields PA_DOUTTGL.DOUTTGL:uint32_t(16 bit)	(Data Out Toggle) */
            public: inline void w_douttgl(uint32_t var){_douttgl = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PA_DOUTTGL;
	/* Port Data in Register */
    public: union PA_DIN{ /* addr: 0x1c */ 
        ReadOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DIN        = 0x0000ffff,
        };
        public: struct fields{
			/* Data in */
            private: uint32_t   _din : 16;
            /* read fields PA_DIN.DIN:uint32_t(16 bit)	(Data in) */
            public: inline uint32_t r_din(void){return (uint32_t)_din;}
            private: uint32_t : 16;
        }fields;
    }PA_DIN;
	/* Port Unlocked Pins Register */
    public: union PA_PINLOCKN{ /* addr: 0x20 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_PINLOCKN   = 0x0000ffff,
        };
        public: struct fields{
			/* Unlocked Pins */
            private: uint32_t   _pinlockn : 16;
            /* read fields PA_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline uint32_t r_pinlockn(void){return (uint32_t)_pinlockn;}
            /* write fields PA_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline void w_pinlockn(uint32_t var){_pinlockn = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PA_PINLOCKN;
    private: uint8_t rsv_1[4];
	/* Over Voltage Disable for All Modes */
    public: union PA_OVTDIS{ /* addr: 0x28 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_OVTDIS     = 0x0000ffff,
        };
        public: struct fields{
			/* Disable Over Voltage Capability */
            private: uint32_t   _ovtdis : 16;
            /* read fields PA_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline uint32_t r_ovtdis(void){return (uint32_t)_ovtdis;}
            /* write fields PA_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline void w_ovtdis(uint32_t var){_ovtdis = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PA_OVTDIS;
    private: uint8_t rsv_2[4];
	/* Port Control Register */
    public: union PB_CTRL{ /* addr: 0x30 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DRIVESTRENGTH = 0x00000001,
            MASK_SLEWRATE   = 0x00000070,
            MASK_DINDIS     = 0x00001000,
            MASK_DRIVESTRENGTHALT = 0x00010000,
            MASK_SLEWRATEALT = 0x00700000,
            MASK_DINDISALT  = 0x10000000,
        };
        public: struct fields{
			/* Drive Strength for Port */
            private: uint32_t   _drivestrength : 1;
            /* read fields PB_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline uint32_t r_drivestrength(void){return (uint32_t)_drivestrength;}
            /* write fields PB_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline void w_drivestrength(uint32_t var){_drivestrength = var & 0x1;}
            private: uint32_t   : 3;
			/* Slewrate Limit for Port */
            private: uint32_t   _slewrate : 3;
            /* read fields PB_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline uint32_t r_slewrate(void){return (uint32_t)_slewrate;}
            /* write fields PB_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline void w_slewrate(uint32_t var){_slewrate = var & 0x7;}
            private: uint32_t   : 5;
			/* Data in Disable */
            private: uint32_t   _dindis : 1;
            /* read fields PB_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline uint32_t r_dindis(void){return (uint32_t)_dindis;}
            /* write fields PB_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline void w_dindis(uint32_t var){_dindis = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Drive Strength for Port */
            private: uint32_t   _drivestrengthalt : 1;
            /* read fields PB_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline uint32_t r_drivestrengthalt(void){return (uint32_t)_drivestrengthalt;}
            /* write fields PB_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline void w_drivestrengthalt(uint32_t var){_drivestrengthalt = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Slewrate Limit for Port */
            private: uint32_t   _slewratealt : 3;
            /* read fields PB_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline uint32_t r_slewratealt(void){return (uint32_t)_slewratealt;}
            /* write fields PB_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline void w_slewratealt(uint32_t var){_slewratealt = var & 0x7;}
            private: uint32_t   : 5;
			/* Alternate Data in Disable */
            private: uint32_t   _dindisalt : 1;
            /* read fields PB_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline uint32_t r_dindisalt(void){return (uint32_t)_dindisalt;}
            /* write fields PB_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline void w_dindisalt(uint32_t var){_dindisalt = var & 0x1;}
            private: uint32_t : 3;
        }fields;
    }PB_CTRL;
	/* Port Pin Mode Low Register */
    public: union PB_MODEL{ /* addr: 0x34 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE0      = 0x0000000f,
            MASK_MODE1      = 0x000000f0,
            MASK_MODE2      = 0x00000f00,
            MASK_MODE3      = 0x0000f000,
            MASK_MODE4      = 0x000f0000,
            MASK_MODE5      = 0x00f00000,
            MASK_MODE6      = 0x0f000000,
            MASK_MODE7      = 0xf0000000,
        };
		/* Pin 0 Mode */
        public: enum mode0_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE0_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE0_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE0_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE0_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE0_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE0_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE0_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE0_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE0_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE0_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE0_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE0_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE0_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE0_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE0_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE0_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 1 Mode */
        public: enum mode1_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE1_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE1_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE1_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE1_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE1_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE1_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE1_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE1_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE1_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE1_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE1_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE1_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE1_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE1_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE1_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE1_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 2 Mode */
        public: enum mode2_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE2_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE2_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE2_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE2_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE2_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE2_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE2_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE2_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE2_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE2_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE2_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE2_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE2_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE2_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE2_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE2_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 3 Mode */
        public: enum mode3_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE3_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE3_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE3_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE3_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE3_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE3_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE3_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE3_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE3_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE3_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE3_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE3_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE3_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE3_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE3_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE3_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 4 Mode */
        public: enum mode4_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE4_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE4_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE4_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE4_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE4_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE4_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE4_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE4_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE4_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE4_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE4_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE4_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE4_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE4_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE4_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE4_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 5 Mode */
        public: enum mode5_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE5_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE5_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE5_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE5_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE5_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE5_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE5_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE5_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE5_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE5_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE5_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE5_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE5_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE5_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE5_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE5_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 6 Mode */
        public: enum mode6_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE6_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE6_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE6_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE6_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE6_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE6_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE6_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE6_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE6_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE6_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE6_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE6_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE6_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE6_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE6_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE6_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 7 Mode */
        public: enum mode7_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE7_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE7_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE7_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE7_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE7_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE7_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE7_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE7_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE7_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE7_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE7_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE7_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE7_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE7_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE7_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE7_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 0 Mode */
            private: uint32_t   _mode0 : 4;
            /* read fields PB_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline mode0_t r_mode0(void){return (mode0_t)_mode0;}
            /* write fields PB_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline void w_mode0(mode0_t var){_mode0 = var & 0xf;}
			/* Pin 1 Mode */
            private: uint32_t   _mode1 : 4;
            /* read fields PB_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline mode1_t r_mode1(void){return (mode1_t)_mode1;}
            /* write fields PB_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline void w_mode1(mode1_t var){_mode1 = var & 0xf;}
			/* Pin 2 Mode */
            private: uint32_t   _mode2 : 4;
            /* read fields PB_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline mode2_t r_mode2(void){return (mode2_t)_mode2;}
            /* write fields PB_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline void w_mode2(mode2_t var){_mode2 = var & 0xf;}
			/* Pin 3 Mode */
            private: uint32_t   _mode3 : 4;
            /* read fields PB_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline mode3_t r_mode3(void){return (mode3_t)_mode3;}
            /* write fields PB_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline void w_mode3(mode3_t var){_mode3 = var & 0xf;}
			/* Pin 4 Mode */
            private: uint32_t   _mode4 : 4;
            /* read fields PB_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline mode4_t r_mode4(void){return (mode4_t)_mode4;}
            /* write fields PB_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline void w_mode4(mode4_t var){_mode4 = var & 0xf;}
			/* Pin 5 Mode */
            private: uint32_t   _mode5 : 4;
            /* read fields PB_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline mode5_t r_mode5(void){return (mode5_t)_mode5;}
            /* write fields PB_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline void w_mode5(mode5_t var){_mode5 = var & 0xf;}
			/* Pin 6 Mode */
            private: uint32_t   _mode6 : 4;
            /* read fields PB_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline mode6_t r_mode6(void){return (mode6_t)_mode6;}
            /* write fields PB_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline void w_mode6(mode6_t var){_mode6 = var & 0xf;}
			/* Pin 7 Mode */
            private: uint32_t   _mode7 : 4;
            /* read fields PB_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline mode7_t r_mode7(void){return (mode7_t)_mode7;}
            /* write fields PB_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline void w_mode7(mode7_t var){_mode7 = var & 0xf;}
        }fields;
    }PB_MODEL;
	/* Port Pin Mode High Register */
    public: union PB_MODEH{ /* addr: 0x38 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE8      = 0x0000000f,
            MASK_MODE9      = 0x000000f0,
            MASK_MODE10     = 0x00000f00,
            MASK_MODE11     = 0x0000f000,
            MASK_MODE12     = 0x000f0000,
            MASK_MODE13     = 0x00f00000,
            MASK_MODE14     = 0x0f000000,
            MASK_MODE15     = 0xf0000000,
        };
		/* Pin 8 Mode */
        public: enum mode8_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE8_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE8_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE8_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE8_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE8_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE8_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE8_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE8_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE8_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE8_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE8_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE8_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE8_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE8_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE8_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE8_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 9 Mode */
        public: enum mode9_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE9_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE9_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE9_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE9_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE9_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE9_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE9_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE9_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE9_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE9_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE9_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE9_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE9_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE9_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE9_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE9_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 10 Mode */
        public: enum mode10_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE10_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE10_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE10_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE10_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE10_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE10_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE10_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE10_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE10_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE10_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE10_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE10_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE10_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE10_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE10_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE10_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 11 Mode */
        public: enum mode11_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE11_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE11_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE11_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE11_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE11_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE11_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE11_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE11_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE11_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE11_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE11_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE11_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE11_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE11_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE11_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE11_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 12 Mode */
        public: enum mode12_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE12_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE12_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE12_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE12_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE12_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE12_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE12_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE12_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE12_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE12_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE12_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE12_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE12_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE12_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE12_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE12_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 13 Mode */
        public: enum mode13_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE13_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE13_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE13_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE13_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE13_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE13_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE13_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE13_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE13_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE13_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE13_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE13_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE13_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE13_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE13_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE13_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 14 Mode */
        public: enum mode14_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE14_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE14_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE14_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE14_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE14_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE14_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE14_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE14_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE14_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE14_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE14_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE14_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE14_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE14_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE14_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE14_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 15 Mode */
        public: enum mode15_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE15_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE15_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE15_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE15_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE15_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE15_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE15_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE15_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE15_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE15_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE15_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE15_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE15_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE15_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE15_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE15_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 8 Mode */
            private: uint32_t   _mode8 : 4;
            /* read fields PB_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline mode8_t r_mode8(void){return (mode8_t)_mode8;}
            /* write fields PB_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline void w_mode8(mode8_t var){_mode8 = var & 0xf;}
			/* Pin 9 Mode */
            private: uint32_t   _mode9 : 4;
            /* read fields PB_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline mode9_t r_mode9(void){return (mode9_t)_mode9;}
            /* write fields PB_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline void w_mode9(mode9_t var){_mode9 = var & 0xf;}
			/* Pin 10 Mode */
            private: uint32_t   _mode10 : 4;
            /* read fields PB_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline mode10_t r_mode10(void){return (mode10_t)_mode10;}
            /* write fields PB_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline void w_mode10(mode10_t var){_mode10 = var & 0xf;}
			/* Pin 11 Mode */
            private: uint32_t   _mode11 : 4;
            /* read fields PB_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline mode11_t r_mode11(void){return (mode11_t)_mode11;}
            /* write fields PB_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline void w_mode11(mode11_t var){_mode11 = var & 0xf;}
			/* Pin 12 Mode */
            private: uint32_t   _mode12 : 4;
            /* read fields PB_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline mode12_t r_mode12(void){return (mode12_t)_mode12;}
            /* write fields PB_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline void w_mode12(mode12_t var){_mode12 = var & 0xf;}
			/* Pin 13 Mode */
            private: uint32_t   _mode13 : 4;
            /* read fields PB_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline mode13_t r_mode13(void){return (mode13_t)_mode13;}
            /* write fields PB_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline void w_mode13(mode13_t var){_mode13 = var & 0xf;}
			/* Pin 14 Mode */
            private: uint32_t   _mode14 : 4;
            /* read fields PB_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline mode14_t r_mode14(void){return (mode14_t)_mode14;}
            /* write fields PB_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline void w_mode14(mode14_t var){_mode14 = var & 0xf;}
			/* Pin 15 Mode */
            private: uint32_t   _mode15 : 4;
            /* read fields PB_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline mode15_t r_mode15(void){return (mode15_t)_mode15;}
            /* write fields PB_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline void w_mode15(mode15_t var){_mode15 = var & 0xf;}
        }fields;
    }PB_MODEH;
	/* Port Data Out Register */
    public: union PB_DOUT{ /* addr: 0x3c */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DOUT       = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out */
            private: uint32_t   _dout : 16;
            /* read fields PB_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline uint32_t r_dout(void){return (uint32_t)_dout;}
            /* write fields PB_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline void w_dout(uint32_t var){_dout = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PB_DOUT;
    private: uint8_t rsv_3[8];
	/* Port Data Out Toggle Register */
    public: union PB_DOUTTGL{ /* addr: 0x48 */ 
        WriteOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DOUTTGL    = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out Toggle */
            private: uint32_t   _douttgl : 16;
            /* write fields PB_DOUTTGL.DOUTTGL:uint32_t(16 bit)	(Data Out Toggle) */
            public: inline void w_douttgl(uint32_t var){_douttgl = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PB_DOUTTGL;
	/* Port Data in Register */
    public: union PB_DIN{ /* addr: 0x4c */ 
        ReadOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DIN        = 0x0000ffff,
        };
        public: struct fields{
			/* Data in */
            private: uint32_t   _din : 16;
            /* read fields PB_DIN.DIN:uint32_t(16 bit)	(Data in) */
            public: inline uint32_t r_din(void){return (uint32_t)_din;}
            private: uint32_t : 16;
        }fields;
    }PB_DIN;
	/* Port Unlocked Pins Register */
    public: union PB_PINLOCKN{ /* addr: 0x50 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_PINLOCKN   = 0x0000ffff,
        };
        public: struct fields{
			/* Unlocked Pins */
            private: uint32_t   _pinlockn : 16;
            /* read fields PB_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline uint32_t r_pinlockn(void){return (uint32_t)_pinlockn;}
            /* write fields PB_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline void w_pinlockn(uint32_t var){_pinlockn = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PB_PINLOCKN;
    private: uint8_t rsv_4[4];
	/* Over Voltage Disable for All Modes */
    public: union PB_OVTDIS{ /* addr: 0x58 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_OVTDIS     = 0x0000ffff,
        };
        public: struct fields{
			/* Disable Over Voltage Capability */
            private: uint32_t   _ovtdis : 16;
            /* read fields PB_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline uint32_t r_ovtdis(void){return (uint32_t)_ovtdis;}
            /* write fields PB_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline void w_ovtdis(uint32_t var){_ovtdis = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PB_OVTDIS;
    private: uint8_t rsv_5[4];
	/* Port Control Register */
    public: union PC_CTRL{ /* addr: 0x60 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DRIVESTRENGTH = 0x00000001,
            MASK_SLEWRATE   = 0x00000070,
            MASK_DINDIS     = 0x00001000,
            MASK_DRIVESTRENGTHALT = 0x00010000,
            MASK_SLEWRATEALT = 0x00700000,
            MASK_DINDISALT  = 0x10000000,
        };
        public: struct fields{
			/* Drive Strength for Port */
            private: uint32_t   _drivestrength : 1;
            /* read fields PC_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline uint32_t r_drivestrength(void){return (uint32_t)_drivestrength;}
            /* write fields PC_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline void w_drivestrength(uint32_t var){_drivestrength = var & 0x1;}
            private: uint32_t   : 3;
			/* Slewrate Limit for Port */
            private: uint32_t   _slewrate : 3;
            /* read fields PC_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline uint32_t r_slewrate(void){return (uint32_t)_slewrate;}
            /* write fields PC_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline void w_slewrate(uint32_t var){_slewrate = var & 0x7;}
            private: uint32_t   : 5;
			/* Data in Disable */
            private: uint32_t   _dindis : 1;
            /* read fields PC_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline uint32_t r_dindis(void){return (uint32_t)_dindis;}
            /* write fields PC_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline void w_dindis(uint32_t var){_dindis = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Drive Strength for Port */
            private: uint32_t   _drivestrengthalt : 1;
            /* read fields PC_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline uint32_t r_drivestrengthalt(void){return (uint32_t)_drivestrengthalt;}
            /* write fields PC_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline void w_drivestrengthalt(uint32_t var){_drivestrengthalt = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Slewrate Limit for Port */
            private: uint32_t   _slewratealt : 3;
            /* read fields PC_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline uint32_t r_slewratealt(void){return (uint32_t)_slewratealt;}
            /* write fields PC_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline void w_slewratealt(uint32_t var){_slewratealt = var & 0x7;}
            private: uint32_t   : 5;
			/* Alternate Data in Disable */
            private: uint32_t   _dindisalt : 1;
            /* read fields PC_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline uint32_t r_dindisalt(void){return (uint32_t)_dindisalt;}
            /* write fields PC_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline void w_dindisalt(uint32_t var){_dindisalt = var & 0x1;}
            private: uint32_t : 3;
        }fields;
    }PC_CTRL;
	/* Port Pin Mode Low Register */
    public: union PC_MODEL{ /* addr: 0x64 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE0      = 0x0000000f,
            MASK_MODE1      = 0x000000f0,
            MASK_MODE2      = 0x00000f00,
            MASK_MODE3      = 0x0000f000,
            MASK_MODE4      = 0x000f0000,
            MASK_MODE5      = 0x00f00000,
            MASK_MODE6      = 0x0f000000,
            MASK_MODE7      = 0xf0000000,
        };
		/* Pin 0 Mode */
        public: enum mode0_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE0_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE0_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE0_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE0_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE0_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE0_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE0_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE0_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE0_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE0_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE0_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE0_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE0_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE0_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE0_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE0_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 1 Mode */
        public: enum mode1_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE1_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE1_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE1_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE1_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE1_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE1_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE1_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE1_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE1_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE1_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE1_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE1_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE1_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE1_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE1_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE1_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 2 Mode */
        public: enum mode2_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE2_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE2_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE2_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE2_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE2_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE2_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE2_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE2_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE2_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE2_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE2_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE2_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE2_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE2_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE2_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE2_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 3 Mode */
        public: enum mode3_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE3_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE3_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE3_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE3_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE3_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE3_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE3_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE3_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE3_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE3_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE3_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE3_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE3_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE3_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE3_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE3_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 4 Mode */
        public: enum mode4_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE4_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE4_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE4_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE4_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE4_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE4_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE4_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE4_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE4_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE4_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE4_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE4_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE4_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE4_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE4_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE4_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 5 Mode */
        public: enum mode5_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE5_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE5_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE5_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE5_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE5_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE5_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE5_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE5_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE5_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE5_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE5_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE5_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE5_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE5_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE5_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE5_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 6 Mode */
        public: enum mode6_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE6_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE6_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE6_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE6_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE6_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE6_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE6_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE6_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE6_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE6_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE6_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE6_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE6_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE6_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE6_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE6_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 7 Mode */
        public: enum mode7_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE7_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE7_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE7_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE7_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE7_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE7_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE7_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE7_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE7_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE7_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE7_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE7_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE7_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE7_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE7_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE7_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 0 Mode */
            private: uint32_t   _mode0 : 4;
            /* read fields PC_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline mode0_t r_mode0(void){return (mode0_t)_mode0;}
            /* write fields PC_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline void w_mode0(mode0_t var){_mode0 = var & 0xf;}
			/* Pin 1 Mode */
            private: uint32_t   _mode1 : 4;
            /* read fields PC_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline mode1_t r_mode1(void){return (mode1_t)_mode1;}
            /* write fields PC_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline void w_mode1(mode1_t var){_mode1 = var & 0xf;}
			/* Pin 2 Mode */
            private: uint32_t   _mode2 : 4;
            /* read fields PC_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline mode2_t r_mode2(void){return (mode2_t)_mode2;}
            /* write fields PC_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline void w_mode2(mode2_t var){_mode2 = var & 0xf;}
			/* Pin 3 Mode */
            private: uint32_t   _mode3 : 4;
            /* read fields PC_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline mode3_t r_mode3(void){return (mode3_t)_mode3;}
            /* write fields PC_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline void w_mode3(mode3_t var){_mode3 = var & 0xf;}
			/* Pin 4 Mode */
            private: uint32_t   _mode4 : 4;
            /* read fields PC_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline mode4_t r_mode4(void){return (mode4_t)_mode4;}
            /* write fields PC_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline void w_mode4(mode4_t var){_mode4 = var & 0xf;}
			/* Pin 5 Mode */
            private: uint32_t   _mode5 : 4;
            /* read fields PC_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline mode5_t r_mode5(void){return (mode5_t)_mode5;}
            /* write fields PC_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline void w_mode5(mode5_t var){_mode5 = var & 0xf;}
			/* Pin 6 Mode */
            private: uint32_t   _mode6 : 4;
            /* read fields PC_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline mode6_t r_mode6(void){return (mode6_t)_mode6;}
            /* write fields PC_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline void w_mode6(mode6_t var){_mode6 = var & 0xf;}
			/* Pin 7 Mode */
            private: uint32_t   _mode7 : 4;
            /* read fields PC_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline mode7_t r_mode7(void){return (mode7_t)_mode7;}
            /* write fields PC_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline void w_mode7(mode7_t var){_mode7 = var & 0xf;}
        }fields;
    }PC_MODEL;
	/* Port Pin Mode High Register */
    public: union PC_MODEH{ /* addr: 0x68 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE8      = 0x0000000f,
            MASK_MODE9      = 0x000000f0,
            MASK_MODE10     = 0x00000f00,
            MASK_MODE11     = 0x0000f000,
            MASK_MODE12     = 0x000f0000,
            MASK_MODE13     = 0x00f00000,
            MASK_MODE14     = 0x0f000000,
            MASK_MODE15     = 0xf0000000,
        };
		/* Pin 8 Mode */
        public: enum mode8_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE8_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE8_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE8_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE8_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE8_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE8_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE8_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE8_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE8_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE8_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE8_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE8_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE8_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE8_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE8_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE8_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 9 Mode */
        public: enum mode9_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE9_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE9_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE9_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE9_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE9_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE9_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE9_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE9_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE9_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE9_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE9_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE9_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE9_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE9_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE9_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE9_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 10 Mode */
        public: enum mode10_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE10_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE10_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE10_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE10_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE10_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE10_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE10_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE10_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE10_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE10_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE10_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE10_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE10_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE10_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE10_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE10_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 11 Mode */
        public: enum mode11_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE11_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE11_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE11_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE11_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE11_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE11_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE11_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE11_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE11_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE11_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE11_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE11_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE11_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE11_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE11_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE11_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 12 Mode */
        public: enum mode12_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE12_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE12_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE12_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE12_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE12_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE12_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE12_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE12_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE12_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE12_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE12_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE12_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE12_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE12_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE12_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE12_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 13 Mode */
        public: enum mode13_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE13_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE13_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE13_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE13_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE13_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE13_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE13_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE13_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE13_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE13_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE13_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE13_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE13_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE13_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE13_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE13_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 14 Mode */
        public: enum mode14_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE14_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE14_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE14_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE14_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE14_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE14_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE14_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE14_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE14_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE14_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE14_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE14_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE14_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE14_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE14_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE14_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 15 Mode */
        public: enum mode15_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE15_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE15_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE15_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE15_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE15_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE15_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE15_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE15_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE15_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE15_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE15_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE15_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE15_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE15_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE15_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE15_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 8 Mode */
            private: uint32_t   _mode8 : 4;
            /* read fields PC_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline mode8_t r_mode8(void){return (mode8_t)_mode8;}
            /* write fields PC_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline void w_mode8(mode8_t var){_mode8 = var & 0xf;}
			/* Pin 9 Mode */
            private: uint32_t   _mode9 : 4;
            /* read fields PC_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline mode9_t r_mode9(void){return (mode9_t)_mode9;}
            /* write fields PC_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline void w_mode9(mode9_t var){_mode9 = var & 0xf;}
			/* Pin 10 Mode */
            private: uint32_t   _mode10 : 4;
            /* read fields PC_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline mode10_t r_mode10(void){return (mode10_t)_mode10;}
            /* write fields PC_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline void w_mode10(mode10_t var){_mode10 = var & 0xf;}
			/* Pin 11 Mode */
            private: uint32_t   _mode11 : 4;
            /* read fields PC_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline mode11_t r_mode11(void){return (mode11_t)_mode11;}
            /* write fields PC_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline void w_mode11(mode11_t var){_mode11 = var & 0xf;}
			/* Pin 12 Mode */
            private: uint32_t   _mode12 : 4;
            /* read fields PC_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline mode12_t r_mode12(void){return (mode12_t)_mode12;}
            /* write fields PC_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline void w_mode12(mode12_t var){_mode12 = var & 0xf;}
			/* Pin 13 Mode */
            private: uint32_t   _mode13 : 4;
            /* read fields PC_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline mode13_t r_mode13(void){return (mode13_t)_mode13;}
            /* write fields PC_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline void w_mode13(mode13_t var){_mode13 = var & 0xf;}
			/* Pin 14 Mode */
            private: uint32_t   _mode14 : 4;
            /* read fields PC_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline mode14_t r_mode14(void){return (mode14_t)_mode14;}
            /* write fields PC_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline void w_mode14(mode14_t var){_mode14 = var & 0xf;}
			/* Pin 15 Mode */
            private: uint32_t   _mode15 : 4;
            /* read fields PC_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline mode15_t r_mode15(void){return (mode15_t)_mode15;}
            /* write fields PC_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline void w_mode15(mode15_t var){_mode15 = var & 0xf;}
        }fields;
    }PC_MODEH;
	/* Port Data Out Register */
    public: union PC_DOUT{ /* addr: 0x6c */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DOUT       = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out */
            private: uint32_t   _dout : 16;
            /* read fields PC_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline uint32_t r_dout(void){return (uint32_t)_dout;}
            /* write fields PC_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline void w_dout(uint32_t var){_dout = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PC_DOUT;
    private: uint8_t rsv_6[8];
	/* Port Data Out Toggle Register */
    public: union PC_DOUTTGL{ /* addr: 0x78 */ 
        WriteOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DOUTTGL    = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out Toggle */
            private: uint32_t   _douttgl : 16;
            /* write fields PC_DOUTTGL.DOUTTGL:uint32_t(16 bit)	(Data Out Toggle) */
            public: inline void w_douttgl(uint32_t var){_douttgl = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PC_DOUTTGL;
	/* Port Data in Register */
    public: union PC_DIN{ /* addr: 0x7c */ 
        ReadOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DIN        = 0x0000ffff,
        };
        public: struct fields{
			/* Data in */
            private: uint32_t   _din : 16;
            /* read fields PC_DIN.DIN:uint32_t(16 bit)	(Data in) */
            public: inline uint32_t r_din(void){return (uint32_t)_din;}
            private: uint32_t : 16;
        }fields;
    }PC_DIN;
	/* Port Unlocked Pins Register */
    public: union PC_PINLOCKN{ /* addr: 0x80 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_PINLOCKN   = 0x0000ffff,
        };
        public: struct fields{
			/* Unlocked Pins */
            private: uint32_t   _pinlockn : 16;
            /* read fields PC_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline uint32_t r_pinlockn(void){return (uint32_t)_pinlockn;}
            /* write fields PC_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline void w_pinlockn(uint32_t var){_pinlockn = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PC_PINLOCKN;
    private: uint8_t rsv_7[4];
	/* Over Voltage Disable for All Modes */
    public: union PC_OVTDIS{ /* addr: 0x88 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_OVTDIS     = 0x0000ffff,
        };
        public: struct fields{
			/* Disable Over Voltage Capability */
            private: uint32_t   _ovtdis : 16;
            /* read fields PC_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline uint32_t r_ovtdis(void){return (uint32_t)_ovtdis;}
            /* write fields PC_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline void w_ovtdis(uint32_t var){_ovtdis = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PC_OVTDIS;
    private: uint8_t rsv_8[4];
	/* Port Control Register */
    public: union PD_CTRL{ /* addr: 0x90 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DRIVESTRENGTH = 0x00000001,
            MASK_SLEWRATE   = 0x00000070,
            MASK_DINDIS     = 0x00001000,
            MASK_DRIVESTRENGTHALT = 0x00010000,
            MASK_SLEWRATEALT = 0x00700000,
            MASK_DINDISALT  = 0x10000000,
        };
        public: struct fields{
			/* Drive Strength for Port */
            private: uint32_t   _drivestrength : 1;
            /* read fields PD_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline uint32_t r_drivestrength(void){return (uint32_t)_drivestrength;}
            /* write fields PD_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline void w_drivestrength(uint32_t var){_drivestrength = var & 0x1;}
            private: uint32_t   : 3;
			/* Slewrate Limit for Port */
            private: uint32_t   _slewrate : 3;
            /* read fields PD_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline uint32_t r_slewrate(void){return (uint32_t)_slewrate;}
            /* write fields PD_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline void w_slewrate(uint32_t var){_slewrate = var & 0x7;}
            private: uint32_t   : 5;
			/* Data in Disable */
            private: uint32_t   _dindis : 1;
            /* read fields PD_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline uint32_t r_dindis(void){return (uint32_t)_dindis;}
            /* write fields PD_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline void w_dindis(uint32_t var){_dindis = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Drive Strength for Port */
            private: uint32_t   _drivestrengthalt : 1;
            /* read fields PD_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline uint32_t r_drivestrengthalt(void){return (uint32_t)_drivestrengthalt;}
            /* write fields PD_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline void w_drivestrengthalt(uint32_t var){_drivestrengthalt = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Slewrate Limit for Port */
            private: uint32_t   _slewratealt : 3;
            /* read fields PD_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline uint32_t r_slewratealt(void){return (uint32_t)_slewratealt;}
            /* write fields PD_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline void w_slewratealt(uint32_t var){_slewratealt = var & 0x7;}
            private: uint32_t   : 5;
			/* Alternate Data in Disable */
            private: uint32_t   _dindisalt : 1;
            /* read fields PD_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline uint32_t r_dindisalt(void){return (uint32_t)_dindisalt;}
            /* write fields PD_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline void w_dindisalt(uint32_t var){_dindisalt = var & 0x1;}
            private: uint32_t : 3;
        }fields;
    }PD_CTRL;
	/* Port Pin Mode Low Register */
    public: union PD_MODEL{ /* addr: 0x94 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE0      = 0x0000000f,
            MASK_MODE1      = 0x000000f0,
            MASK_MODE2      = 0x00000f00,
            MASK_MODE3      = 0x0000f000,
            MASK_MODE4      = 0x000f0000,
            MASK_MODE5      = 0x00f00000,
            MASK_MODE6      = 0x0f000000,
            MASK_MODE7      = 0xf0000000,
        };
		/* Pin 0 Mode */
        public: enum mode0_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE0_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE0_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE0_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE0_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE0_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE0_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE0_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE0_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE0_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE0_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE0_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE0_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE0_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE0_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE0_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE0_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 1 Mode */
        public: enum mode1_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE1_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE1_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE1_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE1_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE1_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE1_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE1_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE1_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE1_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE1_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE1_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE1_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE1_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE1_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE1_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE1_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 2 Mode */
        public: enum mode2_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE2_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE2_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE2_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE2_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE2_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE2_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE2_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE2_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE2_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE2_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE2_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE2_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE2_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE2_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE2_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE2_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 3 Mode */
        public: enum mode3_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE3_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE3_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE3_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE3_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE3_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE3_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE3_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE3_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE3_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE3_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE3_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE3_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE3_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE3_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE3_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE3_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 4 Mode */
        public: enum mode4_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE4_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE4_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE4_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE4_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE4_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE4_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE4_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE4_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE4_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE4_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE4_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE4_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE4_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE4_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE4_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE4_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 5 Mode */
        public: enum mode5_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE5_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE5_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE5_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE5_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE5_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE5_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE5_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE5_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE5_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE5_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE5_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE5_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE5_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE5_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE5_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE5_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 6 Mode */
        public: enum mode6_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE6_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE6_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE6_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE6_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE6_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE6_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE6_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE6_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE6_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE6_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE6_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE6_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE6_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE6_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE6_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE6_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 7 Mode */
        public: enum mode7_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE7_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE7_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE7_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE7_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE7_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE7_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE7_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE7_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE7_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE7_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE7_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE7_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE7_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE7_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE7_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE7_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 0 Mode */
            private: uint32_t   _mode0 : 4;
            /* read fields PD_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline mode0_t r_mode0(void){return (mode0_t)_mode0;}
            /* write fields PD_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline void w_mode0(mode0_t var){_mode0 = var & 0xf;}
			/* Pin 1 Mode */
            private: uint32_t   _mode1 : 4;
            /* read fields PD_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline mode1_t r_mode1(void){return (mode1_t)_mode1;}
            /* write fields PD_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline void w_mode1(mode1_t var){_mode1 = var & 0xf;}
			/* Pin 2 Mode */
            private: uint32_t   _mode2 : 4;
            /* read fields PD_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline mode2_t r_mode2(void){return (mode2_t)_mode2;}
            /* write fields PD_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline void w_mode2(mode2_t var){_mode2 = var & 0xf;}
			/* Pin 3 Mode */
            private: uint32_t   _mode3 : 4;
            /* read fields PD_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline mode3_t r_mode3(void){return (mode3_t)_mode3;}
            /* write fields PD_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline void w_mode3(mode3_t var){_mode3 = var & 0xf;}
			/* Pin 4 Mode */
            private: uint32_t   _mode4 : 4;
            /* read fields PD_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline mode4_t r_mode4(void){return (mode4_t)_mode4;}
            /* write fields PD_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline void w_mode4(mode4_t var){_mode4 = var & 0xf;}
			/* Pin 5 Mode */
            private: uint32_t   _mode5 : 4;
            /* read fields PD_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline mode5_t r_mode5(void){return (mode5_t)_mode5;}
            /* write fields PD_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline void w_mode5(mode5_t var){_mode5 = var & 0xf;}
			/* Pin 6 Mode */
            private: uint32_t   _mode6 : 4;
            /* read fields PD_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline mode6_t r_mode6(void){return (mode6_t)_mode6;}
            /* write fields PD_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline void w_mode6(mode6_t var){_mode6 = var & 0xf;}
			/* Pin 7 Mode */
            private: uint32_t   _mode7 : 4;
            /* read fields PD_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline mode7_t r_mode7(void){return (mode7_t)_mode7;}
            /* write fields PD_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline void w_mode7(mode7_t var){_mode7 = var & 0xf;}
        }fields;
    }PD_MODEL;
	/* Port Pin Mode High Register */
    public: union PD_MODEH{ /* addr: 0x98 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE8      = 0x0000000f,
            MASK_MODE9      = 0x000000f0,
            MASK_MODE10     = 0x00000f00,
            MASK_MODE11     = 0x0000f000,
            MASK_MODE12     = 0x000f0000,
            MASK_MODE13     = 0x00f00000,
            MASK_MODE14     = 0x0f000000,
            MASK_MODE15     = 0xf0000000,
        };
		/* Pin 8 Mode */
        public: enum mode8_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE8_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE8_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE8_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE8_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE8_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE8_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE8_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE8_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE8_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE8_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE8_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE8_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE8_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE8_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE8_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE8_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 9 Mode */
        public: enum mode9_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE9_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE9_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE9_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE9_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE9_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE9_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE9_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE9_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE9_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE9_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE9_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE9_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE9_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE9_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE9_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE9_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 10 Mode */
        public: enum mode10_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE10_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE10_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE10_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE10_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE10_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE10_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE10_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE10_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE10_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE10_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE10_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE10_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE10_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE10_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE10_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE10_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 11 Mode */
        public: enum mode11_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE11_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE11_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE11_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE11_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE11_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE11_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE11_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE11_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE11_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE11_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE11_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE11_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE11_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE11_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE11_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE11_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 12 Mode */
        public: enum mode12_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE12_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE12_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE12_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE12_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE12_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE12_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE12_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE12_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE12_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE12_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE12_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE12_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE12_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE12_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE12_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE12_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 13 Mode */
        public: enum mode13_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE13_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE13_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE13_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE13_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE13_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE13_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE13_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE13_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE13_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE13_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE13_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE13_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE13_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE13_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE13_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE13_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 14 Mode */
        public: enum mode14_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE14_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE14_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE14_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE14_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE14_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE14_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE14_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE14_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE14_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE14_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE14_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE14_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE14_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE14_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE14_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE14_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 15 Mode */
        public: enum mode15_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE15_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE15_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE15_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE15_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE15_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE15_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE15_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE15_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE15_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE15_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE15_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE15_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE15_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE15_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE15_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE15_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 8 Mode */
            private: uint32_t   _mode8 : 4;
            /* read fields PD_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline mode8_t r_mode8(void){return (mode8_t)_mode8;}
            /* write fields PD_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline void w_mode8(mode8_t var){_mode8 = var & 0xf;}
			/* Pin 9 Mode */
            private: uint32_t   _mode9 : 4;
            /* read fields PD_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline mode9_t r_mode9(void){return (mode9_t)_mode9;}
            /* write fields PD_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline void w_mode9(mode9_t var){_mode9 = var & 0xf;}
			/* Pin 10 Mode */
            private: uint32_t   _mode10 : 4;
            /* read fields PD_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline mode10_t r_mode10(void){return (mode10_t)_mode10;}
            /* write fields PD_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline void w_mode10(mode10_t var){_mode10 = var & 0xf;}
			/* Pin 11 Mode */
            private: uint32_t   _mode11 : 4;
            /* read fields PD_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline mode11_t r_mode11(void){return (mode11_t)_mode11;}
            /* write fields PD_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline void w_mode11(mode11_t var){_mode11 = var & 0xf;}
			/* Pin 12 Mode */
            private: uint32_t   _mode12 : 4;
            /* read fields PD_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline mode12_t r_mode12(void){return (mode12_t)_mode12;}
            /* write fields PD_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline void w_mode12(mode12_t var){_mode12 = var & 0xf;}
			/* Pin 13 Mode */
            private: uint32_t   _mode13 : 4;
            /* read fields PD_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline mode13_t r_mode13(void){return (mode13_t)_mode13;}
            /* write fields PD_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline void w_mode13(mode13_t var){_mode13 = var & 0xf;}
			/* Pin 14 Mode */
            private: uint32_t   _mode14 : 4;
            /* read fields PD_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline mode14_t r_mode14(void){return (mode14_t)_mode14;}
            /* write fields PD_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline void w_mode14(mode14_t var){_mode14 = var & 0xf;}
			/* Pin 15 Mode */
            private: uint32_t   _mode15 : 4;
            /* read fields PD_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline mode15_t r_mode15(void){return (mode15_t)_mode15;}
            /* write fields PD_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline void w_mode15(mode15_t var){_mode15 = var & 0xf;}
        }fields;
    }PD_MODEH;
	/* Port Data Out Register */
    public: union PD_DOUT{ /* addr: 0x9c */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DOUT       = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out */
            private: uint32_t   _dout : 16;
            /* read fields PD_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline uint32_t r_dout(void){return (uint32_t)_dout;}
            /* write fields PD_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline void w_dout(uint32_t var){_dout = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PD_DOUT;
    private: uint8_t rsv_9[8];
	/* Port Data Out Toggle Register */
    public: union PD_DOUTTGL{ /* addr: 0xa8 */ 
        WriteOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DOUTTGL    = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out Toggle */
            private: uint32_t   _douttgl : 16;
            /* write fields PD_DOUTTGL.DOUTTGL:uint32_t(16 bit)	(Data Out Toggle) */
            public: inline void w_douttgl(uint32_t var){_douttgl = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PD_DOUTTGL;
	/* Port Data in Register */
    public: union PD_DIN{ /* addr: 0xac */ 
        ReadOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DIN        = 0x0000ffff,
        };
        public: struct fields{
			/* Data in */
            private: uint32_t   _din : 16;
            /* read fields PD_DIN.DIN:uint32_t(16 bit)	(Data in) */
            public: inline uint32_t r_din(void){return (uint32_t)_din;}
            private: uint32_t : 16;
        }fields;
    }PD_DIN;
	/* Port Unlocked Pins Register */
    public: union PD_PINLOCKN{ /* addr: 0xb0 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_PINLOCKN   = 0x0000ffff,
        };
        public: struct fields{
			/* Unlocked Pins */
            private: uint32_t   _pinlockn : 16;
            /* read fields PD_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline uint32_t r_pinlockn(void){return (uint32_t)_pinlockn;}
            /* write fields PD_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline void w_pinlockn(uint32_t var){_pinlockn = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PD_PINLOCKN;
    private: uint8_t rsv_10[4];
	/* Over Voltage Disable for All Modes */
    public: union PD_OVTDIS{ /* addr: 0xb8 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_OVTDIS     = 0x0000ffff,
        };
        public: struct fields{
			/* Disable Over Voltage Capability */
            private: uint32_t   _ovtdis : 16;
            /* read fields PD_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline uint32_t r_ovtdis(void){return (uint32_t)_ovtdis;}
            /* write fields PD_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline void w_ovtdis(uint32_t var){_ovtdis = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PD_OVTDIS;
    private: uint8_t rsv_11[4];
	/* Port Control Register */
    public: union PE_CTRL{ /* addr: 0xc0 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DRIVESTRENGTH = 0x00000001,
            MASK_SLEWRATE   = 0x00000070,
            MASK_DINDIS     = 0x00001000,
            MASK_DRIVESTRENGTHALT = 0x00010000,
            MASK_SLEWRATEALT = 0x00700000,
            MASK_DINDISALT  = 0x10000000,
        };
        public: struct fields{
			/* Drive Strength for Port */
            private: uint32_t   _drivestrength : 1;
            /* read fields PE_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline uint32_t r_drivestrength(void){return (uint32_t)_drivestrength;}
            /* write fields PE_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline void w_drivestrength(uint32_t var){_drivestrength = var & 0x1;}
            private: uint32_t   : 3;
			/* Slewrate Limit for Port */
            private: uint32_t   _slewrate : 3;
            /* read fields PE_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline uint32_t r_slewrate(void){return (uint32_t)_slewrate;}
            /* write fields PE_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline void w_slewrate(uint32_t var){_slewrate = var & 0x7;}
            private: uint32_t   : 5;
			/* Data in Disable */
            private: uint32_t   _dindis : 1;
            /* read fields PE_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline uint32_t r_dindis(void){return (uint32_t)_dindis;}
            /* write fields PE_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline void w_dindis(uint32_t var){_dindis = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Drive Strength for Port */
            private: uint32_t   _drivestrengthalt : 1;
            /* read fields PE_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline uint32_t r_drivestrengthalt(void){return (uint32_t)_drivestrengthalt;}
            /* write fields PE_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline void w_drivestrengthalt(uint32_t var){_drivestrengthalt = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Slewrate Limit for Port */
            private: uint32_t   _slewratealt : 3;
            /* read fields PE_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline uint32_t r_slewratealt(void){return (uint32_t)_slewratealt;}
            /* write fields PE_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline void w_slewratealt(uint32_t var){_slewratealt = var & 0x7;}
            private: uint32_t   : 5;
			/* Alternate Data in Disable */
            private: uint32_t   _dindisalt : 1;
            /* read fields PE_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline uint32_t r_dindisalt(void){return (uint32_t)_dindisalt;}
            /* write fields PE_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline void w_dindisalt(uint32_t var){_dindisalt = var & 0x1;}
            private: uint32_t : 3;
        }fields;
    }PE_CTRL;
	/* Port Pin Mode Low Register */
    public: union PE_MODEL{ /* addr: 0xc4 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE0      = 0x0000000f,
            MASK_MODE1      = 0x000000f0,
            MASK_MODE2      = 0x00000f00,
            MASK_MODE3      = 0x0000f000,
            MASK_MODE4      = 0x000f0000,
            MASK_MODE5      = 0x00f00000,
            MASK_MODE6      = 0x0f000000,
            MASK_MODE7      = 0xf0000000,
        };
		/* Pin 0 Mode */
        public: enum mode0_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE0_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE0_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE0_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE0_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE0_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE0_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE0_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE0_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE0_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE0_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE0_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE0_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE0_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE0_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE0_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE0_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 1 Mode */
        public: enum mode1_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE1_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE1_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE1_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE1_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE1_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE1_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE1_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE1_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE1_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE1_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE1_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE1_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE1_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE1_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE1_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE1_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 2 Mode */
        public: enum mode2_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE2_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE2_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE2_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE2_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE2_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE2_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE2_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE2_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE2_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE2_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE2_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE2_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE2_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE2_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE2_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE2_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 3 Mode */
        public: enum mode3_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE3_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE3_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE3_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE3_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE3_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE3_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE3_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE3_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE3_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE3_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE3_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE3_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE3_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE3_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE3_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE3_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 4 Mode */
        public: enum mode4_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE4_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE4_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE4_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE4_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE4_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE4_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE4_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE4_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE4_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE4_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE4_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE4_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE4_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE4_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE4_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE4_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 5 Mode */
        public: enum mode5_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE5_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE5_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE5_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE5_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE5_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE5_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE5_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE5_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE5_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE5_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE5_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE5_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE5_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE5_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE5_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE5_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 6 Mode */
        public: enum mode6_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE6_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE6_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE6_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE6_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE6_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE6_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE6_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE6_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE6_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE6_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE6_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE6_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE6_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE6_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE6_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE6_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 7 Mode */
        public: enum mode7_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE7_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE7_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE7_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE7_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE7_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE7_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE7_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE7_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE7_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE7_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE7_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE7_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE7_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE7_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE7_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE7_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 0 Mode */
            private: uint32_t   _mode0 : 4;
            /* read fields PE_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline mode0_t r_mode0(void){return (mode0_t)_mode0;}
            /* write fields PE_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline void w_mode0(mode0_t var){_mode0 = var & 0xf;}
			/* Pin 1 Mode */
            private: uint32_t   _mode1 : 4;
            /* read fields PE_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline mode1_t r_mode1(void){return (mode1_t)_mode1;}
            /* write fields PE_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline void w_mode1(mode1_t var){_mode1 = var & 0xf;}
			/* Pin 2 Mode */
            private: uint32_t   _mode2 : 4;
            /* read fields PE_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline mode2_t r_mode2(void){return (mode2_t)_mode2;}
            /* write fields PE_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline void w_mode2(mode2_t var){_mode2 = var & 0xf;}
			/* Pin 3 Mode */
            private: uint32_t   _mode3 : 4;
            /* read fields PE_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline mode3_t r_mode3(void){return (mode3_t)_mode3;}
            /* write fields PE_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline void w_mode3(mode3_t var){_mode3 = var & 0xf;}
			/* Pin 4 Mode */
            private: uint32_t   _mode4 : 4;
            /* read fields PE_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline mode4_t r_mode4(void){return (mode4_t)_mode4;}
            /* write fields PE_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline void w_mode4(mode4_t var){_mode4 = var & 0xf;}
			/* Pin 5 Mode */
            private: uint32_t   _mode5 : 4;
            /* read fields PE_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline mode5_t r_mode5(void){return (mode5_t)_mode5;}
            /* write fields PE_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline void w_mode5(mode5_t var){_mode5 = var & 0xf;}
			/* Pin 6 Mode */
            private: uint32_t   _mode6 : 4;
            /* read fields PE_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline mode6_t r_mode6(void){return (mode6_t)_mode6;}
            /* write fields PE_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline void w_mode6(mode6_t var){_mode6 = var & 0xf;}
			/* Pin 7 Mode */
            private: uint32_t   _mode7 : 4;
            /* read fields PE_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline mode7_t r_mode7(void){return (mode7_t)_mode7;}
            /* write fields PE_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline void w_mode7(mode7_t var){_mode7 = var & 0xf;}
        }fields;
    }PE_MODEL;
	/* Port Pin Mode High Register */
    public: union PE_MODEH{ /* addr: 0xc8 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE8      = 0x0000000f,
            MASK_MODE9      = 0x000000f0,
            MASK_MODE10     = 0x00000f00,
            MASK_MODE11     = 0x0000f000,
            MASK_MODE12     = 0x000f0000,
            MASK_MODE13     = 0x00f00000,
            MASK_MODE14     = 0x0f000000,
            MASK_MODE15     = 0xf0000000,
        };
		/* Pin 8 Mode */
        public: enum mode8_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE8_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE8_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE8_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE8_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE8_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE8_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE8_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE8_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE8_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE8_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE8_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE8_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE8_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE8_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE8_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE8_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 9 Mode */
        public: enum mode9_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE9_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE9_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE9_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE9_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE9_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE9_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE9_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE9_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE9_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE9_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE9_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE9_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE9_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE9_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE9_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE9_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 10 Mode */
        public: enum mode10_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE10_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE10_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE10_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE10_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE10_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE10_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE10_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE10_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE10_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE10_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE10_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE10_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE10_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE10_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE10_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE10_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 11 Mode */
        public: enum mode11_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE11_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE11_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE11_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE11_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE11_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE11_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE11_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE11_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE11_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE11_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE11_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE11_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE11_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE11_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE11_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE11_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 12 Mode */
        public: enum mode12_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE12_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE12_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE12_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE12_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE12_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE12_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE12_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE12_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE12_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE12_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE12_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE12_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE12_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE12_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE12_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE12_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 13 Mode */
        public: enum mode13_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE13_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE13_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE13_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE13_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE13_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE13_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE13_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE13_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE13_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE13_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE13_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE13_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE13_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE13_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE13_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE13_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 14 Mode */
        public: enum mode14_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE14_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE14_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE14_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE14_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE14_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE14_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE14_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE14_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE14_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE14_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE14_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE14_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE14_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE14_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE14_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE14_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 15 Mode */
        public: enum mode15_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE15_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE15_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE15_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE15_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE15_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE15_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE15_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE15_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE15_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE15_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE15_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE15_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE15_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE15_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE15_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE15_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 8 Mode */
            private: uint32_t   _mode8 : 4;
            /* read fields PE_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline mode8_t r_mode8(void){return (mode8_t)_mode8;}
            /* write fields PE_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline void w_mode8(mode8_t var){_mode8 = var & 0xf;}
			/* Pin 9 Mode */
            private: uint32_t   _mode9 : 4;
            /* read fields PE_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline mode9_t r_mode9(void){return (mode9_t)_mode9;}
            /* write fields PE_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline void w_mode9(mode9_t var){_mode9 = var & 0xf;}
			/* Pin 10 Mode */
            private: uint32_t   _mode10 : 4;
            /* read fields PE_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline mode10_t r_mode10(void){return (mode10_t)_mode10;}
            /* write fields PE_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline void w_mode10(mode10_t var){_mode10 = var & 0xf;}
			/* Pin 11 Mode */
            private: uint32_t   _mode11 : 4;
            /* read fields PE_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline mode11_t r_mode11(void){return (mode11_t)_mode11;}
            /* write fields PE_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline void w_mode11(mode11_t var){_mode11 = var & 0xf;}
			/* Pin 12 Mode */
            private: uint32_t   _mode12 : 4;
            /* read fields PE_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline mode12_t r_mode12(void){return (mode12_t)_mode12;}
            /* write fields PE_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline void w_mode12(mode12_t var){_mode12 = var & 0xf;}
			/* Pin 13 Mode */
            private: uint32_t   _mode13 : 4;
            /* read fields PE_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline mode13_t r_mode13(void){return (mode13_t)_mode13;}
            /* write fields PE_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline void w_mode13(mode13_t var){_mode13 = var & 0xf;}
			/* Pin 14 Mode */
            private: uint32_t   _mode14 : 4;
            /* read fields PE_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline mode14_t r_mode14(void){return (mode14_t)_mode14;}
            /* write fields PE_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline void w_mode14(mode14_t var){_mode14 = var & 0xf;}
			/* Pin 15 Mode */
            private: uint32_t   _mode15 : 4;
            /* read fields PE_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline mode15_t r_mode15(void){return (mode15_t)_mode15;}
            /* write fields PE_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline void w_mode15(mode15_t var){_mode15 = var & 0xf;}
        }fields;
    }PE_MODEH;
	/* Port Data Out Register */
    public: union PE_DOUT{ /* addr: 0xcc */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DOUT       = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out */
            private: uint32_t   _dout : 16;
            /* read fields PE_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline uint32_t r_dout(void){return (uint32_t)_dout;}
            /* write fields PE_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline void w_dout(uint32_t var){_dout = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PE_DOUT;
    private: uint8_t rsv_12[8];
	/* Port Data Out Toggle Register */
    public: union PE_DOUTTGL{ /* addr: 0xd8 */ 
        WriteOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DOUTTGL    = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out Toggle */
            private: uint32_t   _douttgl : 16;
            /* write fields PE_DOUTTGL.DOUTTGL:uint32_t(16 bit)	(Data Out Toggle) */
            public: inline void w_douttgl(uint32_t var){_douttgl = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PE_DOUTTGL;
	/* Port Data in Register */
    public: union PE_DIN{ /* addr: 0xdc */ 
        ReadOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DIN        = 0x0000ffff,
        };
        public: struct fields{
			/* Data in */
            private: uint32_t   _din : 16;
            /* read fields PE_DIN.DIN:uint32_t(16 bit)	(Data in) */
            public: inline uint32_t r_din(void){return (uint32_t)_din;}
            private: uint32_t : 16;
        }fields;
    }PE_DIN;
	/* Port Unlocked Pins Register */
    public: union PE_PINLOCKN{ /* addr: 0xe0 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_PINLOCKN   = 0x0000ffff,
        };
        public: struct fields{
			/* Unlocked Pins */
            private: uint32_t   _pinlockn : 16;
            /* read fields PE_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline uint32_t r_pinlockn(void){return (uint32_t)_pinlockn;}
            /* write fields PE_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline void w_pinlockn(uint32_t var){_pinlockn = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PE_PINLOCKN;
    private: uint8_t rsv_13[4];
	/* Over Voltage Disable for All Modes */
    public: union PE_OVTDIS{ /* addr: 0xe8 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_OVTDIS     = 0x0000ffff,
        };
        public: struct fields{
			/* Disable Over Voltage Capability */
            private: uint32_t   _ovtdis : 16;
            /* read fields PE_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline uint32_t r_ovtdis(void){return (uint32_t)_ovtdis;}
            /* write fields PE_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline void w_ovtdis(uint32_t var){_ovtdis = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PE_OVTDIS;
    private: uint8_t rsv_14[4];
	/* Port Control Register */
    public: union PF_CTRL{ /* addr: 0xf0 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DRIVESTRENGTH = 0x00000001,
            MASK_SLEWRATE   = 0x00000070,
            MASK_DINDIS     = 0x00001000,
            MASK_DRIVESTRENGTHALT = 0x00010000,
            MASK_SLEWRATEALT = 0x00700000,
            MASK_DINDISALT  = 0x10000000,
        };
        public: struct fields{
			/* Drive Strength for Port */
            private: uint32_t   _drivestrength : 1;
            /* read fields PF_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline uint32_t r_drivestrength(void){return (uint32_t)_drivestrength;}
            /* write fields PF_CTRL.DRIVESTRENGTH:uint32_t(1 bit)	(Drive Strength for Port) */
            public: inline void w_drivestrength(uint32_t var){_drivestrength = var & 0x1;}
            private: uint32_t   : 3;
			/* Slewrate Limit for Port */
            private: uint32_t   _slewrate : 3;
            /* read fields PF_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline uint32_t r_slewrate(void){return (uint32_t)_slewrate;}
            /* write fields PF_CTRL.SLEWRATE:uint32_t(3 bit)	(Slewrate Limit for Port) */
            public: inline void w_slewrate(uint32_t var){_slewrate = var & 0x7;}
            private: uint32_t   : 5;
			/* Data in Disable */
            private: uint32_t   _dindis : 1;
            /* read fields PF_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline uint32_t r_dindis(void){return (uint32_t)_dindis;}
            /* write fields PF_CTRL.DINDIS:uint32_t(1 bit)	(Data in Disable) */
            public: inline void w_dindis(uint32_t var){_dindis = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Drive Strength for Port */
            private: uint32_t   _drivestrengthalt : 1;
            /* read fields PF_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline uint32_t r_drivestrengthalt(void){return (uint32_t)_drivestrengthalt;}
            /* write fields PF_CTRL.DRIVESTRENGTHALT:uint32_t(1 bit)	(Alternate Drive Strength for Port) */
            public: inline void w_drivestrengthalt(uint32_t var){_drivestrengthalt = var & 0x1;}
            private: uint32_t   : 3;
			/* Alternate Slewrate Limit for Port */
            private: uint32_t   _slewratealt : 3;
            /* read fields PF_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline uint32_t r_slewratealt(void){return (uint32_t)_slewratealt;}
            /* write fields PF_CTRL.SLEWRATEALT:uint32_t(3 bit)	(Alternate Slewrate Limit for Port) */
            public: inline void w_slewratealt(uint32_t var){_slewratealt = var & 0x7;}
            private: uint32_t   : 5;
			/* Alternate Data in Disable */
            private: uint32_t   _dindisalt : 1;
            /* read fields PF_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline uint32_t r_dindisalt(void){return (uint32_t)_dindisalt;}
            /* write fields PF_CTRL.DINDISALT:uint32_t(1 bit)	(Alternate Data in Disable) */
            public: inline void w_dindisalt(uint32_t var){_dindisalt = var & 0x1;}
            private: uint32_t : 3;
        }fields;
    }PF_CTRL;
	/* Port Pin Mode Low Register */
    public: union PF_MODEL{ /* addr: 0xf4 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE0      = 0x0000000f,
            MASK_MODE1      = 0x000000f0,
            MASK_MODE2      = 0x00000f00,
            MASK_MODE3      = 0x0000f000,
            MASK_MODE4      = 0x000f0000,
            MASK_MODE5      = 0x00f00000,
            MASK_MODE6      = 0x0f000000,
            MASK_MODE7      = 0xf0000000,
        };
		/* Pin 0 Mode */
        public: enum mode0_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE0_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE0_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE0_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE0_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE0_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE0_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE0_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE0_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE0_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE0_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE0_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE0_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE0_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE0_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE0_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE0_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 1 Mode */
        public: enum mode1_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE1_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE1_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE1_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE1_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE1_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE1_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE1_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE1_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE1_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE1_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE1_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE1_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE1_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE1_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE1_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE1_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 2 Mode */
        public: enum mode2_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE2_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE2_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE2_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE2_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE2_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE2_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE2_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE2_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE2_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE2_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE2_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE2_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE2_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE2_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE2_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE2_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 3 Mode */
        public: enum mode3_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE3_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE3_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE3_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE3_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE3_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE3_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE3_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE3_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE3_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE3_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE3_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE3_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE3_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE3_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE3_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE3_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 4 Mode */
        public: enum mode4_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE4_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE4_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE4_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE4_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE4_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE4_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE4_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE4_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE4_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE4_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE4_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE4_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE4_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE4_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE4_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE4_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 5 Mode */
        public: enum mode5_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE5_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE5_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE5_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE5_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE5_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE5_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE5_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE5_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE5_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE5_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE5_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE5_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE5_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE5_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE5_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE5_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 6 Mode */
        public: enum mode6_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE6_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE6_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE6_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE6_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE6_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE6_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE6_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE6_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE6_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE6_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE6_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE6_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE6_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE6_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE6_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE6_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 7 Mode */
        public: enum mode7_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE7_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE7_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE7_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE7_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE7_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE7_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE7_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE7_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE7_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE7_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE7_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE7_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE7_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE7_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE7_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE7_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 0 Mode */
            private: uint32_t   _mode0 : 4;
            /* read fields PF_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline mode0_t r_mode0(void){return (mode0_t)_mode0;}
            /* write fields PF_MODEL.MODE0:mode0_t(4 bit)	(Pin 0 Mode) */
            public: inline void w_mode0(mode0_t var){_mode0 = var & 0xf;}
			/* Pin 1 Mode */
            private: uint32_t   _mode1 : 4;
            /* read fields PF_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline mode1_t r_mode1(void){return (mode1_t)_mode1;}
            /* write fields PF_MODEL.MODE1:mode1_t(4 bit)	(Pin 1 Mode) */
            public: inline void w_mode1(mode1_t var){_mode1 = var & 0xf;}
			/* Pin 2 Mode */
            private: uint32_t   _mode2 : 4;
            /* read fields PF_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline mode2_t r_mode2(void){return (mode2_t)_mode2;}
            /* write fields PF_MODEL.MODE2:mode2_t(4 bit)	(Pin 2 Mode) */
            public: inline void w_mode2(mode2_t var){_mode2 = var & 0xf;}
			/* Pin 3 Mode */
            private: uint32_t   _mode3 : 4;
            /* read fields PF_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline mode3_t r_mode3(void){return (mode3_t)_mode3;}
            /* write fields PF_MODEL.MODE3:mode3_t(4 bit)	(Pin 3 Mode) */
            public: inline void w_mode3(mode3_t var){_mode3 = var & 0xf;}
			/* Pin 4 Mode */
            private: uint32_t   _mode4 : 4;
            /* read fields PF_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline mode4_t r_mode4(void){return (mode4_t)_mode4;}
            /* write fields PF_MODEL.MODE4:mode4_t(4 bit)	(Pin 4 Mode) */
            public: inline void w_mode4(mode4_t var){_mode4 = var & 0xf;}
			/* Pin 5 Mode */
            private: uint32_t   _mode5 : 4;
            /* read fields PF_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline mode5_t r_mode5(void){return (mode5_t)_mode5;}
            /* write fields PF_MODEL.MODE5:mode5_t(4 bit)	(Pin 5 Mode) */
            public: inline void w_mode5(mode5_t var){_mode5 = var & 0xf;}
			/* Pin 6 Mode */
            private: uint32_t   _mode6 : 4;
            /* read fields PF_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline mode6_t r_mode6(void){return (mode6_t)_mode6;}
            /* write fields PF_MODEL.MODE6:mode6_t(4 bit)	(Pin 6 Mode) */
            public: inline void w_mode6(mode6_t var){_mode6 = var & 0xf;}
			/* Pin 7 Mode */
            private: uint32_t   _mode7 : 4;
            /* read fields PF_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline mode7_t r_mode7(void){return (mode7_t)_mode7;}
            /* write fields PF_MODEL.MODE7:mode7_t(4 bit)	(Pin 7 Mode) */
            public: inline void w_mode7(mode7_t var){_mode7 = var & 0xf;}
        }fields;
    }PF_MODEL;
	/* Port Pin Mode High Register */
    public: union PF_MODEH{ /* addr: 0xf8 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_MODE8      = 0x0000000f,
            MASK_MODE9      = 0x000000f0,
            MASK_MODE10     = 0x00000f00,
            MASK_MODE11     = 0x0000f000,
            MASK_MODE12     = 0x000f0000,
            MASK_MODE13     = 0x00f00000,
            MASK_MODE14     = 0x0f000000,
            MASK_MODE15     = 0xf0000000,
        };
		/* Pin 8 Mode */
        public: enum mode8_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE8_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE8_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE8_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE8_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE8_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE8_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE8_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE8_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE8_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE8_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE8_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE8_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE8_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE8_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE8_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE8_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 9 Mode */
        public: enum mode9_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE9_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE9_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE9_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE9_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE9_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE9_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE9_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE9_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE9_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE9_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE9_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE9_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE9_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE9_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE9_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE9_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 10 Mode */
        public: enum mode10_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE10_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE10_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE10_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE10_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE10_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE10_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE10_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE10_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE10_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE10_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE10_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE10_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE10_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE10_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE10_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE10_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 11 Mode */
        public: enum mode11_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE11_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE11_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE11_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE11_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE11_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE11_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE11_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE11_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE11_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE11_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE11_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE11_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE11_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE11_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE11_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE11_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 12 Mode */
        public: enum mode12_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE12_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE12_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE12_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE12_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE12_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE12_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE12_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE12_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE12_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE12_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE12_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE12_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE12_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE12_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE12_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE12_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 13 Mode */
        public: enum mode13_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE13_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE13_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE13_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE13_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE13_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE13_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE13_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE13_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE13_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE13_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE13_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE13_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE13_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE13_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE13_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE13_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 14 Mode */
        public: enum mode14_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE14_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE14_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE14_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE14_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE14_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE14_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE14_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE14_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE14_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE14_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE14_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE14_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE14_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE14_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE14_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE14_WIREDANDALTPULLUPFILTER = 15,
        };
		/* Pin 15 Mode */
        public: enum mode15_t {
			/* Input disabled. Pullup if DOUT is set. */
            MODE15_DISABLED   = 0,
			/* Input enabled. Filter if DOUT is set */
            MODE15_INPUT      = 1,
			/* Input enabled. DOUT determines pull direction */
            MODE15_INPUTPULL  = 2,
			/* Input enabled with filter. DOUT determines pull direction */
            MODE15_INPUTPULLFILTER = 3,
			/* Push-pull output */
            MODE15_PUSHPULL   = 4,
			/* Push-pull using alternate control */
            MODE15_PUSHPULLALT = 5,
			/* Wired-or output */
            MODE15_WIREDOR    = 6,
			/* Wired-or output with pull-down */
            MODE15_WIREDORPULLDOWN = 7,
			/* Open-drain output */
            MODE15_WIREDAND   = 8,
			/* Open-drain output with filter */
            MODE15_WIREDANDFILTER = 9,
			/* Open-drain output with pullup */
            MODE15_WIREDANDPULLUP = 10,
			/* Open-drain output with filter and pullup */
            MODE15_WIREDANDPULLUPFILTER = 11,
			/* Open-drain output using alternate control */
            MODE15_WIREDANDALT = 12,
			/* Open-drain output using alternate control with filter */
            MODE15_WIREDANDALTFILTER = 13,
			/* Open-drain output using alternate control with pullup */
            MODE15_WIREDANDALTPULLUP = 14,
			/* Open-drain output using alternate control with filter and pullup */
            MODE15_WIREDANDALTPULLUPFILTER = 15,
        };
        public: struct fields{
			/* Pin 8 Mode */
            private: uint32_t   _mode8 : 4;
            /* read fields PF_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline mode8_t r_mode8(void){return (mode8_t)_mode8;}
            /* write fields PF_MODEH.MODE8:mode8_t(4 bit)	(Pin 8 Mode) */
            public: inline void w_mode8(mode8_t var){_mode8 = var & 0xf;}
			/* Pin 9 Mode */
            private: uint32_t   _mode9 : 4;
            /* read fields PF_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline mode9_t r_mode9(void){return (mode9_t)_mode9;}
            /* write fields PF_MODEH.MODE9:mode9_t(4 bit)	(Pin 9 Mode) */
            public: inline void w_mode9(mode9_t var){_mode9 = var & 0xf;}
			/* Pin 10 Mode */
            private: uint32_t   _mode10 : 4;
            /* read fields PF_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline mode10_t r_mode10(void){return (mode10_t)_mode10;}
            /* write fields PF_MODEH.MODE10:mode10_t(4 bit)	(Pin 10 Mode) */
            public: inline void w_mode10(mode10_t var){_mode10 = var & 0xf;}
			/* Pin 11 Mode */
            private: uint32_t   _mode11 : 4;
            /* read fields PF_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline mode11_t r_mode11(void){return (mode11_t)_mode11;}
            /* write fields PF_MODEH.MODE11:mode11_t(4 bit)	(Pin 11 Mode) */
            public: inline void w_mode11(mode11_t var){_mode11 = var & 0xf;}
			/* Pin 12 Mode */
            private: uint32_t   _mode12 : 4;
            /* read fields PF_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline mode12_t r_mode12(void){return (mode12_t)_mode12;}
            /* write fields PF_MODEH.MODE12:mode12_t(4 bit)	(Pin 12 Mode) */
            public: inline void w_mode12(mode12_t var){_mode12 = var & 0xf;}
			/* Pin 13 Mode */
            private: uint32_t   _mode13 : 4;
            /* read fields PF_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline mode13_t r_mode13(void){return (mode13_t)_mode13;}
            /* write fields PF_MODEH.MODE13:mode13_t(4 bit)	(Pin 13 Mode) */
            public: inline void w_mode13(mode13_t var){_mode13 = var & 0xf;}
			/* Pin 14 Mode */
            private: uint32_t   _mode14 : 4;
            /* read fields PF_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline mode14_t r_mode14(void){return (mode14_t)_mode14;}
            /* write fields PF_MODEH.MODE14:mode14_t(4 bit)	(Pin 14 Mode) */
            public: inline void w_mode14(mode14_t var){_mode14 = var & 0xf;}
			/* Pin 15 Mode */
            private: uint32_t   _mode15 : 4;
            /* read fields PF_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline mode15_t r_mode15(void){return (mode15_t)_mode15;}
            /* write fields PF_MODEH.MODE15:mode15_t(4 bit)	(Pin 15 Mode) */
            public: inline void w_mode15(mode15_t var){_mode15 = var & 0xf;}
        }fields;
    }PF_MODEH;
	/* Port Data Out Register */
    public: union PF_DOUT{ /* addr: 0xfc */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_DOUT       = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out */
            private: uint32_t   _dout : 16;
            /* read fields PF_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline uint32_t r_dout(void){return (uint32_t)_dout;}
            /* write fields PF_DOUT.DOUT:uint32_t(16 bit)	(Data Out) */
            public: inline void w_dout(uint32_t var){_dout = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PF_DOUT;
    private: uint8_t rsv_15[8];
	/* Port Data Out Toggle Register */
    public: union PF_DOUTTGL{ /* addr: 0x108 */ 
        WriteOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DOUTTGL    = 0x0000ffff,
        };
        public: struct fields{
			/* Data Out Toggle */
            private: uint32_t   _douttgl : 16;
            /* write fields PF_DOUTTGL.DOUTTGL:uint32_t(16 bit)	(Data Out Toggle) */
            public: inline void w_douttgl(uint32_t var){_douttgl = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PF_DOUTTGL;
	/* Port Data in Register */
    public: union PF_DIN{ /* addr: 0x10c */ 
        ReadOnly<uint32_t> r;
        public: enum mask_t{
            MASK_DIN        = 0x0000ffff,
        };
        public: struct fields{
			/* Data in */
            private: uint32_t   _din : 16;
            /* read fields PF_DIN.DIN:uint32_t(16 bit)	(Data in) */
            public: inline uint32_t r_din(void){return (uint32_t)_din;}
            private: uint32_t : 16;
        }fields;
    }PF_DIN;
	/* Port Unlocked Pins Register */
    public: union PF_PINLOCKN{ /* addr: 0x110 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_PINLOCKN   = 0x0000ffff,
        };
        public: struct fields{
			/* Unlocked Pins */
            private: uint32_t   _pinlockn : 16;
            /* read fields PF_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline uint32_t r_pinlockn(void){return (uint32_t)_pinlockn;}
            /* write fields PF_PINLOCKN.PINLOCKN:uint32_t(16 bit)	(Unlocked Pins) */
            public: inline void w_pinlockn(uint32_t var){_pinlockn = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PF_PINLOCKN;
    private: uint8_t rsv_16[4];
	/* Over Voltage Disable for All Modes */
    public: union PF_OVTDIS{ /* addr: 0x118 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_OVTDIS     = 0x0000ffff,
        };
        public: struct fields{
			/* Disable Over Voltage Capability */
            private: uint32_t   _ovtdis : 16;
            /* read fields PF_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline uint32_t r_ovtdis(void){return (uint32_t)_ovtdis;}
            /* write fields PF_OVTDIS.OVTDIS:uint32_t(16 bit)	(Disable Over Voltage Capability) */
            public: inline void w_ovtdis(uint32_t var){_ovtdis = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }PF_OVTDIS;
    private: uint8_t rsv_17[740];
	/* External Interrupt Port Select Low Register */
    public: union EXTIPSELL{ /* addr: 0x400 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_EXTIPSEL0  = 0x0000000f,
            MASK_EXTIPSEL1  = 0x000000f0,
            MASK_EXTIPSEL2  = 0x00000f00,
            MASK_EXTIPSEL3  = 0x0000f000,
            MASK_EXTIPSEL4  = 0x000f0000,
            MASK_EXTIPSEL5  = 0x00f00000,
            MASK_EXTIPSEL6  = 0x0f000000,
            MASK_EXTIPSEL7  = 0xf0000000,
        };
		/* External Interrupt 0 Port Select */
        public: enum extipsel0_t {
			/* Port A group selected for external interrupt 0 */
            EXTIPSEL0_PORTA      = 0,
			/* Port B group selected for external interrupt 0 */
            EXTIPSEL0_PORTB      = 1,
			/* Port C group selected for external interrupt 0 */
            EXTIPSEL0_PORTC      = 2,
			/* Port D group selected for external interrupt 0 */
            EXTIPSEL0_PORTD      = 3,
			/* Port F group selected for external interrupt 0 */
            EXTIPSEL0_PORTF      = 5,
        };
		/* External Interrupt 1 Port Select */
        public: enum extipsel1_t {
			/* Port A group selected for external interrupt 1 */
            EXTIPSEL1_PORTA      = 0,
			/* Port B group selected for external interrupt 1 */
            EXTIPSEL1_PORTB      = 1,
			/* Port C group selected for external interrupt 1 */
            EXTIPSEL1_PORTC      = 2,
			/* Port D group selected for external interrupt 1 */
            EXTIPSEL1_PORTD      = 3,
			/* Port F group selected for external interrupt 1 */
            EXTIPSEL1_PORTF      = 5,
        };
		/* External Interrupt 2 Port Select */
        public: enum extipsel2_t {
			/* Port A group selected for external interrupt 2 */
            EXTIPSEL2_PORTA      = 0,
			/* Port B group selected for external interrupt 2 */
            EXTIPSEL2_PORTB      = 1,
			/* Port C group selected for external interrupt 2 */
            EXTIPSEL2_PORTC      = 2,
			/* Port D group selected for external interrupt 2 */
            EXTIPSEL2_PORTD      = 3,
			/* Port F group selected for external interrupt 2 */
            EXTIPSEL2_PORTF      = 5,
        };
		/* External Interrupt 3 Port Select */
        public: enum extipsel3_t {
			/* Port A group selected for external interrupt 3 */
            EXTIPSEL3_PORTA      = 0,
			/* Port B group selected for external interrupt 3 */
            EXTIPSEL3_PORTB      = 1,
			/* Port C group selected for external interrupt 3 */
            EXTIPSEL3_PORTC      = 2,
			/* Port D group selected for external interrupt 3 */
            EXTIPSEL3_PORTD      = 3,
			/* Port F group selected for external interrupt 3 */
            EXTIPSEL3_PORTF      = 5,
        };
		/* External Interrupt 4 Port Select */
        public: enum extipsel4_t {
			/* Port A group selected for external interrupt 4 */
            EXTIPSEL4_PORTA      = 0,
			/* Port B group selected for external interrupt 4 */
            EXTIPSEL4_PORTB      = 1,
			/* Port C group selected for external interrupt 4 */
            EXTIPSEL4_PORTC      = 2,
			/* Port D group selected for external interrupt 4 */
            EXTIPSEL4_PORTD      = 3,
			/* Port F group selected for external interrupt 4 */
            EXTIPSEL4_PORTF      = 5,
        };
		/* External Interrupt 5 Port Select */
        public: enum extipsel5_t {
			/* Port A group selected for external interrupt 5 */
            EXTIPSEL5_PORTA      = 0,
			/* Port B group selected for external interrupt 5 */
            EXTIPSEL5_PORTB      = 1,
			/* Port C group selected for external interrupt 5 */
            EXTIPSEL5_PORTC      = 2,
			/* Port D group selected for external interrupt 5 */
            EXTIPSEL5_PORTD      = 3,
			/* Port F group selected for external interrupt 5 */
            EXTIPSEL5_PORTF      = 5,
        };
		/* External Interrupt 6 Port Select */
        public: enum extipsel6_t {
			/* Port A group selected for external interrupt 6 */
            EXTIPSEL6_PORTA      = 0,
			/* Port B group selected for external interrupt 6 */
            EXTIPSEL6_PORTB      = 1,
			/* Port C group selected for external interrupt 6 */
            EXTIPSEL6_PORTC      = 2,
			/* Port D group selected for external interrupt 6 */
            EXTIPSEL6_PORTD      = 3,
			/* Port F group selected for external interrupt 6 */
            EXTIPSEL6_PORTF      = 5,
        };
		/* External Interrupt 7 Port Select */
        public: enum extipsel7_t {
			/* Port A group selected for external interrupt 7 */
            EXTIPSEL7_PORTA      = 0,
			/* Port B group selected for external interrupt 7 */
            EXTIPSEL7_PORTB      = 1,
			/* Port C group selected for external interrupt 7 */
            EXTIPSEL7_PORTC      = 2,
			/* Port D group selected for external interrupt 7 */
            EXTIPSEL7_PORTD      = 3,
			/* Port F group selected for external interrupt 7 */
            EXTIPSEL7_PORTF      = 5,
        };
        public: struct fields{
			/* External Interrupt 0 Port Select */
            private: uint32_t   _extipsel0 : 4;
            /* read fields EXTIPSELL.EXTIPSEL0:extipsel0_t(4 bit)	(External Interrupt 0 Port Select) */
            public: inline extipsel0_t r_extipsel0(void){return (extipsel0_t)_extipsel0;}
            /* write fields EXTIPSELL.EXTIPSEL0:extipsel0_t(4 bit)	(External Interrupt 0 Port Select) */
            public: inline void w_extipsel0(extipsel0_t var){_extipsel0 = var & 0xf;}
			/* External Interrupt 1 Port Select */
            private: uint32_t   _extipsel1 : 4;
            /* read fields EXTIPSELL.EXTIPSEL1:extipsel1_t(4 bit)	(External Interrupt 1 Port Select) */
            public: inline extipsel1_t r_extipsel1(void){return (extipsel1_t)_extipsel1;}
            /* write fields EXTIPSELL.EXTIPSEL1:extipsel1_t(4 bit)	(External Interrupt 1 Port Select) */
            public: inline void w_extipsel1(extipsel1_t var){_extipsel1 = var & 0xf;}
			/* External Interrupt 2 Port Select */
            private: uint32_t   _extipsel2 : 4;
            /* read fields EXTIPSELL.EXTIPSEL2:extipsel2_t(4 bit)	(External Interrupt 2 Port Select) */
            public: inline extipsel2_t r_extipsel2(void){return (extipsel2_t)_extipsel2;}
            /* write fields EXTIPSELL.EXTIPSEL2:extipsel2_t(4 bit)	(External Interrupt 2 Port Select) */
            public: inline void w_extipsel2(extipsel2_t var){_extipsel2 = var & 0xf;}
			/* External Interrupt 3 Port Select */
            private: uint32_t   _extipsel3 : 4;
            /* read fields EXTIPSELL.EXTIPSEL3:extipsel3_t(4 bit)	(External Interrupt 3 Port Select) */
            public: inline extipsel3_t r_extipsel3(void){return (extipsel3_t)_extipsel3;}
            /* write fields EXTIPSELL.EXTIPSEL3:extipsel3_t(4 bit)	(External Interrupt 3 Port Select) */
            public: inline void w_extipsel3(extipsel3_t var){_extipsel3 = var & 0xf;}
			/* External Interrupt 4 Port Select */
            private: uint32_t   _extipsel4 : 4;
            /* read fields EXTIPSELL.EXTIPSEL4:extipsel4_t(4 bit)	(External Interrupt 4 Port Select) */
            public: inline extipsel4_t r_extipsel4(void){return (extipsel4_t)_extipsel4;}
            /* write fields EXTIPSELL.EXTIPSEL4:extipsel4_t(4 bit)	(External Interrupt 4 Port Select) */
            public: inline void w_extipsel4(extipsel4_t var){_extipsel4 = var & 0xf;}
			/* External Interrupt 5 Port Select */
            private: uint32_t   _extipsel5 : 4;
            /* read fields EXTIPSELL.EXTIPSEL5:extipsel5_t(4 bit)	(External Interrupt 5 Port Select) */
            public: inline extipsel5_t r_extipsel5(void){return (extipsel5_t)_extipsel5;}
            /* write fields EXTIPSELL.EXTIPSEL5:extipsel5_t(4 bit)	(External Interrupt 5 Port Select) */
            public: inline void w_extipsel5(extipsel5_t var){_extipsel5 = var & 0xf;}
			/* External Interrupt 6 Port Select */
            private: uint32_t   _extipsel6 : 4;
            /* read fields EXTIPSELL.EXTIPSEL6:extipsel6_t(4 bit)	(External Interrupt 6 Port Select) */
            public: inline extipsel6_t r_extipsel6(void){return (extipsel6_t)_extipsel6;}
            /* write fields EXTIPSELL.EXTIPSEL6:extipsel6_t(4 bit)	(External Interrupt 6 Port Select) */
            public: inline void w_extipsel6(extipsel6_t var){_extipsel6 = var & 0xf;}
			/* External Interrupt 7 Port Select */
            private: uint32_t   _extipsel7 : 4;
            /* read fields EXTIPSELL.EXTIPSEL7:extipsel7_t(4 bit)	(External Interrupt 7 Port Select) */
            public: inline extipsel7_t r_extipsel7(void){return (extipsel7_t)_extipsel7;}
            /* write fields EXTIPSELL.EXTIPSEL7:extipsel7_t(4 bit)	(External Interrupt 7 Port Select) */
            public: inline void w_extipsel7(extipsel7_t var){_extipsel7 = var & 0xf;}
        }fields;
    }EXTIPSELL;
	/* External Interrupt Port Select High Register */
    public: union EXTIPSELH{ /* addr: 0x404 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_EXTIPSEL8  = 0x0000000f,
            MASK_EXTIPSEL9  = 0x000000f0,
            MASK_EXTIPSEL10 = 0x00000f00,
            MASK_EXTIPSEL11 = 0x0000f000,
            MASK_EXTIPSEL12 = 0x000f0000,
            MASK_EXTIPSEL13 = 0x00f00000,
            MASK_EXTIPSEL14 = 0x0f000000,
            MASK_EXTIPSEL15 = 0xf0000000,
        };
		/* External Interrupt 8 Port Select */
        public: enum extipsel8_t {
			/* Port A group selected for external interrupt 8 */
            EXTIPSEL8_PORTA      = 0,
			/* Port B group selected for external interrupt 8 */
            EXTIPSEL8_PORTB      = 1,
			/* Port C group selected for external interrupt 8 */
            EXTIPSEL8_PORTC      = 2,
			/* Port D group selected for external interrupt 8 */
            EXTIPSEL8_PORTD      = 3,
			/* Port F group selected for external interrupt 8 */
            EXTIPSEL8_PORTF      = 5,
        };
		/* External Interrupt 9 Port Select */
        public: enum extipsel9_t {
			/* Port A group selected for external interrupt 9 */
            EXTIPSEL9_PORTA      = 0,
			/* Port B group selected for external interrupt 9 */
            EXTIPSEL9_PORTB      = 1,
			/* Port C group selected for external interrupt 9 */
            EXTIPSEL9_PORTC      = 2,
			/* Port D group selected for external interrupt 9 */
            EXTIPSEL9_PORTD      = 3,
			/* Port F group selected for external interrupt 9 */
            EXTIPSEL9_PORTF      = 5,
        };
		/* External Interrupt 10 Port Select */
        public: enum extipsel10_t {
			/* Port A group selected for external interrupt 10 */
            EXTIPSEL10_PORTA      = 0,
			/* Port B group selected for external interrupt 10 */
            EXTIPSEL10_PORTB      = 1,
			/* Port C group selected for external interrupt 10 */
            EXTIPSEL10_PORTC      = 2,
			/* Port D group selected for external interrupt 10 */
            EXTIPSEL10_PORTD      = 3,
			/* Port F group selected for external interrupt 10 */
            EXTIPSEL10_PORTF      = 5,
        };
		/* External Interrupt 11 Port Select */
        public: enum extipsel11_t {
			/* Port A group selected for external interrupt 11 */
            EXTIPSEL11_PORTA      = 0,
			/* Port B group selected for external interrupt 11 */
            EXTIPSEL11_PORTB      = 1,
			/* Port C group selected for external interrupt 11 */
            EXTIPSEL11_PORTC      = 2,
			/* Port D group selected for external interrupt 11 */
            EXTIPSEL11_PORTD      = 3,
			/* Port F group selected for external interrupt 11 */
            EXTIPSEL11_PORTF      = 5,
        };
		/* External Interrupt 12 Port Select */
        public: enum extipsel12_t {
			/* Port A group selected for external interrupt 12 */
            EXTIPSEL12_PORTA      = 0,
			/* Port B group selected for external interrupt 12 */
            EXTIPSEL12_PORTB      = 1,
			/* Port C group selected for external interrupt 12 */
            EXTIPSEL12_PORTC      = 2,
			/* Port D group selected for external interrupt 12 */
            EXTIPSEL12_PORTD      = 3,
			/* Port F group selected for external interrupt 12 */
            EXTIPSEL12_PORTF      = 5,
        };
		/* External Interrupt 13 Port Select */
        public: enum extipsel13_t {
			/* Port A group selected for external interrupt 13 */
            EXTIPSEL13_PORTA      = 0,
			/* Port B group selected for external interrupt 13 */
            EXTIPSEL13_PORTB      = 1,
			/* Port C group selected for external interrupt 13 */
            EXTIPSEL13_PORTC      = 2,
			/* Port D group selected for external interrupt 13 */
            EXTIPSEL13_PORTD      = 3,
			/* Port F group selected for external interrupt 13 */
            EXTIPSEL13_PORTF      = 5,
        };
		/* External Interrupt 14 Port Select */
        public: enum extipsel14_t {
			/* Port A group selected for external interrupt 14 */
            EXTIPSEL14_PORTA      = 0,
			/* Port B group selected for external interrupt 14 */
            EXTIPSEL14_PORTB      = 1,
			/* Port C group selected for external interrupt 14 */
            EXTIPSEL14_PORTC      = 2,
			/* Port D group selected for external interrupt 14 */
            EXTIPSEL14_PORTD      = 3,
			/* Port F group selected for external interrupt 14 */
            EXTIPSEL14_PORTF      = 5,
        };
		/* External Interrupt 15 Port Select */
        public: enum extipsel15_t {
			/* Port A group selected for external interrupt 15 */
            EXTIPSEL15_PORTA      = 0,
			/* Port B group selected for external interrupt 15 */
            EXTIPSEL15_PORTB      = 1,
			/* Port C group selected for external interrupt 15 */
            EXTIPSEL15_PORTC      = 2,
			/* Port D group selected for external interrupt 15 */
            EXTIPSEL15_PORTD      = 3,
			/* Port F group selected for external interrupt 15 */
            EXTIPSEL15_PORTF      = 5,
        };
        public: struct fields{
			/* External Interrupt 8 Port Select */
            private: uint32_t   _extipsel8 : 4;
            /* read fields EXTIPSELH.EXTIPSEL8:extipsel8_t(4 bit)	(External Interrupt 8 Port Select) */
            public: inline extipsel8_t r_extipsel8(void){return (extipsel8_t)_extipsel8;}
            /* write fields EXTIPSELH.EXTIPSEL8:extipsel8_t(4 bit)	(External Interrupt 8 Port Select) */
            public: inline void w_extipsel8(extipsel8_t var){_extipsel8 = var & 0xf;}
			/* External Interrupt 9 Port Select */
            private: uint32_t   _extipsel9 : 4;
            /* read fields EXTIPSELH.EXTIPSEL9:extipsel9_t(4 bit)	(External Interrupt 9 Port Select) */
            public: inline extipsel9_t r_extipsel9(void){return (extipsel9_t)_extipsel9;}
            /* write fields EXTIPSELH.EXTIPSEL9:extipsel9_t(4 bit)	(External Interrupt 9 Port Select) */
            public: inline void w_extipsel9(extipsel9_t var){_extipsel9 = var & 0xf;}
			/* External Interrupt 10 Port Select */
            private: uint32_t   _extipsel10 : 4;
            /* read fields EXTIPSELH.EXTIPSEL10:extipsel10_t(4 bit)	(External Interrupt 10 Port Select) */
            public: inline extipsel10_t r_extipsel10(void){return (extipsel10_t)_extipsel10;}
            /* write fields EXTIPSELH.EXTIPSEL10:extipsel10_t(4 bit)	(External Interrupt 10 Port Select) */
            public: inline void w_extipsel10(extipsel10_t var){_extipsel10 = var & 0xf;}
			/* External Interrupt 11 Port Select */
            private: uint32_t   _extipsel11 : 4;
            /* read fields EXTIPSELH.EXTIPSEL11:extipsel11_t(4 bit)	(External Interrupt 11 Port Select) */
            public: inline extipsel11_t r_extipsel11(void){return (extipsel11_t)_extipsel11;}
            /* write fields EXTIPSELH.EXTIPSEL11:extipsel11_t(4 bit)	(External Interrupt 11 Port Select) */
            public: inline void w_extipsel11(extipsel11_t var){_extipsel11 = var & 0xf;}
			/* External Interrupt 12 Port Select */
            private: uint32_t   _extipsel12 : 4;
            /* read fields EXTIPSELH.EXTIPSEL12:extipsel12_t(4 bit)	(External Interrupt 12 Port Select) */
            public: inline extipsel12_t r_extipsel12(void){return (extipsel12_t)_extipsel12;}
            /* write fields EXTIPSELH.EXTIPSEL12:extipsel12_t(4 bit)	(External Interrupt 12 Port Select) */
            public: inline void w_extipsel12(extipsel12_t var){_extipsel12 = var & 0xf;}
			/* External Interrupt 13 Port Select */
            private: uint32_t   _extipsel13 : 4;
            /* read fields EXTIPSELH.EXTIPSEL13:extipsel13_t(4 bit)	(External Interrupt 13 Port Select) */
            public: inline extipsel13_t r_extipsel13(void){return (extipsel13_t)_extipsel13;}
            /* write fields EXTIPSELH.EXTIPSEL13:extipsel13_t(4 bit)	(External Interrupt 13 Port Select) */
            public: inline void w_extipsel13(extipsel13_t var){_extipsel13 = var & 0xf;}
			/* External Interrupt 14 Port Select */
            private: uint32_t   _extipsel14 : 4;
            /* read fields EXTIPSELH.EXTIPSEL14:extipsel14_t(4 bit)	(External Interrupt 14 Port Select) */
            public: inline extipsel14_t r_extipsel14(void){return (extipsel14_t)_extipsel14;}
            /* write fields EXTIPSELH.EXTIPSEL14:extipsel14_t(4 bit)	(External Interrupt 14 Port Select) */
            public: inline void w_extipsel14(extipsel14_t var){_extipsel14 = var & 0xf;}
			/* External Interrupt 15 Port Select */
            private: uint32_t   _extipsel15 : 4;
            /* read fields EXTIPSELH.EXTIPSEL15:extipsel15_t(4 bit)	(External Interrupt 15 Port Select) */
            public: inline extipsel15_t r_extipsel15(void){return (extipsel15_t)_extipsel15;}
            /* write fields EXTIPSELH.EXTIPSEL15:extipsel15_t(4 bit)	(External Interrupt 15 Port Select) */
            public: inline void w_extipsel15(extipsel15_t var){_extipsel15 = var & 0xf;}
        }fields;
    }EXTIPSELH;
	/* External Interrupt Pin Select Low Register */
    public: union EXTIPINSELL{ /* addr: 0x408 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_EXTIPINSEL0 = 0x00000003,
            MASK_EXTIPINSEL1 = 0x00000030,
            MASK_EXTIPINSEL2 = 0x00000300,
            MASK_EXTIPINSEL3 = 0x00003000,
            MASK_EXTIPINSEL4 = 0x00030000,
            MASK_EXTIPINSEL5 = 0x00300000,
            MASK_EXTIPINSEL6 = 0x03000000,
            MASK_EXTIPINSEL7 = 0x30000000,
        };
		/* External Interrupt 0 Pin Select */
        public: enum extipinsel0_t {
			/* Pin 0 */
            EXTIPINSEL0_PIN0       = 0,
			/* Pin 1 */
            EXTIPINSEL0_PIN1       = 1,
			/* Pin 2 */
            EXTIPINSEL0_PIN2       = 2,
			/* Pin 3 */
            EXTIPINSEL0_PIN3       = 3,
        };
		/* External Interrupt 1 Pin Select */
        public: enum extipinsel1_t {
			/* Pin 0 */
            EXTIPINSEL1_PIN0       = 0,
			/* Pin 1 */
            EXTIPINSEL1_PIN1       = 1,
			/* Pin 2 */
            EXTIPINSEL1_PIN2       = 2,
			/* Pin 3 */
            EXTIPINSEL1_PIN3       = 3,
        };
		/* External Interrupt 2 Pin Select */
        public: enum extipinsel2_t {
			/* Pin 0 */
            EXTIPINSEL2_PIN0       = 0,
			/* Pin 1 */
            EXTIPINSEL2_PIN1       = 1,
			/* Pin 2 */
            EXTIPINSEL2_PIN2       = 2,
			/* Pin 3 */
            EXTIPINSEL2_PIN3       = 3,
        };
		/* External Interrupt 3 Pin Select */
        public: enum extipinsel3_t {
			/* Pin 0 */
            EXTIPINSEL3_PIN0       = 0,
			/* Pin 1 */
            EXTIPINSEL3_PIN1       = 1,
			/* Pin 2 */
            EXTIPINSEL3_PIN2       = 2,
			/* Pin 3 */
            EXTIPINSEL3_PIN3       = 3,
        };
		/* External Interrupt 4 Pin Select */
        public: enum extipinsel4_t {
			/* Pin 4 */
            EXTIPINSEL4_PIN4       = 0,
			/* Pin 5 */
            EXTIPINSEL4_PIN5       = 1,
			/* Pin 6 */
            EXTIPINSEL4_PIN6       = 2,
			/* Pin 7 */
            EXTIPINSEL4_PIN7       = 3,
        };
		/* External Interrupt 5 Pin Select */
        public: enum extipinsel5_t {
			/* Pin 4 */
            EXTIPINSEL5_PIN4       = 0,
			/* Pin 5 */
            EXTIPINSEL5_PIN5       = 1,
			/* Pin 6 */
            EXTIPINSEL5_PIN6       = 2,
			/* Pin 7 */
            EXTIPINSEL5_PIN7       = 3,
        };
		/* External Interrupt 6 Pin Select */
        public: enum extipinsel6_t {
			/* Pin 4 */
            EXTIPINSEL6_PIN4       = 0,
			/* Pin 5 */
            EXTIPINSEL6_PIN5       = 1,
			/* Pin 6 */
            EXTIPINSEL6_PIN6       = 2,
			/* Pin 7 */
            EXTIPINSEL6_PIN7       = 3,
        };
		/* External Interrupt 7 Pin Select */
        public: enum extipinsel7_t {
			/* Pin 4 */
            EXTIPINSEL7_PIN4       = 0,
			/* Pin 5 */
            EXTIPINSEL7_PIN5       = 1,
			/* Pin 6 */
            EXTIPINSEL7_PIN6       = 2,
			/* Pin 7 */
            EXTIPINSEL7_PIN7       = 3,
        };
        public: struct fields{
			/* External Interrupt 0 Pin Select */
            private: uint32_t   _extipinsel0 : 2;
            /* read fields EXTIPINSELL.EXTIPINSEL0:extipinsel0_t(2 bit)	(External Interrupt 0 Pin Select) */
            public: inline extipinsel0_t r_extipinsel0(void){return (extipinsel0_t)_extipinsel0;}
            /* write fields EXTIPINSELL.EXTIPINSEL0:extipinsel0_t(2 bit)	(External Interrupt 0 Pin Select) */
            public: inline void w_extipinsel0(extipinsel0_t var){_extipinsel0 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 1 Pin Select */
            private: uint32_t   _extipinsel1 : 2;
            /* read fields EXTIPINSELL.EXTIPINSEL1:extipinsel1_t(2 bit)	(External Interrupt 1 Pin Select) */
            public: inline extipinsel1_t r_extipinsel1(void){return (extipinsel1_t)_extipinsel1;}
            /* write fields EXTIPINSELL.EXTIPINSEL1:extipinsel1_t(2 bit)	(External Interrupt 1 Pin Select) */
            public: inline void w_extipinsel1(extipinsel1_t var){_extipinsel1 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 2 Pin Select */
            private: uint32_t   _extipinsel2 : 2;
            /* read fields EXTIPINSELL.EXTIPINSEL2:extipinsel2_t(2 bit)	(External Interrupt 2 Pin Select) */
            public: inline extipinsel2_t r_extipinsel2(void){return (extipinsel2_t)_extipinsel2;}
            /* write fields EXTIPINSELL.EXTIPINSEL2:extipinsel2_t(2 bit)	(External Interrupt 2 Pin Select) */
            public: inline void w_extipinsel2(extipinsel2_t var){_extipinsel2 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 3 Pin Select */
            private: uint32_t   _extipinsel3 : 2;
            /* read fields EXTIPINSELL.EXTIPINSEL3:extipinsel3_t(2 bit)	(External Interrupt 3 Pin Select) */
            public: inline extipinsel3_t r_extipinsel3(void){return (extipinsel3_t)_extipinsel3;}
            /* write fields EXTIPINSELL.EXTIPINSEL3:extipinsel3_t(2 bit)	(External Interrupt 3 Pin Select) */
            public: inline void w_extipinsel3(extipinsel3_t var){_extipinsel3 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 4 Pin Select */
            private: uint32_t   _extipinsel4 : 2;
            /* read fields EXTIPINSELL.EXTIPINSEL4:extipinsel4_t(2 bit)	(External Interrupt 4 Pin Select) */
            public: inline extipinsel4_t r_extipinsel4(void){return (extipinsel4_t)_extipinsel4;}
            /* write fields EXTIPINSELL.EXTIPINSEL4:extipinsel4_t(2 bit)	(External Interrupt 4 Pin Select) */
            public: inline void w_extipinsel4(extipinsel4_t var){_extipinsel4 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 5 Pin Select */
            private: uint32_t   _extipinsel5 : 2;
            /* read fields EXTIPINSELL.EXTIPINSEL5:extipinsel5_t(2 bit)	(External Interrupt 5 Pin Select) */
            public: inline extipinsel5_t r_extipinsel5(void){return (extipinsel5_t)_extipinsel5;}
            /* write fields EXTIPINSELL.EXTIPINSEL5:extipinsel5_t(2 bit)	(External Interrupt 5 Pin Select) */
            public: inline void w_extipinsel5(extipinsel5_t var){_extipinsel5 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 6 Pin Select */
            private: uint32_t   _extipinsel6 : 2;
            /* read fields EXTIPINSELL.EXTIPINSEL6:extipinsel6_t(2 bit)	(External Interrupt 6 Pin Select) */
            public: inline extipinsel6_t r_extipinsel6(void){return (extipinsel6_t)_extipinsel6;}
            /* write fields EXTIPINSELL.EXTIPINSEL6:extipinsel6_t(2 bit)	(External Interrupt 6 Pin Select) */
            public: inline void w_extipinsel6(extipinsel6_t var){_extipinsel6 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 7 Pin Select */
            private: uint32_t   _extipinsel7 : 2;
            /* read fields EXTIPINSELL.EXTIPINSEL7:extipinsel7_t(2 bit)	(External Interrupt 7 Pin Select) */
            public: inline extipinsel7_t r_extipinsel7(void){return (extipinsel7_t)_extipinsel7;}
            /* write fields EXTIPINSELL.EXTIPINSEL7:extipinsel7_t(2 bit)	(External Interrupt 7 Pin Select) */
            public: inline void w_extipinsel7(extipinsel7_t var){_extipinsel7 = var & 0x3;}
            private: uint32_t : 2;
        }fields;
    }EXTIPINSELL;
	/* External Interrupt Pin Select High Register */
    public: union EXTIPINSELH{ /* addr: 0x40c */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_EXTIPINSEL8 = 0x00000003,
            MASK_EXTIPINSEL9 = 0x00000030,
            MASK_EXTIPINSEL10 = 0x00000300,
            MASK_EXTIPINSEL11 = 0x00003000,
            MASK_EXTIPINSEL12 = 0x00030000,
            MASK_EXTIPINSEL13 = 0x00300000,
            MASK_EXTIPINSEL14 = 0x03000000,
            MASK_EXTIPINSEL15 = 0x30000000,
        };
		/* External Interrupt 8 Pin Select */
        public: enum extipinsel8_t {
			/* Pin 8 */
            EXTIPINSEL8_PIN8       = 0,
			/* Pin 9 */
            EXTIPINSEL8_PIN9       = 1,
			/* Pin 10 */
            EXTIPINSEL8_PIN10      = 2,
			/* Pin 11 */
            EXTIPINSEL8_PIN11      = 3,
        };
		/* External Interrupt 9 Pin Select */
        public: enum extipinsel9_t {
			/* Pin 8 */
            EXTIPINSEL9_PIN8       = 0,
			/* Pin 9 */
            EXTIPINSEL9_PIN9       = 1,
			/* Pin 10 */
            EXTIPINSEL9_PIN10      = 2,
			/* Pin 11 */
            EXTIPINSEL9_PIN11      = 3,
        };
		/* External Interrupt 10 Pin Select */
        public: enum extipinsel10_t {
			/* Pin 8 */
            EXTIPINSEL10_PIN8       = 0,
			/* Pin 9 */
            EXTIPINSEL10_PIN9       = 1,
			/* Pin 10 */
            EXTIPINSEL10_PIN10      = 2,
			/* Pin 11 */
            EXTIPINSEL10_PIN11      = 3,
        };
		/* External Interrupt 11 Pin Select */
        public: enum extipinsel11_t {
			/* Pin 8 */
            EXTIPINSEL11_PIN8       = 0,
			/* Pin 9 */
            EXTIPINSEL11_PIN9       = 1,
			/* Pin 10 */
            EXTIPINSEL11_PIN10      = 2,
			/* Pin 11 */
            EXTIPINSEL11_PIN11      = 3,
        };
		/* External Interrupt 12 Pin Select */
        public: enum extipinsel12_t {
			/* Pin 12 */
            EXTIPINSEL12_PIN12      = 0,
			/* Pin 13 */
            EXTIPINSEL12_PIN13      = 1,
			/* Pin 14 */
            EXTIPINSEL12_PIN14      = 2,
			/* Pin 15 */
            EXTIPINSEL12_PIN15      = 3,
        };
		/* External Interrupt 13 Pin Select */
        public: enum extipinsel13_t {
			/* Pin 12 */
            EXTIPINSEL13_PIN12      = 0,
			/* Pin 13 */
            EXTIPINSEL13_PIN13      = 1,
			/* Pin 14 */
            EXTIPINSEL13_PIN14      = 2,
			/* Pin 15 */
            EXTIPINSEL13_PIN15      = 3,
        };
		/* External Interrupt 14 Pin Select */
        public: enum extipinsel14_t {
			/* Pin 12 */
            EXTIPINSEL14_PIN12      = 0,
			/* Pin 13 */
            EXTIPINSEL14_PIN13      = 1,
			/* Pin 14 */
            EXTIPINSEL14_PIN14      = 2,
			/* Pin 15 */
            EXTIPINSEL14_PIN15      = 3,
        };
		/* External Interrupt 15 Pin Select */
        public: enum extipinsel15_t {
			/* Pin 12 */
            EXTIPINSEL15_PIN12      = 0,
			/* Pin 13 */
            EXTIPINSEL15_PIN13      = 1,
			/* Pin 14 */
            EXTIPINSEL15_PIN14      = 2,
			/* Pin 15 */
            EXTIPINSEL15_PIN15      = 3,
        };
        public: struct fields{
			/* External Interrupt 8 Pin Select */
            private: uint32_t   _extipinsel8 : 2;
            /* read fields EXTIPINSELH.EXTIPINSEL8:extipinsel8_t(2 bit)	(External Interrupt 8 Pin Select) */
            public: inline extipinsel8_t r_extipinsel8(void){return (extipinsel8_t)_extipinsel8;}
            /* write fields EXTIPINSELH.EXTIPINSEL8:extipinsel8_t(2 bit)	(External Interrupt 8 Pin Select) */
            public: inline void w_extipinsel8(extipinsel8_t var){_extipinsel8 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 9 Pin Select */
            private: uint32_t   _extipinsel9 : 2;
            /* read fields EXTIPINSELH.EXTIPINSEL9:extipinsel9_t(2 bit)	(External Interrupt 9 Pin Select) */
            public: inline extipinsel9_t r_extipinsel9(void){return (extipinsel9_t)_extipinsel9;}
            /* write fields EXTIPINSELH.EXTIPINSEL9:extipinsel9_t(2 bit)	(External Interrupt 9 Pin Select) */
            public: inline void w_extipinsel9(extipinsel9_t var){_extipinsel9 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 10 Pin Select */
            private: uint32_t   _extipinsel10 : 2;
            /* read fields EXTIPINSELH.EXTIPINSEL10:extipinsel10_t(2 bit)	(External Interrupt 10 Pin Select) */
            public: inline extipinsel10_t r_extipinsel10(void){return (extipinsel10_t)_extipinsel10;}
            /* write fields EXTIPINSELH.EXTIPINSEL10:extipinsel10_t(2 bit)	(External Interrupt 10 Pin Select) */
            public: inline void w_extipinsel10(extipinsel10_t var){_extipinsel10 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 11 Pin Select */
            private: uint32_t   _extipinsel11 : 2;
            /* read fields EXTIPINSELH.EXTIPINSEL11:extipinsel11_t(2 bit)	(External Interrupt 11 Pin Select) */
            public: inline extipinsel11_t r_extipinsel11(void){return (extipinsel11_t)_extipinsel11;}
            /* write fields EXTIPINSELH.EXTIPINSEL11:extipinsel11_t(2 bit)	(External Interrupt 11 Pin Select) */
            public: inline void w_extipinsel11(extipinsel11_t var){_extipinsel11 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 12 Pin Select */
            private: uint32_t   _extipinsel12 : 2;
            /* read fields EXTIPINSELH.EXTIPINSEL12:extipinsel12_t(2 bit)	(External Interrupt 12 Pin Select) */
            public: inline extipinsel12_t r_extipinsel12(void){return (extipinsel12_t)_extipinsel12;}
            /* write fields EXTIPINSELH.EXTIPINSEL12:extipinsel12_t(2 bit)	(External Interrupt 12 Pin Select) */
            public: inline void w_extipinsel12(extipinsel12_t var){_extipinsel12 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 13 Pin Select */
            private: uint32_t   _extipinsel13 : 2;
            /* read fields EXTIPINSELH.EXTIPINSEL13:extipinsel13_t(2 bit)	(External Interrupt 13 Pin Select) */
            public: inline extipinsel13_t r_extipinsel13(void){return (extipinsel13_t)_extipinsel13;}
            /* write fields EXTIPINSELH.EXTIPINSEL13:extipinsel13_t(2 bit)	(External Interrupt 13 Pin Select) */
            public: inline void w_extipinsel13(extipinsel13_t var){_extipinsel13 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 14 Pin Select */
            private: uint32_t   _extipinsel14 : 2;
            /* read fields EXTIPINSELH.EXTIPINSEL14:extipinsel14_t(2 bit)	(External Interrupt 14 Pin Select) */
            public: inline extipinsel14_t r_extipinsel14(void){return (extipinsel14_t)_extipinsel14;}
            /* write fields EXTIPINSELH.EXTIPINSEL14:extipinsel14_t(2 bit)	(External Interrupt 14 Pin Select) */
            public: inline void w_extipinsel14(extipinsel14_t var){_extipinsel14 = var & 0x3;}
            private: uint32_t   : 2;
			/* External Interrupt 15 Pin Select */
            private: uint32_t   _extipinsel15 : 2;
            /* read fields EXTIPINSELH.EXTIPINSEL15:extipinsel15_t(2 bit)	(External Interrupt 15 Pin Select) */
            public: inline extipinsel15_t r_extipinsel15(void){return (extipinsel15_t)_extipinsel15;}
            /* write fields EXTIPINSELH.EXTIPINSEL15:extipinsel15_t(2 bit)	(External Interrupt 15 Pin Select) */
            public: inline void w_extipinsel15(extipinsel15_t var){_extipinsel15 = var & 0x3;}
            private: uint32_t : 2;
        }fields;
    }EXTIPINSELH;
	/* External Interrupt Rising Edge Trigger Register */
    public: union EXTIRISE{ /* addr: 0x410 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_EXTIRISE   = 0x0000ffff,
        };
        public: struct fields{
			/* External Interrupt N Rising Edge Trigger Enable */
            private: uint32_t   _extirise : 16;
            /* read fields EXTIRISE.EXTIRISE:uint32_t(16 bit)	(External Interrupt N Rising Edge Trigger Enable) */
            public: inline uint32_t r_extirise(void){return (uint32_t)_extirise;}
            /* write fields EXTIRISE.EXTIRISE:uint32_t(16 bit)	(External Interrupt N Rising Edge Trigger Enable) */
            public: inline void w_extirise(uint32_t var){_extirise = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }EXTIRISE;
	/* External Interrupt Falling Edge Trigger Register */
    public: union EXTIFALL{ /* addr: 0x414 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_EXTIFALL   = 0x0000ffff,
        };
        public: struct fields{
			/* External Interrupt N Falling Edge Trigger Enable */
            private: uint32_t   _extifall : 16;
            /* read fields EXTIFALL.EXTIFALL:uint32_t(16 bit)	(External Interrupt N Falling Edge Trigger Enable) */
            public: inline uint32_t r_extifall(void){return (uint32_t)_extifall;}
            /* write fields EXTIFALL.EXTIFALL:uint32_t(16 bit)	(External Interrupt N Falling Edge Trigger Enable) */
            public: inline void w_extifall(uint32_t var){_extifall = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }EXTIFALL;
	/* External Interrupt Level Register */
    public: union EXTILEVEL{ /* addr: 0x418 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_EM4WU0     = 0x00010000,
            MASK_EM4WU1     = 0x00020000,
            MASK_EM4WU4     = 0x00100000,
            MASK_EM4WU8     = 0x01000000,
            MASK_EM4WU9     = 0x02000000,
            MASK_EM4WU12    = 0x10000000,
        };
        public: struct fields{
            private: uint32_t   : 16;
			/* EM4 Wake Up Level for EM4WU0 Pin */
            private: uint32_t   _em4wu0 : 1;
            /* read fields EXTILEVEL.EM4WU0:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU0 Pin) */
            public: inline uint32_t r_em4wu0(void){return (uint32_t)_em4wu0;}
            /* write fields EXTILEVEL.EM4WU0:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU0 Pin) */
            public: inline void w_em4wu0(uint32_t var){_em4wu0 = var & 0x1;}
			/* EM4 Wake Up Level for EM4WU1 Pin */
            private: uint32_t   _em4wu1 : 1;
            /* read fields EXTILEVEL.EM4WU1:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU1 Pin) */
            public: inline uint32_t r_em4wu1(void){return (uint32_t)_em4wu1;}
            /* write fields EXTILEVEL.EM4WU1:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU1 Pin) */
            public: inline void w_em4wu1(uint32_t var){_em4wu1 = var & 0x1;}
            private: uint32_t   : 2;
			/* EM4 Wake Up Level for EM4WU4 Pin */
            private: uint32_t   _em4wu4 : 1;
            /* read fields EXTILEVEL.EM4WU4:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU4 Pin) */
            public: inline uint32_t r_em4wu4(void){return (uint32_t)_em4wu4;}
            /* write fields EXTILEVEL.EM4WU4:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU4 Pin) */
            public: inline void w_em4wu4(uint32_t var){_em4wu4 = var & 0x1;}
            private: uint32_t   : 3;
			/* EM4 Wake Up Level for EM4WU8 Pin */
            private: uint32_t   _em4wu8 : 1;
            /* read fields EXTILEVEL.EM4WU8:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU8 Pin) */
            public: inline uint32_t r_em4wu8(void){return (uint32_t)_em4wu8;}
            /* write fields EXTILEVEL.EM4WU8:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU8 Pin) */
            public: inline void w_em4wu8(uint32_t var){_em4wu8 = var & 0x1;}
			/* EM4 Wake Up Level for EM4WU9 Pin */
            private: uint32_t   _em4wu9 : 1;
            /* read fields EXTILEVEL.EM4WU9:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU9 Pin) */
            public: inline uint32_t r_em4wu9(void){return (uint32_t)_em4wu9;}
            /* write fields EXTILEVEL.EM4WU9:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU9 Pin) */
            public: inline void w_em4wu9(uint32_t var){_em4wu9 = var & 0x1;}
            private: uint32_t   : 2;
			/* EM4 Wake Up Level for EM4WU12 Pin */
            private: uint32_t   _em4wu12 : 1;
            /* read fields EXTILEVEL.EM4WU12:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU12 Pin) */
            public: inline uint32_t r_em4wu12(void){return (uint32_t)_em4wu12;}
            /* write fields EXTILEVEL.EM4WU12:uint32_t(1 bit)	(EM4 Wake Up Level for EM4WU12 Pin) */
            public: inline void w_em4wu12(uint32_t var){_em4wu12 = var & 0x1;}
            private: uint32_t : 3;
        }fields;
    }EXTILEVEL;
	/* Interrupt Flag Register */
    public: union IF{ /* addr: 0x41c */ 
        ReadOnly<uint32_t> r;
        public: enum mask_t{
            MASK_EXT        = 0x0000ffff,
            MASK_EM4WU      = 0xffff0000,
        };
        public: struct fields{
			/* External Pin Interrupt Flag */
            private: uint32_t   _ext : 16;
            /* read fields IF.EXT:uint32_t(16 bit)	(External Pin Interrupt Flag) */
            public: inline uint32_t r_ext(void){return (uint32_t)_ext;}
			/* EM4 Wake Up Pin Interrupt Flag */
            private: uint32_t   _em4wu : 16;
            /* read fields IF.EM4WU:uint32_t(16 bit)	(EM4 Wake Up Pin Interrupt Flag) */
            public: inline uint32_t r_em4wu(void){return (uint32_t)_em4wu;}
        }fields;
    }IF;
	/* Interrupt Flag Set Register */
    public: union IFS{ /* addr: 0x420 */ 
        WriteOnly<uint32_t> r;
        public: enum mask_t{
            MASK_EXT        = 0x0000ffff,
            MASK_EM4WU      = 0xffff0000,
        };
        public: struct fields{
			/* Set EXT Interrupt Flag */
            private: uint32_t   _ext : 16;
            /* write fields IFS.EXT:uint32_t(16 bit)	(Set EXT Interrupt Flag) */
            public: inline void w_ext(uint32_t var){_ext = var & 0xffff;}
			/* Set EM4WU Interrupt Flag */
            private: uint32_t   _em4wu : 16;
            /* write fields IFS.EM4WU:uint32_t(16 bit)	(Set EM4WU Interrupt Flag) */
            public: inline void w_em4wu(uint32_t var){_em4wu = var & 0xffff;}
        }fields;
    }IFS;
	/* Interrupt Flag Clear Register */
    public: union IFC{ /* addr: 0x424 */ 
        WriteOnly<uint32_t> r;
        public: enum mask_t{
            MASK_EXT        = 0x0000ffff,
            MASK_EM4WU      = 0xffff0000,
        };
        public: struct fields{
			/* Clear EXT Interrupt Flag */
            private: uint32_t   _ext : 16;
            /* write fields IFC.EXT:uint32_t(16 bit)	(Clear EXT Interrupt Flag) */
            public: inline void w_ext(uint32_t var){_ext = var & 0xffff;}
			/* Clear EM4WU Interrupt Flag */
            private: uint32_t   _em4wu : 16;
            /* write fields IFC.EM4WU:uint32_t(16 bit)	(Clear EM4WU Interrupt Flag) */
            public: inline void w_em4wu(uint32_t var){_em4wu = var & 0xffff;}
        }fields;
    }IFC;
	/* Interrupt Enable Register */
    public: union IEN{ /* addr: 0x428 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_EXT        = 0x0000ffff,
            MASK_EM4WU      = 0xffff0000,
        };
        public: struct fields{
			/* EXT Interrupt Enable */
            private: uint32_t   _ext : 16;
            /* read fields IEN.EXT:uint32_t(16 bit)	(EXT Interrupt Enable) */
            public: inline uint32_t r_ext(void){return (uint32_t)_ext;}
            /* write fields IEN.EXT:uint32_t(16 bit)	(EXT Interrupt Enable) */
            public: inline void w_ext(uint32_t var){_ext = var & 0xffff;}
			/* EM4WU Interrupt Enable */
            private: uint32_t   _em4wu : 16;
            /* read fields IEN.EM4WU:uint32_t(16 bit)	(EM4WU Interrupt Enable) */
            public: inline uint32_t r_em4wu(void){return (uint32_t)_em4wu;}
            /* write fields IEN.EM4WU:uint32_t(16 bit)	(EM4WU Interrupt Enable) */
            public: inline void w_em4wu(uint32_t var){_em4wu = var & 0xffff;}
        }fields;
    }IEN;
	/* EM4 Wake Up Enable Register */
    public: union EM4WUEN{ /* addr: 0x42c */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_EM4WUEN    = 0xffff0000,
        };
        public: struct fields{
            private: uint32_t   : 16;
			/* EM4 Wake Up Enable */
            private: uint32_t   _em4wuen : 16;
            /* read fields EM4WUEN.EM4WUEN:uint32_t(16 bit)	(EM4 Wake Up Enable) */
            public: inline uint32_t r_em4wuen(void){return (uint32_t)_em4wuen;}
            /* write fields EM4WUEN.EM4WUEN:uint32_t(16 bit)	(EM4 Wake Up Enable) */
            public: inline void w_em4wuen(uint32_t var){_em4wuen = var & 0xffff;}
        }fields;
    }EM4WUEN;
    private: uint8_t rsv_18[16];
	/* I/O Routing Pin Enable Register */
    public: union ROUTEPEN{ /* addr: 0x440 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_SWCLKTCKPEN = 0x00000001,
            MASK_SWDIOTMSPEN = 0x00000002,
            MASK_TDOPEN     = 0x00000004,
            MASK_TDIPEN     = 0x00000008,
            MASK_SWVPEN     = 0x00000010,
        };
        public: struct fields{
			/* Serial Wire Clock and JTAG Test Clock Pin Enable */
            private: uint32_t   _swclktckpen : 1;
            /* read fields ROUTEPEN.SWCLKTCKPEN:uint32_t(1 bit)	(Serial Wire Clock and JTAG Test Clock Pin Enable) */
            public: inline uint32_t r_swclktckpen(void){return (uint32_t)_swclktckpen;}
            /* write fields ROUTEPEN.SWCLKTCKPEN:uint32_t(1 bit)	(Serial Wire Clock and JTAG Test Clock Pin Enable) */
            public: inline void w_swclktckpen(uint32_t var){_swclktckpen = var & 0x1;}
			/* Serial Wire Data and JTAG Test Mode Select Pin Enable */
            private: uint32_t   _swdiotmspen : 1;
            /* read fields ROUTEPEN.SWDIOTMSPEN:uint32_t(1 bit)	(Serial Wire Data and JTAG Test Mode Select Pin Enable) */
            public: inline uint32_t r_swdiotmspen(void){return (uint32_t)_swdiotmspen;}
            /* write fields ROUTEPEN.SWDIOTMSPEN:uint32_t(1 bit)	(Serial Wire Data and JTAG Test Mode Select Pin Enable) */
            public: inline void w_swdiotmspen(uint32_t var){_swdiotmspen = var & 0x1;}
			/* JTAG Test Debug Output Pin Enable */
            private: uint32_t   _tdopen : 1;
            /* read fields ROUTEPEN.TDOPEN:uint32_t(1 bit)	(JTAG Test Debug Output Pin Enable) */
            public: inline uint32_t r_tdopen(void){return (uint32_t)_tdopen;}
            /* write fields ROUTEPEN.TDOPEN:uint32_t(1 bit)	(JTAG Test Debug Output Pin Enable) */
            public: inline void w_tdopen(uint32_t var){_tdopen = var & 0x1;}
			/* JTAG Test Debug Input Pin Enable */
            private: uint32_t   _tdipen : 1;
            /* read fields ROUTEPEN.TDIPEN:uint32_t(1 bit)	(JTAG Test Debug Input Pin Enable) */
            public: inline uint32_t r_tdipen(void){return (uint32_t)_tdipen;}
            /* write fields ROUTEPEN.TDIPEN:uint32_t(1 bit)	(JTAG Test Debug Input Pin Enable) */
            public: inline void w_tdipen(uint32_t var){_tdipen = var & 0x1;}
			/* Serial Wire Viewer Output Pin Enable */
            private: uint32_t   _swvpen : 1;
            /* read fields ROUTEPEN.SWVPEN:uint32_t(1 bit)	(Serial Wire Viewer Output Pin Enable) */
            public: inline uint32_t r_swvpen(void){return (uint32_t)_swvpen;}
            /* write fields ROUTEPEN.SWVPEN:uint32_t(1 bit)	(Serial Wire Viewer Output Pin Enable) */
            public: inline void w_swvpen(uint32_t var){_swvpen = var & 0x1;}
            private: uint32_t : 27;
        }fields;
    }ROUTEPEN;
	/* I/O Routing Location Register */
    public: union ROUTELOC0{ /* addr: 0x444 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_SWVLOC     = 0x0000003f,
        };
		/* I/O Location */
        public: enum swvloc_t {
			/* Location 0 */
            SWVLOC_LOC0       = 0,
			/* Location 1 */
            SWVLOC_LOC1       = 1,
			/* Location 2 */
            SWVLOC_LOC2       = 2,
			/* Location 3 */
            SWVLOC_LOC3       = 3,
        };
        public: struct fields{
			/* I/O Location */
            private: uint32_t   _swvloc : 6;
            /* read fields ROUTELOC0.SWVLOC:swvloc_t(6 bit)	(I/O Location) */
            public: inline swvloc_t r_swvloc(void){return (swvloc_t)_swvloc;}
            /* write fields ROUTELOC0.SWVLOC:swvloc_t(6 bit)	(I/O Location) */
            public: inline void w_swvloc(swvloc_t var){_swvloc = var & 0x3f;}
            private: uint32_t : 26;
        }fields;
    }ROUTELOC0;
    private: uint8_t rsv_19[8];
	/* Input Sense Register */
    public: union INSENSE{ /* addr: 0x450 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_INT        = 0x00000001,
            MASK_EM4WU      = 0x00000002,
        };
        public: struct fields{
			/* Interrupt Sense Enable */
            private: uint32_t   _int : 1;
            /* read fields INSENSE.INT:uint32_t(1 bit)	(Interrupt Sense Enable) */
            public: inline uint32_t r_int(void){return (uint32_t)_int;}
            /* write fields INSENSE.INT:uint32_t(1 bit)	(Interrupt Sense Enable) */
            public: inline void w_int(uint32_t var){_int = var & 0x1;}
			/* EM4WU Interrupt Sense Enable */
            private: uint32_t   _em4wu : 1;
            /* read fields INSENSE.EM4WU:uint32_t(1 bit)	(EM4WU Interrupt Sense Enable) */
            public: inline uint32_t r_em4wu(void){return (uint32_t)_em4wu;}
            /* write fields INSENSE.EM4WU:uint32_t(1 bit)	(EM4WU Interrupt Sense Enable) */
            public: inline void w_em4wu(uint32_t var){_em4wu = var & 0x1;}
            private: uint32_t : 30;
        }fields;
    }INSENSE;
	/* Configuration Lock Register */
    public: union LOCK{ /* addr: 0x454 */ 
        ReadWrite<uint32_t> r;
        public: enum mask_t{
            MASK_LOCKKEY    = 0x0000ffff,
        };
		/* Configuration Lock Key */
        public: enum lockkey_t {
			/* "" */
            LOCKKEY_UNLOCKED   = 0,
			/* "" */
            LOCKKEY_LOCKED     = 1,
        };
        public: struct fields{
			/* Configuration Lock Key */
            private: uint32_t   _lockkey : 16;
            /* read fields LOCK.LOCKKEY:lockkey_t(16 bit)	(Configuration Lock Key) */
            public: inline lockkey_t r_lockkey(void){return (lockkey_t)_lockkey;}
            /* write fields LOCK.LOCKKEY:lockkey_t(16 bit)	(Configuration Lock Key) */
            public: inline void w_lockkey(lockkey_t var){_lockkey = var & 0xffff;}
            private: uint32_t : 16;
        }fields;
    }LOCK;
}; /* end of GPIO */ 
#endif

```
