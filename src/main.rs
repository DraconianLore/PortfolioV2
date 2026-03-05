mod projects;

use dioxus::{events::TouchEvent, prelude::*};

fn main() {
    LaunchBuilder::new().launch(App);
}

fn strip_protocol(url: &'static str) -> &'static str {
    url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url)
}

#[component]
fn App() -> Element {
    let global_css = include_str!("../styles.css");
    let mobile_css = include_str!("../styles-mobile.css");

    rsx! {
        div { class: "app-root",
            link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Exo:wght@400;600;700&display=swap" }
            style { "{global_css}" }
            style { "{mobile_css}" }
            Header {}
            main { class: "page-content",
                Carousel {}
            }
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header { class: "site-header",
            h1 { "Steven Wing's Portfolio" }
            a { class: "resume-link", href: "https://www.stevenwing.dev", target: "_blank",
                "View Resume"
            }
        }
    }
}

#[component]
fn Carousel() -> Element {
    let mut page = use_signal(|| 0usize);
    let mut touch_start_x = use_signal(|| None::<f64>);
    let mut touch_end_x = use_signal(|| None::<f64>);
    let projects = projects::all();
    let minor_projects = projects::others();
    let contributions = projects::contributions();
    let total_slides = projects.len().saturating_add(1);
    let max_index = total_slides.saturating_sub(1);

    let current_index = (*page.read()).min(max_index);
    let current = projects.get(current_index).copied();

    let header_title = current.map(|p| p.name).unwrap_or("Other Projects");
    let header_tagline = current
        .map(|p| p.tagline)
        .unwrap_or("More hackathon builds, experiments, and open-source collaborations.");

    let render_project = |proj: &projects::Project| {
        let media_src = format!("/static/images/projects/{}", proj.image);
        let media_alt = format!("{} preview", proj.name);
        let is_video = proj.image.ends_with(".webm") || proj.image.ends_with(".mp4");
        let project_logo_src = proj
            .project_logo
            .map(|logo| format!("/static/images/projects/{}", logo))
            .unwrap_or_else(|| "/static/images/live-demo.png".to_string());

        rsx! {
            div { key: "{proj.name}", class: "project-frame",
                div { class: "project-body",
                    div { class: "project-media",
                        if is_video {
                            video {
                                class: "project-media-content",
                                autoplay: true,
                                loop: true,
                                muted: true,
                                playsinline: true,
                                src: "{media_src}"
                            }
                        } else {
                            a { href: "{media_src}", target: "_blank", title: "View full size",
                                img { class: "project-media-content", src: "{media_src}", alt: "{media_alt}" }
                            }
                        }
                    }
                    if !proj.logos.is_empty() {
                        div { class: "project-logos",
                            
                            div { class: "project-logos-grid",
                                {
                                    proj.logos.iter().map(|logo| rsx! {
                                        a { href: "{logo.link}", target: "_blank", title: "{logo.title}",
                                            img { src: "/static/images/{logo.image}", alt: "{logo.title}", class: "project-logo" }
                                        }
                                    })
                                }
                            }
                        }
                    }
                    div { class: "project-description",
                        {
                            proj.description.iter().map(|block| rsx! {
                                match block.block_type {
                                    projects::DescriptionType::Heading => rsx! {
                                        h3 { class: "project-section-heading", "{block.content}" }
                                    },
                                    projects::DescriptionType::Paragraph => rsx! {
                                        pre { class: "project-section-text", "{block.content}" }
                                    },
                                }
                            })
                        }
                    }
                    if proj.github.is_some() || proj.live.is_some() {
                        div { class: "project-meta",
                            span { class: "meta-label", "Project Links" }
                            div { class: "project-link-grid",
                                if let Some(url) = proj.live {
                                    a { href: "{url}", target: "_blank",
                                        div { class: "project-link-card",
                                            img { class: "project-link-icon", src: "{project_logo_src}", alt: "Live demo" }
                                            span { class: "project-link-url", "{strip_protocol(url)}" }
                                        }
                                    }
                                }
                                if let Some(repo) = proj.github {
                                    a { href: "{repo}", target: "_blank",
                                        div { class: "project-link-card",
                                            img { class: "project-link-icon", src: "/static/images/github-mark.svg", alt: "GitHub" }
                                            span { class: "project-link-url", "{strip_protocol(repo)}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                }
            }
        }
    };

    let render_other_projects = || {
        rsx! {
            div { class: "project-frame",
                div { class: "project-body other-projects",
                    div { class: "other-projects-intro",
                        p { class: "project-section-text", "A sampling of smaller utilities, hackathon prototypes, and experiments." }
                        p { class: "project-section-text", "Each repo digs deeper into realtime collaboration, API design, or UI flourishes that didn't make the main carousel." }
                    }
                    div { class: "other-projects-grid",
                        {
                            minor_projects.iter().map(|proj| rsx! {
                                div { class: "other-project-card",
                                    a { class: "other-project-title", href: "{proj.github}", target: "_blank", "{proj.name}" }
                                    p { class: "other-project-description", "{proj.description}" }
                                }
                            })
                        }
                    }
                    div { class: "other-projects-contrib",
                        h3 { "Open Source Contributions" }
                        div { class: "contrib-grid",
                            {
                                contributions.iter().map(|item| rsx! {
                                    div { class: "contrib-card",
                                        a { class: "contrib-title", href: "{item.link}", target: "_blank", "{item.name}" }
                                        p { class: "contrib-description", "{item.description}" }
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }
    };

    let touch_start_handler = {
        let mut touch_start_x = touch_start_x.clone();
        move |evt: TouchEvent| {
            if let Some(touch) = evt.data().touches().first() {
                let coords = touch.client_coordinates();
                touch_start_x.set(Some(coords.x as f64));
            }
        }
    };

    let touch_move_handler = {
        let mut touch_end_x = touch_end_x.clone();
        move |evt: TouchEvent| {
            if let Some(touch) = evt.data().touches().first() {
                let coords = touch.client_coordinates();
                touch_end_x.set(Some(coords.x as f64));
            }
        }
    };

    let touch_end_handler = {
        let mut touch_start_x = touch_start_x.clone();
        let mut touch_end_x = touch_end_x.clone();
        let mut page_prev = page.clone();
        let mut page_next = page.clone();
        move |_| {
            let start = touch_start_x.with(|value| *value);
            let end = touch_end_x.with(|value| *value);
            touch_start_x.set(None);
            touch_end_x.set(None);
            if let (Some(start), Some(end)) = (start, end) {
                let delta = end - start;
                if delta.abs() > 35.0 {
                    if delta < 0.0 {
                        page_next.with_mut(|idx| {
                            if *idx == max_index {
                                *idx = 0;
                            } else {
                                *idx += 1;
                            }
                        });
                    } else {
                        page_prev.with_mut(|idx| {
                            if *idx == 0 {
                                *idx = max_index;
                            } else {
                                *idx -= 1;
                            }
                        });
                    }
                }
            }
        }
    };

    let slide_offset = (*page.read() as f64) * 100.0;
    let track_style = format!("transform: translateX(-{}%)", slide_offset);

    rsx! {
        section {
            class: "carousel-shell",
            ontouchstart: touch_start_handler,
            ontouchmove: touch_move_handler,
            ontouchend: touch_end_handler,
            div { class: "carousel-header",
                button {
                    class: "carousel-btn",
                    title: "Previous project",
                    onclick: {
                        let mut page = page.clone();
                        move |_| {
                            page.with_mut(|idx| {
                                if *idx == 0 {
                                    *idx = max_index;
                                } else {
                                    *idx -= 1;
                                }
                            });
                        }
                    },
                    "←"
                }
                div { class: "carousel-meta",
                    span { class: "carousel-index",
                        "{*page.read() + 1:02}/{total_slides:02}"
                    }
                    h2 { class: "carousel-title", "{header_title}" }
                    p { class: "carousel-tagline", "{header_tagline}" }
                }
                button {
                    class: "carousel-btn",
                    title: "Next project",
                    onclick: {
                        let mut page = page.clone();
                        move |_| {
                            page.with_mut(|idx| {
                                if *idx == max_index {
                                    *idx = 0;
                                } else {
                                    *idx += 1;
                                }
                            });
                        }
                    },
                    "→"
                }
            }
            div { class: "carousel-track",
                style: "{track_style}",
                {
                    projects.iter().map(|proj| render_project(proj))
                }
                { render_other_projects() }
            }
            button {
                class: "carousel-nav carousel-nav-left",
                title: "Previous project",
                onclick: {
                    let mut page = page.clone();
                    move |_| {
                        page.with_mut(|idx| {
                            if *idx == 0 {
                                *idx = max_index;
                            } else {
                                *idx -= 1;
                            }
                        });
                    }
                },
                "◄"
            }
            button {
                class: "carousel-nav carousel-nav-right",
                title: "Next project",
                onclick: {
                    let mut page = page.clone();
                    move |_| {
                        page.with_mut(|idx| {
                            if *idx == max_index {
                                *idx = 0;
                            } else {
                                *idx += 1;
                            }
                        });
                    }
                },
                "►"
            }
        }
    }
}
