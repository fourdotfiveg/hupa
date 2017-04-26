//! Errors are all defined in this module

error_chain! {
    foreign_links {
        AppDirs(::app_dirs::AppDirsError) #[doc = "Error from app_dirs crate"];
        Json(::json::Error) #[doc = "Error from json crate"];
        Io(::std::io::Error) #[doc = "IO error"];
        Var(::std::env::VarError) #[doc = "Var error"];
    }

    errors {
        /// Metadata is invalid
        InvalidMetadata {
            description("metadata is invalid")
            display("metadata is invalid")
        }
        /// Invalid metadata format
        InvalidMetadataFormat(f: String) {
            description("specified metadata format is invalid")
            display("specified metadata format is invalid: {}", f)
        }
        /// Error when origin file is missing
        MissingBackup(p: String)  {
            description("backup file is missing")
            display("backup file is misssing, restore can't be made: {}", p)
        }
        /// Error when backup file is missing
        MissingOrigin(p: String)  {
            description("origin file is missing")
            display("origin file is misssing, backup can't be made: {}", p)
        }
        /// Error when there is not metadata path in config
        MissingMetadataPath {
            description("there is no metadata path in config")
            display("there is no metadata path in config")
        }
        /// Error when there is not metadata format in config
        MissingMetadataFormat {
            description("there is not metadata format in config")
            display("there is not metadata format in config")
        }
        /// Error when variable is not valid
        InvalidValue(v: String) {
            description("value is not a boolean")
            display("value {} is not a boolean", v)
        }
        /// Error when there is no value for variable
        MissingValue(n: String) {
            description("value is missing for variable")
            display("variable {} doesn't have value", n)
        }
    }
}
