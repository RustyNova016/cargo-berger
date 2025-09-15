use std::process::ExitStatus;
use std::process::Output;

use color_eyre::eyre::eyre;

use crate::ColEyre;
use crate::ColEyreVal;

/// Create an eyre error on bad status
pub fn assert_status(status: ExitStatus) -> ColEyre {
    if !status.success() {
        return Err(eyre!("Command returned non zero exit status"));
    }

    Ok(())
}

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

pub fn unwrap_status_out(out: Output) -> ColEyreVal<String> {
    if !out.status.success() {
        return Err(eyre!(
            "Couldn't run command:\n{}\n\n{}",
            String::from_utf8_lossy(&out.stdout).to_string(),
            String::from_utf8_lossy(&out.stderr).to_string()
        ));
    }

    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}
