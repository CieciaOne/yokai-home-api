# YOKAI HOME API

Multifunctional API servig articles, network devices status with support for WOL and also configurable rss feed

## Setup

To run the server you need to first start the postgres db with `docker-compose up`, and after it is started run the migrations `sqlx migrate run` from the root project directory.
For that you may need to install sqlx cli tools using `cargo install sqlx-cli`
