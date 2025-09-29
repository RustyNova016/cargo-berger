use std::process::ExitStatus;
use std::process::exit;

use extend::ext;

#[ext]
pub impl ExitStatus {
    /// Exit the app on a non zero status. 
    fn exit_on_non_zero(&self) {
        if !self.success() {
            exit(self.code().unwrap_or(1))
        }
    }
}
