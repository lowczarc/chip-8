# 🎮 Chip8 Emulator 🦀

A basic Chip8 Emulator written in Rust I made to get my feet wet with writing emulators. 🏊‍

![Chip8 Emulator](https://raw.githubusercontent.com/lowczarc/chip-8-emulator/main/assets/example.png)

## 📚 Background

The Chip8 is an interpreted programming language, developed in the mid-1970s, that was used on some early microcomputers and home video game consoles. Despite its age, it's a great way to get into emulator development due to its simplicity and well-documented nature.

## 🧰 Structure

The repository consists of two main folders:

- `emulator/` - This contains the main Chip8 emulator. You'll find all of the necessary files and the core logic inside this folder.
- `asm/` - This is an assembler for the Chip8, written in (ugly) Python 🐍.

## 🚀 Getting started

If you want to try this out or play around with the code, you can do the following:

**Clone the repository:**
```sh
git clone https://github.com/lowczarc/chip-8-emulator.git
```

**Build and run the emulator (from within the `emulator/` directory):**
```sh
cargo run <ch8_rom>
```
There are some examples of roms in the `asm/` directory and a ton of them you can find using the power of the *information superhighway*.

**Run the assembler (from within the `asm/` directory):**
```sh
python main.py <c8asm_input> <ch8_output>
```

## 🧑‍💻 Contribution

This project was primarily built for my personal learning and isn't really intended for contribution. However, if you find a bug 🐞, feel free to create an issue or a pull request!

## 📝 License

This project is licensed under the "I don't care about licenses, do what the hell you want with it" license

## 📚 References

I used [Cowgod's Chip-8 Technical reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
