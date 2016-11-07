use result::Error;

error_chain!{
    types {
        BuildQueryError, BuildQueryErrorKind, BuildQueryChainErr, BuildQueryResult;
    }

    links { }

    foreign_links { }

    errors {
        QueryError(err: Box<Error>){
            description("Query Error")
            display("Query Error: {:?}", err)
        }
        // Match against _ instead, more variants may be added in the future
        #[doc(hidden)] __Nonexhaustive
    }
}

impl From<Error> for BuildQueryError{
    fn from(e: Error) -> Self {
        BuildQueryErrorKind::QueryError(Box::new(e)).into()
    }
}
