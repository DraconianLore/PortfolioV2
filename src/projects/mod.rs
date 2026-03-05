#[derive(Clone, Copy)]
pub enum DescriptionType {
    Heading,
    Paragraph,
}

#[derive(Clone, Copy)]
pub struct DescriptionBlock {
    pub block_type: DescriptionType,
    pub content: &'static str,
}

#[derive(Clone, Copy)]
pub struct Project {
    pub name: &'static str,
    pub tagline: &'static str,
    pub description: &'static [DescriptionBlock],
    pub tech_stack: &'static str,
    pub image: &'static str,
    pub github: Option<&'static str>,
    pub live: Option<&'static str>,
    pub project_logo: Option<&'static str>,
    pub logos: &'static [&'static logos::Logo],
}

#[derive(Clone, Copy)]
pub struct MinorProject {
    pub name: &'static str,
    pub github: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Copy)]
pub struct Contribution {
    pub name: &'static str,
    pub link: &'static str,
    pub description: &'static str,
}

pub mod logos;
pub mod talesmyth;
pub mod portfolio;
pub mod dmpt;
pub mod gamematch;
pub mod runescribe;
pub mod quotechat;
pub mod donate_it;
pub mod koios;
pub mod minor_projects;

const PROJECTS: &[Project] = &[
    talesmyth::PROJECT,
    portfolio::PROJECT,
    dmpt::PROJECT,
    gamematch::PROJECT,
    runescribe::PROJECT,
    quotechat::PROJECT,
    donate_it::PROJECT,
    koios::PROJECT,
];

pub fn all() -> &'static [Project] {
    PROJECTS
}

pub fn others() -> &'static [MinorProject] {
    minor_projects::MINOR_PROJECTS
}

pub fn contributions() -> &'static [Contribution] {
    minor_projects::OPEN_SOURCE_CONTRIBUTIONS
}
