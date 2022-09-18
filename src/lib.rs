#![allow(unused)]
#![feature(never_type)]
mod utils {
    pub mod constants;
    pub mod types;

    pub use constants::*;
    pub use types::*;
}

mod user {
    pub mod login;

    pub use login::*;
}

mod forum {
    pub mod forum_list;
    pub mod topic_list;

    pub use forum_list::ForumList;
}

mod structs {
    pub mod board;
    pub mod command;
    pub mod topic;

    pub use board::Board;
    pub use command::UrlWithQuery;
    pub use topic::Topic;
}
