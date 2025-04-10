use tera::{Tera, Context};

fn main() {
    // 1. Chargement des templates
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Erreur de chargement des templates: {}", e);
            return;
        }
    };

    // 2. Création du contexte
    let mut context = Context::new();
    context.insert("title", &"First Steps with Tera");
    context.insert("site_name", &"example.com");
    context.insert("message", &"Hello, world!");

    // 3. Rendu du template
    match tera.render("index.html", &context) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Erreur de rendu: {}", e),
    }
}