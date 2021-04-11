use error_chain::error_chain;
#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
error_chain! {
    foreign_links {
       TerraError ( terra_rust_api::errors::Error);

    }
    errors {

    }
}
