# z2fish: boot Fish from (Zsh as LoginShell) 

A Rust implementation of:

```sh
zsh -lic "fish ..." # ⚠️  nested quotes limitation
```

Or,

```sh
#!/bin/zsh

zsh -lic zsh -t << EOF
fish "$@"
EOF
```

`z2fish` is a demo project for learning Rust subprocess, codec & logging.
It's not efficient. Following methods cost roughly the same time:

```sh
time zsh -lic "fish -c 'exit'"
time ./z2fish.sh -c "exit"
time z2fish_rs/target/release/z2fish -c "exit"
```

