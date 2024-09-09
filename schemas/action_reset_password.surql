DEFINE TABLE action_reset_password SCHEMAFULL DROP PERMISSIONS FULL;
DEFINE FIELD email ON TABLE action_reset_password TYPE string ASSERT is::email($value);
DEFINE FIELD secret ON TABLE action_reset_password TYPE string;
DEFINE FIELD password ON TABLE action_reset_password TYPE string VALUE crypto::argon2::generate($value);