use clap::Parser;
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;
use std::process::exit;
use std::{collections::HashMap, fs};
use termion::color;

/// Récupère les menus de restaurants crous et les affiche joliment dans le terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Restaurants à afficher le menu. Les noms sont ceux inscrit comme alias dans la config. Si aucun restaurants n'est renseigné, le menu affiché sera celui du champ "default" de la config
    restaurants: Vec<String>,

    /// Nombre de jours à afficher
    #[arg(short, long, default_value_t = 1)]
    days: u8,

    /// Affiche les restaurants enregistrés dans la config
    #[arg(short, long)]
    list: bool,
}

fn inner(elem: Option<ElementRef>) -> String {
    elem.map(|e| e.inner_html()).unwrap_or(String::from(""))
}

struct Menu {
    date: String,
    meals: Vec<Meal>,
}
struct Meal {
    style: String,
    food: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    aliases: HashMap<String, String>,
    default: Option<String>,
}

impl Config {
    fn get_or_create() -> Self {
        let opt = dirs::config_dir();
        if opt.is_none() {
            return Self {
                aliases: HashMap::new(),
                default: None,
            };
        }
        let mut path = opt.unwrap();
        path.push("crous");
        path.push("crous.toml");
        if !path.exists() {
            let mut dir = path.clone();
            dir.pop();
            let _ = fs::create_dir_all(dir);
            let _ = fs::write(
                &path,
                "default=\"ru\"\n[aliases]\n    ru = \"<url_du_ru>\"\n",
            );
            eprintln!("Configuration created in ~/.config/crous/crous.toml. Please set the url of your restaurant inside.");
            exit(1);
        }

        match fs::read_to_string(path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error loading config: {}", e);
                    Config {
                        aliases: HashMap::new(),
                        default: None,
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading file ~/.config/crous/crous.toml: {}", e);
                Config {
                    aliases: HashMap::new(),
                    default: None,
                }
            }
        }
    }
}

fn get_menus(url: String) -> Vec<Menu> {
    let result = reqwest::blocking::get(url.clone()).and_then(|resp| resp.text());
    if result.is_err() {
        eprintln!("Error fetching data from url {}", url);
        return Vec::new();
    }
    let response = result.unwrap();
    let document = Html::parse_document(&response);

    let menu_selector = Selector::parse("div.menu").unwrap();
    let date_selector = Selector::parse("time.menu_date_title").unwrap();
    let meal_selector = Selector::parse("div.meal").unwrap();
    let meal_foodies_li_selector = Selector::parse("ul.meal_foodies>li").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let mut menus = Vec::new();
    for menu in document.select(&menu_selector) {
        let date = inner(menu.select(&date_selector).next());
        let meal = menu.select(&meal_selector).next().unwrap();
        //let meal_title = inner(meal.select(&meal_title_selector).next());
        let mut meals = Vec::new();
        for foodies in meal.select(&meal_foodies_li_selector) {
            let style = foodies.text().next().unwrap_or("");
            let food = foodies
                .select(&li_selector)
                .map(|e| e.inner_html())
                .map(|str| str.replace(".", "").trim().to_string())
                .collect::<Vec<String>>()
                .join(", ");
            meals.push(Meal {
                style: style.to_string(),
                food,
            });
        }
        menus.push(Menu { date, meals });
    }
    menus
}

fn display_menus(menus: Vec<Menu>, days: u8) {
    let mut d = 1;
    for menu in menus {
        println!(
            "{}{}{}",
            color::Bg(color::Blue),
            menu.date,
            color::Bg(color::Reset)
        );
        for meal in menu.meals {
            print!(
                "  {}{}{}: ",
                color::Fg(color::Green),
                meal.style,
                color::Fg(color::Reset)
            );
            println!("{}", meal.food);
        }
        d += 1;
        if d > days {
            break;
        }
    }
}

fn main() {
    let args = Args::parse();
    let config = Config::get_or_create();
    if args.list {
        let mut keys: Vec<&String> = config.aliases.keys().collect();
        keys.sort();
        for key in keys {
            let s = if config.default.as_ref() == Some(key) {
                " (default)"
            } else {
                ""
            };
            println!("{}{}", key, s);
        }
        return;
    }
    if args.restaurants.is_empty() {
        if let Some(default) = config.default {
            if let Some(url) = config.aliases.get(&default) {
                let menus = get_menus(url.to_string());
                display_menus(menus, args.days);
            }
        }
    } else {
        for ru in args.restaurants {
            if let Some(url) = config.aliases.get(&ru) {
                let menus = get_menus(url.to_string());
                display_menus(menus, args.days);
            }
        }
    }
}
