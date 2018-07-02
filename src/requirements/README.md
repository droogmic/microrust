# Requirements

## Knowledge

The only knowledge requirement to read this book is to know *some* Rust. It's hard to
quantify *some* but a good benchmark is having read and understood the first 14 chapters of [the Rust book][thebook].

[thebook]: https://doc.rust-lang.org/nightly/book/

## Hardware

To follow this material you'll only need a [micro:bit].

[micro:bit]: http://tech.microbit.org/hardware/

You can purchase the BBC micro:bit from [a large list of international resellers][resellers].

[resellers]: https://microbit.org/resellers/

<p align="center">
<img title="microbit" src="https://microbit.org/images/microbit-front.png" width="45%">
<img title="microbit" src="https://microbit.org/images/microbit-back.png" width="45%">
</p>

> **FAQ**: Wait, why do I need this specific device?

It makes my life and yours much easier.

The material is much, much more approachable if we don't have to worry about hardware differences.
Trust me on this one.

> **FAQ**: Can I follow this material with a different development board?

Maybe? It depends mainly on two things: your previous experience with microcontrollers and/or
whether there already exists a high level crate. A list of boards with high level crates available can be found [here][bsc].

[bsc]: https://github.com/rust-embedded/awesome-embedded-rust#board-support-crates

With other development boards, this text would lose most if not all its beginner friendliness
and "easy to follow"-ness, IMO.

There are other similar guides for different hardware. For a full list see [this list][books].
[books]: https://github.com/rust-embedded/awesome-embedded-rust/#books-blogs-and-training-materials
The following are worth a special mention:
- [Discovery](https://japaric.github.io/discovery/) by @japaric: The genesis guide which this is based on. Uses the STM32F3DISCOVERY.



If you have a different cortex-m development board and you don't consider yourself a total beginner, you are
better off starting with the [cortex-m-quickstart] project template.

[cortex-m-quickstart]: https://docs.rs/crate/cortex-m-quickstart
