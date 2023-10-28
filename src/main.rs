struct Label {
    label_title: String,
    label_content: Vec<String>,
    label_color: String,
    scene: Scene,
}

struct Scene {
    scene_title: String,
    scene_image: String,
}

impl Scene {
    fn new(scene_title: &str, scene_image: &str) -> Scene {
        Scene {
            scene_title: String::from(scene_title),
            scene_image: String::from(scene_image),
        }
    }
}

impl Label {
    fn new(label_title: &str, label_content: Vec<String>, label_color: &str) -> Label {
        Label {
            label_title: String::from(label_title),
            label_content,
            label_color: String::from(label_color),
            scene: Scene {
                scene_title: String::from(""),
                scene_image: String::from(""),
            },
        }
    }

    fn set_content(&mut self, label_content: Vec<String>) {
        self.label_content = label_content;
    }

    fn add_content(&mut self, label_content: String) {
        self.label_content.push(label_content);

    }

    fn set_title(&mut self, label_title: &str) {
        self.label_title = String::from(label_title);
    }

    fn set_color(&mut self, label_color: &str) {
        self.label_color = String::from(label_color);
    }

    fn set_scene(&mut self, scene_title: &str, scene_image: &str) {
        self.scene.scene_title = String::from(scene_title);
        self.scene.scene_image = String::from(scene_image);
    }

}

struct Character { // Changed the struct name from Char to Character
    char_title: String,
    char_name: String,
    char_image: String,
    char_color: String,
}

impl Character { // Changed the struct name from Char to Character
    fn new(char_title: &str, char_name: &str, char_image: &str, char_color: &str) -> Character {
        Character { // Changed the struct name from Char to Character
            char_title: String::from(char_title),
            char_name: String::from(char_name),
            char_image: String::from(char_image),
            char_color: String::from(char_color),
        }
    }

    fn set_char_title(&mut self, char_title: &str) {
        self.char_title = String::from(char_title);
    }

    fn set_char_name(&mut self, char_name: &str) {
        self.char_name = String::from(char_name);
    }
}

pub struct Screen {
    pub screen_title: String,
    screen_buttons: Button, // Changed the struct name from button to Button
    screen_background: String,
}

impl Screen {
    fn new(screen_title: &str, screen_buttons: Button, screen_background: &str) -> Screen {
        Screen {
            screen_title: String::from(screen_title),
            screen_buttons,
            screen_background: String::from(screen_background),
        }
    }

    fn set_screen_title(&mut self, screen_title: &str) {
        self.screen_title = String::from(screen_title);
    }

    fn set_screen_background(&mut self, screen_background: &str) {
        self.screen_background = String::from(screen_background);
    }

    fn set_screen_buttons(&mut self, screen_buttons: Button) {
        self.screen_buttons = screen_buttons;
    }
}

struct Button { // Changed the struct name from button to Button
    button_text: String,
    button_action: Action,
}

impl Button { // Changed the struct name from button to Button
    fn new(button_text: &str, button_action: Action) -> Button { // Changed the struct name from button to Button
        Button { // Changed the struct name from button to Button
            button_text: String::from(button_text),
            button_action,
        }
    }

    fn set_button_text(&mut self, button_text: &str) {
        self.button_text = String::from(button_text);
    }

    fn set_button_action(&mut self, button_action: Action) {
        self.button_action = button_action;
        match self.button_action.action_type.as_str() {
            "go to" => {
                println!("You are going to the {}", self.button_action.action_target);
            }
            "jump" => {
                println!("You are jumping");
            }
            _ => {
                println!("You are doing something else");
            }
    }
}
}

struct Action {
    action_type: String,
    action_target: String,
}

impl Action {
    fn new(action_type: &str, action_target: &str) -> Action {
        Action {
            action_type: String::from(action_type),
            action_target: String::from(action_target),
        }
    }
}

fn main() {
    // Create a new Label instance
    let initial_label_content = vec![
        "This is the first line of the label.".to_string(),
        "This is the second line of the label.".to_string(),
    ];

    let mut my_label = Label::new("start", initial_label_content, "white");

    // Create a new Character instance
    let mut my_char = Character::new("mc", "luke", "luke.png", "white");

    // Create a new Screen instance
    let mut my_screen = Screen::new("", Button::new("",
    Action::new("", "")), "");

    let mut my_button = Button::new("", Action::new("", ""));

    let mut button1: Button = Button::new("", Action::new("", ""));
    // Access and print the label's fields
    // println!("Label title: {}", my_label.label_title);
    // println!("Label content: {:?}", my_label.label_content);
    // println!("Label color: {}", my_label.label_color);
    // println!("");

    // // Access and print the character's fields
    // println!("Character title: {}", my_char.char_title);
    // println!("Character name: {}", my_char.char_name);
    // println!("Character image: {}", my_char.char_image);
    // println!("Character color: {}", my_char.char_color);
    // println!("");

    my_button.set_button_text("Go to the bathroom");
    my_button.set_button_action(Action::new("go to", "bathroom"));

    button1.set_button_text("Go to the bathroom");
    button1.set_button_action(Action::new("jump", "Kitchen"));


    my_screen.set_screen_title("Bathroom");
    my_screen.set_screen_background("bathroom.png");
    my_screen.set_screen_buttons(my_button); 
    


    // Access and print the screen's fields
    println!("Screen title: {}", my_screen.screen_title);
    println!("Screen button text: {}", my_screen.screen_buttons.button_text);
    println!("Screen background: {}", my_screen.screen_background);
}
