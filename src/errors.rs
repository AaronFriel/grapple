error_chain! {
    types{
        Error, ErrorKind, ResultExt, Result;
    }
    links {}
    foreign_links {
    }
    errors {

        //
        ConfigurationError(errs: Vec<Error>) {
            description("configuration file not loaded")
            display("configuration file not loaded, errors:\n{:?}", errs)
        }

        ConfigFileError(f: String) {
            description("failed to load configuration file")
            display("failed to load configuration file '{}'", f)
        }

        //
        ConfigParseError {
            description("error parsing configuration file")
        }
    }
}


        // ConfigurationError(t: Vec<Error>) {
        //     description("no configuration file loaded"),
        //     display("errors loading possible configurations:\n{}", t)
        // }


        // SerdeYaml(::serde_yaml::Error);
        // SerdeJson(::serde_json::Error);