use serde_json::Value;
use tera::{Context, Tera};

// Renders from serde_json serialized Value
pub fn generate_form_html(json_data: Value, template_path: Option<&str>) -> Result<String, String> {
    let tera = Tera::new("**/templates/*.html.tera").map_err(|e| e.to_string())?;
    let mut context = Context::new();
    context.insert("data", &json_data);
    let template = template_path.unwrap_or("templates/config.html.tera");

    let rendered_html = tera.render(template, &context).map_err(|e| e.to_string());
    Ok(rendered_html?)
}
