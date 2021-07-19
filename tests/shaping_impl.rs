pub fn shape(font: &str, font_size: usize, variations: &[(&str, f32)], input: &str) -> String {
    let file = std::fs::read(font).unwrap();
    let font = swash::FontRef::from_index(&file, 0).unwrap();
    let mut context = swash::shape::ShapeContext::new();
    let builder = context
        .builder(font)
        .size(font_size as f32)
        .variations(variations);

    let mut advance = 0.0;
    let mut output = Vec::new();
    let mut shaper = builder.build();

    shaper.add_str(input);
    shaper.shape_with(|cluster| {
        cluster.glyphs.iter().for_each(|glyph| {
            if advance == 0.0 {
                output.push(format!(
                    "{}",
                    font.glyph_name(glyph.id)
                        .map(|x| x.to_string())
                        .unwrap_or(format!("gid{}", glyph.id))
                ));
            } else {
                output.push(format!(
                    "{}@{},{}",
                    font.glyph_name(glyph.id)
                        .map(|x| x.to_string())
                        .unwrap_or(format!("gid{}", glyph.id)),
                    (glyph.x + advance).ceil() as usize,
                    glyph.y as usize,
                ));
            }
            advance += glyph.advance;
        });
    });

    let collected: String = output
        .iter()
        .enumerate()
        .map(|(idx, i)| {
            if idx == 0 {
                i.to_owned()
            } else {
                format!("|{}", i)
            }
        })
        .collect();
    format!("[{}]", collected)
}
