use serde::{Deserialize, Serialize};

use serde_json::{Map, Value};

use crate::rbx::{
    error::Error,
    types::GroupId,
    util::{encode_path_segment as seg, QueryString},
};

use crate::rbx::http::{self, HttpClient};

impl std::fmt::Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct GetGroupParams {
    pub group_id: GroupId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupResponse {
    pub path: String,
    pub create_time: String,
    pub update_time: String,
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub owner: Option<String>,
    pub member_count: u64,
    pub public_entry_allowed: bool,
    pub locked: bool,
    pub verified: bool,
}

pub struct GetGroupShoutParams {
    pub group_id: GroupId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetGroupShoutResponse {
    pub path: String,
    pub create_time: String,
    pub update_time: String,
    pub content: String,
    pub poster: Option<String>,
}

pub struct ListGroupRolesParams {
    pub group_id: GroupId,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
}

/// Permissions of a group role. Missing permissions default to `false`.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct GroupRolePermission {
    pub view_wall_posts: bool,
    pub create_wall_posts: bool,
    pub delete_wall_posts: bool,
    pub view_group_shout: bool,
    pub create_group_shout: bool,
    pub change_rank: bool,
    pub accept_requests: bool,
    pub exile_members: bool,
    pub manage_relationships: bool,
    pub view_audit_log: bool,
    pub spend_group_funds: bool,
    pub advertise_group: bool,
    pub create_avatar_items: bool,
    pub manage_avatar_items: bool,
    pub manage_group_universes: bool,
    pub view_universe_analytics: bool,
    pub create_api_keys: bool,
    pub manage_api_keys: bool,
    pub ban_members: bool,
    pub view_forums: bool,
    pub manage_categories: bool,
    pub create_posts: bool,
    pub lock_posts: bool,
    pub pin_posts: bool,
    pub remove_posts: bool,
    pub create_comments: bool,
    pub remove_comments: bool,
    pub manage_blocked_words: bool,
    pub view_blocked_words: bool,
    pub bypass_slow_mode: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupRole {
    pub path: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub rank: u32,
    pub member_count: Option<u64>,
    pub permissions: Option<GroupRolePermission>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupRolesResponse {
    pub group_roles: Vec<GroupRole>,
    pub next_page_token: Option<String>,
}

pub struct ListGroupMembershipsParams {
    pub group_id: GroupId,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub filter: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembership {
    pub path: String,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub update_time: String,
    pub user: String,
    /// Primary role. Prefer `roles`, which lists every role the member holds.
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub roles: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupMembershipsResponse {
    pub group_memberships: Vec<GroupMembership>,
    pub next_page_token: Option<String>,
}

pub async fn get_group(
    http: &HttpClient,
    params: &GetGroupParams,
) -> Result<GetGroupResponse, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/groups/{groupId}",
        groupId = params.group_id,
    );

    let res = http.get(url).send().await?;

    http::json::<GetGroupResponse>(res).await
}

pub async fn get_group_shout(
    http: &HttpClient,
    params: &GetGroupShoutParams,
) -> Result<GetGroupShoutResponse, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/groups/{groupId}/shout",
        groupId = params.group_id,
    );

    let res = http.get(url).send().await?;

    http::json::<GetGroupShoutResponse>(res).await
}

pub async fn list_group_roles(
    http: &HttpClient,
    params: &ListGroupRolesParams,
) -> Result<ListGroupRolesResponse, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/groups/{groupId}/roles",
        groupId = params.group_id,
    );

    let mut query: QueryString = vec![];
    if let Some(max_page_size) = &params.max_page_size {
        query.push(("maxPageSize", max_page_size.to_string()))
    }
    if let Some(page_token) = &params.page_token {
        query.push(("pageToken", page_token.clone()));
    }

    let res = http.get(url).query(&query).send().await?;

    http::json::<ListGroupRolesResponse>(res).await
}

