use crate::traits::post_service::{BasePost, Post, PostService};
use rbatis::rbatis::RBatis;
use screw_components::dyn_result::{DError, DResult};

pub fn create_rbatis_post_service(rb: RBatis) -> Box<dyn PostService> {
    Box::new(RbatisPostService { rb })
}

impl_insert!(BasePost {}, "post");

impl Post {
    #[sql(
        "
        SELECT COUNT(*) 
        FROM post
    "
    )]
    async fn count(rb: &RBatis) -> rbatis::Result<i64> {
        impled!()
    }
    #[sql(
        "
        SELECT post.*, author.slug AS author_slug 
        FROM post 
        LEFT JOIN author ON post.author_id = author.id
        WHERE post.id = #{id} 
        LIMIT 1
    "
    )]
    async fn select_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<Option<Post>> {
        impled!()
    }
    #[sql(
        "
        SELECT post.*, author.slug AS author_slug 
        FROM post 
        LEFT JOIN author ON post.author_id = author.id 
        WHERE post.slug = #{slug} 
        LIMIT 1
    "
    )]
    async fn select_by_slug(rb: &RBatis, slug: &String) -> rbatis::Result<Option<Post>> {
        impled!()
    }
    #[sql(
        "
        SELECT post.*, author.slug AS author_slug 
        FROM post 
        LEFT JOIN author ON post.author_id = author.id 
        LIMIT #{limit} 
        OFFSET #{offset}
    "
    )]
    async fn select_all_with_limit_and_offset(
        rb: &RBatis,
        limit: &i64,
        offset: &i64,
    ) -> rbatis::Result<Vec<Post>> {
        impled!()
    }
}

struct RbatisPostService {
    rb: RBatis,
}

#[async_trait]
impl PostService for RbatisPostService {
    async fn posts_count(&self) -> DResult<i64> {
        Ok(Post::count(&self.rb).await?)
    }
    async fn posts(&self, offset: &i64, limit: &i64) -> DResult<Vec<Post>> {
        Ok(Post::select_all_with_limit_and_offset(&self.rb, limit, offset).await?)
    }
    async fn post_by_id(&self, id: &i64) -> DResult<Option<Post>> {
        Ok(Post::select_by_id(&self.rb, id).await?)
    }
    async fn post_by_slug(&self, slug: &String) -> DResult<Option<Post>> {
        Ok(Post::select_by_slug(&self.rb, slug).await?)
    }
    async fn create_post(&self, post: &BasePost) -> DResult<i64> {
        let insert_result = BasePost::insert(&mut self.rb.clone(), post).await?;
        let last_insert_id = insert_result
            .last_insert_id
            .as_i64()
            .ok_or::<DError>("wrond last_insert_id".into())?;
        Ok(last_insert_id)
    }
}
