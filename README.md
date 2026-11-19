## 🎮NES emulator written in Rust!

That's my second emulator that was written in Rust. That's an educational project I wrote this one using Rafael Baguzman book.
P.S. Thank you for this book, Rafael!

---

### 📸Screenshots
<img width="350" height="350" alt="image" src="https://github.com/user-attachments/assets/89d6a94d-2891-414c-9f26-1bb5b1f306b7" />
<img width="350" height="350" alt="image" src="https://github.com/user-attachments/assets/4c86d371-dae7-4807-a467-ffc06c21ee15" />

---
### 🕹️What kind of game you can play?

You can play classic games what was created early 80s. If you see that games more than 40kb, you should find another emulator.

---

### 🚀How to play?

1. Create a folder in the root directory of the project and put your `.nes` ROM files inside.
2. Run this following command and enjoy NES games on your computer!

```bash
cargo run --release --name_of_your_rom.nes
```

---

### 🎮Controls

| Key | NES Button |
| :--- | :--- |
| <kbd>Z</kbd> | **A** |
| <kbd>X</kbd> | **B** |
| <kbd>Space</kbd> | **Select** |
| <kbd>Enter</kbd> | **Start** |
| <kbd>Arrow Keys</kbd> | **D-Pad** (Up / Down / Left / Right) |

---


### 💡What I learned and what's next?

I fell in love with Rust and creating emulators, after this project. I didn't implement APU, mappers for different games, decent UI, but I got knowledge that'll help me keep going and create something really cool in future!

I'm going to get deeper in system programming and after that re-create NES emulator using this knowledge.

---

### What was hard and what was easy?

Creating a CPU was very interesting and pretty logical for me. I almost wrote all opcodes by myself and even make a mistake trying to write all tests for every opcode and flag by myself:D.
Creating a PPU was a hell of a task. I don't completely understand each piece of code, even now. Thankfully, I understand now, that I more CPU-person than PPU, if you know what i mean.

So, implementing APU is something from extraterrestrial technologies for me, but I promise myself that I'll learn about it.

---
