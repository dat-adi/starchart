/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::env;

use sqlx::sqlite::SqlitePoolOptions;
use url::Url;

use crate::*;

use db_core::tests::*;

#[actix_rt::test]
async fn everything_works() {
    const HOSTNAME: &str = "https://test-gitea.example.com";
    const HTML_PROFILE_URL: &str = "https://test-gitea.example.com/user1";
    const HTML_PROFILE_PHOTO_URL_2: &str = "https://test-gitea.example.com/profile-photo/user2";
    const USERNAME: &str = "user1";
    const USERNAME2: &str = "user2";

    const REPO_NAME: &str = "starchart";
    const HTML_REPO_URL: &str = "https://test-gitea.example.com/user1/starchart";
    const TAGS: [&str; 3] = ["test", "starchart", "spider"];

    let hostname = Url::parse(HOSTNAME).unwrap();
    let hostname = get_hostname(&hostname);

    let create_forge_msg = CreateForge {
        hostname: &hostname,
        forge_type: ForgeImplementation::Gitea,
    };

    let add_user_msg = AddUser {
        hostname: &hostname,
        html_link: HTML_PROFILE_URL,
        profile_photo: None,
        username: USERNAME,
    };

    let add_user_msg_2 = AddUser {
        hostname: &hostname,
        html_link: HTML_PROFILE_PHOTO_URL_2,
        profile_photo: Some(HTML_PROFILE_PHOTO_URL_2),
        username: USERNAME2,
    };

    let url = env::var("SQLITE_DATABASE_URL").expect("Set SQLITE_DATABASE_URL env var");
    let pool_options = SqlitePoolOptions::new().max_connections(2);
    let connection_options = ConnectionOptions::Fresh(Fresh { pool_options, url });
    let db = connection_options.connect().await.unwrap();
    db.migrate().await.unwrap();

    let add_repo_msg = AddRepository {
        html_link: HTML_REPO_URL,
        name: REPO_NAME,
        tags: Some(TAGS.into()),
        owner: USERNAME,
        website: None,
        description: None,
        hostname: &hostname,
    };

    adding_forge_works(
        &db,
        create_forge_msg,
        add_user_msg,
        add_user_msg_2,
        add_repo_msg,
    )
    .await;
}

#[actix_rt::test]
async fn forge_type_exists() {
    let url = env::var("SQLITE_DATABASE_URL").expect("Set SQLITE_DATABASE_URL env var");
    let pool_options = SqlitePoolOptions::new().max_connections(2);
    let connection_options = ConnectionOptions::Fresh(Fresh { pool_options, url });
    let db = connection_options.connect().await.unwrap();

    db.migrate().await.unwrap();
    forge_type_exists_helper(&db).await;
}
