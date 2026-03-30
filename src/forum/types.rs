//! Auto-generated types for Lolzteam Public API: Forum.
//! DO NOT EDIT -- run `cargo run --bin lzt-codegen -- generate`.

#![allow(clippy::all)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Query parameters for `Categories.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_category_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_forum_id: Option<i64>,
}


/// Query parameters for `Chatbox.Index`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxIndexQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<ForumRoomIdmodel>,
}


/// Query parameters for `Chatbox.GetMessages`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetMessagesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_message_id: Option<i64>,
    pub room_id: ForumRoomIdmodel,
}


/// Query parameters for `Chatbox.GetLeaderboard`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetLeaderboardQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}


/// Query parameters for `Chatbox.Online`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxOnlineQuery {
    pub room_id: ForumRoomIdmodel,
}


/// Query parameters for `Chatbox.ReportReasons`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxReportReasonsQuery {
    pub message_id: i64,
}


/// Query parameters for `Conversations.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


/// Query parameters for `Conversations.Messages.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


/// Query parameters for `Assets.Css`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AssetsCssQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<Vec<String>>,
}


/// Query parameters for `Forms.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


/// Query parameters for `Forums.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_category_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_forum_id: Option<i64>,
}


/// Query parameters for `Forums.Followed`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowedQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<bool>,
}


/// Query parameters for `Navigation.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NavigationListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<i64>,
}


/// Query parameters for `Notifications.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NotificationsListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}


/// Query parameters for `Pages.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_page_id: Option<i64>,
}


/// Query parameters for `Posts.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_of_post_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i64>,
}


/// Query parameters for `Posts.Comments.Get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsGetQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_comment: Option<i64>,
    pub post_id: i64,
}


/// Query parameters for `Posts.Likes`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsLikesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


/// Query parameters for `ProfilePosts.Comments.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    pub profile_post_id: i64,
}


/// Query parameters for `ProfilePosts.Delete`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsDeleteQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}


/// Query parameters for `Tags.Find`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsFindQuery {
    pub tag: String,
}


/// Query parameters for `Tags.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


/// Query parameters for `Tags.Get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


/// Query parameters for `Threads.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prefix_ids[]")]
    pub prefix_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prefix_ids_not[]")]
    pub prefix_ids_not: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_create_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_tag_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_update_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_only: Option<bool>,
}


/// Query parameters for `Threads.Followed`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<bool>,
}


/// Query parameters for `Threads.Unread`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}


/// Query parameters for `Threads.Recent`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}


/// Query parameters for `Threads.Get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsGetQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_include: Option<Vec<String>>,
}


/// Query parameters for `Users.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


/// Query parameters for `Users.Find`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFindQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}


/// Query parameters for `Users.Ignored`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersIgnoredQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<bool>,
}


/// Query parameters for `Users.Get`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersGetQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_include: Option<Vec<String>>,
}


/// Query parameters for `Users.Claims`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersClaimsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}


/// Query parameters for `Users.Followers`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


/// Query parameters for `Users.Followings`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowingsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


/// Query parameters for `Users.IgnoreEdit`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersIgnoreEditQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_conversations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_view_profile: Option<bool>,
}


/// Query parameters for `Users.Likes`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersLikesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}


/// Query parameters for `ProfilePosts.List`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts_user_id: Option<i64>,
}


/// Query parameters for `Users.Contents`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AssetsCssResponse {
    pub contents: String,
    pub system_info: ForumRespSystemInfo,
}


