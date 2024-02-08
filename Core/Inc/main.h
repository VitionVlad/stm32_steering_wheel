/* USER CODE BEGIN Header */
/**
  ******************************************************************************
  * @file           : main.h
  * @brief          : Header for main.c file.
  *                   This file contains the common defines of the application.
  ******************************************************************************
  * @attention
  *
  * Copyright (c) 2023 STMicroelectronics.
  * All rights reserved.
  *
  * This software is licensed under terms that can be found in the LICENSE file
  * in the root directory of this software component.
  * If no LICENSE file comes with this software, it is provided AS-IS.
  *
  ******************************************************************************
  */
/* USER CODE END Header */

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef __MAIN_H
#define __MAIN_H

#ifdef __cplusplus
extern "C" {
#endif

/* Includes ------------------------------------------------------------------*/
#include "stm32f4xx_hal.h"

/* Private includes ----------------------------------------------------------*/
/* USER CODE BEGIN Includes */

/* USER CODE END Includes */

/* Exported types ------------------------------------------------------------*/
/* USER CODE BEGIN ET */

/* USER CODE END ET */

/* Exported constants --------------------------------------------------------*/
/* USER CODE BEGIN EC */

/* USER CODE END EC */

/* Exported macro ------------------------------------------------------------*/
/* USER CODE BEGIN EM */

/* USER CODE END EM */

/* Exported functions prototypes ---------------------------------------------*/
void Error_Handler(void);

/* USER CODE BEGIN EFP */

/* USER CODE END EFP */

/* Private defines -----------------------------------------------------------*/
#define pedals1_Pin GPIO_PIN_5
#define pedals1_GPIO_Port GPIOD
#define pedals2_Pin GPIO_PIN_6
#define pedals2_GPIO_Port GPIOD
#define pedals3_Pin GPIO_PIN_7
#define pedals3_GPIO_Port GPIOD
#define gearbox_Pin GPIO_PIN_3
#define gearbox_GPIO_Port GPIOB
#define sw_Pin GPIO_PIN_5
#define sw_GPIO_Port GPIOB
#define dt_Pin GPIO_PIN_6
#define dt_GPIO_Port GPIOB
#define clk_Pin GPIO_PIN_7
#define clk_GPIO_Port GPIOB
#define panel3_Pin GPIO_PIN_8
#define panel3_GPIO_Port GPIOB
#define panel4_Pin GPIO_PIN_9
#define panel4_GPIO_Port GPIOB
#define panel1_Pin GPIO_PIN_0
#define panel1_GPIO_Port GPIOE
#define panel2_Pin GPIO_PIN_1
#define panel2_GPIO_Port GPIOE

/* USER CODE BEGIN Private defines */

/* USER CODE END Private defines */

#ifdef __cplusplus
}
#endif

#endif /* __MAIN_H */
