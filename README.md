# rust-password-hasher
Argon2 based Password hasher.

## To run
```bash
# password hash for 'password'
$ cargo r 'password'

password -> $argon2$lanes=4,mem=12,len=128,ver=1,pmax=128,norm=nfkc,passes=3,xhmac=none,pmin=8,len-calc=chars$UOaiqOpY8VBn/4LVn3FPmg$niSAH7NbTRh2SIgw3ah5fC9572n0GfZ+AhpHnMen58SRJHe8fFXpKAdcwl88emjRiiYXj8ZiDuUmDs9y/Flp4jf57ci0LuSuzK1SiS+rxsmU1q2rDnKI22zwhGFgwXK7Doj7uvcUZVMZEAnsbjvhwV2vQjIvwQZLOmv1YuHKLAA
```

```bash
# password hash for 'hello' <-- invalid, minimum length 8
$ cargo r 'hello'

> thread 'main' panicked at src/main.rs:6:78:
> called `Result::unwrap()` on an `Err` value: PasswordTooShort { min: 8, actual: 5 }
> note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```bash
# password hash for 'hello world'
$ cargo r 'hello world'

hello world -> $argon2$pmax=128,mem=12,ver=1,pmin=8,lanes=4,norm=nfkc,passes=3,xhmac=none,len=128,len-calc=chars$3tko3jGIk470m6xQ4I30/Q$or8Sm3hwCgahhYA5dD2YHaifY3cFWfKmBel4bUb1XPRVyhwnQ5nMNF3YYpRBrXWoRor7NJCC9bzBdeKk9ibyjC/HzRi9Zj1FU977IcgMd8wEyIZCgQj6tkS2faeAmTyvKpybHjVELoUx1N474BIS+Mi01Am7AqpAyoz5uNYKDt0
```

```bash
# password hash for 'hello&world'
$ cargo r 'hello&world'

hello&world -> $argon2$norm=nfkc,lanes=4,mem=12,ver=1,passes=3,len=128,len-calc=chars,pmax=128,pmin=8,xhmac=none$kHj7QVLC9yHC7jDs3Eo5qA$PrSb2S9SEUtU/XI5odSscswNtbvG2U2nsOHi53hy6mviu4SqD7FPsC9fJvVT0WHUTVVmwoMW2941NJ53GiVlR3PhaF1SCdR3MaY81BsGUB8FZqxh8734naLHl9V820jeHSdsgDvASksn3hLvVfxsYnjRObXIR+LwuoTa5P+gmzE
```
