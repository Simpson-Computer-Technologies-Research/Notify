# Notify ![Stars](https://img.shields.io/github/stars/realTristan/Notify?color=brightgreen) ![Watchers](https://img.shields.io/github/watchers/realTristan/Notify?label=Watchers)
![banner](https://user-images.githubusercontent.com/75189508/190282954-aecd8cb4-d076-4132-8535-dee24221e689.png)

# About
<h3>Why Rust?</h3>

- This project was made for a person who messaged me about creating a discord bot for them. I wanted to learn Rust and I thought this would be a great project to do so.

- Aside from my desire to learn the language, I decided that Rust would be the best option for this project as iterating over the sqlite database rows for every message sent in a channel can take quite some time. Luckily Rust is extremely fast which makes
performing these iterations not as time consuming.

# Quick Usage
<h3>Running the bot</h3>

```rust
$ TOKEN="YOUR BOT TOKEN" DATABASE_URL="sqlite:database.sqlite" cargo run --release

```

# License
MIT License

Copyright (c) 2022 Tristan Simpson

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
