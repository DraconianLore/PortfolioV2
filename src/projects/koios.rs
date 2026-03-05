use super::{logos, DescriptionBlock, DescriptionType, Project};

const DESCRIPTION: &[DescriptionBlock] = &[
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "Κοῖος is an immersive game where users enlist as agents in a secret organization where they are encouraged to take a break from their daily lives and complete missions by interacting with the world around them.",
    },
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "Initially users are presented with basic training missions to build up trust within the organisation, taking photos of objects, taking selfies with other people or places, and sending them off to be verified by other agents(including new agents)",
    },
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "As the agents gain trust, they gain ranks in the organisation which leads to harder missions including:\nEncryption missions - where the agent is given a word or phrase and must encrypt it using a type of cypher provided.\nDecryption missions - where the agent is given an encrypted message and must discover the type of encryption used and decypher the message.",
    },
];

pub const PROJECT: Project = Project {
    name: "Koios",
    tagline: "Real-world missions + cipher puzzles",
    description: DESCRIPTION,
    tech_stack: "Ruby, Rails, React Native, PostgreSQL",
    image: "koios.png",
    github: Some("https://github.com/DraconianLore/Koios"),
    live: None,
    project_logo: None,
    logos: &[
        &logos::RUBY,
        &logos::RAILS,
        &logos::REACT_NATIVE,
        &logos::POSTGRES,
    ],
};
