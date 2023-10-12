use crate::{
    common::{Channel, Db, Item, Store},
    feed::model::ChannelModel,
};

use anyhow::{Ok, Result};
use log::info;
use rss::Channel as RssChannel;
use std::time::Duration;
use tokio::time::interval;

pub struct FeedManager {
    db: Db,
    store: Store,
    interval: Duration,
}

impl FeedManager {
    pub fn new(db: Db, store: Store, interval: Duration) -> Self {
        Self {
            db,
            store,
            interval,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Started Network Status Monitor");
        let mut interval = interval(self.interval);
        loop {
            interval.tick().await;
            self.store.lock().await.clear();
            let db = &self.db.lock().await.clone();
            let channels = sqlx::query_as!(ChannelModel, "SELECT * FROM channels;")
                .fetch_all(db)
                .await?;

            for channel in channels {
                self.refresh(channel).await?;
            }
        }
    }

    async fn refresh(&mut self, channel: ChannelModel) -> Result<()> {
        let channel_update = reqwest::get(channel.url).await?.bytes().await?;
        let new_channel = RssChannel::read_from(&channel_update[..])?;
        let items = new_channel
            .items
            .iter()
            .map(|item| Item::new(item.title.to_owned(), item.link.to_owned()))
            .collect();
        self.store.lock().await.insert(
            channel.id,
            Channel::new(new_channel.title, new_channel.link, items),
        );
        Ok(())
    }
}
