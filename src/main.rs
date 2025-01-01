mod lib;
mod publish;
mod install;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[derive(Debug)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
    
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    ///install a specific package
    Install {
        ///name of package to install, defults to latest release, to specify
        ///version add @V:[version] after package name, e.g. `locate_zahirs_dad@V:[1.0.1]`
        package: String
    },
    
    ///publish a package to gpm
    Publish {
        ///manifest.toml
        #[clap(long)]
        manifest: PathBuf,

        ///file/files to publish
        #[clap(long)]
        files: PathBuf,
    }
}

fn main() {
    let args = Args::parse();
     
    match &args.cmd {
        Commands::Install { package } => {
            install::install(package)
        },
        Commands::Publish { manifest, files } => {}
    }
}
