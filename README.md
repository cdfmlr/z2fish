# z2fish: boot Fish from (Zsh as LoginShell) 

A Rust implementation of:

```sh
zsh -lic "fish ..." # ⚠️  bug here: nested quotes limitation
```

Or,

```sh
#!/bin/zsh

# file: z2fish.sh
# 
# Added a layer of `zsh -t` to fix the nest quotes problem
# p.s. `zsh -lit` does not work.

zsh -lic zsh -t << EOF
fish "$@"
EOF
```

`z2fish` is a demo project for learning Rust subprocess, codec, logging & cargo vendor.
It's not efficient. Following methods cost roughly the same time:

```sh
$ time zsh -lic "fish -c 'exit'"
________________________________________________________
Executed in  181.72 millis    fish           external
   usr time   69.92 millis    0.23 millis   69.69 millis
   sys time   23.28 millis    1.64 millis   21.63 millis

$ time ./z2fish.sh -c "exit"

________________________________________________________
Executed in  182.07 millis    fish           external
   usr time   70.14 millis    0.22 millis   69.92 millis
   sys time   22.44 millis    1.19 millis   21.25 millis

$ time z2fish_rs/target/release/z2fish -c "exit"

________________________________________________________
Executed in  195.32 millis    fish           external
   usr time   70.93 millis    0.21 millis   70.72 millis
   sys time   29.00 millis    1.19 millis   27.81 millis
```

