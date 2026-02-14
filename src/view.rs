use crate::album::Album;
use crate::browser::DirPage;
use askama::Template;
use serde;
use serde_json::json;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")]
// using the template in this path, relative
// to the `templates` dir in the crate root
struct IndexTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
    // in your template
    album: &'a str,
    total: usize,
    prefix: &'a str,
    admin: bool,
    testurl: &'a str,
    mode: &'a str,
}

pub fn render_index(album: &Album, prefix: &str, admin: bool) -> String {
    // let album_json = serde_json::to_string(&album).unwrap();
    let album_json = json!({
        "name": album.name,
        "images": &album.images,
    });
    // .unwrap();

    let template = IndexTemplate {
        name: album.name.as_str(),
        album: &album_json.to_string(),
        total: album.images.len(),
        testurl: "/a/a name with space/subfolder",
        prefix,
        mode: "serve",
        admin,
    };
    template.render().unwrap()
}

pub fn render_browser(dir: DirPage, prefix: &str, admin: bool) -> String {
    // let album_json = serde_json::to_string(&album).unwrap();
    let album_json = json!({
        "name": &dir.name,
        "path": &dir.path,
        "images": &dir.images,
        "folders": &dir.dirs
    });
    // .unwrap();
    let mode = format!("browse");
    let template = IndexTemplate {
        name: &dir.name.as_str(),
        album: &album_json.to_string(),
        total: dir.images.len(),
        testurl: "/a/a name with space/subfolder",
        mode: &mode,
        prefix,
        admin,
    };
    template.render().unwrap()
}
