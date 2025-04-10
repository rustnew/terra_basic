# Description Complète du Code Tera en Rust

Ce code démontre l'utilisation basique du moteur de templates Tera en Rust pour générer du contenu HTML dynamique. Voici une analyse détaillée :

## 1. Initialisation du moteur Tera
```rust
let tera = match Tera::new("templates/**/*.html") {
    Ok(t) => t,
    Err(e) => {
        eprintln!("Erreur de chargement des templates: {}", e);
        return;
    }
};
```
- **`Tera::new()`** charge tous les fichiers `.html` du dossier `templates/` et ses sous-dossiers
- Le pattern `**/*.html` est un glob qui signifie "tous les fichiers HTML dans tous les sous-répertoires"
- La gestion d'erreur est faite via `match` pour éviter les panics (meilleure pratique Rust)

## 2. Création du contexte
```rust
let mut context = Context::new();
context.insert("title", &"First Steps with Tera");
context.insert("site_name", &"example.com");
context.insert("message", &"Hello, world!");
```
- **`Context`** est un conteneur de données clé-valeur
- Chaque `insert()` ajoute une variable disponible dans le template
- Les données sont passées par référence (`&str`) pour éviter les copies inutiles

## 3. Rendu du template
```rust
match tera.render("index.html", &context) {
    Ok(output) => println!("{}", output),
    Err(e) => eprintln!("Erreur de rendu: {}", e),
}
```
- **`render()`** combine le template avec les données du contexte
- Cherche le fichier `index.html` dans les templates chargés
- Gère proprement les erreurs (fichier manquant, syntaxe invalide, etc.)

## Structure Implicite du Projet
Pour que ce code fonctionne, votre projet doit avoir cette structure :
```
.
├── Cargo.toml
├── src/
│   └── main.rs
└── templates/
    └── index.html
```

## Exemple de Template (index.html)
```html
<!DOCTYPE html>
<html>
<head>
    <title>{{ title }} | {{ site_name }}</title>
</head>
<body>
    <h1>{{ message }}</h1>
    <p>Bienvenue sur {{ site_name }}</p>
</body>
</html>
```

## Flux d'Exécution
1. Chargement et parsing des templates
2. Préparation des données dans le contexte
3. Fusion du template avec les données
4. Sortie du HTML généré

## Bonnes Pratiques Illustrées
- Gestion d'erreur explicite (pas de `.unwrap()`)
- Utilisation de références pour les strings
- Organisation modulaire (templates séparés du code)
- Utilisation de types safe (le compilateur vérifie les types des variables du template)

Ce code représente une base solide pour générer du contenu web dynamique en Rust, facilement extensible pour des cas plus complexes.
