#[link(name = "hammer",
       vers = "0.1-pre",
       url = "")];

#[crate_type = "lib"];
#[link_args = "-lhammer"];

pub mod cbits;
pub mod hammer;
