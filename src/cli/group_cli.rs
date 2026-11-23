use clap::{Args, Subcommand};
use rbxcloud::rbx::{
    types::{GroupId, RobloxUserId},
    v2::{group::GroupListParams, GroupClient},
};

use super::to_json;

#[derive(Debug, Args)]
pub struct GroupCommon {
    /// Group ID
    #[clap(short, long, value_parser)]
    group_id: u64,

    /// Pretty-print the JSON response
    #[clap(short, long, value_parser, default_value_t = false)]
    pretty: bool,

    /// Roblox Open Cloud API Key
    #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
    api_key: String,
}

#[derive(Debug, Args)]
pub struct GroupPaging {
    /// Max items returned per page
    #[clap(short, long, value_parser)]
    max_page_size: Option<u32>,

    /// Next page token
    #[clap(short, long, value_parser)]
    next_page_token: Option<String>,
}

impl GroupPaging {
    fn params(self, filter: Option<String>) -> GroupListParams {
        GroupListParams {
            max_page_size: self.max_page_size,
            page_token: self.next_page_token,
            filter,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum GroupCommands {
    /// Get info about the group
    Get {
        /// Group ID
        #[clap(short, long, value_parser)]
        group_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// Get the current shout and other metadata
    Shout {
        /// Group ID
        #[clap(short, long, value_parser)]
        group_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Only return the shout message string
        #[clap(short, long, value_parser, default_value_t = false)]
        only_message: bool,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// List the roles of a group
    Roles {
        /// Group ID
        #[clap(short, long, value_parser)]
        group_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Max items returned per page
        #[clap(short, long, value_parser)]
        max_page_size: Option<u32>,

        /// Next page token
        #[clap(short, long, value_parser)]
        next_page_token: Option<String>,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },

    /// List the memberships of a group
    Memberships {
        /// Group ID
        #[clap(short, long, value_parser)]
        group_id: u64,

        /// Pretty-print the JSON response
        #[clap(short, long, value_parser, default_value_t = false)]
        pretty: bool,

        /// Max items returned per page
        #[clap(short, long, value_parser)]
        max_page_size: Option<u32>,

        /// Filter
        #[clap(short, long, value_parser)]
        filter: Option<String>,

        /// Next page token
        #[clap(short, long, value_parser)]
        next_page_token: Option<String>,

        /// Roblox Open Cloud API Key
        #[clap(short, long, value_parser, env = "RBXCLOUD_API_KEY")]
        api_key: String,
    },
    /// List pending requests to join the group
    JoinRequests {
        #[clap(flatten)]
        common: GroupCommon,

        #[clap(flatten)]
        paging: GroupPaging,

        /// Filter, e.g. "user == 'users/123'"
        #[clap(short, long, value_parser)]
        filter: Option<String>,
    },

    /// Accept a join request
    AcceptJoin {
        #[clap(flatten)]
        common: GroupCommon,

        /// Join request ID (the last segment of its `path`)
        #[clap(short, long, value_parser)]
        join_request_id: String,
    },

    /// Decline a join request
    DeclineJoin {
        #[clap(flatten)]
        common: GroupCommon,

        /// Join request ID (the last segment of its `path`)
        #[clap(short, long, value_parser)]
        join_request_id: String,
    },

    /// Get a single role
    Role {
        #[clap(flatten)]
        common: GroupCommon,

        /// Role ID
        #[clap(short, long, value_parser)]
        role_id: u64,
    },

    /// Give a member a role
    AssignRole {
        #[clap(flatten)]
        common: GroupCommon,

        /// User ID of the member
        #[clap(short = 'U', long, value_parser)]
        user_id: u64,

        /// Role ID
        #[clap(short, long, value_parser)]
        role_id: u64,
    },

    /// Remove a role from a member
    UnassignRole {
        #[clap(flatten)]
        common: GroupCommon,

        /// User ID of the member
        #[clap(short = 'U', long, value_parser)]
        user_id: u64,

        /// Role ID
        #[clap(short, long, value_parser)]
        role_id: u64,
    },

    /// List forum categories
    ForumCategories {
        #[clap(flatten)]
        common: GroupCommon,

        #[clap(flatten)]
        paging: GroupPaging,
    },

    /// List posts in a forum category
    ForumPosts {
        #[clap(flatten)]
        common: GroupCommon,

        #[clap(flatten)]
        paging: GroupPaging,

        /// Forum category ID
        #[clap(short, long, value_parser)]
        category_id: String,
    },

    /// List comments on a forum post
    ForumComments {
        #[clap(flatten)]
        common: GroupCommon,

        #[clap(flatten)]
        paging: GroupPaging,

        /// Forum category ID
        #[clap(short, long, value_parser)]
        category_id: String,

        /// Forum post ID
        #[clap(short = 'i', long, value_parser)]
        post_id: String,
    },
}

#[derive(Debug, Args)]
pub struct Group {
    #[clap(subcommand)]
    command: GroupCommands,
}

impl Group {
    pub async fn run(self) -> anyhow::Result<Option<String>> {
        match self.command {
            GroupCommands::Get {
                group_id,
                api_key,
                pretty,
            } => {
                let client = super::v2_client(&api_key)?;
                let group = client.group(GroupId(group_id));
                let res = group.get_info().await;
                match res {
                    Ok(group_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&group_info)?
                        } else {
                            serde_json::to_string(&group_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            GroupCommands::Shout {
                group_id,
                pretty,
                only_message,
                api_key,
            } => {
                let client = super::v2_client(&api_key)?;
                let group = client.group(GroupId(group_id));
                let res = group.get_shout().await;
                match res {
                    Ok(group_info) => {
                        if only_message {
                            return Ok(Some(group_info.content));
                        }
                        let r = if pretty {
                            serde_json::to_string_pretty(&group_info)?
                        } else {
                            serde_json::to_string(&group_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            GroupCommands::Roles {
                group_id,
                api_key,
                pretty,
                max_page_size,
                next_page_token,
            } => {
                let client = super::v2_client(&api_key)?;
                let group = client.group(GroupId(group_id));
                let res = group.list_roles(max_page_size, next_page_token).await;
                match res {
                    Ok(group_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&group_info)?
                        } else {
                            serde_json::to_string(&group_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }

            GroupCommands::Memberships {
                group_id,
                api_key,
                pretty,
                max_page_size,
                next_page_token,
                filter,
            } => {
                let client = super::v2_client(&api_key)?;
                let group = client.group(GroupId(group_id));
                let res = group
                    .list_memberships(max_page_size, filter, next_page_token)
                    .await;
                match res {
                    Ok(group_info) => {
                        let r = if pretty {
                            serde_json::to_string_pretty(&group_info)?
                        } else {
                            serde_json::to_string(&group_info)?
                        };
                        Ok(Some(r))
                    }
                    Err(err) => Err(anyhow::anyhow!(err)),
                }
            }
            GroupCommands::JoinRequests {
                common,
                paging,
                filter,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .group(GroupId(common.group_id))
                    .list_join_requests(&paging.params(filter))
                    .await?;
                to_json(&res, common.pretty)
            }

            GroupCommands::AcceptJoin {
                common,
                join_request_id,
            } => {
                let client = super::v2_client(&common.api_key)?;
                client
                    .group(GroupId(common.group_id))
                    .accept_join_request(&join_request_id)
                    .await?;
                Ok(Some(format!("accepted join request {join_request_id}")))
            }

            GroupCommands::DeclineJoin {
                common,
                join_request_id,
            } => {
                let client = super::v2_client(&common.api_key)?;
                client
                    .group(GroupId(common.group_id))
                    .decline_join_request(&join_request_id)
                    .await?;
                Ok(Some(format!("declined join request {join_request_id}")))
            }

            GroupCommands::Role { common, role_id } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .group(GroupId(common.group_id))
                    .get_role(role_id)
                    .await?;
                to_json(&res, common.pretty)
            }

            GroupCommands::AssignRole {
                common,
                user_id,
                role_id,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let group = client.group(GroupId(common.group_id));
                let membership_id = membership_id(&group, user_id).await?;
                let res = group.assign_role(&membership_id, role_id).await?;
                to_json(&res, common.pretty)
            }

            GroupCommands::UnassignRole {
                common,
                user_id,
                role_id,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let group = client.group(GroupId(common.group_id));
                let membership_id = membership_id(&group, user_id).await?;
                let res = group.unassign_role(&membership_id, role_id).await?;
                to_json(&res, common.pretty)
            }

            GroupCommands::ForumCategories { common, paging } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .group(GroupId(common.group_id))
                    .list_forum_categories(&paging.params(None))
                    .await?;
                to_json(&res, common.pretty)
            }

            GroupCommands::ForumPosts {
                common,
                paging,
                category_id,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .group(GroupId(common.group_id))
                    .list_forum_posts(&category_id, &paging.params(None))
                    .await?;
                to_json(&res, common.pretty)
            }

            GroupCommands::ForumComments {
                common,
                paging,
                category_id,
                post_id,
            } => {
                let client = super::v2_client(&common.api_key)?;
                let res = client
                    .group(GroupId(common.group_id))
                    .list_forum_comments(&category_id, &post_id, &paging.params(None))
                    .await?;
                to_json(&res, common.pretty)
            }
        }
    }
}

/// Look up a member's membership ID from their user ID.
async fn membership_id(group: &GroupClient, user_id: u64) -> anyhow::Result<String> {
    let membership = group
        .find_membership(RobloxUserId(user_id))
        .await?
        .ok_or_else(|| anyhow::anyhow!("user {user_id} is not a member of this group"))?;
    Ok(membership
        .path
        .rsplit('/')
        .next()
        .unwrap_or_default()
        .to_string())
}
