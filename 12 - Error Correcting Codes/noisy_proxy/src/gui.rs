use conrod::{self, Color, Colorable, Slider, Theme, Positionable, Widget, Sizeable, Labelable};
use piston_window::{self, EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings, RenderEvent};
use find_folder;

use std::sync::{Mutex, Arc};

type Backend = (<piston_window::G2d<'static> as conrod::Graphics>::Texture, Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

pub fn launch(slider_value: Arc<Mutex<f64>>) {
    let mut window: PistonWindow = WindowSettings::new("Noise Slider", [1024, 100])
        .exit_on_esc(true)
        .build()
        .expect("Failed to create window");

    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets")
            .expect("Couldn't find assets folder");
        let font_path = assets.join("NotoSans-Regular.ttf");
        let glyph_cache = Glyphs::new(&font_path, window.factory.clone());
        let mut theme = Theme::default();
        theme.background_color = Color::Rgba(100.0, 100.0, 100.0, 100.0);
        Ui::new(glyph_cache.expect("Failed to load glyphs"), Theme::default())
    };

    window.set_ups(60);

    while let Some(event) = window.next() {
        ui.handle_event(&event);
        event.update(|_| ui.set_widgets(|ref mut ui| {
            let current_slider_value: f64 = *slider_value.lock().unwrap();

            // Generate the ID for the Slider.
            widget_ids!(CANVAS, SLIDER);

            conrod::Canvas::new().set(CANVAS, ui);

            // Draw the slider and indicate the current value as its label
            conrod::Slider::new(current_slider_value, 0.0, 20.0)
                .skew(5.0)
                .rgba(100.0, 100.0, 100.0, 100.0)
                .middle_of(CANVAS)
                .kid_area_wh_of(CANVAS)
                .label(&format!("{}", current_slider_value))
                .react(|new_value| {
                    let mut slider_value = slider_value.lock().unwrap();
                    *slider_value = new_value;
                })
                .set(SLIDER, ui);
        }));

        window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
    }
}
