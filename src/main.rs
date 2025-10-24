use std::{fs::{self, File}, io::Write};

use maud::{html, PreEscaped};

struct Project {
    /// Used to identify relevant files on the filesystem: The markdown for
    /// the projection description, plus the project thumbnail.
    id_str: &'static str,

    title: &'static str,
    tags: Vec<&'static str>,

    play_link: Option<&'static str>,
    source_link: Option<&'static str>,
}

fn projects() -> Vec<Project> {
    vec![
        Project {
            id_str: "bens-beams",
            title: "ben's beams",
            tags: vec!["rust", "wasm", "gamedev"],
            play_link: Some("https://some-games-by-bee.itch.io/bens-beams"),
            source_link: Some("https://github.com/yourname3/jamfest-2025")
        },
        Project {
            id_str: "arachno-drome",
            title: "Arachno Drome",
            tags: vec!["godot", "gamedev", "collaborative"],
            play_link: Some("https://justin1l8.itch.io/arachno-drome"),
            source_link: None,   
        }
    ]
}

fn build_about(project: &Project) -> PreEscaped<String> {
    let markdown = fs::read_to_string(format!("./portfolio/{}.md", project.id_str))
        .unwrap();

    let parser = pulldown_cmark::Parser::new(&markdown);

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    PreEscaped(html)
}

fn build_thumbnail(project: &Project) -> PreEscaped<String> {
    html! {
        .thumbnail style=(format!("background-image: url('./thumbnails/{}.png')", project.id_str)) {
            .thumbnail-content {
                .thumbnail-row {
                    span .title { (project.title) }
                    @for tag in &project.tags {
                        span .tag { (tag) }
                    }
                }
                .thumbnail-about {
                    (build_about(project))
                }
                .thumbnail-row {
                    a .call .about href="#" { "about" }
                    @match project.play_link {
                        Some(link) => { a .call href=(link) { "play" } }
                        None => {}
                    }
                    @match project.source_link {
                        Some(link) => { a .call href=(link) { "source code" } }
                        None => {}
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
        }
    }
}

fn main() {
    let projects = projects();

    fs::create_dir_all("./dist").unwrap();
    fs::create_dir_all("./dist/thumbnails").unwrap();

    let portfolio = build_portfolio(&projects);
    //let mut file = File::create("./dist/index.html").unwrap();
    //file.write_all(&portfolio.0.as_bytes()).unwrap();

    fs::write("./dist/index.html", &portfolio.0).unwrap();

    fs::copy("./portfolio.css", "./dist/portfolio.css").unwrap();
    for project in &projects {
        let id = project.id_str;
        fs::copy(format!("./thumbnails/{id}.png"), format!("./dist/thumbnails/{id}.png")).unwrap();
    }
}
