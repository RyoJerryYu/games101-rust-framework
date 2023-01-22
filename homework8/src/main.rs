mod application;
mod drawer;
mod rope;

use application::{App, AppConfig, Application};
use drawer::{Drawer, SimpleDrawer, SimpleDrawerConfig};
use utils::{
    graphic::{save_image, start_loop, Action, Control},
    rasterizer::Rasterizable,
};

fn main() {
    let save_path = "output.png";
    let mut r = SimpleDrawer::new(1000, 1000, SimpleDrawerConfig::default());
    let mut app = Application::new(AppConfig::default());

    let (w, h) = r.size();
    start_loop(w, h, move |actions, display_image| {
        for action in actions {
            match action {
                Action::Stop => {
                    save_image(&r, &save_path)?;
                    return Ok(Control::Stop);
                }
                _ => app.handle_event(action),
            }
        }

        r.clear();
        app.render(&mut r);

        display_image(&mut r)?;
        Ok(Control::Continue)
    })
}
