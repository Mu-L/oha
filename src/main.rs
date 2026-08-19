use clap::{CommandFactory, Parser};
use oha::{Opts, run};

fn main() {
    let opts = Opts::parse();

    if let Some(shell) = opts.completions {
        clap_complete::generate(shell, &mut Opts::command(), "oha", &mut std::io::stdout());
        return;
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(opts.worker_threads.get())
        .enable_all()
        .build()
        .unwrap();

    if let Err(e) = rt.block_on(run(opts)) {
        eprintln!("Error: {e}");
        std::process::exit(libc::EXIT_FAILURE);
    }
}
