use std::{collections::HashMap, fs, path::Path};

use gray_matter::{engine::TOML, Matter, ParsedEntity};
use maud::{html, PreEscaped};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct ProjectFrontMatter {
    title: String,
    tags: Vec<String>,

    links: HashMap<String, String>,
}

struct Project {
    front: ProjectFrontMatter,

    image_link: String,
    about_html: PreEscaped<String>,
}

fn parse_project<P: AsRef<Path>>(file_path: P) -> Project {
    let matter = Matter::<TOML>::new();
    let input = fs::read_to_string(&file_path)
        .unwrap();
    let result= matter.parse(&input);
    let result: ParsedEntity<ProjectFrontMatter> = match result {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Error: {}", err);
            panic!("Invalid project file");
        }
    };

    // Parse the markdown from the content of the file.
    let parser = pulldown_cmark::Parser::new(&result.content);

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    let md_name = file_path.as_ref().file_stem().unwrap().to_string_lossy().to_string();
    let image_link = format!("./portfolio/{}.png", md_name.split_once('-').unwrap().1);

    Project {
        front: result.data.unwrap(),
        image_link,
        about_html: PreEscaped(html)
    }
}

fn projects() -> Vec<Project> {
    let mut projects = vec![];
    for entry in fs::read_dir("./portfolio").unwrap() {
        let Ok(entry) = entry else { continue; };
        if let Some(ext) = entry.path().extension() && ext == "md" {
            projects.push(parse_project(entry.path()));
        }
    }
    projects
}

fn build_thumbnail(project: &Project) -> PreEscaped<String> {
    html! {
        .thumbnail style=(format!("background-image: url('{}')", project.image_link)) {
            .thumbnail-content {
                .thumbnail-row {
                    span .title { (project.front.title) }
                    @for tag in &project.front.tags {
                        " " span .tag { (tag) }
                    }
                }
                .thumbnail-about {
                    (project.about_html)
                }
                .thumbnail-row {
                    a .call .about href="#" { "about" }
                    @for (name, url) in &project.front.links {
                        " " a.call href=(url) { (name) }
                    }
                }
            }
            .thumbnail-about-bg {}
        }
    }
}

fn build_portfolio(projects: &Vec<Project>) -> PreEscaped<String> {
    html! {
        html {
            head {
                meta charset="utf-8";
                title { "Benjamin Wall -- Portfolio" }
                link rel="stylesheet" type="text/css" href="portfolio.css";
            }
            body {
                .header-container {
                    .header {
                        span .title { "benjamin wall" }
                        span {
                            // TODO: Figure out a nice way to insert these two spans
                            // here. Maybe markdown?
                            "Hello! Welcome to my portfolio. I'm a programmer with
                            interest in embedded systems, compilers,
                            game development, and more."
                        }
                        span {
                            "You can see a representative sample of my projects
                            below. You can sort the projects based on topic using
                            the tags along the top of each icon, or learn more about
                            each one by following the links along the bottom of each
                            one."
                        }
                    }
                }
                .thumbnail-container {
                    @for project in projects {
                        (build_thumbnail(project))
                    }
                }
            }
            script src="./portfolio.js" {}
        }
    }
}

fn main() {
    let projects = projects();

    fs::create_dir_all("./dist").unwrap();
    fs::create_dir_all("./dist/portfolio").unwrap();

    let portfolio = build_portfolio(&projects);
    //let mut file = File::create("./dist/index.html").unwrap();
    //file.write_all(&portfolio.0.as_bytes()).unwrap();

    fs::write("./dist/index.html", &portfolio.0).unwrap();

    fs::copy("./portfolio.css", "./dist/portfolio.css").unwrap();
    fs::copy("./portfolio.js", "./dist/portfolio.js").unwrap();
    for project in &projects {
        let path = &project.image_link;
        fs::copy(format!("{path}"), format!("./dist/{path}")).unwrap();
    }
}
