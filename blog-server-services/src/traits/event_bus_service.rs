use blog_generic::events::{NewPostPublished, SubscriptionStateChanged};
use serde::Serialize;
#[async_trait]
pub trait Publish<T>: Send + Sync
where
    T: Serialize,
{
    async fn publish(&self, event: T) -> ();
}

pub trait EventBusService: Publish<SubscriptionStateChanged> + Publish<NewPostPublished> {}
