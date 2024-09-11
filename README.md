## A GameBoy emulator in Rust and SDL2.

I made it to get familiar with the Rust language everyone is talking about recently and also to understand the emulation process itself. 
The emulator was made as a library so that it can be run on most of the platforms. 
It's still in development and there is still a lot to do but it's mostly playable right now.

### Usage
Clone the repo and cd inside.
```
git clone https://github.com/OxayMint/rustboy.git
```
```
cd rustboy
```

Build a release version of rustboy.
```
cargo build --release
```

Run the generated binary inside the /target folder
```
./target/rustboy ./path-to/rom.gb
```

You can also use the core itself in your own application. I will probably add it to crates at some point to make it easier for importing but for now you can find it at cores/GBcore. There may be more emulator cores in the future so this main application is just an interface for it. I'm also planning to move the rendering part out of the core so that it only returns a pixel buffer that you can render however you prefer.

### Roadmap
[Blargg tests](https://github.com/retrio/gb-test-roms) passing: 

    ✅ 11/11 on cpu_instrs.gb

    ✅ 3/3 on mem_timing.gb

    ✅ dmg-acid2.gb
    
    ➖ dmg_sound.gb

✅ Support for MBC ROMs (Zelda, Pokemon etc.).

TODO:
- Add GB Color Mode support.
- Add custom Input editing support. 


