# Deepslate

Deepslate was going to be an implementation of [Polar](https://github.com/hollow-cube/polar) for Rust ([Valence](https://github.com/valence-rs/valence)), but [Polar](https://github.com/hollow-cube/polar)... it does weird stuff (from the point of view of Rust), weird stuff that no one needs... VarInts *cough* so... I decided to make it more rust-friendly and then optimized it a couple times (too many) and here we are. I'm not going to say that it's better ([although...](#comparison)), but it's not Java so at the very least we have that. This is very much WIP, but I decided to get it on Github so if anyone was interested they could help/use it.

It is written fully from scratch, even if borrowing from my previous projects (but who doesn't do that), such as [lurcar](https://github.com/0skar16/lurcar)'s precursor [QRCA](https://github.com/0skar16/qrca) for the writer/reader thing-ama-jig.

As you may notice this also lacks any checks or any docs whatsoever. If I wanted to believe that if I can understand this code almost a year ([QRCA](https://github.com/0skar16/qrca)) after writing it, everyone should be, I'd probably be delusional. So until I can bring myself to write those, if u write any or do any changes you think are positive, hit me up, make a pull request, whatever the hell you want to be honest. Honestly, even an issue would help (please don't do anything like "the code is unreadable" because I won't even have an idea what you're talking about).

I also have an implementation of this for [Valence](https://github.com/valence-rs/valence), so if anyone needs it right now, again, hit me up, otherwise, I will release it once it is good enough for me.

## Roadmap (at least what I can think of rn):
- [ ] Fix errors in some places
- [x] Update packages
- [x] Do something with the big biome enum 
- [x] Scrap or implement compression ([#3](https://github.com/0skar16/deepslate/issues/3))
- [x] After adding bitcode the bit packing of the block pallete references may not be needed, so check that and scrap if need be ([#3](https://github.com/0skar16/deepslate/issues/3))
- [ ] Switch to using lurcar as the archive ([#2](https://github.com/0skar16/deepslate/issues/2))
- [x] (BIG) Rewrite to implement multi-file support ([#2](https://github.com/0skar16/deepslate/issues/2))
- [ ] Add block entities and entities ([#4](https://github.com/0skar16/deepslate/issues/4))
- [ ] Docs: ([#1](https://github.com/0skar16/deepslate/issues/1))
    - [ ] lib.rs
    - [ ] main.rs
    - [ ] reader.rs
    - [ ] writer.rs
- [ ] Better error messages
- [ ] Error/corruption checking (the last thing we need is a bitcode error)
- [ ] Improve writing
- [x] Isolate the converter from the main code

## Comparison?

| Scenario        | Deepslate (v0.2.0, zstd)         | Polar (v1, zstd) | Polar (v1, uncompressed) | TNT (v1) | Anvil |
|-----------------|----------------------------------|------------------|--------------------------|----------|-------|
| 1.19.4 Region   | 4.8mb (WOW)                      | 5.9mb            | 26.1mb                   | 115.9mb  | 9.7mb |

*The difference may also be the fault of the lack of block entities*

*I also should say that it is 2am as of writing this so your mileage with this README may vary.*

