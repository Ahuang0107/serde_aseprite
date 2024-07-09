#[test]
fn serialize() {
    // aseprite -b -data tests/sample.json --format=json-array --sheet-type=packed --sheet-pack --split-layers --filename-format='{layer}#{frame}' --list-layers --list-tags tests/sample.aseprite
    let output = serde_aseprite::AsepriteDate::from("tests/sample.json").unwrap();
    println!("{output:#?}");
    assert!(output.frames.len() > 0);
    assert!(output.meta.frame_tags.len() > 0);
    assert!(output.meta.layers.len() > 0);
}
