use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let shell = select_shell()?;
    let features = select_features(&shell)?;
    let config_content = generate_config(&shell, &features)?;
    let config_path = get_config_path(&shell)?;
    write_config(&config_path, &config_content)?;
    println!("{} config has been generated at {:?}", shell, config_path);
    Ok(())
}

fn select_shell() -> io::Result<String> {
    let shells = vec!["Fish", "Zsh"];
    let shell_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the shell to configure")
        .items(&shells)
        .interact()?;
    Ok(shells[shell_selection].to_string())
}

fn select_features(shell: &str) -> io::Result<Vec<String>> {
    let items = match shell {
        "Fish" => vec!["Homebrew initialization", "zoxide initialization"],
        "Zsh" => vec!["Homebrew initialization", "zoxide initialization"],
        _ => vec![],
    };
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select features to include in your config")
        .items(&items)
        .interact()?;
    Ok(selections
        .into_iter()
        .map(|i| items[i].to_string())
        .collect())
}

fn generate_config(shell: &str, features: &[String]) -> io::Result<String> {
    let mut config_content = String::new();
    for feature in features {
        match (shell, feature.as_str()) {
            ("Fish", "Homebrew initialization") => {
                config_content.push_str(
                    r#"
if test -d /opt/homebrew
    set -gx PATH /opt/homebrew/bin $PATH
    set -gx MANPATH /opt/homebrew/share/man $MANPATH
    set -gx INFOPATH /opt/homebrew/share/info $INFOPATH
end
"#,
                );
            }
            ("Fish", "zoxide initialization") => {
                config_content.push_str(
                    r#"
if type -q zoxide
    zoxide init fish | source
end
"#,
                );
            }
            ("Zsh", "Homebrew initialization") => {
                config_content.push_str(
                    r#"
if [ -d /opt/homebrew ]; then
    export PATH=/opt/homebrew/bin:$PATH
    export MANPATH=/opt/homebrew/share/man:$MANPATH
    export INFOPATH=/opt/homebrew/share/info:$INFOPATH
fi
"#,
                );
            }
            ("Zsh", "zoxide initialization") => {
                config_content.push_str(
                    r#"
if command -v zoxide &> /dev/null; then
    eval "$(zoxide init zsh)"
fi
"#,
                );
            }
            _ => {}
        }
    }
    Ok(config_content)
}

fn get_config_path(shell: &str) -> io::Result<PathBuf> {
    let config_path = match shell {
        "Fish" => dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".config/fish/config.fish"),
        "Zsh" => dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".zshrc"),
        _ => PathBuf::from("config"),
    };
    Ok(config_path)
}

fn write_config(config_path: &PathBuf, config_content: &str) -> io::Result<()> {
    let mut file = File::create(config_path)?;
    file.write_all(config_content.as_bytes())?;
    Ok(())
}
