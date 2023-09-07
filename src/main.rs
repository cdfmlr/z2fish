use std::env;

use z2fish::{run_boot, run_shell, SHELL_PREFIX};

fn main() {
    let mut args = env::args().peekable();
    dbg!(&args);

    let cmd = args.next().unwrap();
    dbg!(&cmd);

    let first_arg = args.peek();
    dbg!(&first_arg);

    if first_arg.is_some() && first_arg.unwrap() == SHELL_PREFIX {
        dbg!("run_shell");
        run_shell(&args.skip(1).next().unwrap())
    } else {
        dbg!("run_boot");
        run_boot(&cmd, args)
    }
}
