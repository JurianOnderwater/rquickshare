use std::fs::File;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::sharing_nearby::FileMetadata;

#[derive(Debug)]
pub struct InternalFileInfo {
    pub meta: FileMetadata,
    pub payload_id: i64,
    pub destination_url: PathBuf,
    pub bytes_transferred: i64,
    pub file: Option<File>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransferMetadata {
    pub id: String,
    pub files: Vec<String>,
    pub pin_code: Option<String>,
    pub text_description: Option<String>,
}
