use super::{logos, DescriptionBlock, DescriptionType, Project};

const DESCRIPTION: &[DescriptionBlock] = &[
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "Slack slash-command app that lets teams fetch famous quotes inline without breaking flow.",
    },
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "Users can type /quote for an instant quote search with reshuffle controls so users always find the perfect response.",
    },
];

pub const PROJECT: Project = Project {
    name: "Quote Chat",
    tagline: "Slash-command quote assistant for Slack",
    description: DESCRIPTION,
    tech_stack: "Express, Node.js, Python, PostgreSQL",
    image: "quoteChat.gif",
    github: Some("https://github.com/alumni-lab/quote-chat"),
    live: None,
    project_logo: None,
    logos: &[
        &logos::EXPRESS,
        &logos::NODEJS,
        &logos::PYTHON,
        &logos::POSTGRES,
    ],
};
