use crate::utils::ui::button::Button;

pub struct NomButtons {
    follow_mouse: Button,
    avoid_mouse: Button,
    click_target: Button,
    control: Button,
    remove: Button,
}

impl NomButtons {
    pub fn new() -> Self {
        let mut remove = Button::new("Remove");
        remove.pos(97.0 + 15.0, 680.0);

        let mut control = Button::new("Control");
        control.pos(20.0 + 15.0, 680.0);

        let mut follow_mouse = Button::new("Follow mouse");
        follow_mouse.pos(20.0, 648.0);

        let mut avoid_mouse = Button::new("Avoid mouse");
        avoid_mouse.pos(132.0, 648.0);

        let mut click_target = Button::new("Click target");
        click_target.pos(236.0, 648.0);

        Self {
            follow_mouse,
            avoid_mouse,
            click_target,
            control,
            remove,
        }
    }

    pub fn draw(&self) {
        self.follow_mouse.draw();
        self.avoid_mouse.draw();
        self.click_target.draw();
        self.control.draw();
        self.remove.draw();
    }

    pub fn update(&mut self) {
        self.follow_mouse.update();
        self.avoid_mouse.update();
        self.click_target.update();
        self.control.update();
        self.remove.update();
    }
}
