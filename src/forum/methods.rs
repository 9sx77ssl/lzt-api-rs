//! Auto-generated methods for Lolzteam Public API: Forum.
//! DO NOT EDIT -- run `cargo run --bin lzt-codegen -- generate`.

#![allow(clippy::all)]

use crate::client::HttpMethod;
use crate::error::Result;
use crate::forum::types::*;

impl crate::forum::ForumApi {
    // -- Users --
    /// `POST /account/secret-answer/reset`
    pub async fn users_sa_reset(&self) -> Result<UsersSaResetResponse> {
        let path = "/account/secret-answer/reset".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /account/secret-answer/reset`
    pub async fn users_sa_cancel_reset(&self) -> Result<serde_json::Value> {
        let path = "/account/secret-answer/reset".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    // -- Batch requests --
    /// `POST /batch`
    pub async fn batch_execute(&self, body: BatchExecuteRequest) -> Result<BatchExecuteResponse> {
        let path = "/batch".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Categories --
    /// `GET /categories`
    pub async fn categories_list(&self, query: CategoriesListQuery) -> Result<CategoriesListResponse> {
        let path = "/categories".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /categories/{category_id}`
    pub async fn categories_get(&self, category_id: i64) -> Result<CategoriesGetResponse> {
        let path = format!("/categories/{category_id}", category_id = category_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    // -- Chatbox --
    /// `GET /chatbox`
    pub async fn chatbox_index(&self, query: ChatboxIndexQuery) -> Result<ChatboxIndexResponse> {
        let path = "/chatbox".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /chatbox/ignore`
    pub async fn chatbox_get_ignore(&self) -> Result<ChatboxGetIgnoreResponse> {
        let path = "/chatbox/ignore".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /chatbox/ignore`
    pub async fn chatbox_post_ignore(&self, body: ChatboxPostIgnoreRequest) -> Result<serde_json::Value> {
        let path = "/chatbox/ignore".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /chatbox/ignore`
    pub async fn chatbox_delete_ignore(&self, body: ChatboxDeleteIgnoreRequest) -> Result<serde_json::Value> {
        let path = "/chatbox/ignore".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /chatbox/messages`
    pub async fn chatbox_get_messages(&self, query: ChatboxGetMessagesQuery) -> Result<ChatboxGetMessagesResponse> {
        let path = "/chatbox/messages".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /chatbox/messages`
    pub async fn chatbox_post_message(&self, body: ChatboxPostMessageRequest) -> Result<ChatboxPostMessageResponse> {
        let path = "/chatbox/messages".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `PUT /chatbox/messages`
    pub async fn chatbox_edit_message(&self, body: ChatboxEditMessageRequest) -> Result<ChatboxEditMessageResponse> {
        let path = "/chatbox/messages".to_string();
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /chatbox/messages`
    pub async fn chatbox_delete_message(&self, body: ChatboxDeleteMessageRequest) -> Result<serde_json::Value> {
        let path = "/chatbox/messages".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /chatbox/messages/leaderboard`
    pub async fn chatbox_get_leaderboard(&self, query: ChatboxGetLeaderboardQuery) -> Result<ChatboxGetLeaderboardResponse> {
        let path = "/chatbox/messages/leaderboard".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /chatbox/messages/online`
    pub async fn chatbox_online(&self, query: ChatboxOnlineQuery) -> Result<ChatboxOnlineResponse> {
        let path = "/chatbox/messages/online".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /chatbox/messages/report`
    pub async fn chatbox_report_reasons(&self, query: ChatboxReportReasonsQuery) -> Result<ChatboxReportReasonsResponse> {
        let path = "/chatbox/messages/report".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /chatbox/messages/report`
    pub async fn chatbox_report(&self, body: ChatboxReportRequest) -> Result<serde_json::Value> {
        let path = "/chatbox/messages/report".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Threads --
    /// `POST /claims`
    pub async fn threads_claim(&self, body: ThreadsClaimRequest) -> Result<ThreadsClaimResponse> {
        let path = "/claims".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /contests`
    pub async fn threads_create_contest(&self, body: ThreadsCreateContestRequest) -> Result<ThreadsCreateContestResponse> {
        let path = "/contests".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /contests/{thread_id}/finish`
    pub async fn threads_finish(&self, thread_id: i64) -> Result<serde_json::Value> {
        let path = format!("/contests/{thread_id}/finish", thread_id = thread_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    // -- Conversations --
    /// `GET /conversations`
    pub async fn conversations_list(&self, query: ConversationsListQuery) -> Result<ConversationsListResponse> {
        let path = "/conversations".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /conversations`
    pub async fn conversations_create(&self, body: ConversationsCreateRequest) -> Result<ConversationsCreateResponse> {
        let path = "/conversations".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `PUT /conversations`
    pub async fn conversations_update(&self, body: ConversationsUpdateRequest) -> Result<ConversationsUpdateResponse> {
        let path = "/conversations".to_string();
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /conversations`
    pub async fn conversations_delete(&self, body: ConversationsDeleteRequest) -> Result<serde_json::Value> {
        let path = "/conversations".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /conversations/messages/{message_id}`
    pub async fn conversations_messages_get(&self, message_id: i64) -> Result<ConversationsMessagesGetResponse> {
        let path = format!("/conversations/messages/{message_id}", message_id = message_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /conversations/read-all`
    pub async fn conversations_read_all(&self) -> Result<ConversationsReadAllResponse> {
        let path = "/conversations/read-all".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /conversations/save`
    pub async fn conversations_save(&self, body: ConversationsSaveRequest) -> Result<serde_json::Value> {
        let path = "/conversations/save".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /conversations/search`
    pub async fn conversations_search(&self, body: ConversationsSearchRequest) -> Result<ConversationsSearchResponse> {
        let path = "/conversations/search".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /conversations/start`
    pub async fn conversations_start(&self, body: ConversationsStartRequest) -> Result<ConversationsStartResponse> {
        let path = "/conversations/start".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /conversations/{conversation_id}`
    pub async fn conversations_get(&self, conversation_id: i64) -> Result<ConversationsGetResponse> {
        let path = format!("/conversations/{conversation_id}", conversation_id = conversation_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /conversations/{conversation_id}/alerts`
    pub async fn conversations_alerts_enable(&self, conversation_id: i64) -> Result<ConversationsAlertsEnableResponse> {
        let path = format!("/conversations/{conversation_id}/alerts", conversation_id = conversation_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /conversations/{conversation_id}/alerts`
    pub async fn conversations_alerts_disable(&self, conversation_id: i64) -> Result<ConversationsAlertsDisableResponse> {
        let path = format!("/conversations/{conversation_id}/alerts", conversation_id = conversation_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /conversations/{conversation_id}/invite`
    pub async fn conversations_invite(&self, conversation_id: i64, body: ConversationsInviteRequest) -> Result<serde_json::Value> {
        let path = format!("/conversations/{conversation_id}/invite", conversation_id = conversation_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /conversations/{conversation_id}/kick`
    pub async fn conversations_kick(&self, conversation_id: i64, body: ConversationsKickRequest) -> Result<serde_json::Value> {
        let path = format!("/conversations/{conversation_id}/kick", conversation_id = conversation_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /conversations/{conversation_id}/messages`
    pub async fn conversations_messages_list(&self, conversation_id: i64, query: ConversationsMessagesListQuery) -> Result<ConversationsMessagesListResponse> {
        let path = format!("/conversations/{conversation_id}/messages", conversation_id = conversation_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /conversations/{conversation_id}/messages`
    pub async fn conversations_messages_create(&self, conversation_id: i64, body: ConversationsMessagesCreateRequest) -> Result<ConversationsMessagesCreateResponse> {
        let path = format!("/conversations/{conversation_id}/messages", conversation_id = conversation_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `PUT /conversations/{conversation_id}/messages/{message_id}`
    pub async fn conversations_messages_edit(&self, conversation_id: i64, message_id: i64, body: ConversationsMessagesEditRequest) -> Result<ConversationsMessagesEditResponse> {
        let path = format!("/conversations/{conversation_id}/messages/{message_id}", conversation_id = conversation_id, message_id = message_id);
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /conversations/{conversation_id}/messages/{message_id}`
    pub async fn conversations_messages_delete(&self, conversation_id: i64, message_id: i64) -> Result<serde_json::Value> {
        let path = format!("/conversations/{conversation_id}/messages/{message_id}", conversation_id = conversation_id, message_id = message_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /conversations/{conversation_id}/messages/{message_id}/stick`
    pub async fn conversations_messages_stick(&self, conversation_id: i64, message_id: i64) -> Result<serde_json::Value> {
        let path = format!("/conversations/{conversation_id}/messages/{message_id}/stick", conversation_id = conversation_id, message_id = message_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /conversations/{conversation_id}/messages/{message_id}/stick`
    pub async fn conversations_messages_unstick(&self, conversation_id: i64, message_id: i64) -> Result<serde_json::Value> {
        let path = format!("/conversations/{conversation_id}/messages/{message_id}/stick", conversation_id = conversation_id, message_id = message_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /conversations/{conversation_id}/read`
    pub async fn conversations_read(&self, conversation_id: i64) -> Result<serde_json::Value> {
        let path = format!("/conversations/{conversation_id}/read", conversation_id = conversation_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /conversations/{conversation_id}/star`
    pub async fn conversations_star(&self, conversation_id: i64) -> Result<ConversationsStarResponse> {
        let path = format!("/conversations/{conversation_id}/star", conversation_id = conversation_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /conversations/{conversation_id}/star`
    pub async fn conversations_unstar(&self, conversation_id: i64) -> Result<ConversationsUnstarResponse> {
        let path = format!("/conversations/{conversation_id}/star", conversation_id = conversation_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    // -- Assets --
    /// `GET /css`
    pub async fn assets_css(&self, query: AssetsCssQuery) -> Result<AssetsCssResponse> {
        let path = "/css".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Forms --
    /// `GET /forms`
    pub async fn forms_list(&self, query: FormsListQuery) -> Result<FormsListResponse> {
        let path = "/forms".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /forms/save`
    pub async fn forms_create(&self, body: FormsCreateRequest) -> Result<FormsCreateResponse> {
        let path = "/forms/save".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Forums --
    /// `GET /forums`
    pub async fn forums_list(&self, query: ForumsListQuery) -> Result<ForumsListResponse> {
        let path = "/forums".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /forums/feed/options`
    pub async fn forums_get_feed_options(&self) -> Result<ForumsGetFeedOptionsResponse> {
        let path = "/forums/feed/options".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `PUT /forums/feed/options`
    pub async fn forums_edit_feed_options(&self, body: ForumsEditFeedOptionsRequest) -> Result<serde_json::Value> {
        let path = "/forums/feed/options".to_string();
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /forums/followed`
    pub async fn forums_followed(&self, query: ForumsFollowedQuery) -> Result<ForumsFollowedResponse> {
        let path = "/forums/followed".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /forums/grouped`
    pub async fn forums_grouped(&self) -> Result<ForumsGroupedResponse> {
        let path = "/forums/grouped".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /forums/{forum_id}`
    pub async fn forums_get(&self, forum_id: i64) -> Result<ForumsGetResponse> {
        let path = format!("/forums/{forum_id}", forum_id = forum_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /forums/{forum_id}/followers`
    pub async fn forums_followers(&self, forum_id: i64) -> Result<ForumsFollowersResponse> {
        let path = format!("/forums/{forum_id}/followers", forum_id = forum_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /forums/{forum_id}/followers`
    pub async fn forums_follow(&self, forum_id: i64, body: ForumsFollowRequest) -> Result<serde_json::Value> {
        let path = format!("/forums/{forum_id}/followers", forum_id = forum_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /forums/{forum_id}/followers`
    pub async fn forums_unfollow(&self, forum_id: i64) -> Result<serde_json::Value> {
        let path = format!("/forums/{forum_id}/followers", forum_id = forum_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    // -- Link Forums --
    /// `GET /link-forums`
    pub async fn links_list(&self) -> Result<LinksListResponse> {
        let path = "/link-forums".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /link-forums/{link_id}`
    pub async fn links_get(&self, link_id: i64) -> Result<LinksGetResponse> {
        let path = format!("/link-forums/{link_id}", link_id = link_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    // -- Navigation --
    /// `GET /navigation`
    pub async fn navigation_list(&self, query: NavigationListQuery) -> Result<NavigationListResponse> {
        let path = "/navigation".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Notifications --
    /// `GET /notifications`
    pub async fn notifications_list(&self, query: NotificationsListQuery) -> Result<NotificationsListResponse> {
        let path = "/notifications".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /notifications/read`
    pub async fn notifications_read(&self, body: NotificationsReadRequest) -> Result<serde_json::Value> {
        let path = "/notifications/read".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /notifications/{notification_id}/content`
    pub async fn notifications_get(&self, notification_id: i64) -> Result<NotificationsGetResponse> {
        let path = format!("/notifications/{notification_id}/content", notification_id = notification_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    // -- Authentication --
    /// `POST /oauth/token`
    pub async fn oauth_token(&self, body: OauthTokenMultipart) -> Result<OauthTokenResponse> {
        let path = "/oauth/token".to_string();
        self.client.request_multipart(HttpMethod::Post, &path, None::<&()>, &body).await
    }

    // -- Pages --
    /// `GET /pages`
    pub async fn pages_list(&self, query: PagesListQuery) -> Result<PagesListResponse> {
        let path = "/pages".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /pages/{page_id}`
    pub async fn pages_get(&self, page_id: i64) -> Result<PagesGetResponse> {
        let path = format!("/pages/{page_id}", page_id = page_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    // -- Posts --
    /// `GET /posts`
    pub async fn posts_list(&self, query: PostsListQuery) -> Result<PostsListResponse> {
        let path = "/posts".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /posts`
    pub async fn posts_create(&self, body: PostsCreateRequest) -> Result<PostsCreateResponse> {
        let path = "/posts".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Post comments --
    /// `GET /posts/comments`
    pub async fn posts_comments_get(&self, query: PostsCommentsGetQuery) -> Result<PostsCommentsGetResponse> {
        let path = "/posts/comments".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /posts/comments`
    pub async fn posts_comments_create(&self, body: PostsCommentsCreateRequest) -> Result<PostsCommentsCreateResponse> {
        let path = "/posts/comments".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `PUT /posts/comments`
    pub async fn posts_comments_edit(&self, body: PostsCommentsEditRequest) -> Result<PostsCommentsEditResponse> {
        let path = "/posts/comments".to_string();
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /posts/comments`
    pub async fn posts_comments_delete(&self, body: PostsCommentsDeleteRequest) -> Result<serde_json::Value> {
        let path = "/posts/comments".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    // -- Posts --
    /// `POST /posts/comments/report`
    pub async fn posts_comments_report(&self, body: PostsCommentsReportRequest) -> Result<serde_json::Value> {
        let path = "/posts/comments/report".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /posts/{post_id}`
    pub async fn posts_get(&self, post_id: i64) -> Result<PostsGetResponse> {
        let path = format!("/posts/{post_id}", post_id = post_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `PUT /posts/{post_id}`
    pub async fn posts_edit(&self, post_id: i64, body: PostsEditRequest) -> Result<PostsEditResponse> {
        let path = format!("/posts/{post_id}", post_id = post_id);
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /posts/{post_id}`
    pub async fn posts_delete(&self, post_id: i64, body: PostsDeleteRequest) -> Result<serde_json::Value> {
        let path = format!("/posts/{post_id}", post_id = post_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /posts/{post_id}/likes`
    pub async fn posts_likes(&self, post_id: i64, query: PostsLikesQuery) -> Result<PostsLikesResponse> {
        let path = format!("/posts/{post_id}/likes", post_id = post_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /posts/{post_id}/likes`
    pub async fn posts_like(&self, post_id: i64) -> Result<serde_json::Value> {
        let path = format!("/posts/{post_id}/likes", post_id = post_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /posts/{post_id}/likes`
    pub async fn posts_unlike(&self, post_id: i64) -> Result<serde_json::Value> {
        let path = format!("/posts/{post_id}/likes", post_id = post_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /posts/{post_id}/report`
    pub async fn posts_report_reasons(&self, post_id: i64) -> Result<PostsReportReasonsResponse> {
        let path = format!("/posts/{post_id}/report", post_id = post_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /posts/{post_id}/report`
    pub async fn posts_report(&self, post_id: i64, body: PostsReportRequest) -> Result<serde_json::Value> {
        let path = format!("/posts/{post_id}/report", post_id = post_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Profile Posts --
    /// `POST /profile-posts`
    pub async fn profile_posts_create(&self, body: ProfilePostsCreateRequest) -> Result<ProfilePostsCreateResponse> {
        let path = "/profile-posts".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Profile Post Comments --
    /// `GET /profile-posts/comments`
    pub async fn profile_posts_comments_list(&self, query: ProfilePostsCommentsListQuery) -> Result<ProfilePostsCommentsListResponse> {
        let path = "/profile-posts/comments".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /profile-posts/comments`
    pub async fn profile_posts_comments_create(&self, body: ProfilePostsCommentsCreateRequest) -> Result<ProfilePostsCommentsCreateResponse> {
        let path = "/profile-posts/comments".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `PUT /profile-posts/comments`
    pub async fn profile_posts_comments_edit(&self, body: ProfilePostsCommentsEditRequest) -> Result<ProfilePostsCommentsEditResponse> {
        let path = "/profile-posts/comments".to_string();
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /profile-posts/comments`
    pub async fn profile_posts_comments_delete(&self, body: ProfilePostsCommentsDeleteRequest) -> Result<serde_json::Value> {
        let path = "/profile-posts/comments".to_string();
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /profile-posts/comments/{comment_id}/report`
    pub async fn profile_posts_comments_report(&self, comment_id: i64, body: ProfilePostsCommentsReportRequest) -> Result<serde_json::Value> {
        let path = format!("/profile-posts/comments/{comment_id}/report", comment_id = comment_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    // -- Profile Posts --
    /// `GET /profile-posts/{profile_post_id}`
    pub async fn profile_posts_get(&self, profile_post_id: i64) -> Result<ProfilePostsGetResponse> {
        let path = format!("/profile-posts/{profile_post_id}", profile_post_id = profile_post_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `PUT /profile-posts/{profile_post_id}`
    pub async fn profile_posts_edit(&self, profile_post_id: i64, body: ProfilePostsEditRequest) -> Result<ProfilePostsEditResponse> {
        let path = format!("/profile-posts/{profile_post_id}", profile_post_id = profile_post_id);
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /profile-posts/{profile_post_id}`
    pub async fn profile_posts_delete(&self, profile_post_id: i64, query: ProfilePostsDeleteQuery) -> Result<serde_json::Value> {
        let path = format!("/profile-posts/{profile_post_id}", profile_post_id = profile_post_id);
        self.client.request_json(HttpMethod::Delete, &path, Some(&query), None::<&()>).await
    }

    // -- Profile Post Comments --
    /// `GET /profile-posts/{profile_post_id}/comments/{comment_id}`
    pub async fn profile_posts_comments_get(&self, profile_post_id: i64, comment_id: i64) -> Result<ProfilePostsCommentsGetResponse> {
        let path = format!("/profile-posts/{profile_post_id}/comments/{comment_id}", profile_post_id = profile_post_id, comment_id = comment_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    // -- Profile Posts --
    /// `GET /profile-posts/{profile_post_id}/likes`
    pub async fn profile_posts_likes(&self, profile_post_id: i64) -> Result<ProfilePostsLikesResponse> {
        let path = format!("/profile-posts/{profile_post_id}/likes", profile_post_id = profile_post_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /profile-posts/{profile_post_id}/likes`
    pub async fn profile_posts_like(&self, profile_post_id: i64) -> Result<serde_json::Value> {
        let path = format!("/profile-posts/{profile_post_id}/likes", profile_post_id = profile_post_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /profile-posts/{profile_post_id}/likes`
    pub async fn profile_posts_unlike(&self, profile_post_id: i64) -> Result<serde_json::Value> {
        let path = format!("/profile-posts/{profile_post_id}/likes", profile_post_id = profile_post_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /profile-posts/{profile_post_id}/report`
    pub async fn profile_posts_report_reasons(&self, profile_post_id: i64) -> Result<ProfilePostsReportReasonsResponse> {
        let path = format!("/profile-posts/{profile_post_id}/report", profile_post_id = profile_post_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /profile-posts/{profile_post_id}/report`
    pub async fn profile_posts_report(&self, profile_post_id: i64, body: ProfilePostsReportRequest) -> Result<serde_json::Value> {
        let path = format!("/profile-posts/{profile_post_id}/report", profile_post_id = profile_post_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /profile-posts/{profile_post_id}/stick`
    pub async fn profile_posts_stick(&self, profile_post_id: i64) -> Result<serde_json::Value> {
        let path = format!("/profile-posts/{profile_post_id}/stick", profile_post_id = profile_post_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /profile-posts/{profile_post_id}/stick`
    pub async fn profile_posts_unstick(&self, profile_post_id: i64) -> Result<serde_json::Value> {
        let path = format!("/profile-posts/{profile_post_id}/stick", profile_post_id = profile_post_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    // -- Searching --
    /// `POST /search`
    pub async fn search_all(&self, body: SearchAllRequest) -> Result<SearchAllResponse> {
        let path = "/search".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /search/posts`
    pub async fn search_posts(&self, body: SearchPostsRequest) -> Result<SearchPostsResponse> {
        let path = "/search/posts".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /search/profile-posts`
    pub async fn search_profile_posts(&self, body: SearchProfilePostsRequest) -> Result<SearchProfilePostsResponse> {
        let path = "/search/profile-posts".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /search/tagged`
    pub async fn search_tagged(&self, body: SearchTaggedRequest) -> Result<SearchTaggedResponse> {
        let path = "/search/tagged".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /search/threads`
    pub async fn search_threads(&self, body: SearchThreadsRequest) -> Result<SearchThreadsResponse> {
        let path = "/search/threads".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /search/users`
    pub async fn search_users(&self, body: SearchUsersRequest) -> Result<SearchUsersResponse> {
        let path = "/search/users".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /search/{search_id}/results`
    pub async fn search_results(&self, search_id: serde_json::Value, body: SearchResultsRequest) -> Result<SearchResultsResponse> {
        let path = format!("/search/{search_id}/results", search_id = search_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, Some(&body)).await
    }

    // -- Content Tagging --
    /// `GET /tags`
    pub async fn tags_popular(&self) -> Result<TagsPopularResponse> {
        let path = "/tags".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /tags/find`
    pub async fn tags_find(&self, query: TagsFindQuery) -> Result<TagsFindResponse> {
        let path = "/tags/find".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /tags/list`
    pub async fn tags_list(&self, query: TagsListQuery) -> Result<TagsListResponse> {
        let path = "/tags/list".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /tags/{tag_id}`
    pub async fn tags_get(&self, tag_id: i64, query: TagsGetQuery) -> Result<TagsGetResponse> {
        let path = format!("/tags/{tag_id}", tag_id = tag_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Threads --
    /// `GET /threads`
    pub async fn threads_list(&self, query: ThreadsListQuery) -> Result<ThreadsListResponse> {
        let path = "/threads".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /threads`
    pub async fn threads_create(&self, body: ThreadsCreateRequest) -> Result<ThreadsCreateResponse> {
        let path = "/threads".to_string();
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /threads/followed`
    pub async fn threads_followed(&self, query: ThreadsFollowedQuery) -> Result<ThreadsFollowedResponse> {
        let path = "/threads/followed".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /threads/new`
    pub async fn threads_unread(&self, query: ThreadsUnreadQuery) -> Result<ThreadsUnreadResponse> {
        let path = "/threads/new".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /threads/recent`
    pub async fn threads_recent(&self, query: ThreadsRecentQuery) -> Result<ThreadsRecentResponse> {
        let path = "/threads/recent".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /threads/{thread_id}`
    pub async fn threads_get(&self, thread_id: i64, query: ThreadsGetQuery) -> Result<ThreadsGetResponse> {
        let path = format!("/threads/{thread_id}", thread_id = thread_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `PUT /threads/{thread_id}`
    pub async fn threads_edit(&self, thread_id: i64, body: ThreadsEditRequest) -> Result<ThreadsEditResponse> {
        let path = format!("/threads/{thread_id}", thread_id = thread_id);
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /threads/{thread_id}`
    pub async fn threads_delete(&self, thread_id: i64, body: ThreadsDeleteRequest) -> Result<serde_json::Value> {
        let path = format!("/threads/{thread_id}", thread_id = thread_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /threads/{thread_id}/bump`
    pub async fn threads_bump(&self, thread_id: i64) -> Result<ThreadsBumpResponse> {
        let path = format!("/threads/{thread_id}/bump", thread_id = thread_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /threads/{thread_id}/followers`
    pub async fn threads_followers(&self, thread_id: i64) -> Result<ThreadsFollowersResponse> {
        let path = format!("/threads/{thread_id}/followers", thread_id = thread_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /threads/{thread_id}/followers`
    pub async fn threads_follow(&self, thread_id: i64, body: ThreadsFollowRequest) -> Result<serde_json::Value> {
        let path = format!("/threads/{thread_id}/followers", thread_id = thread_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `DELETE /threads/{thread_id}/followers`
    pub async fn threads_unfollow(&self, thread_id: i64) -> Result<serde_json::Value> {
        let path = format!("/threads/{thread_id}/followers", thread_id = thread_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /threads/{thread_id}/hide`
    pub async fn threads_hide(&self, thread_id: i64) -> Result<ThreadsHideResponse> {
        let path = format!("/threads/{thread_id}/hide", thread_id = thread_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /threads/{thread_id}/move`
    pub async fn threads_move(&self, thread_id: i64, body: ThreadsMoveRequest) -> Result<serde_json::Value> {
        let path = format!("/threads/{thread_id}/move", thread_id = thread_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /threads/{thread_id}/navigation`
    pub async fn threads_navigation(&self, thread_id: i64) -> Result<ThreadsNavigationResponse> {
        let path = format!("/threads/{thread_id}/navigation", thread_id = thread_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /threads/{thread_id}/poll`
    pub async fn threads_poll_get(&self, thread_id: i64) -> Result<ThreadsPollGetResponse> {
        let path = format!("/threads/{thread_id}/poll", thread_id = thread_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /threads/{thread_id}/poll/votes`
    pub async fn threads_poll_vote(&self, thread_id: i64, body: ThreadsPollVoteRequest) -> Result<serde_json::Value> {
        let path = format!("/threads/{thread_id}/poll/votes", thread_id = thread_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /threads/{thread_id}/star`
    pub async fn threads_star(&self, thread_id: i64) -> Result<serde_json::Value> {
        let path = format!("/threads/{thread_id}/star", thread_id = thread_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /threads/{thread_id}/star`
    pub async fn threads_unstar(&self, thread_id: i64) -> Result<serde_json::Value> {
        let path = format!("/threads/{thread_id}/star", thread_id = thread_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    // -- Users --
    /// `GET /users`
    pub async fn users_list(&self, query: UsersListQuery) -> Result<UsersListResponse> {
        let path = "/users".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /users/fields`
    pub async fn users_fields(&self) -> Result<UsersFieldsResponse> {
        let path = "/users/fields".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /users/find`
    pub async fn users_find(&self, query: UsersFindQuery) -> Result<UsersFindResponse> {
        let path = "/users/find".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /users/ignored`
    pub async fn users_ignored(&self, query: UsersIgnoredQuery) -> Result<UsersIgnoredResponse> {
        let path = "/users/ignored".to_string();
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /users/secret-answer/types`
    pub async fn users_secret_answer_types(&self) -> Result<UsersSecretAnswerTypesResponse> {
        let path = "/users/secret-answer/types".to_string();
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /users/{user_id}`
    pub async fn users_get(&self, user_id: ForumUserIdmodel, query: UsersGetQuery) -> Result<UsersGetResponse> {
        let path = format!("/users/{user_id}", user_id = user_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `PUT /users/{user_id}`
    pub async fn users_edit(&self, user_id: ForumUserIdmodel, body: UsersEditRequest) -> Result<serde_json::Value> {
        let path = format!("/users/{user_id}", user_id = user_id);
        self.client.request_json(HttpMethod::Put, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /users/{user_id}/avatar`
    pub async fn users_avatar_upload(&self, user_id: ForumUserIdmodel, body: UsersAvatarUploadMultipart) -> Result<UsersAvatarUploadResponse> {
        let path = format!("/users/{user_id}/avatar", user_id = user_id);
        self.client.request_multipart(HttpMethod::Post, &path, None::<&()>, &body).await
    }

    /// `DELETE /users/{user_id}/avatar`
    pub async fn users_avatar_delete(&self, user_id: ForumUserIdmodel) -> Result<serde_json::Value> {
        let path = format!("/users/{user_id}/avatar", user_id = user_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /users/{user_id}/avatar/crop`
    pub async fn users_avatar_crop(&self, user_id: ForumUserIdmodel, body: UsersAvatarCropRequest) -> Result<UsersAvatarCropResponse> {
        let path = format!("/users/{user_id}/avatar/crop", user_id = user_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `POST /users/{user_id}/background`
    pub async fn users_background_upload(&self, user_id: ForumUserIdmodel, body: UsersBackgroundUploadMultipart) -> Result<UsersBackgroundUploadResponse> {
        let path = format!("/users/{user_id}/background", user_id = user_id);
        self.client.request_multipart(HttpMethod::Post, &path, None::<&()>, &body).await
    }

    /// `DELETE /users/{user_id}/background`
    pub async fn users_background_delete(&self, user_id: ForumUserIdmodel) -> Result<serde_json::Value> {
        let path = format!("/users/{user_id}/background", user_id = user_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `POST /users/{user_id}/background/crop`
    pub async fn users_background_crop(&self, user_id: ForumUserIdmodel, body: UsersBackgroundCropRequest) -> Result<UsersBackgroundCropResponse> {
        let path = format!("/users/{user_id}/background/crop", user_id = user_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, Some(&body)).await
    }

    /// `GET /users/{user_id}/claims`
    pub async fn users_claims(&self, user_id: ForumUserIdmodel, query: UsersClaimsQuery) -> Result<UsersClaimsResponse> {
        let path = format!("/users/{user_id}/claims", user_id = user_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /users/{user_id}/followers`
    pub async fn users_followers(&self, user_id: ForumUserIdmodel, query: UsersFollowersQuery) -> Result<UsersFollowersResponse> {
        let path = format!("/users/{user_id}/followers", user_id = user_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /users/{user_id}/followers`
    pub async fn users_follow(&self, user_id: ForumUserIdmodel) -> Result<serde_json::Value> {
        let path = format!("/users/{user_id}/followers", user_id = user_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `DELETE /users/{user_id}/followers`
    pub async fn users_unfollow(&self, user_id: ForumUserIdmodel) -> Result<serde_json::Value> {
        let path = format!("/users/{user_id}/followers", user_id = user_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /users/{user_id}/followings`
    pub async fn users_followings(&self, user_id: ForumUserIdmodel, query: UsersFollowingsQuery) -> Result<UsersFollowingsResponse> {
        let path = format!("/users/{user_id}/followings", user_id = user_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `POST /users/{user_id}/ignore`
    pub async fn users_ignore(&self, user_id: ForumUserIdmodel) -> Result<serde_json::Value> {
        let path = format!("/users/{user_id}/ignore", user_id = user_id);
        self.client.request_json(HttpMethod::Post, &path, None::<&()>, None::<&()>).await
    }

    /// `PUT /users/{user_id}/ignore`
    pub async fn users_ignore_edit(&self, user_id: ForumUserIdmodel, query: UsersIgnoreEditQuery) -> Result<serde_json::Value> {
        let path = format!("/users/{user_id}/ignore", user_id = user_id);
        self.client.request_json(HttpMethod::Put, &path, Some(&query), None::<&()>).await
    }

    /// `DELETE /users/{user_id}/ignore`
    pub async fn users_unignore(&self, user_id: ForumUserIdmodel) -> Result<serde_json::Value> {
        let path = format!("/users/{user_id}/ignore", user_id = user_id);
        self.client.request_json(HttpMethod::Delete, &path, None::<&()>, None::<&()>).await
    }

    /// `GET /users/{user_id}/likes`
    pub async fn users_likes(&self, user_id: ForumUserIdmodel, query: UsersLikesQuery) -> Result<UsersLikesResponse> {
        let path = format!("/users/{user_id}/likes", user_id = user_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Profile Posts --
    /// `GET /users/{user_id}/profile-posts`
    pub async fn profile_posts_list(&self, user_id: ForumUserIdmodel, query: ProfilePostsListQuery) -> Result<ProfilePostsListResponse> {
        let path = format!("/users/{user_id}/profile-posts", user_id = user_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    // -- Users --
    /// `GET /users/{user_id}/timeline`
    pub async fn users_contents(&self, user_id: ForumUserIdmodel, query: UsersContentsQuery) -> Result<UsersContentsResponse> {
        let path = format!("/users/{user_id}/timeline", user_id = user_id);
        self.client.request_json(HttpMethod::Get, &path, Some(&query), None::<&()>).await
    }

    /// `GET /users/{user_id}/trophies`
    pub async fn users_trophies(&self, user_id: ForumUserIdmodel) -> Result<UsersTrophiesResponse> {
        let path = format!("/users/{user_id}/trophies", user_id = user_id);
        self.client.request_json(HttpMethod::Get, &path, None::<&()>, None::<&()>).await
    }

}
