use std::process::Output;

use serde::de::DeserializeOwned;

use crate::ColEyre;

#[extend::ext]
pub impl Output {
    fn into_string(&self) -> String {
        String::from_utf8_lossy(&self.stdout).to_string()
    }

    fn deserialize<T>(&self) -> ColEyre<T>
    where
        T: DeserializeOwned,
    {
        Ok(serde_json::from_str(&self.into_string())?)
    }
}
