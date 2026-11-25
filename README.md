# RavomIO Examples

Official starter projects for **[RavomIO](https://github.com/Ravomi/RavomIO)** (`mio`):

| Example | Board | Category | Framework |
| --- | --- | --- | --- |
| `stm32f103-blinky` | `bluepill_f103c8` (STM32F103C8T6) | ARM Cortex-M3 | `embassy` |
| `stm32f401-rtic` | `stm32f401ccu6` (STM32F401CCU6) | ARM Cortex-M4 | `rtic` |
| `rp2350-embassy` | `pico2_rp2350` (RP2350) | ARM Cortex-M33 | `embassy` |
| `py32f030-embassy` | `py32f030` (Puya PY32F030) | ARM Cortex-M0+ | `embassy` |
| `ch32v-riscv` | `adafruit_qtpy_ch32v203` (WCH CH32V) | RISC-V MCU | `bare-metal` |
| `esp32p4-riscv` | `esp32p4_evboard` (ESP32-P4) | RISC-V MCU | `esp-idf` |
| `hpm6750-riscv` | `hpm6750evkmini` (HPM6750) | RISC-V MCU | `embassy` |
| `mirui-embedded-ui` | `esp32s3` (ESP32-S3) | Embedded UI | `mirui` |
| `step-mxo2-rust-hdl` | `step_mxo2_lpc` (Lattice MachXO2) | FPGA | `rust-hdl` |
| `tang-nano-9k-fpga` | `tang_nano_9k` (Gowin GW1NR-9C) | FPGA | `rust-hdl` |
| `ulx3s-ecp5-fpga` | `ulx3s_85f` (Lattice ECP5-85F) | FPGA | `rust-hdl` |
| `milkv-duo-linux` | `milkv_duo` (CV1800B) | Linux SBC | `linux` |
| `luckfox-pico-linux` | `luckfox_pico` (RV1106) | Linux SBC | `linux` |

## Usage

```bash
cd stm32f103-blinky
mio build
mio flash --monitor
```
