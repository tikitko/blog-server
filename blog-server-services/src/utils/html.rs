pub fn clean(src: &str) -> String {
    ammonia::Builder::default()
        .add_generic_attributes(&["style"])
        .add_tag_attributes("table", &["border"])
        .add_allowed_classes("img", &["article-img"])
        .add_tags(&["video"])
        .add_tag_attributes("video", &["controls", "autoplay", "loop"])
        .add_allowed_classes("video", &["article-img"])
        .add_tags(&["source"])
        .add_tag_attributes("source", &["src", "type"])
        .add_tags(&["iframe"])
        .add_tag_attributes(
            "iframe",
            &[
                "src",
                "allowfullscreen",
                "width",
                "height",
                "frameBorder",
                "allow",
                "loading",
            ],
        )
        .add_allowed_classes("iframe", &["article-iframe"])
        .clean(src)
        .to_string()
}

pub fn to_plain(src: &str) -> String {
    html2text::from_read(src.as_bytes(), usize::MAX)
}
