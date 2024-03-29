# Crous

Un programme en ligne de commande simple qui récupère les menus d'un restaurant Crous.

![example](./example.png)

# Utilisation

```
Utilisation: crous [OPTIONS] [RESTAURANTS]...

Arguments:
  [RESTAURANTS]...  Restaurants à afficher le menu (utilise les alias)

Options:
  -d, --days <DAYS>  Nombre de jours à afficher [default: 1]
```

# Configuration

Le nom des restaurants (en arguments) sont définie dans le fichier de configuration `~/.config/crous/crous.toml` avec le format suivant:

```toml
default = "haut-carre"
[aliases]
    haut-carre = "https://www.crous-bordeaux.fr/restaurant/restaurant-administratif-le-haut-carre-3/"
    ru2 = "https://www.crous-bordeaux.fr/restaurant/resto-u-n2/"
```


# Installation

- Installer rust et cargo (https://rustup.rs/)
- Cloner le dépôt
- `cd crous`
- Compiler avec `cargo build --release` (le binaire est dans `target/release/`
- Optionnel: Installer avec `cargo install --path .`

