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

    fn exit_on_non_zero_with<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        if !self.success() {
            f();
            exit(self.code().unwrap_or(1))
        }
    }
}
