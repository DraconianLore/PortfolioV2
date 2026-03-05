use super::{logos, DescriptionBlock, DescriptionType, Project};

const DESCRIPTION: &[DescriptionBlock] = &[
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "Created during the 2019 Vanhacks < For Social Good > hackathon",
    },
    DescriptionBlock {
        block_type: DescriptionType::Paragraph,
        content: "This application allows not for profit organizations to post their IT problems such as 'We need a website' for developers/designers to match with to donate their time and experience to helping out their community.",
    },
];

pub const PROJECT: Project = Project {
    name: "Donate IT",
    tagline: "Connecting nonprofits with volunteer devs",
    description: DESCRIPTION,
    tech_stack: "Ruby, Rails, Angular, PostgreSQL",
    image: "donateIT.gif",
    github: Some("https://github.com/ChesterCorin/vanhacks-2019-frontend"),
    live: None,
    project_logo: Some("dITLogo.png"),
    logos: &[
        &logos::RUBY,
        &logos::RAILS,
        &logos::ANGULAR,
        &logos::POSTGRES,
    ],
};
