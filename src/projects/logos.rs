#[derive(Clone, Copy)]
pub struct Logo {
    pub title: &'static str,
    pub image: &'static str,
    pub link: &'static str,
}

pub const REACT: Logo = Logo {
    title: "React",
    image: "react.png",
    link: "https://react.dev",
};

pub const REACT_NATIVE: Logo = Logo {
    title: "React Native",
    image: "react-native.png",
    link: "https://reactnative.dev",
};

pub const RUST: Logo = Logo {
    title: "Rust",
    image: "rust-logo-64x64.png",
    link: "https://www.rust-lang.org",
};

pub const DIOXUS: Logo = Logo {
    title: "Dioxus",
    image: "dioxus.png",
    link: "https://dioxuslabs.com",
};

pub const TRUNK: Logo = Logo {
    title: "Trunk",
    image: "trunk.png",
    link: "https://trunkrs.dev",
};

pub const RUBY: Logo = Logo {
    title: "Ruby",
    image: "ruby.png",
    link: "https://www.ruby-lang.org",
};

pub const RAILS: Logo = Logo {
    title: "Rails",
    image: "rails.png",
    link: "https://rubyonrails.org",
};

pub const POSTGRES: Logo = Logo {
    title: "PostgreSQL",
    image: "psql.svg",
    link: "https://www.postgresql.org",
};

pub const ASTRO: Logo = Logo {
    title: "Astro",
    image: "astro.png",
    link: "https://astro.build",
};

pub const WEBSOCKETS: Logo = Logo {
    title: "WebSockets",
    image: "websockets.svg",
    link: "https://websockets.spec.whatwg.org",
};

pub const EXPRESS: Logo = Logo {
    title: "Express",
    image: "express.png",
    link: "https://expressjs.com",
};

pub const NODEJS: Logo = Logo {
    title: "Node.js",
    image: "nodejs-dark.png",
    link: "https://nodejs.org",
};

pub const PYTHON: Logo = Logo {
    title: "Python",
    image: "python-logo.png",
    link: "https://www.python.org",
};

pub const ANGULAR: Logo = Logo {
    title: "Angular",
    image: "angular.png",
    link: "https://angular.io",
};
