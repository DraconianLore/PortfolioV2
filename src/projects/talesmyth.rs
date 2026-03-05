use super::{logos, DescriptionBlock, DescriptionType, Project};

const DESCRIPTION: &[DescriptionBlock] = &[
    DescriptionBlock {
        block_type: DescriptionType::Heading,
        content: "Worldbuilding as it should be",
    },
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "Talesmyth is a worldbuilding and story-planning tool for writers, game masters, and game designers. It provides a unified workspace to create interconnected worlds with characters, locations, items, factions, lore, and other story elements, replacing scattered notes with automatic linking, consistent context, genre templates, and optional AI for brainstorming while keeping you in creative control.",
    },
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "I built Talesmyth from zero to one, defining its vision, UX, and data model, then coding the full-stack app powering talesmyth.com, including entity-linking, AI summarization and generation, and inline authoring. I continue iterating with active users, incorporating feedback to refine workflows, add features like templates and visualizations, and grow its creative community.",
    },
];

pub const PROJECT: Project = Project {
    name: "Talesmyth",
    tagline: "Worldbuilding tool for authors, GMs, and more",
    description: DESCRIPTION,
    tech_stack: "React, Rails, PostgreSQL, Astro",
    image: "talesmyth.webm",
    github: None,
    live: Some("https://talesmyth.com"),
    project_logo: Some("talesmythLogo.webp"),
    logos: &[
        &logos::REACT,
        &logos::RAILS,
        &logos::POSTGRES,
        &logos::ASTRO,
    ],
};
