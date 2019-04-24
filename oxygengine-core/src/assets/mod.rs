pub mod asset;
pub mod database;
pub mod protocol;
pub mod protocols;
pub mod system;

use crate::{
    app::AppBuilder,
    assets::{
        database::AssetsDatabase,
        protocols::{binary::BinaryAssetProtocol, set::SetAssetProtocol, text::TextAssetProtocol},
        system::AssetsSystem,
    },
    fetch::FetchEngine,
};

pub fn bundle_installer<'a, 'b, FE: 'static, ADS>(
    builder: &mut AppBuilder<'a, 'b>,
    (fetch_engine, mut assets_database_setup): (FE, ADS),
) where
    FE: FetchEngine,
    ADS: FnMut(&mut AssetsDatabase),
{
    let mut database = AssetsDatabase::new(fetch_engine);
    database.register(BinaryAssetProtocol);
    database.register(TextAssetProtocol);
    database.register(SetAssetProtocol);
    assets_database_setup(&mut database);
    builder.install_resource(database);
    builder.install_thread_local_system(AssetsSystem);
}
