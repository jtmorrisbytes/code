// use serde::{Deserialize, Serialize};
// #[repr(transparent)]
// #[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Eq, Hash)]
// #[cfg_attr(
//     feature = "rocket",
//     derive(rocket::FromForm, rocket::UriDisplayPath, rocket::UriDisplayQuery)
// )]
// #[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
// #[cfg_attr(feature = "sqlx", sqlx(transparent, type_name = "varchar"))]
// #[serde(transparent)]

// pub struct Vin(String);
// impl std::ops::Deref for Vin {
//     type Target = String;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl std::convert::From<String> for Vin {
//     fn from(value: String) -> Self {
//         Self(value)
//     }
// }
// impl Default for Vin {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }

// impl<'r> rocket::request::FromParam<'r> for Vin {
//     type Error = anyhow::Error;
//     fn from_param(param: &'r str) -> Result<Self, Self::Error> {
//         Ok(Self(param.to_string()))
//     }
// }

// #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
// #[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
// pub struct Vehicle {
//     // pub id: i64,
//     // #[serde(flatten)]
//     pub vin: Vin,
//     pub make: String,
//     pub model: String,
//     pub color: String,
//     pub year: String,
// }
// #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
// #[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
// pub struct NewVehicle {
//     pub vin: String,
//     pub make: String,
//     pub model: String,
//     pub color: String,
//     pub year: String,
// }

// #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
// pub struct VehicleSearchOptions {
//     pub vin: Option<Vin>,
//     pub make: Option<String>,
//     pub model: Option<String>,
//     pub color: Option<String>,
//     pub year: Option<String>,
// }

// pub mod db {
//     // we want to keep track of a list of vehicles and its location history
//     // this data will then be used to estimate milage / trip data like uber and lyft do.
//     // we can take the data from the location history and copy it into another table and mark it as buisess, personal, etc

//     // we may want to create seperate tables for each unique vehicle since this table may be quite large.
//     // (we want to keep track of turn by turn like)
//     use crate::vehicles::{NewVehicle, Vehicle, VehicleSearchOptions};
//     pub async fn pg_insert_vehicle_returning(
//         connection: &mut sqlx::PgConnection,
//         new_vehicle: NewVehicle,
//     ) -> anyhow::Result<Vehicle> {
//         let NewVehicle {
//             vin,
//             make,
//             model,
//             year,
//             color,
//         } = new_vehicle;
//         let vehicle = sqlx::query_file_as!(
//             Vehicle,
//             "sql/vehicles/insert_vehicle.psql",
//             vin,
//             make,
//             model,
//             color,
//             year
//         )
//         .fetch_one(connection)
//         .await?;
//         Ok(vehicle)
//     }
//     pub async fn pg_search_vehicles(
//         connection: &mut sqlx::PgConnection,
//         search_options: VehicleSearchOptions,
//     ) -> anyhow::Result<Vec<Vehicle>> {
//         // these are hard coded for now until we need to change them
//         const LIMIT: i64 = 1000;
//         // the offset starts at zero!
//         const OFFSET: i64 = 0;

//         let vin: crate::vehicles::Vin = search_options.vin.unwrap_or_default();
//         let make = search_options.make.unwrap_or_default();
//         let model = search_options.model.unwrap_or_default();
//         let color = search_options.color.unwrap_or_default();
//         let year = search_options.year.unwrap_or_default();
//         let vehicles = sqlx::query_file_as!(
//             Vehicle,
//             "sql/vehicles/search_vehicles.psql",
//             &vin,
//             make,
//             model,
//             color,
//             year,
//             LIMIT,
//             OFFSET
//         )
//         .fetch_all(connection)
//         .await?;
//         Ok(vehicles)
//     }
//     pub async fn pg_get_distinct_vehicle_makes(
//         connection: &mut sqlx::PgConnection,
//     ) -> anyhow::Result<Vec<String>> {
//         let results = sqlx::query_file!("sql/vehicles/get_distinct_vehicle_makes.psql")
//             .fetch_all(connection)
//             .await?
//             .into_iter()
//             .map(|row| row.make)
//             .collect();
//         Ok(results)
//     }
//     pub async fn pg_get_distinct_vehicle_models(
//         connection: &mut sqlx::PgConnection,
//     ) -> anyhow::Result<Vec<String>> {
//         let results = sqlx::query_file!("sql/vehicles/get_distinct_vehicle_models.psql")
//             .fetch_all(connection)
//             .await?
//             .into_iter()
//             .map(|row| row.model)
//             .collect();
//         Ok(results)
//     }
//     pub async fn pg_get_distinct_vehicle_years(
//         connection: &mut sqlx::PgConnection,
//     ) -> anyhow::Result<Vec<String>> {
//         let results = sqlx::query_file!("sql/vehicles/get_distinct_vehicle_years.psql")
//             .fetch_all(connection)
//             .await?
//             .into_iter()
//             .map(|row| row.year)
//             .collect();

