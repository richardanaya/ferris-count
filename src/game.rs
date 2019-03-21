use web_dom::*;
const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

pub struct GameState {
    ferris_count: i32,
    window: Element,
    image: DOMReference,
    ctx: DOMReference,
    frame_time: f64,
    request_animation_frame_listener: EventListener,
    input: Element,
    input_change_listener: EventListener,
}

impl GameState {
    pub fn new() -> GameState {
        let win = window();
        let doc = window::get_document(win);
        let screen = element::query_selector(doc, "#screen");
        let input = element::query_selector(doc, "select");
        return GameState {
            ferris_count: 1,
            window: win,
            image: element::query_selector(doc, "#ferris"),
            ctx: htmlcanvas::get_context(screen, "2d"),
            frame_time: date::now(),
            request_animation_frame_listener: create_event_listener(),
            input: input,
            input_change_listener: create_event_listener(),
        };
    }

    pub fn init(&mut self) {
        window::request_animation_frame(self.window, self.request_animation_frame_listener);
        eventtarget::add_event_listener(self.input, "change", self.input_change_listener);
    }

    pub fn route_event(&mut self, listener: EventListener, _event: Event) {
        if listener == self.request_animation_frame_listener {
            self.run();
            window::request_animation_frame(self.window, self.request_animation_frame_listener);
        } else if listener == self.input_change_listener {
            let value = htmlinput::get_value(self.input);
            self.ferris_count = value.parse::<i32>().unwrap();
        }
    }

    pub fn run(&mut self) {
        self.clear();
        for _ in 0..self.ferris_count{
            drawing::draw_image(self.ctx, self.image, 0, 0, 128, 86,(math::random()*(WIDTH-128) as f32) as i32, (math::random()*(HEIGHT-86) as f32) as i32, 128, 86);
        }
        let diff = date::now()-self.frame_time;
        self.frame_time = date::now();
        if diff != 0.0 {
            let fps = (1000.0/diff) as i32;
            drawing::set_fill_style(self.ctx, "white");
            drawing::set_font(self.ctx, "30px Arial");
            drawing::fill_text(self.ctx, &fps.to_string(), 50, 50, 1000);
        }
    }

    pub fn clear(&self) {
        drawing::clear_rect(self.ctx, 0, 0, WIDTH, HEIGHT);
    }
}
