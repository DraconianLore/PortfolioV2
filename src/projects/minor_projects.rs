use super::{Contribution, MinorProject};

pub const MINOR_PROJECTS: &[MinorProject] = &[
    MinorProject {
        name: "Jungle",
        github: "https://github.com/DraconianLore/jungle-rails",
        description: "An e-commerce application built with Ruby on Rails.",
    },
    MinorProject {
        name: "ChattyApp",
        github: "https://github.com/DraconianLore/chattyApp",
        description: "A compact anonymous chatroom built with Node.js, React, and WebSockets.",
    },
    MinorProject {
        name: "Conligo",
        github: "https://github.com/wonseobshin/conligo",
        description: "A smart todo list that categorizes items the user adds by querying external APIs.",
    },
    MinorProject {
        name: "Tweeter",
        github: "https://github.com/DraconianLore/tweeter",
        description: "A single-page AJAX-based Twitter clone built with jQuery, HTML5, and CSS3.",
    },
];

pub const OPEN_SOURCE_CONTRIBUTIONS: &[Contribution] = &[
    Contribution {
        name: "MapKnitter",
        link: "https://mapknitter.org",
        description: "Contributed UI fixes and performance improvements to the public lab aerial mapping tool.",
    },
    Contribution {
        name: "FreeCodeCamp",
        link: "https://github.com/freeCodeCamp/freeCodeCamp",
        description: "Minor accessibility fixes and copy updates across curriculum projects.",
    },
];
