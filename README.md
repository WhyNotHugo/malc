MALC
====

MacBook Automatic Light Calibrator (*MALC*) is a small app that set a MacBook's
keyboard backlight brightness according to ambient brightness (using the
MacBook's light sensor).

It has been tested on a 2013 MBA, but should work on any other MacBook that
uses the applesmc kernel module.

Design
------

MALC simply runs a loop checking ambient light every 100ms, and updating the
keyboard backlight accordingly.

Rust was used as a language of choice because:

 * I wanted to keep the footprint as light as possible.
 * I wanted to try Rust, and this seems like something simple to start.

Caveats
-------

 * Manually altering the keyboard backlight isn't really possible, since the
   value is quickly reset according to ambient light.
 * Ambient light -> keyboard brightness mapping still needs some work. The
   backlight is triggered too soon, and is usually brighter than desired.

Licence
-------

This software is distributed under the ISC Licence. See LICENCE.md for details.

Copyright (c) 2016, Hugo Osvaldo Barrera <hugo@barrera.io>
