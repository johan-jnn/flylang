use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum LangPkgCLI {
    /// Install a new package in the current project
    #[command()]
    Install {
        /// The package to install in the project (format: <package_name>[:<version>])
        packages: Vec<String>,

        /// If the packages should be installed globaly
        #[arg(short = 'G', long)]
        global: bool,
    },

    /// Init a new flylang project
    #[command()]
    Init {
        /// The name of the project
        #[arg(default_value = ".")]
        name: String,
    },
}
