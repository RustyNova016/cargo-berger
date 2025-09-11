use std::process::Output;

use color_eyre::eyre::eyre;

use crate::ColEyre;

pub fn unwrap_status(out: Output) -> ColEyre {
    if !out.status.success() {
        return Err(eyre!(
            "Couldn't run command:\n{}\n\n{}",
            String::from_utf8_lossy(&out.stdout).to_string(),
            String::from_utf8_lossy(&out.stderr).to_string()
        ));
    }

    Ok(())
}
