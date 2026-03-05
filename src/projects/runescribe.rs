use super::{logos, DescriptionBlock, DescriptionType, Project};

const DESCRIPTION: &[DescriptionBlock] = &[
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "A tool for players using the custom Rune Scribe class I created for Dungeons & Dragons 5th Edition.\nThis tool helps players navigate the complexities of the new magic system created for this class, and allows players to add notes and tags to each spell, as well as filter by tag or type.",
    },
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "The admin side includes websockets connections so the Admin user can send messages to online players, add new structures and notify players in realtime, increase players/party level and get level-up choices from each player to decide what to unlock for each player as they level up.",
    },
];

pub const PROJECT: Project = Project {
    name: "Runescribe Familiar",
    tagline: "Rune Scribe spellbook + GM console",
    description: DESCRIPTION,
    tech_stack: "Ruby, Rails, React, WebSockets, PostgreSQL",
    image: "runescribe.webm",
    github: Some("https://github.com/DraconianLore/rune_scribe"),
    live: None,
    project_logo: None,
    logos: &[
        &logos::RUBY,
        &logos::RAILS,
        &logos::REACT,
        &logos::WEBSOCKETS,
        &logos::POSTGRES,
    ],
};
