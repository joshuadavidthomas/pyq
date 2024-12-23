use clap::{Parser, Subcommand};
use pyo3::prelude::*;
use std::collections::BTreeSet;
use std::env;

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
fn main() -> PyResult<()> {
    let args: Vec<String> = std::iter::once("pyq".to_string())
        .chain(env::args().skip(2))
        .collect();

    let cli = Cli::try_parse_from(args).unwrap_or_else(|e| {
        e.exit();
    });

    match cli.command {
        Commands::Version => {
            // We still need Python for this specific operation
            Python::with_gil(|py| {
                let sys = py.import("sys")?;
                let version: String = sys.getattr("version")?.extract()?;
                println!("Python version: {}", version);
                Ok::<(), PyErr>(())
            })?;
        }
        Commands::Package { names } => {
            // We still need Python for this specific operation
            Python::with_gil(|py| {
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
                Ok::<(), PyErr>(())
            })?;
        }
    }
    Ok(())
}

#[pymodule]
fn pyq(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(main))?;
    Ok(())
}
