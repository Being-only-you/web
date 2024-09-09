-- Function to check if a user has permission to view a post
DEFINE FUNCTION fn::can_view_post($user_id: string, $post_id: record(post)) {
    LET $post = (SELECT * FROM type::table($post_id) WHERE id = $post_id);
    RETURN $post.profile_type = 'professional' OR 
           ($post.profile_type = 'personal' AND 
            $post.author IN (SELECT personal_connections FROM user WHERE id = $user_id));
}

-- Function to check if a user has permission to view a comment
DEFINE FUNCTION fn::can_view_comment($user_id: string, $comment_id: record(comment)) {
    LET $comment = (SELECT * FROM type::table($comment_id) WHERE id = $comment_id);
    RETURN fn::can_view_post($user_id, $comment.post);
}

-- Function to get viewable posts for a user
DEFINE FUNCTION fn::get_viewable_posts($user_id: string) {
    RETURN (
        SELECT * FROM post 
        WHERE profile_type = 'professional' OR 
              (profile_type = 'personal' AND 
               author IN (SELECT personal_connections FROM user WHERE id = $user_id))
    );
}

-- Function to get viewable comments for a user
DEFINE FUNCTION fn::get_viewable_comments($user_id: string) {
    RETURN (
        SELECT * FROM comment 
        WHERE fn::can_view_post($user_id, post)
    );
}