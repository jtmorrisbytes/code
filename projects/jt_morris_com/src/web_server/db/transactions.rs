

use super::{PgPool,schema};
use time;



use super::schema::transactions;
use bigdecimal::{BigDecimal, Zero};


use diesel::{prelude::*,QueryDsl};

pub const VENDOR_DEFAULT_VALUE: &str = "UNKNOWN";

#[allow(unused)]
#[derive(PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(
    not(all(target_arch = "wasm32", target_os = "unknown")),
    derive(Queryable,Insertable)
)]
#[cfg_attr(not(all(target_arch="wasm32",target_os="unknown")),diesel(table_name=schema::transactions))]
pub struct Transaction {
    id: uuid::Uuid,
    pub total_cost: BigDecimal,
    pub vendor: String,
    pub date: time::Date,
    pub user_id: uuid::Uuid,
}
impl Transaction {
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
}
pub struct TransactionsUsersTransactions;
#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[cfg_attr(any(feature = "debug", debug_assertions), derive(Debug))]
#[derive(Clone)]
pub struct BankOfAmericaTransaction {
    pub date: time::Date,
    pub description: String,
    pub amount: BigDecimal,
    #[allow(unused)]
    pub running_balance: BigDecimal,
}
// we need a way to import transactions
// start with bank of america
pub struct BankOfAmericaTransactionImportErrors {
    pub database_errors: Vec<anyhow::Error>,
    pub parse_errors: Vec<anyhow::Error>,
}
impl BankOfAmericaTransactionImportErrors {
    pub fn new() -> Self {
        Self {
            database_errors: Vec::new(),
            parse_errors: Vec::new(),
        }
    }
}
pub const BANK_OF_AMERICA_DATE_FORMAT_STR: &str = "%m/%d/%Y";

