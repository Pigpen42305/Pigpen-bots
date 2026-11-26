use std::env::var;

pub fn setup_env() -> std::io::Result<bool> {
    #[cfg(target_os = "windows")]
    {
        use std::env::{current_exe, set_var};
        use std::io::{Error, ErrorKind};
        use std::{fs::File, io::Read};

        let mut path = current_exe()?
            .parent()
            .ok_or(Error::from(ErrorKind::NotFound))?
            .parent()
            .ok_or(Error::from(ErrorKind::NotFound))?
            .parent()
            .ok_or(Error::from(ErrorKind::NotFound))?
            .to_path_buf();
        path.push(".env");
        let Ok(mut env_file) = File::open(&path) else {
            return Ok(false);
        };
        let mut buffer = String::new();
        let Ok(_) = env_file.read_to_string(&mut buffer) else {
            return Ok(false);
        };
        for (key, value) in buffer
            .split_terminator('\n')
            .map(|x| x.trim())
            .into_iter()
            .map(|x| x.split_once('=').expect("Invalid format!"))
        {
            // SAFETY: This is safe to call on Windows.
            unsafe { set_var(key, value) };
        }
    }
    Ok(if let Err(_) = var("DISCORD_TOKEN") {
        false
    } else {
        true
    })
}
