use suborbital::runnable::*;
use suborbital::{req, resp, file};

struct ServeFile{}

impl Runnable for ServeFile {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let mut filename = req::url_param("file");

        if filename.ends_with("/") {
            filename = format!("{}index.html", filename)
        }

        let file = match file::get_static(format!("app{}", filename).as_str()) {
            Some(f) => f,
            None => return Err(RunErr::new(404, "not found"))
        };

        if filename.ends_with(".html") {
            resp::content_type("text/html")
        } else if filename.ends_with(".wasm") {
            resp::content_type("application/wasm")
        } else if filename.ends_with(".css") {
            resp::content_type("text/css")
        } else if filename.ends_with(".js") {
            resp::content_type("application/javascript")
        }

        Ok(file)
    }
}


// initialize the runner, do not edit below //
static RUNNABLE: &ServeFile = &ServeFile{};

#[no_mangle]
pub extern fn init() {
    use_runnable(RUNNABLE);
}
