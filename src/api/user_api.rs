use sapper::{
    Request, 
    Response, 
    Result as SapperResult, 
    Error as SapperError, 
    Module as SapperModule,
    Router as SapperRouter};
use sapper_std::{JsonParams, SessionVal};
use serde_json;

pub struct UserApi;

impl UserApi {

    pub fn article_create(req: &mut Request) -> SapperResult<Response> {

    }

}


impl SapperModule for UserApi {
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        // check permission, logined
        // Post request needs logining first
        

    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.post("/article/create", Self::article_create);
        router.post("/article/edit", Self::article_edit);
        router.post("/article/delete", Self::article_delete);

        router.get("/article/get_by_id", Self::article_get_by_id);
        router.get("/article/paging", Self::article_paging);
        router.get("/article/paging_by_section", Self::article_paging_by_section);

        Ok(())
    }
}


