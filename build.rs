fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
    embed_resource::compile("app.rc", embed_resource::NONE);
}