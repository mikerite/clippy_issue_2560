#![feature(plugin)]
#![cfg_attr(feature = "clippy", plugin(clippy(conf_file = "../../bad_file.toml")))]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
