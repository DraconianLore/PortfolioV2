use super::{logos, DescriptionBlock, DescriptionType, Project};

const DESCRIPTION: &[DescriptionBlock] = &[
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "A single-page app version of my portfolio built with Dioxus + Rust to and showcase my other projects.",
    },
];

pub const PROJECT: Project = Project {
    name: "Portfolio",
    tagline: "Dioxus-powered personal site",
    description: DESCRIPTION,
    tech_stack: "Dioxus, Rust",
    image: "portfolio.png",
    github: Some("https://github.com/DraconianLore/Portfolio"),
    live: Some("https://projects.stevenwing.dev"),
    project_logo: None,
    logos: &[
        &logos::RUST,
        &logos::DIOXUS,
    ],
};
