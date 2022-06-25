#![allow(unused)]
#![feature(never_type)]
mod utils {
    pub mod constants;
    pub mod helper_types;
    pub mod type_alias;

    pub use constants::*;
    pub use helper_types::*;
    pub use type_alias::*;
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

mod board;
