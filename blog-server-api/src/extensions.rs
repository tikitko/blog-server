use blog_server_services::impls::{
    create_entity_comment_service, create_entity_post_service, create_rbatis_author_service,
    create_rbatis_comment_service, create_rbatis_post_service, create_social_service,
};
use blog_server_services::traits::author_service::AuthorService;
use blog_server_services::traits::comment_service::CommentService;
use blog_server_services::traits::entity_comment_service::EntityCommentService;
use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::event_bus_service::EventBusService;
use blog_server_services::traits::post_service::PostService;
use blog_server_services::traits::social_service::SocialService;
use rbatis::rbatis::RBatis;
use std::sync::Arc;

pub trait Resolve<T>: Send + Sync {
    fn resolve(&self) -> T;
}

pub trait ExtensionsProviderType:
    Resolve<Arc<Box<dyn AuthorService>>>
    + Resolve<Arc<Box<dyn PostService>>>
    + Resolve<Arc<Box<dyn CommentService>>>
    + Resolve<Arc<Box<dyn EntityCommentService>>>
    + Resolve<Arc<Box<dyn EntityPostService>>>
    + Resolve<Arc<Box<dyn EventBusService>>>
    + Resolve<Arc<Box<dyn SocialService>>>
{
}

struct ExtensionsProvider {
    author_service: Arc<Box<dyn AuthorService>>,
    post_service: Arc<Box<dyn PostService>>,
    comment_service: Arc<Box<dyn CommentService>>,
    entity_comment_service: Arc<Box<dyn EntityCommentService>>,
    entity_post_service: Arc<Box<dyn EntityPostService>>,
    event_bus_service: Arc<Box<dyn EventBusService>>,
    social_service: Arc<Box<dyn SocialService>>,
}

impl ExtensionsProviderType for ExtensionsProvider {}

impl Resolve<Arc<Box<dyn AuthorService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn AuthorService>> {
        self.author_service.clone()
    }
}

impl Resolve<Arc<Box<dyn PostService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn PostService>> {
        self.post_service.clone()
    }
}

impl Resolve<Arc<Box<dyn CommentService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn CommentService>> {
        self.comment_service.clone()
    }
}

impl Resolve<Arc<Box<dyn EntityCommentService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn EntityCommentService>> {
        self.entity_comment_service.clone()
    }
}

impl Resolve<Arc<Box<dyn EntityPostService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn EntityPostService>> {
        self.entity_post_service.clone()
    }
}

impl Resolve<Arc<Box<dyn EventBusService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn EventBusService>> {
        self.event_bus_service.clone()
    }
}

impl Resolve<Arc<Box<dyn SocialService>>> for ExtensionsProvider {
    fn resolve(&self) -> Arc<Box<dyn SocialService>> {
        self.social_service.clone()
    }
}

pub fn make_extensions(
    rbatis: RBatis,
    event_bus: Box<dyn EventBusService>,
) -> impl ExtensionsProviderType {
    let authors_service = Arc::new(create_rbatis_author_service(rbatis.clone()));
    let event_bus = Arc::new(event_bus);
    ExtensionsProvider {
        author_service: authors_service.clone(),
        post_service: Arc::new(create_rbatis_post_service(rbatis.clone())),
        comment_service: Arc::new(create_rbatis_comment_service(rbatis.clone())),
        entity_comment_service: Arc::new(create_entity_comment_service(authors_service.clone())),
        entity_post_service: Arc::new(create_entity_post_service(authors_service.clone())),
        event_bus_service: event_bus.clone(),
        social_service: Arc::new(create_social_service(
            authors_service.clone(),
            event_bus.clone(),
        )),
    }
}