//         Ok(results)
//     }
//     pub async fn pg_get_distinct_vehicle_colors(
//         connection: &mut sqlx::PgConnection,
//     ) -> anyhow::Result<Vec<String>> {
//         let results = sqlx::query_file!("sql/vehicles/get_distinct_vehicle_colors.psql")
//             .fetch_all(connection)
//             .await?
//             .into_iter()
//             .map(|row| row.color)
//             .collect();
//         Ok(results)
//     }
//     pub async fn pg_get_distinct_vehicle_vins(
//         connection: &mut sqlx::PgConnection,
//     ) -> anyhow::Result<Vec<String>> {
//         let results = sqlx::query_file!("sql/vehicles/get_distinct_vehicle_vins.psql")
//             .fetch_all(connection)
//             .await?
//             .into_iter()
//             .map(|row| row.vin)
//             .collect();
//         Ok(results)
//     }

//     pub async fn pg_find_vehicle_by_vin(
//         connection: &mut sqlx::PgConnection,
//         vin: &crate::vehicles::Vin,
//     ) -> Result<Option<Vehicle>, anyhow::Error> {
//         Ok(
//             sqlx::query_file_as!(Vehicle, "sql/vehicles/find_vehicle_by_vin.psql", vin)
//                 .fetch_optional(connection)
//                 .await?,
//         )
//     }
// }

// pub mod web_server {

//     // VEHICLES
//     #[rocket::get("/vehicles?<vin>&<make>&<model>&<year>&<color>", format = "text/html")]
//     pub async fn render_vehicles(
//         pool: rocket_db_pools::Connection<crate::PrimaryDatabase>,
//         vin: Option<crate::vehicles::Vin>,
//         make: Option<String>,
//         model: Option<String>,
//         year: Option<String>,
//         color: Option<String>,
//     ) -> rocket_dyn_templates::Template {
//         // provide a search hint by getting a list of vehicle colors
//         // run all 5 queries at the same time in the background by spawing these in seperate tasks

//         use std::collections::HashMap;

//         use crate::vehicles::{Vehicle, VehicleSearchOptions};
//         let conn_result1 = pool.acquire().await;
//         let conn_result2 = pool.acquire().await;
//         let conn_result3 = pool.acquire().await;
//         let conn_result4 = pool.acquire().await;
//         let conn_result5 = pool.acquire().await;
//         let conn_result6 = pool.acquire().await;

//         let colors_handle: rocket::tokio::task::JoinHandle<
//             Result<Vec<std::string::String>, anyhow::Error>,
//         > = rocket::tokio::spawn(async move {
//             let mut conn = conn_result1?;
//             let r = crate::vehicles::db::pg_get_distinct_vehicle_colors(&mut conn).await?;
//             conn.close().await?;
//             Ok(r)
//         });
//         let makes_handle: rocket::tokio::task::JoinHandle<
//             Result<Vec<std::string::String>, anyhow::Error>,
//         > = rocket::tokio::spawn(async move {
//             let mut conn = conn_result2?;

//             let r = crate::vehicles::db::pg_get_distinct_vehicle_makes(&mut conn).await?;
//             conn.close().await?;
//             Ok(r)
//         });
//         let models_handle: rocket::tokio::task::JoinHandle<
//             Result<Vec<std::string::String>, anyhow::Error>,
//         > = rocket::tokio::spawn(async move {
//             let mut conn = conn_result3?;
//             let r = crate::vehicles::db::pg_get_distinct_vehicle_models(&mut conn).await?;
//             conn.close().await?;
//             Ok(r)
//         });
//         let years_handle: rocket::tokio::task::JoinHandle<
//             Result<Vec<std::string::String>, anyhow::Error>,
//         > = rocket::tokio::spawn(async move {
//             let mut conn = conn_result4?;
//             let r = crate::vehicles::db::pg_get_distinct_vehicle_years(&mut conn).await?;
//             conn.close().await?;
//             Ok(r)
//         });
//         let vins_handle: rocket::tokio::task::JoinHandle<
//             Result<Vec<std::string::String>, anyhow::Error>,
//         > = rocket::tokio::spawn(async move {
//             let mut conn = conn_result5?;

