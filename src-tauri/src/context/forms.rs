use serde_json::Value;
use tera::{Context, Tera};

// Renders from serde_json serialized Value
pub fn generate_form_html(json_data: Value, template_path: Option<&str>) -> Result<String, String> {
    // Init tera templating
    let tera = Tera::new("**/templates/*.html.tera").map_err(|e| e.to_string())?;
    println!("\n\n!!\nTERA INIT RES: {:#?}", tera);

    // Create a Tera ctx
    let mut context = Context::new();
    context.insert("data", &json_data);
    println!("\n\n!!\nTERA CTX DATA INSERT: {:#?}", context);

    // Use a default template if none is provided
    let template = template_path.unwrap_or("templates/config.html.tera");
    println!("\n\n!!\nTEMPLATE PATH ARG: {:#?}", template);

    let rendered_html = tera.render(template, &context).map_err(|e| e.to_string());
    println!("\n\n!!\nRENDERED HTML: {:#?}", rendered_html);

    Ok(rendered_html?)
}
