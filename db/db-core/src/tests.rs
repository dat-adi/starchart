/*
 * ForgeFlux StarChart - A federated software forge spider
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
//! Test utilities
use crate::prelude::*;

/// adding forge works
pub async fn adding_forge_works<'a, T: SCDatabase>(
    db: &T,
    create_forge_msg: CreateForge<'a>,
    add_user_msg: AddUser<'a>,
    add_user_msg2: AddUser<'a>,
    add_repo_msg: AddRepository<'a>,
) {
    let _ = db.delete_forge_instance(create_forge_msg.hostname).await;
    db.create_forge_isntance(&create_forge_msg).await.unwrap();
    assert!(
        db.forge_exists(create_forge_msg.hostname).await.unwrap(),
        "forge creation failed, forge existance check failure"
    );

    // add user
    db.add_user(&add_user_msg).await.unwrap();
    db.add_user(&add_user_msg2).await.unwrap();
    // verify user exists
    assert!(db.user_exists(add_user_msg.username, None).await.unwrap());
    assert!(db
        .user_exists(add_user_msg.username, Some(add_user_msg.hostname))
        .await
        .unwrap());

    // add repository
    db.create_repository(&add_repo_msg).await.unwrap();
    // verify repo exists
    assert!(db
        .repository_exists(add_repo_msg.name, add_repo_msg.owner, add_repo_msg.hostname)
        .await
        .unwrap());
    // delete repository
    db.delete_repository(add_repo_msg.owner, add_repo_msg.name, add_repo_msg.hostname)
        .await
        .unwrap();
    assert!(!db
        .repository_exists(add_repo_msg.name, add_repo_msg.owner, add_repo_msg.hostname)
        .await
        .unwrap());

    // delete user
    db.delete_user(add_user_msg.username, add_user_msg.hostname)
        .await
        .unwrap();
    assert!(!db
        .user_exists(add_user_msg.username, Some(add_user_msg.hostname))
        .await
        .unwrap());
}

/// test if all forge type implementations are loaded into DB
pub async fn forge_type_exists_helper<T: SCDatabase>(db: &T) {
    for f in [ForgeImplementation::Gitea].iter() {
        println!("Testing forge implementation exists for: {}", f.to_str());
        assert!(db.forge_type_exists(f).await.unwrap());
    }
}