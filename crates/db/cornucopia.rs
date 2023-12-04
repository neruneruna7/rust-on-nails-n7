// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod users
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq, )] pub struct User
{ pub id : i32,pub email : String,}pub struct UserBorrowed < 'a >
{ pub id : i32,pub email : &'a str,} impl < 'a > From < UserBorrowed <
'a >> for User
{
    fn
    from(UserBorrowed { id,email,} : UserBorrowed < 'a >)
    -> Self { Self { id,email: email.into(),} }
}pub struct UserQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> UserBorrowed,
    mapper : fn(UserBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > UserQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(UserBorrowed) -> R) -> UserQuery
    < 'a, C, R, N >
    {
        UserQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn get_users() -> GetUsersStmt
{ GetUsersStmt(cornucopia_async :: private :: Stmt :: new("SELECT 
    id,
    email
FROM users")) } pub
struct GetUsersStmt(cornucopia_async :: private :: Stmt) ; impl
GetUsersStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> UserQuery < 'a, C,
User, 0 >
{
    UserQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { UserBorrowed { id : row.get(0),email : row.get(1),} }, mapper : | it | { <User>::from(it) },
    }
} }}}