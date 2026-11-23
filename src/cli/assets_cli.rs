use std::path::Path;

use clap::{Args, Subcommand};

use rbxcloud::rbx::{
    error::Error,
    v1::{
        assets::{
            AssetCreation, AssetCreationContext, AssetCreator, AssetGroupCreator, AssetType,
            AssetUserCreator,
        },
        ArchiveAsset, CreateAsset, GetAsset, GetAssetOperation, UpdateAsset,
    },
};
use rbxcloud::rbx::{
    types::RobloxUserId,
    v1::assets::{AssetPermissionAction, PermissionSubject},
};

use super::to_json;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum CreatorType {
    User,
    Group,
}

#[derive(Debug, Args)]
pub struct AssetCommon {
    /// Pretty-print the JSON response
    #[clap(short, long, value_parser, default_value_t = false)]
    pretty: bool,

    /// Roblox Open Cloud API Key
    #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
    api_key: String,
}

/// Who to grant asset permissions to.
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum PermissionSubjectType {
    User,
    Group,
    Universe,
}

#[derive(Debug, Subcommand)]
pub enum AssetsCommands {
    /// Create an asset
    Create {
        /// Asset type
        #[clap(short = 't', long, value_enum)]
        asset_type: Option<AssetType>,

        /// Display name
        #[clap(short = 'n', long, value_parser)]
        display_name: String,

        /// Description
        #[clap(short, long, value_parser)]
        description: String,

        /// Expected Robux price
        #[clap(short, long, value_parser)]
        expected_price: Option<u64>,

        /// Creator ID
        #[clap(short = 'i', long, value_parser)]
        creator_id: u64,

        /// Creator type
        #[clap(short, long, value_enum)]
        creator_type: CreatorType,

        /// File (full or relative path)
        #[clap(short, long, value_parser)]
        filepath: String,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Update an asset
    Update {
        /// Asset type
        #[clap(short = 't', long, value_enum)]
        asset_type: Option<AssetType>,

        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        /// File (full or relative path)
        #[clap(short, long, value_parser)]
        filepath: String,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Get asset operation information
    GetOperation {
        /// Operation ID
        #[clap(short = 'i', long, value_parser)]
        operation_id: String,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Get asset information
    Get {
        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        #[clap(short = 'm', long, value_parser)]
        read_mask: Option<String>,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Archive an asset
    Archive {
        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Restore an archived asset
    Restore {
        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
    /// List an asset's versions, newest first
    Versions {
        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        /// Max items returned per page
        #[clap(short, long, value_parser)]
        max_page_size: Option<u32>,

        /// Next page token
        #[clap(short, long, value_parser)]
        next_page_token: Option<String>,

        #[clap(flatten)]
        common: AssetCommon,
    },

    /// Get a specific version of an asset
    GetVersion {
        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        /// Version number
        #[clap(short = 'V', long, value_parser)]
        version: u64,

        #[clap(flatten)]
        common: AssetCommon,
    },

    /// Roll an asset back to a previous version
    Rollback {
        /// Asset ID
        #[clap(short = 'i', long, value_parser)]
        asset_id: u64,

        /// Version number to restore
        #[clap(short = 'V', long, value_parser)]
        version: u64,

        #[clap(flatten)]
        common: AssetCommon,
    },

    /// Grant a user, group, or experience permission to use or edit assets
    GrantPermission {
        /// Asset IDs (repeat or comma-separate)
        #[clap(
            short = 'i',
            long,
            value_parser,
            value_delimiter = ',',
            required = true
        )]
        asset_ids: Vec<u64>,

        /// Type of subject receiving access
        #[clap(short = 't', long, value_enum)]
        subject_type: PermissionSubjectType,

        /// ID of the user, group, or universe receiving access
        #[clap(short = 's', long, value_parser)]
        subject_id: u64,

        /// Permission to grant
        #[clap(short = 'g', long, value_enum, default_value = "use")]
        action: AssetPermissionAction,

        /// Also grant access to the assets' dependencies
        #[clap(short = 'd', long, value_parser, default_value_t = false)]
        dependencies: bool,

        #[clap(flatten)]
        common: AssetCommon,
    },

    /// Show a user's asset upload quotas
    Quotas {
        /// User ID
        #[clap(short = 'U', long, value_parser)]
        user_id: u64,

        /// Filter, e.g. "quotaType == RATE_LIMIT_UPLOAD && assetType == Audio"
        #[clap(short, long, value_parser)]
        filter: Option<String>,

        #[clap(flatten)]
        common: AssetCommon,
    },
}

#[derive(Debug, Args)]
pub struct Assets {
    #[clap(subcommand)]
    command: AssetsCommands,
}

fn create_context_from_creator_type(
    creator_type: CreatorType,
    creator_id: u64,
    expected_price: Option<u64>,
) -> AssetCreationContext {
    let expected_price = expected_price.unwrap_or(0);
    match creator_type {
        CreatorType::User => AssetCreationContext {
            expected_price: Some(expected_price),
            creator: AssetCreator::User(AssetUserCreator {
                user_id: creator_id.to_string(),
            }),
        },
        CreatorType::Group => AssetCreationContext {
            expected_price: Some(expected_price),
            creator: AssetCreator::Group(AssetGroupCreator {
                group_id: creator_id.to_string(),
            }),
        },
    }
}

fn infer_asset_type_from_filepath(filepath: &String) -> Result<AssetType, Error> {
    let path = Path::new(filepath);
    match path.extension() {
        Some(ext) => {
            let ext = ext
                .to_str()
                .ok_or_else(|| Error::InferAssetTypeError(filepath.into()))?;
            AssetType::try_from_extension(ext)
        }
        None => Err(Error::InferAssetTypeError(filepath.into())),
    }
}

impl Assets {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            AssetsCommands::Create {
                asset_type,
                display_name,
                description,
                expected_price,
                creator_id,
                creator_type,
                filepath,
                pretty,
                api_key,
            } => {
                let rbx_cloud = super::v1_client(&api_key)?;
                let assets = rbx_cloud.assets();
                let creation_context =
                    create_context_from_creator_type(creator_type, creator_id, expected_price);
                let asset_type = match asset_type {
                    Some(t) => Ok(t),
                    None => infer_asset_type_from_filepath(&filepath),
                }?;
                let res = assets
                    .create(&CreateAsset {
                        asset: AssetCreation {
                            asset_type,
                            display_name,
                            description,
                            creation_context,
                        },
                        filepath: filepath.clone(),
                    })
                    .await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            AssetsCommands::Update {
                asset_type,
                asset_id,
                filepath,
                pretty,
                api_key,
            } => {
                let rbx_cloud = super::v1_client(&api_key)?;
                let assets = rbx_cloud.assets();
                let asset_type = match asset_type {
                    Some(t) => Ok(t),
                    None => infer_asset_type_from_filepath(&filepath),
                }?;
                let res = assets
                    .update(&UpdateAsset {
                        asset_id,
                        asset_type,
                        filepath,
                    })
                    .await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            AssetsCommands::GetOperation {
                operation_id,
                pretty,
                api_key,
            } => {
                let rbx_cloud = super::v1_client(&api_key)?;
                let assets = rbx_cloud.assets();
                let res = assets
                    .get_operation(&GetAssetOperation { operation_id })
                    .await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            AssetsCommands::Get {
                asset_id,
                read_mask,
                pretty,
                api_key,
            } => {
                let rbx_cloud = super::v1_client(&api_key)?;
                let assets = rbx_cloud.assets();
                let res = assets
                    .get(&GetAsset {
                        asset_id,
                        read_mask,
                    })
                    .await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            AssetsCommands::Archive {
                asset_id,
                pretty,
                api_key,
            } => {
                let rbx_cloud = super::v1_client(&api_key)?;
                let assets = rbx_cloud.assets();
                let res = assets.archive(&ArchiveAsset { asset_id }).await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            AssetsCommands::Restore {
                asset_id,
                pretty,
                api_key,
            } => {
                let rbx_cloud = super::v1_client(&api_key)?;
                let assets = rbx_cloud.assets();
                let res = assets.restore(&ArchiveAsset { asset_id }).await;
                match res {
                    Ok(data) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&data)?
                        } else {
                            serde_json::to_string(&data)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
            AssetsCommands::Versions {
                asset_id,
                max_page_size,
                next_page_token,
                common,
            } => {
                let rbx_cloud = super::v1_client(&common.api_key)?;
                let res = rbx_cloud
                    .assets()
                    .list_versions(asset_id, max_page_size, next_page_token)
                    .await?;
                to_json(&res, common.pretty)
            }

            AssetsCommands::GetVersion {
                asset_id,
                version,
                common,
            } => {
                let rbx_cloud = super::v1_client(&common.api_key)?;
                let res = rbx_cloud.assets().get_version(asset_id, version).await?;
                to_json(&res, common.pretty)
            }

            AssetsCommands::Rollback {
                asset_id,
                version,
                common,
            } => {
                let rbx_cloud = super::v1_client(&common.api_key)?;
                let res = rbx_cloud.assets().rollback(asset_id, version).await?;
                to_json(&res, common.pretty)
            }

            AssetsCommands::GrantPermission {
                asset_ids,
                subject_type,
                subject_id,
                action,
                dependencies,
                common,
            } => {
                let subject = match subject_type {
                    PermissionSubjectType::User => PermissionSubject::User(subject_id),
                    PermissionSubjectType::Group => PermissionSubject::Group(subject_id),
                    PermissionSubjectType::Universe => PermissionSubject::Universe(subject_id),
                };
                let rbx_cloud = super::v1_client(&common.api_key)?;
                let res = rbx_cloud
                    .assets()
                    .grant_permissions(subject, action, &asset_ids, dependencies)
                    .await?;
                to_json(&res, common.pretty)
            }

            AssetsCommands::Quotas {
                user_id,
                filter,
                common,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .user()
                    .list_asset_quotas(RobloxUserId(user_id), filter, None, None)
                    .await?;
                to_json(&res, common.pretty)
            }
        }
    }
}
