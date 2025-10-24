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
    video_link: Option<String>,
    about_html: PreEscaped<String>,
}

fn markdown<P: AsRef<Path>>(md_path: P) -> PreEscaped<String> {
    let markdown = fs::read_to_string(md_path).unwrap();
    let parser = pulldown_cmark::Parser::new(&markdown);

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    PreEscaped(html)
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
    let project_id = md_name.split_once('-').unwrap().1;

    let image_link = format!("./portfolio/{}.png", project_id);
    let video_link = format!("./portfolio/{}.mp4", project_id);
    let video_link = match fs::exists(&video_link) {
        Ok(true) => Some(video_link),
        _ => None
    };

    Project {
        front: result.data.unwrap(),
        image_link,
        video_link,
        about_html: PreEscaped(html)
    }
}

fn projects() -> Vec<Project> {
    let mut projects = vec![];
    for entry in fs::read_dir("./portfolio").unwrap() {
        let Ok(entry) = entry else { continue; };
        if let Some(ext) = entry.path().extension() && ext == "md" {
            // Only process project files that begin with a numeral.
            let file_name = entry.path().file_name().unwrap().to_string_lossy().to_string();
            match file_name.bytes().nth(0) {
                Some(b'0'..b'9') => {},
                _ => continue,
            }
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
            @match &project.video_link {
                Some(link) => {
                    video .thumbnail-video
                        data-src=(link)
                        preload="none"
                        poster="./portfolio/black_square.png"
                        muted
                        loop
                        autoplay
                        playsinline
                    { }
                }
                None => {}
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
                        (markdown("./portfolio/header.md"))
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

        if let Some(path) = &project.video_link {
            fs::copy(path, format!("./dist/{path}")).unwrap();
        }
    }
}
