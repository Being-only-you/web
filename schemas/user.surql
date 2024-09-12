DEFINE TABLE user SCHEMAFULL 
  PERMISSIONS 
    FOR select, update WHERE ($scope = 'user' && id = $auth.id) 
      OR $scope = 'admin' 
    FOR delete WHERE ($scope = 'user' && id = $auth.id) OR $scope = 'admin' 
    FOR create WHERE $scope = 'admin';

DEFINE FIELD email ON TABLE user TYPE string ASSERT string::is::email($value);
DEFINE FIELD username ON TABLE user TYPE string ASSERT $value = /^[a-z0-9](?:[a-z0-9._-]{1,18}[a-z0-9.])$/;
DEFINE FIELD password ON TABLE user TYPE string
  PERMISSIONS 
    FOR select NONE 
    FOR update WHERE $scope = 'admin' 
      OR string::len(<string> (select * from user where id = $auth.id and crypto::argon2::compare(password, $oldpassword))) > 2
;

DEFINE FIELD created ON TABLE user TYPE datetime VALUE $before OR time::now();
DEFINE FIELD updated ON TABLE user TYPE datetime VALUE time::now();

DEFINE FIELD personal_profile ON TABLE user TYPE record(personal_profile);
DEFINE FIELD professional_profile ON TABLE user TYPE record(professional_profile);

DEFINE INDEX email ON TABLE user COLUMNS email UNIQUE;
DEFINE INDEX username ON TABLE user COLUMNS username UNIQUE;