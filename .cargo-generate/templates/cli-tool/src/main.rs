use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::exit;

mod application;
mod plugins;

const CONFIGS_DEFAULT_PATH: &str = "/etc/{{project-name}}/";

/// Operations on config files
#[derive(Subcommand)]
enum CommandConfig {
    /// Dump parsed config file. Helps to find typos
    Dump,
    /// Print config files documentation
    Documentation,
}

/// {{project-name}} main command
#[derive(Subcommand)]
enum CommandLine {
    /// Operations on config files
    #[command(subcommand)]
    Config(CommandConfig),
    /// Run CLI application
    Run(crate::application::Run),
}

/// Application command line
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ApplicationCommandLine {
    /// Path to configuration files
    #[clap(short, default_value = CONFIGS_DEFAULT_PATH)]
    configs_path: String,
    /// Command to run
    #[clap(subcommand)]
    command: CommandLine,
}

struct Application {
    pub command_line: ApplicationCommandLine,
    pub plugins: Vec<Box<dyn webapp_core::plugin::PluginMetadata>>,
}

impl Application {
    fn config_documentation(&self) {
        println!(
            "Configuration file format. Default path is {}.\nAll YAML configs has form {:?}\n",
            CONFIGS_DEFAULT_PATH,
            std::path::Path::new(&self.command_line.configs_path).join("<PLUGIN_NAME>.yaml")
        );
        for plugin in &self.plugins {
            if let Some(doc) = plugin.config_documentation() {
                println!(
                    "Config documentation for plugin {:?}:\n\n{}\n",
                    plugin.plugin_name(),
                    doc
                )
            }
        }
    }

    fn config_dump(&self) {
        println!("Below is dumped configuration files for all plugins\n",);
        let mut has_error = false;
        for plugin in &self.plugins {
            match plugin.config_dump() {
                Ok(Some(dump)) => {
                    println!("Config for plugin {:?}:\n\n{}", plugin.plugin_name(), dump)
                }
                Ok(None) => (),
                Err(err) => {
                    eprintln!(
                        "Failed to dump config for plugin {:?}: {}\n",
                        plugin.plugin_name(),
                        err
                    );
                    has_error = true
                }
            }
        }
        if has_error {
            exit(1)
        }
    }

    async fn run_command(&self) -> Result<()> {
        match &self.command_line.command {
            CommandLine::Config(CommandConfig::Dump) => {
                self.config_dump();
                Ok(())
            }
            CommandLine::Config(CommandConfig::Documentation) => {
                self.config_documentation();
                Ok(())
            }
            CommandLine::Run(v) => {
                v.run(
                    std::path::Path::new(&self.command_line.configs_path),
                    &self.plugins,
                )
                .await
            }
        }
    }

    pub async fn run(&self) {
        if let Err(err) = self.run_command().await {
            eprintln!("{}", err);
        }
    }
}

#[actix_web::main]
async fn main() {
    let command_line = ApplicationCommandLine::parse();
    let plugins = crate::plugins::register(std::path::Path::new(&command_line.configs_path))
        .expect("Failed to add plugins");
    let app = Application {
        command_line,
        plugins,
    };
    app.run().await
}