pub async fn list_group_memberships(
    http: &HttpClient,
    params: &ListGroupMembershipsParams,
) -> Result<ListGroupMembershipsResponse, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/groups/{groupId}/memberships",
        groupId = params.group_id,
    );

    let mut query: QueryString = vec![];
    if let Some(max_page_size) = &params.max_page_size {
        query.push(("maxPageSize", max_page_size.to_string()))
    }
    if let Some(page_token) = &params.page_token {
        query.push(("pageToken", page_token.clone()));
    }
    if let Some(filter) = &params.filter {
        query.push(("filter", filter.clone()));
    }

    let res = http.get(url).query(&query).send().await?;

    http::json::<ListGroupMembershipsResponse>(res).await
}

// ------------------------------------------------------------ join requests

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupJoinRequest {
    pub path: String,
    #[serde(default)]
    pub create_time: String,
    /// User resource path (`users/{id}`).
    #[serde(default)]
    pub user: String,
}

impl GroupJoinRequest {
    /// The join request ID (last segment of `path`).
    pub fn id(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or_default()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupJoinRequestsResponse {
    #[serde(default)]
    pub group_join_requests: Vec<GroupJoinRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// Paging and filtering for group list methods.
#[derive(Debug, Clone, Default)]
pub struct GroupListParams {
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub filter: Option<String>,
}

fn list_query(params: &GroupListParams) -> QueryString {
    let mut query: QueryString = vec![];
    if let Some(n) = params.max_page_size {
        query.push(("maxPageSize", n.to_string()));
    }
    if let Some(token) = &params.page_token {
        query.push(("pageToken", token.clone()));
    }
    if let Some(filter) = &params.filter {
        query.push(("filter", filter.clone()));
    }
    query
}

fn group_url(group_id: GroupId, rest: &str) -> String {
    http::url(&format!("/cloud/v2/groups/{group_id}{rest}"))
}

/// List pending requests to join a group. Filter by user with
/// `user == 'users/123'`.
pub async fn list_join_requests(
    http: &HttpClient,
    group_id: GroupId,
    params: &GroupListParams,
) -> Result<ListGroupJoinRequestsResponse, Error> {
    let res = http
        .get(group_url(group_id, "/join-requests"))
        .query(&list_query(params))
        .send()
        .await?;
    http::json(res).await
}

/// Accept a join request.
pub async fn accept_join_request(
    http: &HttpClient,
    group_id: GroupId,
    join_request_id: &str,
) -> Result<(), Error> {
    let url = group_url(
        group_id,
        &format!("/join-requests/{}:accept", seg(join_request_id)),
    );
    let res = http.post(url).json(&Map::new()).send().await?;
    http::empty(res).await
}

/// Decline a join request.
pub async fn decline_join_request(
    http: &HttpClient,
    group_id: GroupId,
    join_request_id: &str,
) -> Result<(), Error> {
    let url = group_url(
        group_id,
        &format!("/join-requests/{}:decline", seg(join_request_id)),
    );
    let res = http.post(url).json(&Map::new()).send().await?;
    http::empty(res).await
}

// -------------------------------------------------------------- roles

/// Get a single role.
pub async fn get_role(
    http: &HttpClient,
    group_id: GroupId,
    role_id: u64,
) -> Result<GroupRole, Error> {
    let res = http
        .get(group_url(group_id, &format!("/roles/{role_id}")))
        .send()
        .await?;
    http::json(res).await
}

#[derive(Serialize)]
struct RoleBody {
    role: String,
}

async fn change_role(
    http: &HttpClient,
    group_id: GroupId,
    membership_id: &str,
    role_id: u64,
    method: &str,
) -> Result<GroupMembership, Error> {
    let url = group_url(
        group_id,
        &format!("/memberships/{}:{method}", seg(membership_id)),
    );
    let body = RoleBody {
        role: format!("groups/{group_id}/roles/{role_id}"),
    };
    let res = http.post(url).json(&body).send().await?;
    http::json(res).await
}

/// Give a member an additional role.
///
/// `membership_id` is the last segment of a [`GroupMembership`]'s `path`; use
/// [`list_group_memberships`] with the filter `user == 'users/123'` to find it.
pub async fn assign_role(
    http: &HttpClient,
    group_id: GroupId,
    membership_id: &str,
    role_id: u64,
) -> Result<GroupMembership, Error> {
    change_role(http, group_id, membership_id, role_id, "assignRole").await
}

/// Remove a role from a member.
pub async fn unassign_role(
    http: &HttpClient,
    group_id: GroupId,
    membership_id: &str,
    role_id: u64,
) -> Result<GroupMembership, Error> {
    change_role(http, group_id, membership_id, role_id, "unassignRole").await
}

// -------------------------------------------------------------- forums

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupForumCategory {
    pub path: String,
    #[serde(default)]
    pub group_forum_category_id: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archiver: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContentMessage {
    #[serde(default)]
    pub plain_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub message_reaction: Vec<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupForumComment {
    pub path: String,
    #[serde(default)]
    pub group_forum_comment_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<ContentMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupForumPost {
    pub path: String,
    #[serde(default)]
    pub group_forum_post_id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub comment_count: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_comment: Option<GroupForumComment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archiver: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupForumCategoriesResponse {
    #[serde(default)]
    pub group_forum_categories: Vec<GroupForumCategory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupForumPostsResponse {
    #[serde(default)]
    pub group_forum_posts: Vec<GroupForumPost>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListGroupForumCommentsResponse {
    #[serde(default)]
    pub group_forum_comments: Vec<GroupForumComment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// List a group's forum categories.
pub async fn list_forum_categories(
    http: &HttpClient,
    group_id: GroupId,
    params: &GroupListParams,
) -> Result<ListGroupForumCategoriesResponse, Error> {
    let res = http
        .get(group_url(group_id, "/forum-categories"))
        .query(&list_query(params))
        .send()
        .await?;
    http::json(res).await
}

/// List posts in a forum category.
pub async fn list_forum_posts(
    http: &HttpClient,
    group_id: GroupId,
    category_id: &str,
    params: &GroupListParams,
) -> Result<ListGroupForumPostsResponse, Error> {
    let url = group_url(
        group_id,
        &format!("/forum-categories/{}/posts", seg(category_id)),
    );
    let res = http.get(url).query(&list_query(params)).send().await?;
    http::json(res).await
}

/// List comments on a forum post.
pub async fn list_forum_comments(
    http: &HttpClient,
    group_id: GroupId,
    category_id: &str,
    post_id: &str,
    params: &GroupListParams,
) -> Result<ListGroupForumCommentsResponse, Error> {
    let url = group_url(
        group_id,
        &format!(
            "/forum-categories/{}/posts/{}/comments",
            seg(category_id),
            seg(post_id)
        ),
    );
    let res = http.get(url).query(&list_query(params)).send().await?;
    http::json(res).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn decode_role_with_partial_permissions() {
        let role: GroupRole = serde_json::from_value(json!({
            "path": "groups/1/roles/2",
            "id": "2",
            "displayName": "Member",
            "rank": 1,
            "permissions": {"viewWallPosts": true, "viewForums": true}
        }))
        .unwrap();
        let perms = role.permissions.unwrap();
        assert!(perms.view_wall_posts && perms.view_forums);
        assert!(!perms.ban_members);
    }

    #[test]
    fn decode_join_requests() {
        let res: ListGroupJoinRequestsResponse = serde_json::from_value(json!({
            "groupJoinRequests": [{
                "path": "groups/1/join-requests/156",
                "createTime": "2024-01-01T00:00:00Z",
                "user": "users/156"
            }]
        }))
        .unwrap();
        assert_eq!(res.group_join_requests[0].id(), "156");
        assert!(res.next_page_token.is_none());
    }

    #[test]
    fn decode_forum_post() {
        let res: ListGroupForumPostsResponse = serde_json::from_value(json!({
            "groupForumPosts": [{
                "path": "groups/1/forum-categories/c/posts/p",
                "groupForumPostId": "p",
                "title": "Hello",
                "commentCount": 3,
                "firstComment": {
                    "path": "groups/1/forum-categories/c/posts/p/comments/x",
                    "groupForumCommentId": "x",
                    "message": {"plainText": "First!", "author": "users/1"}
                }
            }],
            "nextPageToken": "t"
        }))
        .unwrap();
        let post = &res.group_forum_posts[0];
        assert_eq!(post.comment_count, 3);
        assert_eq!(
            post.first_comment
                .as_ref()
                .unwrap()
                .message
                .as_ref()
                .unwrap()
                .plain_text,
            "First!"
        );
    }
}
