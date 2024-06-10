#[cfg(not(target_os = "macos"))]
const CRATE_MACOS_ONLY: () = panic!("Crate uses osascript which is MacOS only");

use std::{
    env, fs,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path,
    process::Command,
};
use tempfile::NamedTempFile;

#[derive(Debug)]
pub enum Error
{
    Io(io::Error),
    Var(env::VarError),
}

fn main() -> Result<(), Error>
{
    if let Some(cmd) = env::args().skip(1).next()
    {
        let path = &std::path::Path::new("/Users")
            .join(env::var("USER").or(env::var("LOGNAME"))?)
            .join("Library/Application Support/xbar/plugins")
            .join("toggle-natural-scrolling.1m.sh");

        if cmd == "install"
        {
            fs::write(&path, include_str!("toggle-natural-scrolling.sh"))?;
            make_executable(path)?;
            println!("Installed script\nRefreshing plugins...");
            return refresh_xbar_plugins().map_err(Into::into);
        }
        else if cmd == "uninstall"
        {
            fs::remove_file(path)?;
            println!("Uninstalled script\nRefreshing plugins...");
            return refresh_xbar_plugins().map_err(Into::into);
        }
        else
        {
            return Ok(println!("Usage: {} [install]", env!("CARGO_PKG_NAME")));
        }
    }

    let mut file = NamedTempFile::new()?;
    write!(file, "{}", include_str!("toggle-natural-scrolling.osa"))?;

    let output =
        Command::new("osascript").arg(file.path()).args(["_", "_"]).output()?;

    if String::from_utf8_lossy(&output.stdout) == "_\n"
    {
        println!("Toggled Natural Scrolling");
    }
    else
    {
        println!(
            "Something went wrong: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    };

    Ok(())
}

fn make_executable(path: &path::Path) -> Result<(), io::Error>
{
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
}

fn refresh_xbar_plugins() -> Result<(), std::io::Error>
{
    return Command::new("open")
        .arg("xbar://app.xbarapp.com/refreshAllPlugins")
        .status()
        .map(drop);
}

impl From<io::Error> for Error
{
    #[rustfmt::skip]
    fn from(value: io::Error) -> Self { Self::Io(value) }
}

impl From<env::VarError> for Error
{
    #[rustfmt::skip]
    fn from(value: env::VarError) -> Self { Self::Var(value) }
}
