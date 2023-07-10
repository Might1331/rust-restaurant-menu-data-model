/*
 * Copyright (C) 2022 Vaticle
 *
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use futures::StreamExt;
use std::fs;
use text_io::read;
use typedb_client::{
    concept::{Attribute, Concept, Value},
    Connection, DatabaseManager, Session,
    SessionType::{Data, Schema},
    TransactionType::{Read, Write},
};

const MENU_DATABASE: &str = "menuDB";

fn new_core_connection() -> typedb_client::Result<Connection> {
    Connection::new_plaintext("localhost:1729")
}

async fn load_data(connection: Connection) -> std::io::Result<()> {
    let data = fs::read_to_string("./src/data.tql")?;
    let databases = DatabaseManager::new(connection.clone());
    // insert data
    let session = Session::new(databases.get(MENU_DATABASE).await.unwrap(), Data)
        .await
        .unwrap();
    let transaction = session.transaction(Write).await.unwrap();
    let _ = transaction.query().insert(data.as_str());
    transaction.commit().await.unwrap();
    println!("\nData Loaded\n");
    Ok(())
}

async fn load_schema(connection: Connection) -> std::io::Result<()> {
    let schema = fs::read_to_string("./src/schema.tql")?;
    let databases = DatabaseManager::new(connection.clone());
    if !databases.contains(MENU_DATABASE).await.unwrap() {
        databases.create(MENU_DATABASE).await.unwrap();
        // define schema
        {
            let session = Session::new(databases.get(MENU_DATABASE).await.unwrap(), Schema)
                .await
                .unwrap();
            let transaction = session.transaction(Write).await.unwrap();
            transaction.query().define(schema.as_str()).await.unwrap();
            transaction.commit().await.unwrap();
        }

        // load data for the first
        load_data(connection.clone()).await?;
        println!("\nSchema Loaded\n");
    } else {
        println!("\nSchema Already Defined\n");
    }

    Ok(())
}

async fn query1(connection: Connection, x: String) -> std::io::Result<()> {
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(MENU_DATABASE).await.unwrap(), Data)
        .await
        .unwrap();
    let transaction = session.transaction(Read).await.unwrap();
    println!("::Q1::");
    let q = format!(
        "match $rf isa raw_food, has name \"{x}\";
        $is_i (raw_food: $rf,dish: $d) isa is_ingredient;
        $sl (seller: $m,product: $d) isa sells;
        $m has name $n,has call_number $c;
        get $n;"
    );
    let mut answer_stream = transaction.query().match_(q.as_str()).unwrap();
    while let Some(result) = answer_stream.next().await {
        match result {
            Ok(concept_map) => {
                for (_, concept) in concept_map {
                    if let Concept::Attribute(Attribute {
                        value: Value::String(value),
                        ..
                    }) = concept
                    {
                        println!("{}", value);
                    }
                }
            }
            Err(err) => {
                panic!("An error occurred fetching answers of a Match query: {err}")
            }
        }
    }
    Ok(())
}

async fn query2(connection: Connection) -> std::io::Result<()> {
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(MENU_DATABASE).await.unwrap(), Data)
        .await
        .unwrap();
    println!("::Q2::");
    let transaction = session.transaction(Read).await.unwrap();
    let q = "
    match $m2 isa menu, has is_vegetarian false,has name $n2;
    $d2 isa dish, has is_vegetarian false;
    $sp2 (restaurant: $m2,$d2) isa speciality;
    get $m2;count;".to_string();
    let answer = transaction
        .query()
        .match_aggregate(q.as_str())
        .await
        .unwrap();
    println!("Answe for Q2:  {}\n", answer.into_i64());
    Ok(())
}

async fn query3(connection: Connection, r: String, p: String) -> std::io::Result<()> {
    let databases = DatabaseManager::new(connection.clone());
    let session = Session::new(databases.get(MENU_DATABASE).await.unwrap(), Data)
        .await
        .unwrap();
    let transaction = session.transaction(Read).await.unwrap();
    println!("::Q3::");
    let q = format!(
        "match $m3 isa menu, has avg_rating>{r},has name $mn3;
        $d3 isa dish, has name $dn3;
        $sl3 (seller: $m3,product: $d3) isa sells, has price $p3;
        $p3>{p};
        $rf3 isa raw_food,has name $rfn3;
        $isn3 (raw_food: $rf3,$d3) isa is_ingredient;
        get $rfn3;"
    );
    let mut answer_stream = transaction.query().match_(q.as_str()).unwrap();
    while let Some(result) = answer_stream.next().await {
        match result {
            Ok(concept_map) => {
                for (_, concept) in concept_map {
                    if let Concept::Attribute(Attribute {
                        value: Value::String(value),
                        ..
                    }) = concept
                    {
                        println!("{}", value);
                    }
                }
            }
            Err(err) => {
                panic!("An error occurred fetching answers of a Match query: {err}")
            }
        }
    }
    Ok(())
}

async fn run_io(connection: Connection) {
    println!("Q1->What places could buy raw_food=$x ?");
    println!("Q2->Get count of non-vegetarian outlets with vegetarian specialities.");
    println!("Q3->Get count raw items sold at places with avg_rating more tha $r and has a dish using it as raw_ingredient with price greater than $p units.");
    println!("What query would you like to make? Enter 1,2 or 3.\n");
    let qtype: i32 = read!();
    if qtype == 1 {
        println!("Enter the name of raw_food x : ");
        let inp: String = read!();
        let x = inp.as_str();

        query1(connection, x.to_string()).await.unwrap();
    } else if qtype == 2 {
        query2(connection).await.unwrap();
    } else if qtype == 3 {
        println!("Enter the avg_rating of Restraunt r : ");
        let inp0: String = read!();
        let r = inp0.as_str();
        println!("Enter the threshold price for raw_ingredient p : ");
        let inp1: String = read!();
        let p = inp1.as_str();

        query3(connection, r.to_string(), p.to_string())
            .await
            .unwrap();
    } else {
        println!("Query entered is not 1,2 or 3\n");
    }
}

#[tokio::main]
async fn main() {
    loop {
        let current_line = line!().to_string();
        let con = new_core_connection().expect(&current_line);
        load_schema(con.clone()).await.unwrap();

        run_io(con.clone()).await;
        println!("Enter 0 to exit or 1 to continue:");
        let query_again: i32 = read!();
        if query_again == 0 {
            break;
        }
    }
}