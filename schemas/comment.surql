DEFINE TABLE comment SCHEMAFULL
  PERMISSIONS
    FOR select WHERE fn::can_view_comment($auth.id, id)
    FOR create, update, delete WHERE author = $auth.id;

DEFINE FIELD post ON TABLE comment TYPE record(post) ASSERT $value != NONE;
DEFINE FIELD author ON TABLE comment TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD content ON TABLE comment TYPE string;
DEFINE FIELD created_at ON TABLE comment TYPE datetime VALUE $before OR time::now();
DEFINE FIELD updated_at ON TABLE comment TYPE datetime VALUE time::now();

DEFINE INDEX post_author ON TABLE comment COLUMNS post, author;