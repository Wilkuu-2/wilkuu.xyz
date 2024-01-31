#[allow(dead_code)]
pub mod etc;
pub mod home;
pub mod posts;


mod filters {
    #[derive(Debug)]
    pub struct RouteError(String);
    impl ::std::fmt::Display for RouteError{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Invalid route: {}", self.0)
        }
    }
    impl ::std::error::Error for RouteError{}

    pub fn route<P>(route: &::axum_named_routes::Routes, p: P) -> ::askama::Result<String>
        where
            P: AsRef<str>,
    {
        match route.get(p.as_ref()) {
            Some(r) => Ok(r.to_str().unwrap_or("Invalid url").to_string().replace("/:id","")),  
            None    => Err(::askama::Error::Custom(Box::new(RouteError(p.as_ref().to_string())))),
        } 

    }
} 
