//! Errors are all defined in this module

error_chain! {
    foreign_links {
        AppDirs(::app_dirs::AppDirsError) #[doc = "Error for app_dirs crate"];
        Json(::json::Error) #[cfg(feature = "text-json")] #[doc = "Error for json crate"];
        Io(::std::io::Error) #[doc = "IO error"];
    }

    errors {
        /// Metadata is invalid
        InvalidMetadata {
            description("Metadata is invalid")
            display("Metadata is invalid")
        }
        /// Error when origin file is missing
        MissingBackup(p: String)  {
            description("Backup file is missing")
            display("Backup file is misssing, restore can't be made: {}", p)
        }
        /// Error when backup file is missing
        MissingOrigin(p: String)  {
            description("Origin file is missing")
            display("Origin file is misssing, backup can't be made: {}", p)
        }
    }
}
