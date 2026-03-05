use super::{logos, DescriptionBlock, DescriptionType, Project};

const DESCRIPTION: &[DescriptionBlock] = &[
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "Users can import details from steam including their friends list and games list. They can then select friends to see what games they share(or most of them share) to find games to play together.",
    },
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "Users can filters by mode(PvP, Co-op, etc) and discount, with discounts flags pulled live from Steam.",
    },
];

pub const PROJECT: Project = Project {
    name: "GameMatch",
    tagline: "A tool for multiple steam users to find multiplayer games they share.",
    description: DESCRIPTION,
    tech_stack: "Ruby, Rails, React, PostgreSQL",
    image: "GameMatch.webm",
    github: Some("https://github.com/DraconianLore/GameLibrary"),
    live: Some("https://gamematch.stevenwing.dev"),
    project_logo: Some("gameMatchLogo.png"),
    logos: &[
        &logos::RUBY,
        &logos::RAILS,
        &logos::REACT,
        &logos::POSTGRES,
    ],
};
