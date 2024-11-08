use crate::prelude::*;

fn get_title_transformer(class: &String) -> fn(String) -> String {
    match class.as_str() {
        "kitty" => |_| "Terminal".to_string(),
        "code-url-handler" => |s| s.replace("Visual Studio Code", "VSCode"),
        "firefox" => |s| s.replace("Mozilla Firefox", "Firefox"),
        "" => |_| "Desktop".to_string(),
        _ => |s| s,
    }
}

fn override_class(class: String) -> String {
    match class.as_str() {
        "code-url-handler" => "vscode".to_string(),
        "" => "hyprland".to_string(),
        _ => class.clone(),
    }
}

fn override_title(title: String, class: &String) -> String {
    get_title_transformer(class)(title)
}

#[allow(non_snake_case)]
pub fn ActiveWindow() -> EventBox {
    widget! {
        fun(interactable::Props) Interactable {
            child: opt Label {}
        }
    }
}