// use crate::PgPooledConnection;
#[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
pub async fn import_bank_of_america_file_for_user(
    _pool: PgPool,
    _user_id: &uuid::Uuid,
    _path: std::path::PathBuf,
) -> Result<(usize, BankOfAmericaTransactionImportErrors), anyhow::Error> {
//     // use core::task;
//     // use futures_util::AsyncReadExt;
//     use rocket::futures::AsyncReadExt;
//     use std::str::FromStr;
//     use time::macros::format_description;

//     let file = tokio::fs::File::open(path).await?;
//     let mut reader = csv_async::AsyncReader::from_reader(tokio::io::BufReader::new(file));
//     let mut records = reader.records();
//     let mut transactions: Vec<BankOfAmericaTransaction> =
//         Vec::<BankOfAmericaTransaction>::with_capacity(25);
//     let mut errors = BankOfAmericaTransactionImportErrors::new();
//     let mut record_number = 0;
//     let mut successfully_uploaded = 0;

//     loop {
//         let record = match records.next().await {
//             Some(Ok(record)) => record,
//             Some(Err(error)) => {
//                 // tracing::error!("Error while importing transactions from file: {error}");
//                 errors.parse_errors.push(anyhow::Error::new(error));
//                 continue;
//             }
//             None => {
//                 break;
//             }
//         };

//         let (date, description, amount, running_balance) = match (
//             record.get(0),
//             record.get(1),
//             record.get(2),
//             record.get(3),
//         ) {
//             (Some(date), Some(description), Some(amount), Some(running_balance)) => {
//                 (date, description, amount, running_balance)
//             }
//             (None, None, None, None) => {
//                 let message = format!("Missing field date at position 1. no fields were parsed for record number {record_number}");
//                 errors.parse_errors.push(anyhow::Error::msg(message));
//                 record_number = record_number + 1;
//                 continue;
//             }
//             (Some(date), None, None, None) => {
//                 let mesage = format!("Missing field description at position 2. fields:[date]=[{date}] record number {record_number}");
//                 errors.parse_errors.push(anyhow::Error::msg(mesage));
//                 record_number = record_number + 1;
//                 continue;
//             }
//             (Some(date), Some(description), None, None) => {
//                 let message = format!("Missing field amount at position 3. fields:[date,description]=[{date},{description}] record number {record_number}");
//                 errors.parse_errors.push(anyhow::Error::msg(message));
//                 record_number = record_number + 1;
//                 continue;
//             }
//             (Some(date), Some(description), Some(amount), None) => {
//                 let message = format!("Missing field running_balance at position 4. fields:[date,description,amount]=[{date},{description},{amount}] record number {record_number}");
//                 errors.parse_errors.push(anyhow::Error::msg(message));
//                 record_number = record_number + 1;
//                 continue;
//             }
//             _ => {
//                 errors.parse_errors.push(anyhow::Error::msg("Reached impossible case for a csv file where one or more fields were Some after a None case"));
//                 record_number = record_number + 1;
//                 continue;
//             }
//         };
//         let date = match time::Date::parse(&date, format_description!("[year]/[day]/[month]")) {
//             Ok(date) => date,
//             Err(e) => {
//                 let message = format!(
//                         "Error while atempting to parse a date for record number {record_number}: '{e}' input: {date}"
//                     );
//                 errors.parse_errors.push(anyhow::Error::msg(message));
//                 record_number = record_number + 1;
//                 continue;
//             }
//         };
//         let amount = match BigDecimal::from_str(amount) {
//             Ok(amount) => amount,
//             Err(error) => {
//                 let message =
//                     format!("While attempting to parse amount for record number {record_number}");
//                 errors
//                     .parse_errors
//                     .push(anyhow::Error::new(error).context(message));
//                 record_number = record_number + 1;
//                 continue;
//             }
//         };
//         let running_balance = match BigDecimal::from_str(running_balance) {
//             Ok(amount) => amount,
//             Err(error) => {
//                 let message =
//                     format!("While attempting to parse amount for record number {record_number}");
//                 errors
//                     .parse_errors
//                     .push(anyhow::Error::new(error).context(message));
//                 record_number = record_number + 1;
//                 continue;
//             }
//         };
//         let record: BankOfAmericaTransaction = BankOfAmericaTransaction {
//             date,
//             description: description.to_string(),
//             amount,
//             running_balance,
//         };
//         // if the size of this type in memory in bytes is greater than 512 MB
//         transactions.push(record);
//         if std::mem::size_of_val(&transactions) > 512_000_000 {
//             // let connection = pool.get()?;
//             // let connection = connection;

//             let transactions_clone = transactions.clone();
//             let pool_clone = pool.clone();
//             let user_id = user_id.clone();
//             let task_result = tokio::task::spawn_blocking(move || {
//                 insert_bank_of_america_transactions_into_transactions_table_for_user(
//                     pool_clone,
//                     transactions_clone,
//                     user_id,
//                 )
//             })
//             .await;

//             match task_result {
//                 Ok(Err(error)) => errors.database_errors.push(error),
//                 Ok(Ok(num_records)) => {
//                     // tracing::info!("inserted {num_records} records");
//                     successfully_uploaded = successfully_uploaded + num_records;
//                 }
//                 Err(e) => errors.database_errors.push(e.into()),
//             };
//             transactions.clear();
//         }
//         // dbg!(record);
//         record_number = record_number + 1;
//     }
//     if transactions.len() > 0 {
//         // let connection = pool.get()?;
//         match insert_bank_of_america_transactions_into_transactions_table_for_user(
//             pool.clone(),
//             transactions,
//             user_id.to_owned(),
//         ) {
//             Ok(num_records) => {
//                 // tracing::info!("inserted {num_records} records");
//                 successfully_uploaded = successfully_uploaded + num_records;
//             }
//             Err(e) => errors.database_errors.push(e),
//         };
//     }
//     return Ok((successfully_uploaded, errors));
todo!();
}
#[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
pub fn insert_bank_of_america_transactions_into_transactions_table_for_user(
    pool: PgPool,
    transactions: Vec<BankOfAmericaTransaction>,
    user_id: uuid::Uuid,
) -> Result<usize, anyhow::Error> {
    // let mut errors: Vec<anyhow::Error> = Vec::new();
    // batch insert transactions
    let query = diesel::insert_into(transactions::table);
    let mut connection = pool.get()?;
    let transactions: Vec<Transaction> = transactions
        .into_iter()
        .map(|b_o_a| {
            Transaction {
                id: uuid::Uuid::new_v4(),
                total_cost: b_o_a.amount,
                date: b_o_a.date,
                // description: b_o_a.description,
                vendor: VENDOR_DEFAULT_VALUE.to_string(),
                user_id: user_id.clone(),
            }
        })
        .collect();
    query
        .values(transactions)
        .execute(&mut connection)
        .map_err(|e| anyhow::Error::new(e))
}

