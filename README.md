<h1 align="center">CopperBoy</h1>
<p align="center">
    <img src="https://github.com/Dhole/CopperBoy/raw/master/copperboy-logo.png" width="256">
</p>

CopperBoy is an [Arduboy](https://arduboy.com/) emulator written in Rust.

## Status

Although this project is in a Work in Progress status, currently the emulator
is capable of playing many of the ArduBoy games at full speed.  You can find a
nice game collection in [eried's
ArduboyCollection](https://github.com/eried/ArduboyCollection).

- Running ArduBoy games in `.hex` format.
- The full AVR ISA is implemented.
- Not all atmega32u4 hardware is implemented.  I've been implementing what the
  games I tested needed to work.
- No optimizations.  Currently the emulator takes about 60% of a single core in
  my PC (Intel(R) Core(TM) i5-4590) when emulating games at 60 fps, when
  running in release mode.  I haven't profiled it yet to optimize it.
- Sound is not yet implemented.

## Usage

Currently there's an SDL frontend included in the emulator.

### Building

First you need to install the `sdl2` and `sdl2_ttf` libraries with your operating system's package manager. 

Then you can build the rust binary:
```
cargo build --release --bin sdl
```

### Running

You can run the emulator with cargo:
```
cargo run --release --bin sdl -- ROM_PATH
```

## Screenshots
<p float="left">
  <img alt="Unicorn Dash"    width="40%" src="https://github.com/Dhole/CopperBoy/raw/master/screenshots/Unicorn_Dash.png"     >
  <img alt="Jump First"      width="40%" src="https://github.com/Dhole/CopperBoy/raw/master/screenshots/Jump_First_0.png"     >
  <img alt="Hollow Seeker"   width="40%" src="https://github.com/Dhole/CopperBoy/raw/master/screenshots/Hollow_seeker_0.png"  >
  <img alt="Hollow Seeker"   width="40%" src="https://github.com/Dhole/CopperBoy/raw/master/screenshots/Hollow_seeker_1.png"  >
  <img alt="Mystic Balloon"  width="40%" src="https://github.com/Dhole/CopperBoy/raw/master/screenshots/Mystic_Balloon_0.png" >
  <img alt="Mystic Balloon"  width="40%" src="https://github.com/Dhole/CopperBoy/raw/master/screenshots/Mystic_Balloon_1.png" >
  <img alt="Starduino Turbo" width="40%" src="https://github.com/Dhole/CopperBoy/raw/master/screenshots/Starduino_Turbo_0.png">
  <img alt="Starduino Turbo" width="40%" src="https://github.com/Dhole/CopperBoy/raw/master/screenshots/Starduino_Turbo_3.png">
</p>

## Reference

- [AVR Instruction Set Manual](https://www.csee.umbc.edu/~alnel1/cmpe311/notes/AtmelAVR8BitISA.pdf)
- [atmega32u4 Datasheet](http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7766-8-bit-AVR-ATmega16U4-32U4_Datasheet.pdf)
- Display Driver: [SSD1306 Datasheet](https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf)
