use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Throw away executable name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("missing query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("missing filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args_test() {
        let args = vec!["executable name".to_string()];

        assert!(Config::parse_args(args.into_iter()).is_err())
    }

    #[test]
    fn one_arg_test() {
        let args = vec!["executable name".to_string(), "query".to_string()];

        assert!(Config::parse_args(args.into_iter()).is_err())
    }

    #[test]
    fn two_args_test() {
        let args = vec![
            "executable name".to_string(),
            "query".to_string(),
            "filename".to_string(),
        ];

        let config = Config::parse_args(args.into_iter()).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }
}
