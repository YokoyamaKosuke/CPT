pub struct CliOpts {
    #[arg(short, long, default_value_t = RunMode::Auto,
        value_name = "MODE", required = false,
        ignore_case = true, value_enum, help = "Mode of operation.")] 
    pub mode: RunMode,
    #[arg(short, long, default_value = ".", value_name = "DEST", required = false, 
          help = "Destination of the extraction results.")]
    pub dest: Option<PathBuf>,
    #[arg(short, long, default_value = "cpt.zip", value_name = "OUTPUT", required = false,
          help = "Output file for the archive.")]
    pub output: Option<PathBuf>,
    #[arg(short, long = "no-recursive", help = "No recursive mode.", default_value_t = false)] 
    pub no_recursive: bool,
    #[arg(short, long, help = "Display verbose output.", default_value_t = false)]
    pub verbose: bool,
    #[arg(value_name = "ARGUMENTS", help = "List of files or directories to be processed.",
          long_help = "extract mode: archive files to be extracted.
    archive mode: files to be archived.
    auto mode: if the arguments have archive files, it will extract them. Otherwise, it will archive the files.")]
      pub args: Vec<PathBuf>,
    }

    #[derive(Debug, Clone, ValueEnum, PartialEq)] 
    pub enum RunMode {
        Auto,
        Archive,
        Extract,
    }