pub type BatchExecuteRequest = Vec<BatchExecuteRequestItem>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchExecuteRequestItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<BTreeMap<String, String>>,
    pub uri: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchExecuteResponse {
    pub jobs: BatchExecuteResponseJobs,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchExecuteResponseJobs {
    pub job_id: BatchExecuteResponseJobsJobId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BatchExecuteResponseJobsJobId {
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesGetResponse {
    pub category: CategoriesGetResponseCategory,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesGetResponseCategory {
    pub category_description: String,
    pub category_id: i64,
    pub category_title: String,
    pub links: CategoriesGetResponseCategoryLinks,
    pub permissions: CategoriesGetResponseCategoryPermissions,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesGetResponseCategoryLinks {
    pub detail: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesGetResponseCategoryPermissions {
    pub delete: bool,
    pub edit: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesListResponse {
    pub categories: Vec<CategoriesListResponseCategoriesItem>,
    pub categories_total: i64,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesListResponseCategoriesItem {
    pub category_description: String,
    pub category_id: i64,
    pub category_title: String,
    pub links: CategoriesListResponseCategoriesItemLinks,
    pub permissions: CategoriesListResponseCategoriesItemPermissions,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesListResponseCategoriesItemLinks {
    pub detail: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CategoriesListResponseCategoriesItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxDeleteIgnoreRequest {
    pub user_id: ForumUserIdmodel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxDeleteMessageRequest {
    pub message_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxEditMessageRequest {
    pub message: String,
    pub message_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxEditMessageResponse {
    pub message: ForumRespChatboxMessageModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetIgnoreResponse {
    pub ignored: Vec<ChatboxGetIgnoreResponseIgnoredItem>,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetIgnoreResponseIgnoredItem {
    pub avatar_date: i64,
    pub background_date: i64,
    pub contest_count: i64,
    pub custom_title: String,
    pub display_banner_id: i64,
    pub display_icon_group_id: i64,
    pub display_style_group_id: i64,
    pub is_banned: bool,
    pub last_activity: i64,
    pub like2_count: i64,
    pub like_count: i64,
    pub message_count: i64,
    pub register_date: i64,
    pub rendered: ChatboxGetIgnoreResponseIgnoredItemRendered,
    pub short_link: serde_json::Value,
    pub trophy_points: i64,
    pub uniq_banner: serde_json::Value,
    pub uniq_username_css: String,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetIgnoreResponseIgnoredItemRendered {
    pub avatars: ChatboxGetIgnoreResponseIgnoredItemRenderedAvatars,
    pub link: String,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetIgnoreResponseIgnoredItemRenderedAvatars {
    pub l: String,
    pub m: String,
    pub s: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetLeaderboardResponse {
    pub leaderboard: Vec<ChatboxGetLeaderboardResponseLeaderboardItem>,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetLeaderboardResponseLeaderboardItem {
    pub avatar_date: i64,
    pub background_date: i64,
    pub contest_count: i64,
    pub count: i64,
    pub custom_title: String,
    pub display_banner_id: i64,
    pub display_icon_group_id: i64,
    pub display_style_group_id: i64,
    pub is_banned: bool,
    pub last_activity: i64,
    pub like2_count: i64,
    pub like_count: i64,
    pub message_count: i64,
    pub register_date: i64,
    pub rendered: ChatboxGetLeaderboardResponseLeaderboardItemRendered,
    pub short_link: serde_json::Value,
    pub trophy_points: i64,
    pub uniq_banner: ChatboxGetLeaderboardResponseLeaderboardItemUniqBanner,
    pub uniq_username_css: String,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetLeaderboardResponseLeaderboardItemRendered {
    pub avatars: ChatboxGetLeaderboardResponseLeaderboardItemRenderedAvatars,
    pub link: String,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetLeaderboardResponseLeaderboardItemRenderedAvatars {
    pub l: String,
    pub m: String,
    pub s: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetLeaderboardResponseLeaderboardItemUniqBanner {
    pub banner_css: String,
    pub banner_icon: String,
    pub banner_text: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxGetMessagesResponse {
    pub messages: Vec<ForumRespChatboxMessageModel>,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxIndexResponse {
    pub ban: serde_json::Value,
    pub commands: Vec<String>,
    pub ignore: Vec<ChatboxIndexResponseIgnoreItem>,
    pub permissions: ChatboxIndexResponsePermissions,
    pub rooms: Vec<ChatboxIndexResponseRoomsItem>,
    #[serde(rename = "roomsOnline")]
    pub rooms_online: ChatboxIndexResponseRoomsOnline,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxIndexResponseIgnoreItem {
    pub avatar_date: i64,
    pub background_date: i64,
    pub contest_count: i64,
    pub custom_title: String,
    pub display_banner_id: i64,
    pub display_icon_group_id: i64,
    pub display_style_group_id: i64,
    pub is_admin: bool,
    pub is_banned: bool,
    pub is_moderator: bool,
    pub is_staff: bool,
    pub last_activity: i64,
    pub like2_count: i64,
    pub like_count: i64,
    pub message_count: i64,
    pub register_date: i64,
    pub rendered: ChatboxIndexResponseIgnoreItemRendered,
    pub short_link: serde_json::Value,
    pub trophy_points: i64,
    pub uniq_banner: serde_json::Value,
    pub uniq_username_css: String,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxIndexResponseIgnoreItemRendered {
    pub avatars: ChatboxIndexResponseIgnoreItemRenderedAvatars,
    pub link: String,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxIndexResponseIgnoreItemRenderedAvatars {
    pub l: String,
    pub m: String,
    pub s: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxIndexResponsePermissions {
    pub ban: bool,
    #[serde(rename = "deleteAnyMessage")]
    pub delete_any_message: bool,
    #[serde(rename = "editAnyMessage")]
    pub edit_any_message: bool,
    #[serde(rename = "postMessage")]
    pub post_message: bool,
    #[serde(rename = "viewAnyMessage")]
    pub view_any_message: bool,
    #[serde(rename = "viewMessages")]
    pub view_messages: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxIndexResponseRoomsItem {
    pub can_report: bool,
    pub eng: bool,
    pub market: bool,
    pub room_id: i64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxIndexResponseRoomsOnline {
    #[serde(rename = "chat:0")]
    pub chat_0: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxOnlineResponse {
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<ChatboxOnlineResponseUsersItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxOnlineResponseUsersItem {
    pub avatar_date: i64,
    pub background_date: i64,
    pub contest_count: i64,
    pub custom_title: String,
    pub display_banner_id: i64,
    pub display_icon_group_id: i64,
    pub display_style_group_id: i64,
    pub is_admin: bool,
    pub is_banned: bool,
    pub is_moderator: bool,
    pub is_staff: bool,
    pub last_activity: i64,
    pub like2_count: i64,
    pub like_count: i64,
    pub message_count: i64,
    pub register_date: i64,
    pub rendered: ChatboxOnlineResponseUsersItemRendered,
    pub short_link: String,
    pub trophy_points: i64,
    pub uniq_banner: ChatboxOnlineResponseUsersItemUniqBanner,
    pub uniq_username_css: String,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxOnlineResponseUsersItemRendered {
    pub avatars: ChatboxOnlineResponseUsersItemRenderedAvatars,
    pub link: String,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxOnlineResponseUsersItemRenderedAvatars {
    pub l: String,
    pub m: String,
    pub s: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxOnlineResponseUsersItemUniqBanner {
    pub banner_css: String,
    pub banner_icon: String,
    pub banner_text: String,
    pub username_icon: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxPostIgnoreRequest {
    pub user_id: ForumUserIdmodel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxPostMessageRequest {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_message_id: Option<i64>,
    pub room_id: ForumRoomIdmodel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxPostMessageResponse {
    pub message: ForumRespChatboxMessageModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxReportReasonsResponse {
    pub reasons: Vec<String>,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChatboxReportRequest {
    pub message_id: i64,
    pub reason: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsAlertsDisableResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsAlertsEnableResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_delete_own_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sticky_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_group: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_invite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsCreateResponse {
    pub conversation: ForumRespConversationModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsDeleteRequest {
    pub conversation_id: i64,
    pub delete_type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsGetResponse {
    pub conversation: ForumRespConversationModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsInviteRequest {
    pub recipients: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsKickRequest {
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsListResponse {
    pub can_start: bool,
    pub conversations: Vec<ForumRespConversationModel>,
    pub folders: Vec<ConversationsListResponseFoldersItem>,
    pub links: ConversationsListResponseLinks,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsListResponseFoldersItem {
    pub id: String,
    pub name: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsListResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesCreateRequest {
    pub message_body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_message_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesCreateResponse {
    pub message: ForumRespConversationMessageModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesEditRequest {
    pub message_body: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesEditResponse {
    pub message: ForumRespConversationModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesGetResponse {
    pub message: ForumRespConversationModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesListResponse {
    pub links: ConversationsMessagesListResponseLinks,
    pub messages: Vec<ForumRespConversationMessageModel>,
    pub messages_total: i64,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsMessagesListResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsReadAllResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsSaveRequest {
    pub link: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsSearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_recipients: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsSearchResponse {
    pub conversations: Vec<ForumRespConversationModel>,
    pub recipients: bool,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsStarResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsStartRequest {
    pub user_id: ForumUserIdmodel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsStartResponse {
    pub conversation: ForumRespConversationModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsUnstarResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_delete_own_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sticky_messages: Option<bool>,
    pub conversation_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_invite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ConversationsUpdateResponse {
    pub conversation: ForumRespConversationModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsCreateRequest {
    pub fields: FormsCreateRequestFields,
    pub form_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsCreateRequestFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "22")]
    pub field_22: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "23")]
    pub field_23: Option<String>,
    #[serde(rename = "24")]
    pub field_24: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "27")]
    pub field_27: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "28")]
    pub field_28: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "29")]
    pub field_29: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "30")]
    pub field_30: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsCreateResponse {
    pub content: FormsCreateResponseContent,
    pub message: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsCreateResponseContent {
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub forum_id: i64,
    pub links: FormsCreateResponseContentLinks,
    pub node_title: String,
    pub permissions: FormsCreateResponseContentPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_closed: bool,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: Vec<serde_json::Value>,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsCreateResponseContentLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsCreateResponseContentPermissions {
    pub delete: bool,
    pub follow: bool,
    pub post: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsListResponse {
    pub forms: Vec<FormsListResponseFormsItem>,
    #[serde(rename = "formsPerPage")]
    pub forms_per_page: i64,
    pub page: i64,
    pub system_info: ForumRespSystemInfo,
    #[serde(rename = "totalForms")]
    pub total_forms: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsListResponseFormsItem {
    pub description: String,
    pub fields: Vec<FormsListResponseFormsItemFieldsItem>,
    pub form_id: i64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsListResponseFormsItemFieldsItem {
    pub default_value: String,
    #[serde(rename = "fieldChoices")]
    pub field_choices: FormsListResponseFormsItemFieldsItemFieldChoices,
    pub field_id: i64,
    pub max_length: i64,
    pub required: i64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct FormsListResponseFormsItemFieldsItemFieldChoices {
    pub buy: String,
    pub sell: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespChatboxMessageModel {
    pub can_report: bool,
    pub date: i64,
    pub is_deleted: bool,
    pub message: String,
    #[serde(rename = "messageJson")]
    pub message_json: String,
    #[serde(rename = "messageRaw")]
    pub message_raw: String,
    pub message_id: i64,
    pub room: ForumRespChatboxMessageModelRoom,
    pub user: ForumRespChatboxMessageModelUser,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespChatboxMessageModelRoom {
    pub can_report: bool,
    pub eng: bool,
    pub market: bool,
    pub room_id: i64,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespChatboxMessageModelUser {
    pub avatar_date: i64,
    pub background_date: i64,
    pub contest_count: i64,
    pub custom_title: String,
    pub display_banner_id: i64,
    pub display_icon_group_id: i64,
    pub display_style_group_id: i64,
    pub is_admin: bool,
    pub is_banned: bool,
    pub is_moderator: bool,
    pub is_staff: bool,
    pub last_activity: i64,
    pub like2_count: i64,
    pub like_count: i64,
    pub message_count: i64,
    pub register_date: i64,
    pub rendered: ForumRespChatboxMessageModelUserRendered,
    pub short_link: String,
    pub trophy_points: i64,
    pub uniq_banner: ForumRespChatboxMessageModelUserUniqBanner,
    pub uniq_username_css: String,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespChatboxMessageModelUserRendered {
    pub avatars: ForumRespChatboxMessageModelUserRenderedAvatars,
    pub link: String,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespChatboxMessageModelUserRenderedAvatars {
    pub l: String,
    pub m: String,
    pub s: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespChatboxMessageModelUserUniqBanner {
    pub banner_css: String,
    pub banner_icon: String,
    pub banner_text: String,
    pub username_icon: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespConversationMessageModel {
    pub conversation_id: i64,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub links: ForumRespConversationMessageModelLinks,
    pub message_body: String,
    pub message_body_html: String,
    pub message_body_plain_text: String,
    pub message_create_date: i64,
    pub message_edit_date: i64,
    pub message_id: i64,
    pub message_is_system: bool,
    pub message_is_unread: i64,
    pub message_need_translate: bool,
    pub permissions: ForumRespConversationMessageModelPermissions,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespConversationMessageModelLinks {
    pub conversation: String,
    pub creator: String,
    pub creator_avatar: String,
    pub detail: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespConversationMessageModelPermissions {
    pub delete: bool,
    pub edit: bool,
    #[serde(rename = "stick-unstick")]
    pub stick_unstick: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespConversationModel {
    pub alerts: i64,
    pub conversation_create_date: i64,
    pub conversation_id: i64,
    pub conversation_is_deleted: bool,
    pub conversation_is_new: bool,
    pub conversation_is_open: bool,
    pub conversation_last_read_date: i64,
    pub conversation_message_count: i64,
    pub conversation_online_count: i64,
    pub conversation_title: String,
    pub conversation_update_date: i64,
    pub creator_is_ignored: bool,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub is_group: i64,
    pub is_starred: i64,
    pub is_unread: i64,
    pub links: ForumRespConversationModelLinks,
    pub permissions: ForumRespConversationModelPermissions,
    pub recipient: ForumRespConversationModelRecipient,
    pub recipients: Vec<ForumRespConversationModelRecipientsItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespConversationModelLinks {
    pub avatar: String,
    pub detail: String,
    pub messages: String,
    pub permalink: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespConversationModelPermissions {
    #[serde(rename = "editOwnPost")]
    pub edit_own_post: bool,
    pub invite: bool,
    pub kick: bool,
    pub manage_invite_links: bool,
    pub reply: bool,
    #[serde(rename = "stickyMessages")]
    pub sticky_messages: bool,
    pub upload_avatar: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespConversationModelRecipient {
    pub avatar: String,
    pub contacts_changed: bool,
    pub is_online: bool,
    pub last_activity: i64,
    pub user_id: i64,
    pub username: String,
    pub username_html: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespConversationModelRecipientsItem {
    pub avatar: String,
    pub contacts_changed: bool,
    pub is_online: bool,
    pub last_activity: i64,
    pub user_id: i64,
    pub username: String,
    pub username_html: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespLinkModel {
    pub link_description: String,
    pub link_id: i64,
    pub link_title: String,
    pub links: ForumRespLinkModelLinks,
    pub permissions: ForumRespLinkModelPermissions,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespLinkModelLinks {
    pub detail: String,
    pub target: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespLinkModelPermissions {
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespNotificationModel {
    pub content_action: String,
    pub content_id: i64,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub links: ForumRespNotificationModelLinks,
    pub notification_create_date: i64,
    pub notification_html: String,
    pub notification_id: i64,
    pub notification_is_unread: bool,
    pub notification_type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespNotificationModelLinks {
    pub content: String,
    pub creator_avatar: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespPostCommentModel {
    pub links: ForumRespPostCommentModelLinks,
    pub permissions: ForumRespPostCommentModelPermissions,
    pub post_comment_body: String,
    pub post_comment_body_html: String,
    pub post_comment_body_plain_text: String,
    pub post_comment_create_date: i64,
    pub post_comment_id: i64,
    pub post_comment_is_deleted: bool,
    pub post_comment_is_published: bool,
    pub post_comment_like_count: i64,
    pub post_comment_update_date: i64,
    pub post_id: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespPostCommentModelLinks {
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub post: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespPostCommentModelPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespPostModel {
    pub links: ForumRespPostModelLinks,
    pub permissions: ForumRespPostModelPermissions,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespPostModelLinks {
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespPostModelPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespProfilePostCommentModel {
    pub comment_body: String,
    pub comment_body_html: String,
    pub comment_body_plain_text: String,
    pub comment_create_date: i64,
    pub comment_id: i64,
    pub comment_user_id: i64,
    pub comment_username: String,
    pub comment_username_html: String,
    pub links: ForumRespProfilePostCommentModelLinks,
    pub permissions: ForumRespProfilePostCommentModelPermissions,
    pub profile_post_id: i64,
    pub timeline_user_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespProfilePostCommentModelLinks {
    pub detail: String,
    pub poster: String,
    pub poster_avatar: String,
    pub profile_post: String,
    pub timeline: String,
    pub timeline_user: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespProfilePostCommentModelPermissions {
    pub delete: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespProfilePostModel {
    pub links: ForumRespProfilePostModelLinks,
    pub permissions: ForumRespProfilePostModelPermissions,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_comment_count: i64,
    pub post_comments_is_disabled: i64,
    pub post_create_date: i64,
    pub post_is_deleted: bool,
    pub post_is_liked: bool,
    pub post_is_published: bool,
    pub post_is_sticked: bool,
    pub post_like_count: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub profile_post_id: i64,
    pub timeline_user: ForumRespUserModel,
    pub timeline_user_id: i64,
    pub timeline_username: String,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespProfilePostModelLinks {
    pub comments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub timeline: String,
    pub timeline_user: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespProfilePostModelPermissions {
    pub comment: bool,
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub report: bool,
    pub stick: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespSystemInfo {
    pub time: i64,
    pub visitor_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModel {
    pub contest: ForumRespThreadModelContest,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: ForumRespThreadModelFirstPost,
    pub forum_id: i64,
    pub last_post: ForumRespThreadModelLastPost,
    pub links: ForumRespThreadModelLinks,
    pub node_title: String,
    pub permissions: ForumRespThreadModelPermissions,
    pub restrictions: ForumRespThreadModelRestrictions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_closed: bool,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_starred: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: ForumRespThreadModelThreadTags,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelContest {
    pub already_participate: bool,
    pub chance_to_win: f64,
    pub count_winners: i64,
    pub finish_date: i64,
    pub is_finished: i64,
    pub is_money_places: i64,
    pub needed_members: i64,
    pub now_count_members: i64,
    pub permissions: ForumRespThreadModelContestPermissions,
    pub prize_data: i64,
    pub prize_type: String,
    pub prize_type_phrase: String,
    pub require_like_count: i64,
    pub require_total_like_count: i64,
    #[serde(rename = "type")]
    pub type_: String,
    pub winners: Vec<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelContestPermissions {
    pub can_finish: bool,
    pub can_participate: bool,
    pub can_participate_error: String,
    pub can_view_user_list: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelFirstPost {
    pub links: ForumRespThreadModelFirstPostLinks,
    pub permissions: ForumRespThreadModelFirstPostPermissions,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_liked: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelFirstPostLinks {
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelLastPost {
    pub links: ForumRespThreadModelLastPostLinks,
    pub permissions: ForumRespThreadModelLastPostPermissions,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_liked: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelLastPostLinks {
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelLastPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelPermissions {
    pub bump: ForumRespThreadModelPermissionsBump,
    pub delete: bool,
    pub edit: bool,
    pub edit_tags: bool,
    pub edit_title: bool,
    pub follow: bool,
    pub post: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelPermissionsBump {
    pub available_count: i64,
    pub can: bool,
    pub error: serde_json::Value,
    pub next_available_time: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelRestrictions {
    pub max_reply_count: i64,
    pub reply_delay: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespThreadModelThreadTags {
    #[serde(rename = "193431")]
    pub field_193431: String,
    #[serde(rename = "206")]
    pub field_206: String,
    #[serde(rename = "97491")]
    pub field_97491: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModel {
    pub balance: String,
    pub banner: String,
    pub birthday: ForumRespUserModelBirthday,
    pub contest_count: i64,
    pub conv_welcome_message: String,
    pub curator_titles: Vec<String>,
    pub currency: String,
    pub custom_title: String,
    pub display_banner_id: i64,
    pub display_icon_group_id: i64,
    pub edit_permissions: ForumRespUserModelEditPermissions,
    pub fields: Vec<ForumRespUserModelFieldsItem>,
    pub hold: String,
    pub is_banned: i64,
    pub links: ForumRespUserModelLinks,
    pub permissions: ForumRespUserModelPermissions,
    pub secret_answer_first_letter: String,
    pub secret_answer_rendered: String,
    pub self_permissions: ForumRespUserModelSelfPermissions,
    pub short_link: String,
    pub trophy_count: i64,
    pub user_deposit: i64,
    pub user_email: String,
    pub user_external_authentications: Vec<ForumRespUserModelUserExternalAuthenticationsItem>,
    pub user_followers: ForumRespUserModelUserFollowers,
    pub user_following: ForumRespUserModelUserFollowing,
    pub user_group_id: i64,
    pub user_groups: Vec<ForumRespUserModelUserGroupsItem>,
    pub user_id: i64,
    pub user_is_followed: bool,
    pub user_is_ignored: bool,
    pub user_is_valid: bool,
    pub user_is_verified: bool,
    pub user_is_visitor: bool,
    pub user_last_seen_date: i64,
    pub user_like2_count: i64,
    pub user_like_count: i64,
    pub user_message_count: i64,
    pub user_register_date: i64,
    pub user_timezone_offset: i64,
    pub user_title: String,
    pub user_unread_conversation_count: i64,
    pub user_unread_notification_count: i64,
    pub username: String,
    pub username_html: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelBirthday {
    pub age: i64,
    pub format: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: ForumRespUserModelBirthdayTimeStamp,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelBirthdayTimeStamp {
    pub date: String,
    pub timezone: String,
    pub timezone_type: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelEditPermissions {
    pub fields: bool,
    pub hide_username_logs: bool,
    pub password: bool,
    pub primary_group_id: bool,
    pub secondary_group_ids: bool,
    pub short_link: bool,
    pub user_dob_day: bool,
    pub user_dob_month: bool,
    pub user_dob_year: bool,
    pub user_email: bool,
    pub user_title: bool,
    pub username: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelFieldsItem {
    pub choices: Vec<ForumRespUserModelFieldsItemChoicesItem>,
    pub description: String,
    pub id: String,
    pub is_multi_choice: bool,
    pub is_required: bool,
    pub position: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub values: Vec<serde_json::Value>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelFieldsItemChoicesItem {
    pub key: String,
    pub value: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelLinks {
    pub avatar: String,
    pub avatar_big: String,
    pub avatar_small: String,
    pub background_l: String,
    pub background_m: String,
    pub detail: String,
    pub followers: String,
    pub followings: String,
    pub ignore: String,
    pub permalink: String,
    pub status: String,
    pub timeline: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelPermissions {
    pub edit: bool,
    pub follow: bool,
    pub ignore: bool,
    pub profile_post: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelSelfPermissions {
    pub create_conversation: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelUserExternalAuthenticationsItem {
    pub provider: String,
    pub provider_key: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelUserFollowers {
    pub count: i64,
    pub users: Vec<ForumRespUserModelUserFollowersUsersItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelUserFollowersUsersItem {
    pub avatar: String,
    pub user_id: i64,
    pub username: String,
    pub username_html: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelUserFollowing {
    pub count: i64,
    pub users: Vec<ForumRespUserModelUserFollowingUsersItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelUserFollowingUsersItem {
    pub avatar: String,
    pub user_id: i64,
    pub username: String,
    pub username_html: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumRespUserModelUserGroupsItem {
    pub display_banner_selectable: bool,
    pub display_group_selectable: bool,
    pub display_icon_selectable: bool,
    pub is_primary_group: bool,
    pub user_group_banner_css_class: String,
    pub user_group_banner_text: String,
    pub user_group_banner_text_en: String,
    pub user_group_icon_class: String,
    pub user_group_id: i64,
    pub user_group_title: String,
    pub user_group_title_en: String,
}


pub type ForumRoomIdmodel = i64;

pub type ForumUserIdmodel = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsEditFeedOptionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids: Option<Vec<i64>>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimal_contest_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_ids: Option<Vec<i64>>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowedResponse {
    pub forums: Vec<ForumsFollowedResponseForumsItem>,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowedResponseForumsItem {
    pub follow: ForumsFollowedResponseForumsItemFollow,
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<ForumsFollowedResponseForumsItemForumPrefixesItem>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: ForumsFollowedResponseForumsItemLinks,
    pub permissions: ForumsFollowedResponseForumsItemPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowedResponseForumsItemFollow {
    pub alert: bool,
    pub email: bool,
    pub post: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowedResponseForumsItemForumPrefixesItem {
    pub group_prefixes: Vec<ForumsFollowedResponseForumsItemForumPrefixesItemGroupPrefixesItem>,
    pub group_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowedResponseForumsItemForumPrefixesItemGroupPrefixesItem {
    pub prefix_id: i64,
    pub prefix_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowedResponseForumsItemLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowedResponseForumsItemPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowersResponse {
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<ForumsFollowersResponseUsersItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowersResponseUsersItem {
    pub follow: ForumsFollowersResponseUsersItemFollow,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsFollowersResponseUsersItemFollow {
    pub alert: bool,
    pub email: bool,
    pub post: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetFeedOptionsResponse {
    pub default_excluded_forums_ids: Vec<i64>,
    pub excluded_forums_ids: Vec<i64>,
    pub forums: Vec<ForumsGetFeedOptionsResponseForumsItem>,
    pub keywords: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetFeedOptionsResponseForumsItem {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_title: String,
    pub has_children: bool,
    pub links: ForumsGetFeedOptionsResponseForumsItemLinks,
    pub parent_node_id: i64,
    pub permissions: ForumsGetFeedOptionsResponseForumsItemPermissions,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetFeedOptionsResponseForumsItemLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetFeedOptionsResponseForumsItemPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetResponse {
    pub forum: ForumsGetResponseForum,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetResponseForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<ForumsGetResponseForumForumPrefixesItem>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: ForumsGetResponseForumLinks,
    pub permissions: ForumsGetResponseForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetResponseForumForumPrefixesItem {
    pub group_prefixes: Vec<ForumsGetResponseForumForumPrefixesItemGroupPrefixesItem>,
    pub group_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetResponseForumForumPrefixesItemGroupPrefixesItem {
    pub prefix_id: i64,
    pub prefix_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetResponseForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGetResponseForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGroupedResponse {
    pub data: ForumsGroupedResponseData,
    pub system_info: ForumRespSystemInfo,
    pub tabs: Vec<ForumsGroupedResponseTabsItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGroupedResponseData {
    #[serde(rename = "0")]
    pub field_0: ForumsGroupedResponseData0,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGroupedResponseData0 {
    #[serde(rename = "0")]
    pub field_0: ForumsGroupedResponseData00,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGroupedResponseData00 {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: ForumsGroupedResponseData00Links,
    pub parent_node_id: i64,
    pub permissions: ForumsGroupedResponseData00Permissions,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGroupedResponseData00Links {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGroupedResponseData00Permissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsGroupedResponseTabsItem {
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    pub link_title: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsListResponse {
    pub forums: Vec<ForumsListResponseForumsItem>,
    pub forums_total: i64,
    pub system_info: ForumRespSystemInfo,
    pub tabs: Vec<ForumsListResponseTabsItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsListResponseForumsItem {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<ForumsListResponseForumsItemForumPrefixesItem>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: ForumsListResponseForumsItemLinks,
    pub permissions: ForumsListResponseForumsItemPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsListResponseForumsItemForumPrefixesItem {
    pub group_prefixes: Vec<ForumsListResponseForumsItemForumPrefixesItemGroupPrefixesItem>,
    pub group_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsListResponseForumsItemForumPrefixesItemGroupPrefixesItem {
    pub prefix_id: i64,
    pub prefix_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsListResponseForumsItemLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsListResponseForumsItemPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForumsListResponseTabsItem {
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    pub link_title: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LinksGetResponse {
    #[serde(rename = "link-forum")]
    pub link_forum: ForumRespLinkModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LinksListResponse {
    #[serde(rename = "link-forums")]
    pub link_forums: Vec<ForumRespLinkModel>,
    #[serde(rename = "link-forums_total")]
    pub link_forums_total: i64,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NavigationListResponse {
    pub elements: Vec<NavigationListResponseElementsItem>,
    pub elements_count: i64,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NavigationListResponseElementsItem {
    pub category_description: String,
    pub category_id: i64,
    pub category_title: String,
    pub has_sub_elements: bool,
    pub links: NavigationListResponseElementsItemLinks,
    pub navigation_id: i64,
    pub navigation_parent_id: i64,
    pub navigation_type: String,
    pub permissions: NavigationListResponseElementsItemPermissions,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NavigationListResponseElementsItemLinks {
    pub detail: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-elements")]
    pub sub_elements: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NavigationListResponseElementsItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NotificationsGetResponse {
    pub notification: ForumRespNotificationModel,
    pub notification_id: i64,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NotificationsListResponse {
    pub links: NotificationsListResponseLinks,
    pub notifications: Vec<ForumRespNotificationModel>,
    pub notifications_total: i64,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NotificationsListResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
    pub read: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NotificationsReadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<i64>,
}


pub type OauthTokenMultipart = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OauthTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    pub token_type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesGetResponse {
    pub page: PagesGetResponsePage,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesGetResponsePage {
    pub links: PagesGetResponsePageLinks,
    pub page_description: String,
    pub page_html: String,
    pub page_id: i64,
    pub page_title: String,
    pub page_view_count: i64,
    pub permissions: PagesGetResponsePagePermissions,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesGetResponsePageLinks {
    pub detail: String,
    pub permalink: String,
    #[serde(rename = "sub-pages")]
    pub sub_pages: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesGetResponsePagePermissions {
    pub delete: bool,
    pub edit: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesListResponse {
    pub pages: Vec<PagesListResponsePagesItem>,
    pub pages_total: i64,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesListResponsePagesItem {
    pub links: PagesListResponsePagesItemLinks,
    pub page_description: String,
    pub page_id: i64,
    pub page_title: String,
    pub permissions: PagesListResponsePagesItemPermissions,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesListResponsePagesItemLinks {
    pub detail: String,
    pub permalink: String,
    #[serde(rename = "sub-pages")]
    pub sub_pages: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PagesListResponsePagesItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsCreateRequest {
    pub comment_body: String,
    pub post_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsCreateResponse {
    pub comment: PostsCommentsCreateResponseComment,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsCreateResponseComment {
    pub links: PostsCommentsCreateResponseCommentLinks,
    pub permissions: PostsCommentsCreateResponseCommentPermissions,
    pub post_comment_body: String,
    pub post_comment_body_html: String,
    pub post_comment_body_plain_text: String,
    pub post_comment_id: i64,
    pub post_comment_is_deleted: bool,
    pub post_comment_is_published: bool,
    pub post_comment_like_count: i64,
    pub post_comment_update_date: i64,
    pub post_id: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsCreateResponseCommentLinks {
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub post: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsCreateResponseCommentPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsDeleteRequest {
    pub post_comment_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsEditRequest {
    pub comment_body: String,
    pub post_comment_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsEditResponse {
    pub comment: PostsCommentsEditResponseComment,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsEditResponseComment {
    pub links: PostsCommentsEditResponseCommentLinks,
    pub permissions: PostsCommentsEditResponseCommentPermissions,
    pub post_comment_body: String,
    pub post_comment_body_html: String,
    pub post_comment_body_plain_text: String,
    pub post_comment_id: i64,
    pub post_comment_is_deleted: bool,
    pub post_comment_is_published: bool,
    pub post_comment_like_count: i64,
    pub post_comment_update_date: i64,
    pub post_id: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsEditResponseCommentLinks {
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub post: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsEditResponseCommentPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsGetResponse {
    pub comments: Vec<ForumRespPostCommentModel>,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCommentsReportRequest {
    pub message: String,
    pub post_comment_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCreateRequest {
    pub post_body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_post_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsCreateResponse {
    pub post: ForumRespPostModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsDeleteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsEditRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_body: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsEditResponse {
    pub post: ForumRespPostModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsGetResponse {
    pub post: ForumRespPostModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsLikesResponse {
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<PostsLikesResponseUsersItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsLikesResponseUsersItem {
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsListResponse {
    pub posts: Vec<ForumRespThreadModel>,
    pub posts_total: i64,
    pub system_info: ForumRespSystemInfo,
    pub thread: ForumRespThreadModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsReportReasonsResponse {
    pub reasons: Vec<String>,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostsReportRequest {
    pub message: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsCreateRequest {
    pub comment_body: String,
    pub profile_post_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsCreateResponse {
    pub comment: ProfilePostsCommentsCreateResponseComment,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsCreateResponseComment {
    pub comment_body: String,
    pub comment_create_date: i64,
    pub comment_id: i64,
    pub comment_user_id: i64,
    pub comment_username: String,
    pub comment_username_html: String,
    pub links: ProfilePostsCommentsCreateResponseCommentLinks,
    pub permissions: ProfilePostsCommentsCreateResponseCommentPermissions,
    pub profile_post_id: i64,
    pub timeline_user_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsCreateResponseCommentLinks {
    pub detail: String,
    pub poster: String,
    pub poster_avatar: String,
    pub profile_post: String,
    pub timeline: String,
    pub timeline_user: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsCreateResponseCommentPermissions {
    pub delete: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsDeleteRequest {
    pub comment_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsEditRequest {
    pub comment_body: String,
    pub comment_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsEditResponse {
    pub comment: ProfilePostsCommentsEditResponseComment,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsEditResponseComment {
    pub comment_body: String,
    pub comment_create_date: i64,
    pub comment_id: i64,
    pub comment_user_id: i64,
    pub comment_username: String,
    pub comment_username_html: String,
    pub links: ProfilePostsCommentsEditResponseCommentLinks,
    pub permissions: ProfilePostsCommentsEditResponseCommentPermissions,
    pub profile_post_id: i64,
    pub timeline_user_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsEditResponseCommentLinks {
    pub detail: String,
    pub poster: String,
    pub poster_avatar: String,
    pub profile_post: String,
    pub timeline: String,
    pub timeline_user: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsEditResponseCommentPermissions {
    pub delete: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsGetResponse {
    pub comment: ForumRespProfilePostCommentModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsListResponse {
    pub comments: Vec<ForumRespProfilePostCommentModel>,
    pub comments_total: i64,
    pub profile_post: ProfilePostsCommentsListResponseProfilePost,
    pub system_info: ForumRespSystemInfo,
    pub timeline_user: ForumRespUserModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsListResponseProfilePost {
    pub links: ProfilePostsCommentsListResponseProfilePostLinks,
    pub permissions: ProfilePostsCommentsListResponseProfilePostPermissions,
    pub post_body: String,
    pub post_comment_count: i64,
    pub post_create_date: i64,
    pub post_is_deleted: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub profile_post_id: i64,
    pub timeline_user_id: i64,
    pub timeline_username: String,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsListResponseProfilePostLinks {
    pub comments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub timeline: String,
    pub timeline_user: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsListResponseProfilePostPermissions {
    pub comment: bool,
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCommentsReportRequest {
    pub message: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCreateRequest {
    pub post_body: String,
    pub user_id: ForumUserIdmodel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCreateResponse {
    pub profile_post: ProfilePostsCreateResponseProfilePost,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCreateResponseProfilePost {
    pub links: ProfilePostsCreateResponseProfilePostLinks,
    pub permissions: ProfilePostsCreateResponseProfilePostPermissions,
    pub post_body: String,
    pub post_comment_count: i64,
    pub post_create_date: i64,
    pub post_is_deleted: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub profile_post_id: i64,
    pub timeline_user_id: i64,
    pub timeline_username: String,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCreateResponseProfilePostLinks {
    pub comments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub timeline: String,
    pub timeline_user: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsCreateResponseProfilePostPermissions {
    pub comment: bool,
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsEditRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_comments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_body: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsEditResponse {
    pub profile_post: ProfilePostsEditResponseProfilePost,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsEditResponseProfilePost {
    pub links: ProfilePostsEditResponseProfilePostLinks,
    pub permissions: ProfilePostsEditResponseProfilePostPermissions,
    pub post_body: String,
    pub post_comment_count: i64,
    pub post_create_date: i64,
    pub post_is_deleted: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub profile_post_id: i64,
    pub timeline_user_id: i64,
    pub timeline_username: String,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsEditResponseProfilePostLinks {
    pub comments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub timeline: String,
    pub timeline_user: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsEditResponseProfilePostPermissions {
    pub comment: bool,
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsGetResponse {
    pub profile_post: ForumRespProfilePostModel,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsLikesResponse {
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<ProfilePostsLikesResponseUsersItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsLikesResponseUsersItem {
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsListResponse {
    #[serde(rename = "canPostOnProfile")]
    pub can_post_on_profile: bool,
    pub links: ProfilePostsListResponseLinks,
    pub profile_posts: Vec<ForumRespProfilePostModel>,
    pub system_info: ForumRespSystemInfo,
    #[serde(rename = "totalProfilePosts")]
    pub total_profile_posts: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsListResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsReportReasonsResponse {
    pub reasons: Vec<String>,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ProfilePostsReportRequest {
    pub message: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ForumUserIdmodel>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponse {
    pub data: Vec<SearchAllResponseDataItem>,
    pub data_total: i64,
    pub links: SearchAllResponseLinks,
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<ForumRespUserModel>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItem {
    pub content_id: String,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: SearchAllResponseDataItemFirstPost,
    pub forum: SearchAllResponseDataItemForum,
    pub forum_id: i64,
    pub last_post: SearchAllResponseDataItemLastPost,
    pub links: SearchAllResponseDataItemLinks,
    pub node_title: String,
    pub permissions: SearchAllResponseDataItemPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_closed: bool,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_starred: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: Vec<serde_json::Value>,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemFirstPost {
    pub links: SearchAllResponseDataItemFirstPostLinks,
    pub permissions: SearchAllResponseDataItemFirstPostPermissions,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_liked: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemFirstPostLinks {
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<serde_json::Value>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: SearchAllResponseDataItemForumLinks,
    pub parent_node_id: i64,
    pub permissions: SearchAllResponseDataItemForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemLastPost {
    pub links: SearchAllResponseDataItemLastPostLinks,
    pub permissions: SearchAllResponseDataItemLastPostPermissions,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_liked: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemLastPostLinks {
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemLastPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub last_poster: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemPermissions {
    pub bump: SearchAllResponseDataItemPermissionsBump,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub post: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseDataItemPermissionsBump {
    pub available_count: i64,
    pub can: bool,
    pub error: serde_json::Value,
    pub next_available_time: serde_json::Value,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchAllResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ForumUserIdmodel>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponse {
    pub data: Vec<SearchPostsResponseDataItem>,
    pub data_total: i64,
    pub links: SearchPostsResponseLinks,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponseDataItem {
    pub content_id: i64,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: SearchPostsResponseDataItemFirstPost,
    pub forum: SearchPostsResponseDataItemForum,
    pub forum_id: i64,
    pub links: SearchPostsResponseDataItemLinks,
    pub permissions: SearchPostsResponseDataItemPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: Vec<serde_json::Value>,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponseDataItemFirstPost {
    pub links: SearchPostsResponseDataItemFirstPostLinks,
    pub permissions: SearchPostsResponseDataItemFirstPostPermissions,
    pub post_attachment_count: i64,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponseDataItemFirstPostLinks {
    pub attachments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponseDataItemFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponseDataItemForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<serde_json::Value>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: SearchPostsResponseDataItemForumLinks,
    pub permissions: SearchPostsResponseDataItemForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponseDataItemForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponseDataItemForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponseDataItemLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponseDataItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub post: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchPostsResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchProfilePostsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchProfilePostsResponse {
    pub data: Vec<SearchProfilePostsResponseDataItem>,
    pub data_total: i64,
    pub links: SearchProfilePostsResponseLinks,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchProfilePostsResponseDataItem {
    pub content_id: i64,
    pub content_type: String,
    pub links: SearchProfilePostsResponseDataItemLinks,
    pub permissions: SearchProfilePostsResponseDataItemPermissions,
    pub post_body: String,
    pub post_comment_count: i64,
    pub post_create_date: i64,
    pub post_is_deleted: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub profile_post_id: i64,
    pub timeline_user: ForumRespUserModel,
    pub timeline_user_id: i64,
    pub timeline_username: String,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchProfilePostsResponseDataItemLinks {
    pub comments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub timeline: String,
    pub timeline_user: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchProfilePostsResponseDataItemPermissions {
    pub comment: bool,
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub report: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchProfilePostsResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponse {
    pub data: Vec<SearchResultsResponseDataItem>,
    pub data_total: i64,
    pub search_tags: SearchResultsResponseSearchTags,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItem {
    pub content_id: i64,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: SearchResultsResponseDataItemFirstPost,
    pub forum: SearchResultsResponseDataItemForum,
    pub forum_id: i64,
    pub links: SearchResultsResponseDataItemLinks,
    pub permissions: SearchResultsResponseDataItemPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: SearchResultsResponseDataItemThreadTags,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemFirstPost {
    pub links: SearchResultsResponseDataItemFirstPostLinks,
    pub permissions: SearchResultsResponseDataItemFirstPostPermissions,
    pub post_attachment_count: i64,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemFirstPostLinks {
    pub attachments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<SearchResultsResponseDataItemForumForumPrefixesItem>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: SearchResultsResponseDataItemForumLinks,
    pub permissions: SearchResultsResponseDataItemForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemForumForumPrefixesItem {
    pub group_prefixes: Vec<SearchResultsResponseDataItemForumForumPrefixesItemGroupPrefixesItem>,
    pub group_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemForumForumPrefixesItemGroupPrefixesItem {
    pub prefix_id: i64,
    pub prefix_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub edit_tags: bool,
    pub edit_title: bool,
    pub follow: bool,
    pub post: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseDataItemThreadTags {
    #[serde(rename = "160179")]
    pub field_160179: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchResultsResponseSearchTags {
    #[serde(rename = "160179")]
    pub field_160179: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponse {
    pub data: Vec<SearchTaggedResponseDataItem>,
    pub data_total: i64,
    pub search_tags: SearchTaggedResponseSearchTags,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItem {
    pub content_id: i64,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: SearchTaggedResponseDataItemFirstPost,
    pub forum: SearchTaggedResponseDataItemForum,
    pub forum_id: i64,
    pub links: SearchTaggedResponseDataItemLinks,
    pub permissions: SearchTaggedResponseDataItemPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: SearchTaggedResponseDataItemThreadTags,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemFirstPost {
    pub links: SearchTaggedResponseDataItemFirstPostLinks,
    pub permissions: SearchTaggedResponseDataItemFirstPostPermissions,
    pub post_attachment_count: i64,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemFirstPostLinks {
    pub attachments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<SearchTaggedResponseDataItemForumForumPrefixesItem>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: SearchTaggedResponseDataItemForumLinks,
    pub permissions: SearchTaggedResponseDataItemForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemForumForumPrefixesItem {
    pub group_prefixes: Vec<SearchTaggedResponseDataItemForumForumPrefixesItemGroupPrefixesItem>,
    pub group_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemForumForumPrefixesItemGroupPrefixesItem {
    pub prefix_id: i64,
    pub prefix_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub edit_tags: bool,
    pub edit_title: bool,
    pub follow: bool,
    pub post: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseDataItemThreadTags {
    #[serde(rename = "160179")]
    pub field_160179: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchTaggedResponseSearchTags {
    #[serde(rename = "160179")]
    pub field_160179: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<ForumUserIdmodel>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponse {
    pub data: Vec<SearchThreadsResponseDataItem>,
    pub data_total: i64,
    pub links: SearchThreadsResponseLinks,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponseDataItem {
    pub content_id: i64,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: SearchThreadsResponseDataItemFirstPost,
    pub forum: SearchThreadsResponseDataItemForum,
    pub forum_id: i64,
    pub links: SearchThreadsResponseDataItemLinks,
    pub permissions: SearchThreadsResponseDataItemPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: Vec<serde_json::Value>,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponseDataItemFirstPost {
    pub links: SearchThreadsResponseDataItemFirstPostLinks,
    pub permissions: SearchThreadsResponseDataItemFirstPostPermissions,
    pub post_attachment_count: i64,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponseDataItemFirstPostLinks {
    pub attachments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponseDataItemFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponseDataItemForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<serde_json::Value>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: SearchThreadsResponseDataItemForumLinks,
    pub permissions: SearchThreadsResponseDataItemForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponseDataItemForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponseDataItemForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponseDataItemLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponseDataItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub post: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchThreadsResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchUsersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SearchUsersResponse {
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<ForumRespUserModel>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsFindResponse {
    pub ids: Vec<i64>,
    pub system_info: ForumRespSystemInfo,
    pub tags: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponse {
    pub links: TagsGetResponseLinks,
    pub system_info: ForumRespSystemInfo,
    pub tag: TagsGetResponseTag,
    pub tagged: Vec<TagsGetResponseTaggedItem>,
    pub tagged_total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTag {
    pub links: TagsGetResponseTagLinks,
    pub tag_id: i64,
    pub tag_text: String,
    pub tag_use_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTagLinks {
    pub detail: String,
    pub permalink: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItem {
    pub content_id: i64,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: TagsGetResponseTaggedItemFirstPost,
    pub forum: TagsGetResponseTaggedItemForum,
    pub forum_id: i64,
    pub links: TagsGetResponseTaggedItemLinks,
    pub permissions: TagsGetResponseTaggedItemPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<TagsGetResponseTaggedItemThreadPrefixesItem>,
    pub thread_tags: TagsGetResponseTaggedItemThreadTags,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemFirstPost {
    pub links: TagsGetResponseTaggedItemFirstPostLinks,
    pub permissions: TagsGetResponseTaggedItemFirstPostPermissions,
    pub post_attachment_count: i64,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemFirstPostLinks {
    pub attachments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<TagsGetResponseTaggedItemForumForumPrefixesItem>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: TagsGetResponseTaggedItemForumLinks,
    pub permissions: TagsGetResponseTaggedItemForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemForumForumPrefixesItem {
    pub group_prefixes: Vec<TagsGetResponseTaggedItemForumForumPrefixesItemGroupPrefixesItem>,
    pub group_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemForumForumPrefixesItemGroupPrefixesItem {
    pub prefix_id: i64,
    pub prefix_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub last_poster: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub post: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemThreadPrefixesItem {
    pub prefix_id: i64,
    pub prefix_title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsGetResponseTaggedItemThreadTags {
    #[serde(rename = "1")]
    pub field_1: String,
    #[serde(rename = "654")]
    pub field_654: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsListResponse {
    pub links: TagsListResponseLinks,
    pub system_info: ForumRespSystemInfo,
    pub tags: TagsListResponseTags,
    pub tags_total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsListResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsListResponseTags {
    #[serde(rename = "1")]
    pub field_1: String,
    #[serde(rename = "10")]
    pub field_10: String,
    #[serde(rename = "11")]
    pub field_11: String,
    #[serde(rename = "12")]
    pub field_12: String,
    #[serde(rename = "14")]
    pub field_14: String,
    #[serde(rename = "15")]
    pub field_15: String,
    #[serde(rename = "16")]
    pub field_16: String,
    #[serde(rename = "17")]
    pub field_17: String,
    #[serde(rename = "18")]
    pub field_18: String,
    #[serde(rename = "19")]
    pub field_19: String,
    #[serde(rename = "2")]
    pub field_2: String,
    #[serde(rename = "20")]
    pub field_20: String,
    #[serde(rename = "3")]
    pub field_3: String,
    #[serde(rename = "4")]
    pub field_4: String,
    #[serde(rename = "5")]
    pub field_5: String,
    #[serde(rename = "6")]
    pub field_6: String,
    #[serde(rename = "7")]
    pub field_7: String,
    #[serde(rename = "8")]
    pub field_8: String,
    #[serde(rename = "9")]
    pub field_9: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsPopularResponse {
    pub system_info: ForumRespSystemInfo,
    pub tags: TagsPopularResponseTags,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TagsPopularResponseTags {
    #[serde(rename = "000")]
    pub field_000: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsBumpResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsClaimRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_hidden_content: Option<bool>,
    pub as_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_funds_receipt: Option<String>,
    pub as_is_market_deal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_market_item_id: Option<i64>,
    pub as_responder: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_tg_login_screenshot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_ignore_group: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_alert_followers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_contacts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_claim: Option<String>,
    pub post_body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_group: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub transfer_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_state: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsClaimResponse {
    pub system_info: ForumRespSystemInfo,
    pub thread: ForumRespThreadModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsCreateContestRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_hidden_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_ignore_group: Option<bool>,
    pub contest_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_winners: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_alert_followers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_contacts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_money_places: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_value: Option<i64>,
    pub post_body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_data_money: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_data_places: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_data_upgrade: Option<i64>,
    pub prize_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_group: Option<i64>,
    pub require_like_count: i64,
    pub require_total_like_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_state: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsCreateContestResponse {
    pub system_info: ForumRespSystemInfo,
    pub thread: ForumRespThreadModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_hidden_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_ignore_group: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_alert_followers: Option<bool>,
    pub forum_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_contacts: Option<bool>,
    pub post_body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_group: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_thread_state: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsCreateResponse {
    pub system_info: ForumRespSystemInfo,
    pub thread: ForumRespThreadModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsDeleteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsEditRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_ask_hidden_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_ignore_group: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_contacts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_group: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsEditResponse {
    pub system_info: ForumRespSystemInfo,
    pub thread: ForumRespThreadModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponse {
    pub system_info: ForumRespSystemInfo,
    pub threads: Vec<ThreadsFollowedResponseThreadsItem>,
    pub threads_total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItem {
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: ThreadsFollowedResponseThreadsItemFirstPost,
    pub follow: ThreadsFollowedResponseThreadsItemFollow,
    pub forum: ThreadsFollowedResponseThreadsItemForum,
    pub forum_id: i64,
    pub links: ThreadsFollowedResponseThreadsItemLinks,
    pub permissions: ThreadsFollowedResponseThreadsItemPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: ThreadsFollowedResponseThreadsItemThreadTags,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemFirstPost {
    pub like_users: Vec<ThreadsFollowedResponseThreadsItemFirstPostLikeUsersItem>,
    pub links: ThreadsFollowedResponseThreadsItemFirstPostLinks,
    pub permissions: ThreadsFollowedResponseThreadsItemFirstPostPermissions,
    pub post_attachment_count: i64,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemFirstPostLikeUsersItem {
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub uniq_username_css: String,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemFirstPostLinks {
    pub attachments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemFollow {
    pub alert: bool,
    pub email: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<serde_json::Value>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: ThreadsFollowedResponseThreadsItemForumLinks,
    pub permissions: ThreadsFollowedResponseThreadsItemForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub edit_tags: bool,
    pub edit_title: bool,
    pub follow: bool,
    pub post: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowedResponseThreadsItemThreadTags {
    #[serde(rename = "1403")]
    pub field_1403: String,
    #[serde(rename = "1953")]
    pub field_1953: String,
    #[serde(rename = "38")]
    pub field_38: String,
    #[serde(rename = "523")]
    pub field_523: String,
    #[serde(rename = "8176")]
    pub field_8176: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowersResponse {
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<ThreadsFollowersResponseUsersItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowersResponseUsersItem {
    pub follow: ThreadsFollowersResponseUsersItemFollow,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsFollowersResponseUsersItemFollow {
    pub alert: bool,
    pub email: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsGetResponse {
    pub system_info: ForumRespSystemInfo,
    pub thread: ForumRespThreadModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsHideResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsListResponse {
    pub forum: ThreadsListResponseForum,
    pub links: ThreadsListResponseLinks,
    pub system_info: ForumRespSystemInfo,
    pub threads: Vec<ForumRespThreadModel>,
    pub threads_total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsListResponseForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<serde_json::Value>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: ThreadsListResponseForumLinks,
    pub permissions: ThreadsListResponseForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsListResponseForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsListResponseForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsListResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsMoveRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_thread_prefix: Option<bool>,
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_id: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_en: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsNavigationResponse {
    pub elements: Vec<ThreadsNavigationResponseElementsItem>,
    pub elements_count: i64,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsNavigationResponseElementsItem {
    pub category_description: String,
    pub category_id: i64,
    pub category_title: String,
    pub has_sub_elements: bool,
    pub links: ThreadsNavigationResponseElementsItemLinks,
    pub navigation_depth: i64,
    pub navigation_id: i64,
    pub navigation_parent_id: i64,
    pub navigation_type: String,
    pub permissions: ThreadsNavigationResponseElementsItemPermissions,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsNavigationResponseElementsItemLinks {
    pub detail: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-elements")]
    pub sub_elements: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsNavigationResponseElementsItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsPollGetResponse {
    pub poll: ThreadsPollGetResponsePoll,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsPollGetResponsePoll {
    pub links: ThreadsPollGetResponsePollLinks,
    pub permissions: ThreadsPollGetResponsePollPermissions,
    pub poll_id: i64,
    pub poll_is_open: bool,
    pub poll_is_voted: bool,
    pub poll_max_votes: i64,
    pub poll_question: String,
    pub poll_vote_count: i64,
    pub responses: Vec<ThreadsPollGetResponsePollResponsesItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsPollGetResponsePollLinks {
    pub vote: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsPollGetResponsePollPermissions {
    pub result: bool,
    pub vote: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsPollGetResponsePollResponsesItem {
    pub response_answer: String,
    pub response_id: i64,
    pub response_vote_count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsPollVoteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_ids: Option<Vec<i64>>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponse {
    pub data: Vec<ThreadsRecentResponseDataItem>,
    pub system_info: ForumRespSystemInfo,
    pub threads: Vec<ForumRespThreadModel>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponseDataItem {
    pub content_id: i64,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: ThreadsRecentResponseDataItemFirstPost,
    pub forum: ThreadsRecentResponseDataItemForum,
    pub forum_id: i64,
    pub links: ThreadsRecentResponseDataItemLinks,
    pub permissions: ThreadsRecentResponseDataItemPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: Vec<serde_json::Value>,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponseDataItemFirstPost {
    pub links: ThreadsRecentResponseDataItemFirstPostLinks,
    pub permissions: ThreadsRecentResponseDataItemFirstPostPermissions,
    pub post_attachment_count: i64,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponseDataItemFirstPostLinks {
    pub attachments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponseDataItemFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponseDataItemForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<serde_json::Value>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: ThreadsRecentResponseDataItemForumLinks,
    pub permissions: ThreadsRecentResponseDataItemForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponseDataItemForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponseDataItemForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponseDataItemLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub last_poster: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsRecentResponseDataItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub post: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponse {
    pub data: Vec<ThreadsUnreadResponseDataItem>,
    pub system_info: ForumRespSystemInfo,
    pub threads: Vec<ForumRespThreadModel>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponseDataItem {
    pub content_id: i64,
    pub content_type: String,
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub first_post: ThreadsUnreadResponseDataItemFirstPost,
    pub forum: ThreadsUnreadResponseDataItemForum,
    pub forum_id: i64,
    pub links: ThreadsUnreadResponseDataItemLinks,
    pub permissions: ThreadsUnreadResponseDataItemPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: Vec<serde_json::Value>,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponseDataItemFirstPost {
    pub like_users: Vec<ThreadsUnreadResponseDataItemFirstPostLikeUsersItem>,
    pub links: ThreadsUnreadResponseDataItemFirstPostLinks,
    pub permissions: ThreadsUnreadResponseDataItemFirstPostPermissions,
    pub post_attachment_count: i64,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponseDataItemFirstPostLikeUsersItem {
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub uniq_username_css: String,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponseDataItemFirstPostLinks {
    pub attachments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponseDataItemFirstPostPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponseDataItemForum {
    pub forum_description: String,
    pub forum_id: i64,
    pub forum_is_followed: bool,
    pub forum_post_count: i64,
    pub forum_prefixes: Vec<serde_json::Value>,
    pub forum_thread_count: i64,
    pub forum_title: String,
    pub links: ThreadsUnreadResponseDataItemForumLinks,
    pub permissions: ThreadsUnreadResponseDataItemForumPermissions,
    pub thread_default_prefix_id: i64,
    pub thread_prefix_is_required: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponseDataItemForumLinks {
    pub detail: String,
    pub followers: String,
    pub permalink: String,
    #[serde(rename = "sub-categories")]
    pub sub_categories: String,
    #[serde(rename = "sub-forums")]
    pub sub_forums: String,
    pub threads: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponseDataItemForumPermissions {
    pub create_thread: bool,
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub tag_thread: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponseDataItemLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub last_poster: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThreadsUnreadResponseDataItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub follow: bool,
    pub post: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersAvatarCropRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersAvatarCropResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersAvatarUploadMultipart {
    pub avatar: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersAvatarUploadResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersBackgroundCropRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersBackgroundCropResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersBackgroundUploadMultipart {
    pub background: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersBackgroundUploadResponse {
    pub message: String,
    pub status: String,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersClaimsResponse {
    pub claims: Vec<UsersClaimsResponseClaimsItem>,
    pub stats: UsersClaimsResponseStats,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersClaimsResponseClaimsItem {
    pub amount: i64,
    pub amount_formatted: String,
    pub author: ForumRespUserModel,
    pub claim_date: i64,
    pub claim_state: String,
    pub message_body: String,
    pub message_body_html: String,
    pub message_body_plain_text: String,
    pub thread_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersClaimsResponseStats {
    pub market: UsersClaimsResponseStatsMarket,
    #[serde(rename = "noMarket")]
    pub no_market: UsersClaimsResponseStatsNoMarket,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersClaimsResponseStatsMarket {
    pub rejected: i64,
    pub settled: i64,
    pub solved: i64,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersClaimsResponseStatsNoMarket {
    pub rejected: i64,
    pub settled: i64,
    pub solved: i64,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponse {
    pub data: Vec<UsersContentsResponseDataItem>,
    pub data_total: i64,
    pub links: UsersContentsResponseLinks,
    pub system_info: ForumRespSystemInfo,
    pub user: ForumRespUserModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponseDataItem {
    pub content_id: i64,
    pub content_type: String,
    pub like_users: Vec<UsersContentsResponseDataItemLikeUsersItem>,
    pub links: UsersContentsResponseDataItemLinks,
    pub permissions: UsersContentsResponseDataItemPermissions,
    pub post_attachment_count: i64,
    pub post_body: String,
    pub post_body_html: String,
    pub post_body_plain_text: String,
    pub post_create_date: i64,
    pub post_id: i64,
    pub post_is_deleted: bool,
    pub post_is_first_post: bool,
    pub post_is_published: bool,
    pub post_like_count: i64,
    pub post_update_date: i64,
    pub poster_user_id: i64,
    pub poster_username: String,
    pub poster_username_html: String,
    pub signature: String,
    pub signature_html: String,
    pub signature_plain_text: String,
    pub thread: UsersContentsResponseDataItemThread,
    pub thread_id: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponseDataItemLikeUsersItem {
    pub display_style_group_id: i64,
    pub is_banned: i64,
    pub uniq_username_css: String,
    pub user_id: i64,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponseDataItemLinks {
    pub attachments: String,
    pub detail: String,
    pub likes: String,
    pub permalink: String,
    pub poster: String,
    pub poster_avatar: String,
    pub report: String,
    pub thread: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponseDataItemPermissions {
    pub delete: bool,
    pub edit: bool,
    pub like: bool,
    pub reply: bool,
    pub report: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponseDataItemThread {
    pub creator_user_id: i64,
    pub creator_username: String,
    pub creator_username_html: String,
    pub forum_id: i64,
    pub links: UsersContentsResponseDataItemThreadLinks,
    pub permissions: UsersContentsResponseDataItemThreadPermissions,
    pub thread_create_date: i64,
    pub thread_id: i64,
    pub thread_is_deleted: bool,
    pub thread_is_followed: bool,
    pub thread_is_published: bool,
    pub thread_is_sticky: bool,
    pub thread_post_count: i64,
    pub thread_prefixes: Vec<serde_json::Value>,
    pub thread_tags: Vec<serde_json::Value>,
    pub thread_title: String,
    pub thread_update_date: i64,
    pub thread_view_count: i64,
    pub user_is_ignored: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponseDataItemThreadLinks {
    pub detail: String,
    pub first_post: String,
    pub first_poster: String,
    pub first_poster_avatar: String,
    pub followers: String,
    pub forum: String,
    pub last_post: String,
    pub last_poster: String,
    pub permalink: String,
    pub posts: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponseDataItemThreadPermissions {
    pub delete: bool,
    pub follow: bool,
    pub post: bool,
    pub upload_attachment: bool,
    pub view: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersContentsResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersEditRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<BTreeMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_invite_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_post_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_receive_news_feed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_send_personal_conversation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_view_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conv_welcome_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_banner_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_icon_group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<UsersEditRequestFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_username_change_logs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_admin_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_answer_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_dob_date: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_dob_year: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_dob_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_dob_month: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_dob_year: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersEditRequestFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_4")]
    pub field_4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jabber: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lztInnovation20Link")]
    pub lzt_innovation20link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lztInnovation30Link")]
    pub lzt_innovation30link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lztInnovationLink")]
    pub lzt_innovation_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vk: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFieldsResponse {
    pub fields: Vec<UsersFieldsResponseFieldsItem>,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFieldsResponseFieldsItem {
    pub description: String,
    pub id: String,
    pub is_required: bool,
    pub position: String,
    pub title: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFindResponse {
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<ForumRespUserModel>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowersResponse {
    pub links: UsersFollowersResponseLinks,
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<UsersFollowersResponseUsersItem>,
    pub users_total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowersResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowersResponseUsersItem {
    pub content_id: i64,
    pub content_type: String,
    pub contest_count: i64,
    pub custom_fields: UsersFollowersResponseUsersItemCustomFields,
    pub custom_title: String,
    pub follow_date: i64,
    pub is_banned: i64,
    pub links: UsersFollowersResponseUsersItemLinks,
    pub permissions: UsersFollowersResponseUsersItemPermissions,
    pub trophy_count: i64,
    pub user_followers_count: i64,
    pub user_following_count: i64,
    pub user_group_id: i64,
    pub user_id: i64,
    pub user_is_followed: bool,
    pub user_is_ignored: bool,
    pub user_is_valid: bool,
    pub user_is_verified: bool,
    pub user_is_visitor: bool,
    pub user_last_seen_date: i64,
    pub user_like2_count: i64,
    pub user_like_count: i64,
    pub user_message_count: i64,
    pub user_register_date: i64,
    pub user_title: String,
    pub username: String,
    pub username_html: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowersResponseUsersItemCustomFields {
    #[serde(rename = "_4")]
    pub field_4: String,
    #[serde(rename = "lztInnovation20Link")]
    pub lzt_innovation20link: String,
    #[serde(rename = "lztInnovation30Link")]
    pub lzt_innovation30link: String,
    #[serde(rename = "lztInnovationLink")]
    pub lzt_innovation_link: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowersResponseUsersItemLinks {
    pub avatar: String,
    pub avatar_big: String,
    pub avatar_small: String,
    pub detail: String,
    pub followers: String,
    pub followings: String,
    pub ignore: String,
    pub permalink: String,
    pub timeline: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowersResponseUsersItemPermissions {
    pub edit: bool,
    pub follow: bool,
    pub ignore: bool,
    pub profile_post: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowingsResponse {
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<UsersFollowingsResponseUsersItem>,
    pub users_total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowingsResponseUsersItem {
    pub content_id: i64,
    pub content_type: String,
    pub contest_count: i64,
    pub custom_fields: UsersFollowingsResponseUsersItemCustomFields,
    pub custom_title: String,
    pub follow_date: i64,
    pub is_banned: i64,
    pub links: UsersFollowingsResponseUsersItemLinks,
    pub permissions: UsersFollowingsResponseUsersItemPermissions,
    pub short_link: String,
    pub trophy_count: i64,
    pub user_followers_count: i64,
    pub user_following_count: i64,
    pub user_group_id: i64,
    pub user_id: i64,
    pub user_is_followed: bool,
    pub user_is_ignored: bool,
    pub user_is_valid: bool,
    pub user_is_verified: bool,
    pub user_is_visitor: bool,
    pub user_last_seen_date: i64,
    pub user_like2_count: i64,
    pub user_like_count: i64,
    pub user_message_count: i64,
    pub user_register_date: i64,
    pub user_title: String,
    pub username: String,
    pub username_html: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowingsResponseUsersItemCustomFields {
    #[serde(rename = "_4")]
    pub field_4: String,
    #[serde(rename = "allowSelfUnban")]
    pub allow_self_unban: Vec<serde_json::Value>,
    pub discord: String,
    pub github: String,
    pub jabber: String,
    #[serde(rename = "lztAwardUserTrophy")]
    pub lzt_award_user_trophy: String,
    #[serde(rename = "lztCuratorNodeTitle")]
    pub lzt_curator_node_title: String,
    #[serde(rename = "lztCuratorNodeTitleEn")]
    pub lzt_curator_node_title_en: String,
    #[serde(rename = "lztDeposit")]
    pub lzt_deposit: String,
    #[serde(rename = "lztInnovation20Link")]
    pub lzt_innovation20link: String,
    #[serde(rename = "lztInnovation30Link")]
    pub lzt_innovation30link: String,
    #[serde(rename = "lztInnovationLink")]
    pub lzt_innovation_link: String,
    #[serde(rename = "lztLikesIncreasing")]
    pub lzt_likes_increasing: String,
    #[serde(rename = "lztLikesZeroing")]
    pub lzt_likes_zeroing: String,
    #[serde(rename = "lztSympathyIncreasing")]
    pub lzt_sympathy_increasing: String,
    #[serde(rename = "lztSympathyZeroing")]
    pub lzt_sympathy_zeroing: String,
    #[serde(rename = "maecenasValue")]
    pub maecenas_value: String,
    #[serde(rename = "scamURL")]
    pub scam_url: String,
    pub steam: String,
    pub telegram: String,
    pub vk: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowingsResponseUsersItemLinks {
    pub avatar: String,
    pub avatar_big: String,
    pub avatar_small: String,
    pub detail: String,
    pub followers: String,
    pub followings: String,
    pub ignore: String,
    pub permalink: String,
    pub timeline: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersFollowingsResponseUsersItemPermissions {
    pub edit: bool,
    pub follow: bool,
    pub ignore: bool,
    pub profile_post: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersGetResponse {
    pub system_info: ForumRespSystemInfo,
    pub user: ForumRespUserModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersIgnoredResponse {
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<UsersIgnoredResponseUsersItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersIgnoredResponseUsersItem {
    pub can_edit: bool,
    pub can_follow: bool,
    pub can_ignore: bool,
    pub can_post_profile: bool,
    pub can_view_profile: bool,
    pub can_view_profile_posts: bool,
    pub can_warn: bool,
    pub contest_count: i64,
    pub conv_welcome_message: String,
    #[serde(rename = "convertedDeposit")]
    pub converted_deposit: i64,
    pub custom_fields: UsersIgnoredResponseUsersItemCustomFields,
    pub deposit: i64,
    pub homepage: String,
    pub ignored_info: UsersIgnoredResponseUsersItemIgnoredInfo,
    pub is_admin: bool,
    pub is_banned: bool,
    pub is_followed: bool,
    pub is_ignored: bool,
    pub is_moderator: bool,
    pub is_staff: bool,
    pub last_activity: i64,
    pub like2_count: i64,
    pub like_count: i64,
    pub location: String,
    pub message_count: i64,
    pub register_date: i64,
    pub rendered: UsersIgnoredResponseUsersItemRendered,
    pub short_link: String,
    pub trophy_points: i64,
    pub user_id: i64,
    pub user_title: String,
    pub username: String,
    pub view_url: String,
    pub warning_points: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersIgnoredResponseUsersItemCustomFields {
    #[serde(rename = "_4")]
    pub field_4: String,
    pub discord: String,
    pub github: String,
    pub jabber: String,
    #[serde(rename = "lztLikesIncreasing")]
    pub lzt_likes_increasing: serde_json::Value,
    #[serde(rename = "lztLikesZeroing")]
    pub lzt_likes_zeroing: serde_json::Value,
    #[serde(rename = "lztSympathyIncreasing")]
    pub lzt_sympathy_increasing: serde_json::Value,
    #[serde(rename = "lztSympathyZeroing")]
    pub lzt_sympathy_zeroing: serde_json::Value,
    pub matrix: serde_json::Value,
    #[serde(rename = "scamURL")]
    pub scam_url: serde_json::Value,
    pub steam: String,
    pub telegram: serde_json::Value,
    pub vk: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersIgnoredResponseUsersItemIgnoredInfo {
    pub ignore_content: i64,
    pub ignore_conversations: i64,
    pub restrict_view_profile: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersIgnoredResponseUsersItemRendered {
    pub avatars: UsersIgnoredResponseUsersItemRenderedAvatars,
    pub backgrounds: Vec<serde_json::Value>,
    pub link: String,
    pub username: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersIgnoredResponseUsersItemRenderedAvatars {
    pub l: String,
    pub m: String,
    pub s: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersLikesResponse {
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub likes: UsersLikesResponseLikes,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    pub system_info: ForumRespSystemInfo,
    #[serde(rename = "totalLikes")]
    pub total_likes: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersLikesResponseLikes {
    #[serde(rename = "1234567890")]
    pub field_1234567890: UsersLikesResponseLikes1234567890,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersLikesResponseLikes1234567890 {
    #[serde(rename = "actionUser")]
    pub action_user: ForumRespUserModel,
    pub content_id: i64,
    pub content_state: String,
    pub content_type: String,
    pub content_user_id: i64,
    pub like_date: i64,
    pub like_id: i64,
    pub like_user_id: i64,
    #[serde(rename = "messageHtml")]
    pub message_html: String,
    pub post_date: i64,
    pub user: ForumRespUserModel,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersListResponse {
    pub links: UsersListResponseLinks,
    pub system_info: ForumRespSystemInfo,
    pub users: Vec<ForumRespUserModel>,
    pub users_total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersListResponseLinks {
    pub next: String,
    pub page: i64,
    pub pages: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersSaResetResponse {
    pub success: bool,
    pub system_info: ForumRespSystemInfo,
    pub waiting_time: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersSecretAnswerTypesResponse {
    pub data: Vec<UsersSecretAnswerTypesResponseDataItem>,
    pub system_info: ForumRespSystemInfo,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersSecretAnswerTypesResponseDataItem {
    #[serde(rename = "renderedPhrase")]
    pub rendered_phrase: String,
    pub sa_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersTrophiesResponse {
    pub system_info: ForumRespSystemInfo,
    pub trophies: Vec<UsersTrophiesResponseTrophiesItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UsersTrophiesResponseTrophiesItem {
    pub description: String,
    pub title: String,
    pub trophy_id: i64,
    pub trophy_url: String,
}