/// Create a function that allows a user to view their total expenses by transaction
/// allow the user to group by vendor and category
#[cfg(not(all(target_arch="wasm32",target_os="unknown")))]

pub fn view_summary_of_transactions_group_by_name_vendor_and_category(
    pool: PgPool,
    user_id: uuid::Uuid,
) -> Result<(), anyhow::Error> {
    let mut connection = pool.get()?;
    /*
       SELECT SUM("transactions"."total_amount"), as "transactions_total_amount",
                  "transactions"."id" as "transactions_id",
                  "transactions_items"."category" as "transactions_items_category",
                  "transactions"."vendor" as "transactions_vendor",
                  "users"."id" as "user_id" from transactions,transactions_items,users
       JOIN ON user_id = users_transactions.user_id AND users_transactions.id = transaction.id
       JOIN ON user_id = transactions_users.user_id AND transactions_users.transaction_id = transaction.id
       JOIN ON transactions_items.transaction_id = transaction.id
       WHERE user_id = $1
       GROUP BY transactions_total_amount,transactions_items_category,transactions_vendor
       ORDER BY
           transactions_total_amount DESC,
           transactions_items_category ASC,
           transactions_vendor ASC
    */
    // schema::transactions::dsl::
    let query = transactions::table
        .filter(transactions::total_cost.lt(BigDecimal::zero()))
        .filter(transactions::user_id.eq(user_id));
    // let query2 = schema::transactions_users::table.inner_join(schema::users::table);
    // .inner_join(schema::transactions_users::table.on(schema::transactions_users::user_id.eq(schema::users::id)));
    // let query = diesel::select(t_u);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query);
    let _transactions: Vec<Transaction> = query.clone().get_results(&mut *connection)?;
    // let sql2 = diesel::debug_query::<diesel::pg::Pg,_>(&query2);
    print!("{sql}");
    Ok(())
}

// #[cfg(test)]
// pub(crate) mod tests {
//     const TEST_AUTH0_USER_ID: &str = "auth0|6682093e90b963e367242a6f";
//     #[tokio::test]
//     pub async fn test_view_summary_of_transactions_group_by_name_vendor_and_category(
//     ) -> Result<(), Box<dyn std::error::Error>> {
//         let pool_1 = crate::tests::DB_POOL.clone();
//         let pool_2 = pool_1.clone();
//         let user_id = tokio::task::spawn_blocking(move || {
//             // let connection = crate::tests::get_connection().unwrap();
//             crate::get_user_id_from_auth0_user_id(pool_1, &TEST_AUTH0_USER_ID).unwrap()
//         })
//         .await
//         .unwrap()
//         .unwrap();
//         // let user_id = uuid::uuid!("17220324-67dc-4dda-8ed8-c8bbb717706a");

//         tokio::task::spawn_blocking(move || {
//             super::view_summary_of_transactions_group_by_name_vendor_and_category(pool_2, user_id)
//         })
//         .await
//         .unwrap()
//         .unwrap();

//         Ok(())
//     }
// }
