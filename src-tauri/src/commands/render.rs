use tera::{Context, Tera};
use tauri::command;


fn get_tera() -> Tera {
    Tera::new("src/templates/**/*.html").expect("Failed to initialize Tera templates")
}

#[command]
pub fn render_home() -> Result<String, String> {
    println!("Rendering home!");
    let tera = get_tera();
    tera.render("home.html", &Context::new()).map_err(|e| e.to_string())
}

#[command]
pub fn get_header_logo() -> Result<String, String> {
    let tera = get_tera();
    let context = Context::new();
    tera.render("header.html", &context).map_err(|e| e.to_string())
}


// Add similar functions for rendering other pages