//             let r = crate::vehicles::db::pg_get_distinct_vehicle_vins(&mut conn).await?;
//             conn.close().await?;
//             Ok(r)
//         });
//         let search_options = VehicleSearchOptions {
//             vin: vin.clone(),
//             make: make.clone(),
//             model: model.clone(),
//             year: year.clone(),
//             color: color.clone(),
//         };
//         let vehicles_handle: rocket::tokio::task::JoinHandle<Result<Vec<Vehicle>, anyhow::Error>> =
//             rocket::tokio::spawn(async move {
//                 let mut conn = conn_result6?;
//                 Ok(crate::vehicles::db::pg_search_vehicles(&mut conn, search_options).await?)
//             });
//         let mut errors: Vec<anyhow::Error> = Vec::with_capacity(5);
//         let mut colors: Vec<String> = Vec::new();
//         let mut makes: Vec<String> = Vec::new();
//         let mut models: Vec<String> = Vec::new();
//         let mut years: Vec<String> = Vec::new();
//         let mut vins: Vec<String> = Vec::new();
//         let mut vehicles: Vec<Vehicle> = Vec::new();

//         match colors_handle.await {
//             Err(join_error) => errors.push(join_error.into()),
//             Ok(Err(db_error)) => errors.push(db_error.into()),
//             Ok(Ok(list)) => colors = list,
//         };
//         match makes_handle.await {
//             Err(join_error) => errors.push(join_error.into()),
//             Ok(Err(db_error)) => errors.push(db_error.into()),
//             Ok(Ok(list)) => makes = list,
//         };
//         match models_handle.await {
//             Err(join_error) => errors.push(join_error.into()),
//             Ok(Err(db_error)) => errors.push(db_error.into()),
//             Ok(Ok(list)) => models = list,
//         };
//         match years_handle.await {
//             Err(join_error) => errors.push(join_error.into()),
//             Ok(Err(db_error)) => errors.push(db_error.into()),
//             Ok(Ok(list)) => years = list,
//         };
//         match vins_handle.await {
//             Err(join_error) => errors.push(join_error.into()),
//             Ok(Err(db_error)) => errors.push(db_error.into()),
//             Ok(Ok(list)) => vins = list,
//         };
//         match vehicles_handle.await {
//             Err(join_error) => errors.push(join_error.into()),
//             Ok(Err(db_error)) => errors.push(db_error.into()),
//             Ok(Ok(list)) => vehicles = list,
//         };
//         let mut uris: HashMap<crate::vehicles::Vin, String> = HashMap::new();

//         for vehicle in &vehicles {
//             uris.insert(
//                 vehicle.vin.clone(),
//                 (rocket::uri!(render_vehicle_overview(vehicle.vin.clone()))).to_string(),
//             );
//         }
//         let errors: Vec<_> = errors.into_iter().map(|e| e.to_string()).collect();
//         rocket_dyn_templates::Template::render(
//             "vehicles",
//             rocket_dyn_templates::context! {vin,make,model,year,color,colors,makes,models,vins,years,errors,vehicles,uris},
//         )
//     }

//     #[rocket::get("/vehicles/<vin>", format = "text/html")]
//     pub async fn render_vehicle_overview(
//         pool: rocket_db_pools::Connection<crate::PrimaryDatabase>,
//         vin: crate::vehicles::Vin,
//     ) -> rocket_dyn_templates::Template {
//         // search for the vehicle by the VIN number, since it is unique
//         let mut conn = pool.acquire().await.unwrap();
//         let find_result: Result<Option<crate::vehicles::Vehicle>, anyhow::Error> =
//             crate::vehicles::db::pg_find_vehicle_by_vin(&mut conn, &vin).await;
//         if find_result.is_err() {
//             return rocket_dyn_templates::Template::render(
//                 "vehicle_details_error",
//                 rocket_dyn_templates::context! {vin},
//             );
//         }
//         let vehicle_option = find_result.unwrap();
//         if vehicle_option.is_none() {
//             return rocket_dyn_templates::Template::render(
//                 "vehicle_details_vehicle_not_found",
//                 rocket_dyn_templates::context! {vin},
//             );
//         }
//         let vehicle = vehicle_option.unwrap();
//         rocket_dyn_templates::Template::render(
//             "vehicle_details",
//             rocket_dyn_templates::context! {vin,vehicle},
//         )
//     }

//     #[rocket::post(
//         "/vehicles",
//         format = "application/x-www-form-urlencoded",
//         data = "<vehicle_form_result>"
//     )]
//     pub async fn create_vehicle_from_form(
//         pool: rocket_db_pools::Connection<crate::PrimaryDatabase>,
//         vehicle_form_result: Result<
//             rocket::form::Form<crate::vehicles::NewVehicle>,
//             rocket::form::Errors<'_>,
//         >,
//     ) -> rocket_dyn_templates::Template {
//         let mut connection = pool.acquire().await.unwrap();
//         let vehicle_form = vehicle_form_result.unwrap();
//         let new_vehicle = vehicle_form.into_inner();
//         crate::vehicles::db::pg_insert_vehicle_returning(&mut connection, new_vehicle)
//             .await
//             .unwrap();
//         render_vehicles(pool, None, None, None, None, None).await
//     }
// }
