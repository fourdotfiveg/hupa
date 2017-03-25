//! Errors are all defined in this module

error_chain! {
    foreign_links {
        AppDirs(::app_dirs::AppDirsError) #[doc = "Error for app_dirs crate"];
        Io(::std::io::Error) #[doc = "IO error"];
    }
}
