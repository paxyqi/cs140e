I completed the assignment 0 (blink) and 1(os), then I turned to the assignment 2, which asked me to fetch 2-fs and os. 
After that, I find the os/std became almost the same as the normol rust std. 
But unfortunately, the std, alloc and core code of rust are all change a lot since 2018. 
Now I am facing the problem for many days. That is, I can't make os/kernel anymore. 
Because I the new std can't be compile. 

The error occur in the os/std/src/collections/hash/map.rs:6
```
   |
6 | use hashbrown::HashMap  as base;
   |     ^^^^^^^^^ maybe a missing crate `hashbrown`?
```
The hashbrown is used be os/std/src/collections/hash/table.rs. Now I have to use this crate to replace it.
And this is the hashbrown: https://github.com/rust-lang/hashbrown

And my morrior seems worked. When I compile, it has out put like this:
```
   Compiling proc-macro2 v1.0.3
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.11
   Compiling libc v0.2.66
   Compiling getrandom v0.1.14
   Compiling cfg-if v0.1.10
   Compiling autocfg v1.0.0
   Compiling hashbrown v0.7.1
   Compiling quote v1.0.2
   Compiling proc-macro-hack v0.5.12
   Compiling const-random-macro v0.1.8
   Compiling const-random v0.1.8
   Compiling ahash v0.3.2
   Compiling std v0.1.0 (/home/pixy/Desktop/RaspberryPi/os/std)
```
I add dependencies in os/std/Cargo.toml
```
[dependencies]
hashbrown = "0.7"
```

It really makes me feel puzzled. If you have any suggestion, please let me know. I work on it for a long time. I don't want to quit it just because if this error.
Or, if you have any idea to change the rust code in os/std, please let me know.

**I changed the os/std/collections/hash/map.rs and set.rs to the latest rust-libstd. Which are not the same as cs140e.**
