use clap::{Parser, Subcommand};
use pyo3::prelude::*;
use std::collections::BTreeSet;

#[derive(Parser)]
#[command(name = "pyq")]
#[command(about = "A CLI tool for Python information", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display Python version
    Version,
    /// Check if Python packages are installed
    Package {
        /// Names of packages to check
        #[arg(required = true)]
        names: Vec<String>,
    },
}

#[pyfunction]
fn main(py: Python) -> PyResult<()> {
    // Get command line arguments from sys.argv
    let args = py
        .import("sys")?
        .getattr("argv")?
        .extract::<Vec<String>>()?;

    // Create new args vector without the script name
    let filtered_args: Vec<String> = args.into_iter().skip(1).collect();

    // Parse the filtered arguments
    let cli = Cli::try_parse_from(filtered_args).unwrap_or_else(|e| {
        e.exit();
    });

    match cli.command {
        Commands::Version => {
            let sys = py.import("sys")?;
            let version: String = sys.getattr("version")?.extract()?;
            println!("Python version: {}", version);
        }
        Commands::Package { names } => {
            let importlib_util = py.import("importlib.util")?;
            let mut installed = BTreeSet::new();
            let mut missing = BTreeSet::new();

            for name in names {
                let spec = importlib_util
                    .getattr("find_spec")?
                    .call1((name.as_str(),))?;
                if spec.is_none() {
                    missing.insert(name);
                } else {
                    installed.insert(name);
                }
            }

            if !installed.is_empty() {
                println!("Installed packages:");
                for name in &installed {
                    println!("  ✅ {}", name);
                }
            }

            if !missing.is_empty() {
                if !installed.is_empty() {
                    println!();
                }
                println!("Missing packages:");
                for name in missing {
                    println!("  ❌ {}", name);
                }
            }
        }
    }
    Ok(())
}

#[pymodule]
fn pyq(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, py)?)?;
    Ok(())
}
