#[test]
fn serialize() {
    // aseprite -b -data tests/sample.json --format=json-array --sheet-type=packed --sheet-pack --split-layers --filename-format='{layer}#{frame}' --list-layers --list-tags tests/sample.aseprite
    let output = serde_aseprite::AsepriteDate::from("tests/sample.json").unwrap();
    println!("{output:#?}");
    assert!(output.frames.len() > 0);
    assert!(output.meta.frame_tags.len() > 0);
    assert!(output.meta.layers.len() > 0);
}

#[test]
fn serialize_2() {
    // aseprite -b -data tests/sample.json --format=json-array --sheet-type=packed --sheet-pack --split-layers --filename-format='{layer}#{frame}' --list-layers --list-tags tests/sample.aseprite
    let bytes = std::fs::read("tests/sample.json").unwrap();
    let output = serde_aseprite::AsepriteDate::from_bytes(bytes.as_slice()).unwrap();
    println!("{output:#?}");
    assert!(output.frames.len() > 0);
    assert!(output.meta.frame_tags.len() > 0);
    assert!(output.meta.layers.len() > 0);
}
