use super::{logos, DescriptionBlock, DescriptionType, Project};

const DESCRIPTION: &[DescriptionBlock] = &[
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "A tool for Dungeon Masters to track their players and NPC's in Dungeons and Dragons.",
    },
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "The user can see an overview of all their players and NPCs, and click into any character to see the full details.\nThey can also level up characters, which auto-fills certain fields, manage player items and notes, and share a player version of characters with the players.",
    },
];

pub const PROJECT: Project = Project {
    name: "DM Player Tracker",
    tagline: "A Dungeons and Dragons player tracker.",
    description: DESCRIPTION,
    tech_stack: "Ruby, Rails, React, PostgreSQL",
    image: "dmplayertracker.gif",
    github: Some("https://github.com/DraconianLore/dmPlayerTracker"),
    live: Some("https://dmpt.stevenwing.dev"),
    project_logo: Some("dmptlogo.png"),
    logos: &[
        &logos::RUBY,
        &logos::RAILS,
        &logos::REACT,
        &logos::POSTGRES,
    ],
};
