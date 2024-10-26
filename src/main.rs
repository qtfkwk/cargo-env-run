use {
    anyhow::{anyhow, Result},
    clap::{builder::Styles, Parser},
    indexmap::{IndexMap, IndexSet},
    serde::Deserialize,
    std::path::PathBuf,
};

const STYLES: Styles = Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo", styles = STYLES)]
enum Cli {
    EnvRun(EnvRun),
}

#[derive(clap::Args, Clone)]
#[command(about, version, max_term_width = 80)]
struct EnvRun {
    command: Vec<String>,
}

#[derive(Deserialize)]
struct CargoConfigToml {
    env: Option<IndexMap<String, String>>,
}

fn main() -> Result<()> {
    let Cli::EnvRun(cli) = Cli::parse();

    if cli.command.is_empty() {
        return Err(anyhow!("No command provided"));
    }

    let mut env = IndexMap::<String, String>::new();
    let mut files = IndexSet::<PathBuf>::new();
    let mut dir = std::env::current_dir().unwrap();
    loop {
        files.insert(dir.join(".cargo/config.toml"));

        if let Some(p) = dir.parent() {
            dir = p.to_path_buf();
        } else {
            break;
        }
    }
    let home_config = directories::UserDirs::new()
        .unwrap()
        .home_dir()
        .join(".cargo/config.toml");
    if !files.contains(&home_config) {
        files.insert(home_config);
    }
    for file in files.iter().rev().filter(|x| x.exists()) {
        let content = std::fs::read_to_string(file)?;
        let mut data: CargoConfigToml = toml::from_str(&content)?;
        if let Some(mut d) = data.env.take() {
            env.append(&mut d);
        }
    }

    let (prog, args) = (&cli.command[0], &cli.command[1..]);
    let mut c = std::process::Command::new(prog)
        .args(args)
        .envs(env)
        .spawn()?;
    match c.wait()?.code() {
        Some(code) => match code {
            0 => Ok(()),
            _ => Err(anyhow!(
                "Command `{prog} {}` failed with code {code}",
                args.join(" ")
            )),
        },
        None => Err(anyhow!("Command `{prog} {}` failed", args.join(" "))),
    }
}
