# Crous

Un programme en ligne de commande simple qui récupère les menus d'un restaurant Crous.

![example](./example.png)

# Utilisation

```
Utilisation: crous [OPTIONS]

Options:
  -u, --url <URL>    Url pour récupérer le menu [default: https://www.crous-bordeaux.fr/restaurant/restaurant-administratif-le-haut-carre-3/]
  -d, --days <DAYS>  Nombre de jours à afficher [default: 1]
```

# Installation

- Installer rust et cargo (https://rustup.rs/)
- Cloner le dépôt
- `cd crous`
- Compiler avec `cargo build --release` (le binaire est dans `target/release/`
- Optionel: Installer avec `cargo install --path .`

