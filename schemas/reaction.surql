DEFINE TABLE reaction SCHEMAFULL
  PERMISSIONS
    FOR select WHERE fn::can_view_post($auth.id, post)
    FOR create, update, delete WHERE user = $auth.id;

DEFINE FIELD post ON TABLE reaction TYPE record(post) ASSERT $value != NONE;
DEFINE FIELD user ON TABLE reaction TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD type ON TABLE reaction TYPE string ASSERT $value IN ['like', 'love', 'haha', 'wow', 'sad', 'angry'];
DEFINE FIELD created_at ON TABLE reaction TYPE datetime VALUE $before OR time::now();

DEFINE INDEX post_user ON TABLE reaction COLUMNS post, user UNIQUE;