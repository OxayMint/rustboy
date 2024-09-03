A GameBoy emulator in Rust and SDL2.

I made it to get familiar with the Rust language everyone is talking about recently and also to understand the emulation process itself. 
The emulator was made as a library so that it can be run on most of the platforms. 
It's still in development and there is still a lot to do but it's mostly playable right now.

###Build
Clone the repo and cd inside. 
```
cargo build --release
```
```
cd target/
```



Roadmap
+ Passing CPU Blargg tests
+ Passing memory timing tests
+ Passing acid-dmg2 rendering test.
+ Support for MBC ROMs (Zelda, Pokemon etc.).
- Implement Audio Processing Unit.
- Add GB Color ROMs support.
- 
