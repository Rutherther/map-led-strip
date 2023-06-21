WIP.

This is a small project project that is made for a board made by LaskaKit (https://github.com/LaskaKit/LED_Czech_Map/),
it has 72 LEDs (WS2812B). This project is written in Rust.

## Aim
Make a command handler and an animation manager.
Allow to send commands using uart and react to them with
chaning states of the LEDs.

There should be commands for starting animations, such as rainbow or a point moving.

## Progress
Currently there is a command handler,
the following commands are available:
  - HELLO_WORLD - respond with Hello world!
  - SET \<ID or NAME\> \<R\> \<G\> \<B\> - Sets the given LED to the given color (255 max). The LED may be specified by an index or by city name. Spaces in names should ber replaced with "_"
  - RESET - reset all LEDs
  - ALL \<R\> \<G\> \<B\> - set all LEDs to this color

Animations are not finished yet.