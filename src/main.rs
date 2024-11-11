use clap::{Parser, Subcommand};
use sysinfo::{Components, Disks, Networks, System};

#[derive(Parser)]
#[command(version, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    Mem {
        #[arg(short, long)]
        show: bool,
        #[arg(short, long)]
        clear: bool,
    },
}
fn main() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Mem { show, clear }) => {
            let mut sys = System::new_all();
            sys.refresh_all();
            if *clear {
                sys.refresh_memory();
            }
            if *show {
                let showmem = sys.total_memory();
                println!("Memory: {} KB", &showmem);
            } else {
                let mem = sys.total_memory();
                println!("Memory: {} KB", &mem);
            }
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
