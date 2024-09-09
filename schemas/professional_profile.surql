DEFINE TABLE professional_profile SCHEMAFULL
  PERMISSIONS
    FOR select WHERE true
    FOR update WHERE ($scope = 'user' && user = $auth.id) OR $scope = 'admin'
    FOR delete WHERE ($scope = 'user' && user = $auth.id) OR $scope = 'admin'
    FOR create WHERE $scope = 'user' OR $scope = 'admin';

DEFINE FIELD user ON TABLE professional_profile TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD name ON TABLE professional_profile TYPE string;
DEFINE FIELD title ON TABLE professional_profile TYPE string;
DEFINE FIELD company ON TABLE professional_profile TYPE string;
DEFINE FIELD summary ON TABLE professional_profile TYPE string;
DEFINE FIELD skills ON TABLE professional_profile TYPE array;
DEFINE FIELD experience ON TABLE professional_profile TYPE array;
DEFINE FIELD education ON TABLE professional_profile TYPE array;
DEFINE FIELD certifications ON TABLE professional_profile TYPE array;
DEFINE FIELD connections ON TABLE professional_profile TYPE array;
DEFINE FIELD cv ON TABLE professional_profile TYPE record(cv);

DEFINE INDEX user ON TABLE professional_profile COLUMNS user UNIQUE;