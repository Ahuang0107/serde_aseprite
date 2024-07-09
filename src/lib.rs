use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AsepriteDate {
    pub size: Size,
    pub frames: Vec<Frame>,
    pub meta: Meta,
}

impl AsepriteDate {
    pub fn from<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(
            std::fs::read_to_string(path)?.as_str(),
        )?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub filename: String,
    /// used to cut from sprite-sheet
    pub frame: Rectangle,
    /// represent the position in the source
    /// the w/h should be same as frame 👆
    pub sprite_source_size: Rectangle,
    pub source_size: Size,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub frame_tags: Vec<FrameTag>,
    pub layers: Vec<MetaLayer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrameTag {
    pub name: String,
    pub from: usize,
    pub to: usize,
    pub direction: String,
    pub color: String,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MetaLayer {
    Layer(Layer),
    Group(Group),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub name: String,
    pub group: Option<String>,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub name: String,
    pub opacity: u8,
    pub blend_mode: BlendMode,
    /// belong to which group
    pub group: Option<String>,
    pub data: Option<String>,
    #[serde(default = "Vec::new")]
    pub cels: Vec<LayerCel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LayerCel {
    pub frame: usize,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BlendMode {
    Normal,
    Darken,
    Multiply,
    ColorBurn,
    Lighten,
    Screen,
    ColorDodge,
    Addition,
    Overlay,
    SoftLight,
    HardLight,
    Difference,
    Exclusion,
    Subtract,
    Divide,
    Hue,
    Saturation,
    Color,
    Luminosity,
}
