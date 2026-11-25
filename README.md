# RavomIO Examples

Official starter projects for **[RavomIO](https://github.com/Ravomi/RavomIO)** (`mio`):

| Example | Board | Framework |
| --- | --- | --- |
| `stm32f103-blinky` | `bluepill_f103c8` (STM32F103C8T6) | `embassy` |
| `rp2350-embassy` | `pico2_rp2350` (RP2350) | `embassy` |
| `step-mxo2-rust-hdl` | `step_mxo2_lpc` (Lattice MachXO2) | `rust-hdl` |
| `mirui-embedded-ui` | `esp32s3` (ESP32-S3) | `mirui` |

## Usage

```bash
cd stm32f103-blinky
mio build
mio flash --monitor
```
