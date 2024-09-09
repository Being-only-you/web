DEFINE TABLE post SCHEMAFULL
  PERMISSIONS
    FOR select WHERE fn::can_view_post($auth.id, id)
    FOR create, update, delete WHERE author = $auth.id;

DEFINE FIELD author ON TABLE post TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD profile_type ON TABLE post TYPE string ASSERT $value IN ['personal', 'professional'];
DEFINE FIELD content ON TABLE post TYPE string;
DEFINE FIELD media_type ON TABLE post TYPE string;
DEFINE FIELD media_url ON TABLE post TYPE string;
DEFINE FIELD created_at ON TABLE post TYPE datetime VALUE $before OR time::now();
DEFINE FIELD updated_at ON TABLE post TYPE datetime VALUE time::now();

DEFINE INDEX author_profile ON TABLE post COLUMNS author, profile_type;