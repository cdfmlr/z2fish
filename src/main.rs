use log::debug;
use std::env;

use z2fish::{run_boot, run_shell, SHELL_PREFIX};

fn main() {
    env_logger::init();

    let mut args = env::args().peekable();
    debug!("&args = {:?}", &args);

    let cmd = args.next().unwrap();
    debug!("&cmd = {:?}", &cmd);

    let first_arg = args.peek();
    debug!("&first_arg = {:?}", &first_arg);

    if first_arg.is_some() && first_arg.unwrap() == SHELL_PREFIX {
        debug!("run_shell");
        run_shell(&args.skip(1).next().unwrap())
    } else {
        debug!("run_boot");
        run_boot(&cmd, args)
    }
}
