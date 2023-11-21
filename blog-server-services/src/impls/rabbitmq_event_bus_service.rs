use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicPublishArguments, Channel, QueueBindArguments},
    connection::{Connection, OpenConnectionArguments},
    error::Error,
    BasicProperties,
};
use blog_generic::events::SubscriptionStateChanged;
use serde::Serialize;

use crate::traits::event_bus_service::{EventBusService, Publish};

enum EventBusError {
    SerializationError,
    PublishingError,
}

pub async fn create_rabbit_event_bus_service(
    connection_string: Option<&str>,
) -> Box<dyn EventBusService> {
    if let Some(connection) = connection_string {
        let connection_configuration: Result<OpenConnectionArguments, _> = connection.try_into();
        let connection_configuration = match connection_configuration {
            Ok(mut config) => {
                config.connection_name("blog_producer");
                config
            }
            Err(_) => {
                println!("Error while connecting to rabbitMQ. Mock service will be created");
                return Box::new(NotificationServiceMock);
            }
        };

        let mut service = RabbitEventBusService::new(connection_configuration);
        if service.connect().await.is_ok() {
            return Box::new(service);
        } else {
            println!("Error while connecting to rabbitMQ. Mock service will be created");
        }
    }

    Box::new(NotificationServiceMock)
}

const ROUTING_KEY: &'static str = "blog.events";
const EXCHANGE_NAME: &'static str = "blog.events";
const QUEUE_NAME: &'static str = "blog.events";

struct RabbitEventBusService {
    connection_configuration: OpenConnectionArguments,
    connection: Option<Connection>,
    channel: Option<Channel>,
}

impl RabbitEventBusService {
    fn new(connection_configuration: OpenConnectionArguments) -> RabbitEventBusService {
        println!("RabbitEventBusService created");
        RabbitEventBusService {
            connection_configuration,
            connection: None,
            channel: None,
        }
    }
}

impl EventBusService for RabbitEventBusService {}

#[async_trait]
trait Connect {
    async fn connect(&mut self) -> Result<(), Error>;
}

//TODO setup correct callback (defaults are "for demo and debugging purposes only")
#[async_trait]
impl Connect for RabbitEventBusService {
    async fn connect(&mut self) -> Result<(), Error> {
        if self.connection.is_some() {
            return Ok(());
        }

        let new_connection = Connection::open(&self.connection_configuration).await?;
        new_connection
            .register_callback(DefaultConnectionCallback)
            .await?;

        let channel = new_connection.open_channel(None).await.unwrap();
        channel.register_callback(DefaultChannelCallback).await?;

        channel
            .queue_bind(QueueBindArguments::new(
                &QUEUE_NAME,
                EXCHANGE_NAME,
                ROUTING_KEY,
            ))
            .await?;

        self.connection = Some(new_connection);
        self.channel = Some(channel);

        Ok(())
    }
}

#[async_trait]
impl Publish<SubscriptionStateChanged> for RabbitEventBusService {
    async fn publish(&self, event: SubscriptionStateChanged) -> () {
        println!(
            "event published: {}, {}",
            event.blog_user_id, event.user_telegram_id
        );

        publish(to_bytes_payload(event), self.channel.clone()).await;
    }
}

async fn publish(payload: Result<Vec<u8>, EventBusError>, channel: Option<Channel>) -> () {
    if let (Ok(payload), Some(channel)) = (payload, channel) {
        let res = internal_publish(payload, &channel).await;
        if res.is_err() {
            println!("Error while publishing message");
        }
    } else {
        println!("Error while parsing event");
    }
}

fn to_bytes_payload<T: Serialize>(event: T) -> Result<Vec<u8>, EventBusError> {
    match serde_json::to_string(&event) {
        Ok(json_string) => Ok(json_string.into_bytes()),
        Err(_) => Err(EventBusError::SerializationError),
    }
}

//TODO add publisher confirms
async fn internal_publish(payload: Vec<u8>, channel: &Channel) -> Result<(), EventBusError> {
    let args = BasicPublishArguments::new(EXCHANGE_NAME, ROUTING_KEY);

    match channel
        .basic_publish(BasicProperties::default(), payload, args)
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(EventBusError::PublishingError),
    }
}

//TODO Mock Service remove after notification service implemented
struct NotificationServiceMock;

impl EventBusService for NotificationServiceMock {}

#[async_trait]
impl Publish<SubscriptionStateChanged> for NotificationServiceMock {
    async fn publish(&self, event: SubscriptionStateChanged) -> () {
        println!(
            "event NOT published. Mock eventBus is used: {}, {}",
            event.blog_user_id, event.user_telegram_id
        );
    }
}
