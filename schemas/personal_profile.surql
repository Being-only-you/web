DEFINE TABLE personal_profile SCHEMAFULL
  PERMISSIONS
    FOR select, update WHERE ($scope = 'user' && user = $auth.id)
      OR $scope = 'admin'
    FOR delete WHERE ($scope = 'user' && user = $auth.id) OR $scope = 'admin'
    FOR create WHERE $scope = 'user' OR $scope = 'admin';

DEFINE FIELD user ON TABLE personal_profile TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD name ON TABLE personal_profile TYPE string;
DEFINE FIELD bio ON TABLE personal_profile TYPE string;
DEFINE FIELD birthdate ON TABLE personal_profile TYPE datetime;
DEFINE FIELD location ON TABLE personal_profile TYPE string;
DEFINE FIELD interests ON TABLE personal_profile TYPE array;
DEFINE FIELD friends ON TABLE personal_profile TYPE array;
DEFINE FIELD friend_requests ON TABLE personal_profile TYPE array;
DEFINE FIELD qr_code ON TABLE personal_profile TYPE string;

DEFINE INDEX user ON TABLE personal_profile COLUMNS user UNIQUE;