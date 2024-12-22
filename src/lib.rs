use clap::{Parser, Subcommand};
use pyo3::prelude::*;
use std::collections::BTreeSet;

#[derive(Parser)]
#[command(name = "py-cli")]
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
fn run_cli(args: Vec<String>) -> PyResult<()> {
    let cli = Cli::try_parse_from(args)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    Python::with_gil(|py| {
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
    })
}

#[pymodule]
#[pyo3(name = "_cli")]
fn pyq(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli, py)?)?;
    Ok(())
}
