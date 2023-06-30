#[test]
fn serialize() {
    let output = serde_aseprite::AsepriteDate::from("tests/sample.json").unwrap();
    assert!(output.frames.len() > 0);
}
