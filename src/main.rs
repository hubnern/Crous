use scraper::{Html, Selector, ElementRef};
use termion::color;
use clap::Parser;

/// Récupère le menu d'un restaurant crous et l'affiche joliement dans le terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Url pour récupérer le menu
    #[arg(short, long, default_value_t = String::from("https://www.crous-bordeaux.fr/restaurant/restaurant-administratif-le-haut-carre-3/"))]
    url: String,

    /// Nombre de jours à afficher
    #[arg(short, long, default_value_t = 1)]
    days: u8
}
fn inner(elem: Option<ElementRef>) -> String {
    elem.map(|e| e.inner_html()).unwrap_or(String::from(""))
}

fn main() {
    let args = Args::parse();

    let result = reqwest::blocking::get(args.url)
        .and_then(|resp| resp.text());
    if result.is_err() {
        eprintln!("Error fetching data from url");
        return;
    }
    let response = result.unwrap();
    let document = Html::parse_document(&response);
    let menu_selector = Selector::parse("div.menu").unwrap();
    let date_selector = Selector::parse("time.menu_date_title").unwrap();
    let meal_selector = Selector::parse("div.meal").unwrap();
    //let meal_title_selector = Selector::parse("div.meal_title").unwrap();
    let meal_foodies_li_selector = Selector::parse("ul.meal_foodies>li").unwrap();
    let li_selector = Selector::parse("li").unwrap();
    let mut days = 1;
    for menu in document.select(&menu_selector) {
        let date = inner(menu.select(&date_selector).next());
        println!("{}{}{}", color::Bg(color::Blue), date, color::Bg(color::Reset));
        for meal in menu.select(&meal_selector) {
            //let meal_title = inner(meal.select(&meal_title_selector).next());
            //println!("  {}", meal_title);
            for foodies in meal.select(&meal_foodies_li_selector) {
                print!("  {}{}{}: ", color::Fg(color::Green), foodies.text().next().unwrap_or(""), color::Fg(color::Reset));
                let foods = foodies.select(&li_selector)
                    .map(|e| e.inner_html())
                    .map(|str| str.replace(".", "").trim().to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                println!("{}", foods);
                //for food in foodies.select(&li_selector) {
                //    print!("{}, ", food.inner_html().replace(".", "").trim());
                //}
                //println!();
            }
        }
        days += 1;
        if days > args.days {
            break;
        }
    } 
}