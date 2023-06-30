use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AsepriteDate {
    pub frames: Vec<Frame>,
}

impl AsepriteDate {
    pub fn from<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(
            std::fs::read_to_string(path)?.as_str(),
        )?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub filename: String,
    /// used to cut from sprite-sheet
    pub frame: Rectangle,
    /// represent the position in the source
    /// the w/h should be same as frame 👆
    pub sprite_source_size: Rectangle,
    pub source_size: Size,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}
