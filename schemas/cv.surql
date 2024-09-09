DEFINE TABLE cv SCHEMAFULL
  PERMISSIONS
    FOR select, update WHERE ($scope = 'user' && user = $auth.id)
      OR $scope = 'admin'
    FOR delete WHERE ($scope = 'user' && user = $auth.id) OR $scope = 'admin'
    FOR create WHERE $scope = 'user' OR $scope = 'admin';

DEFINE FIELD user ON TABLE cv TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD full_name ON TABLE cv TYPE string;
DEFINE FIELD email ON TABLE cv TYPE string ASSERT is::email($value);
DEFINE FIELD phone ON TABLE cv TYPE string;
DEFINE FIELD summary ON TABLE cv TYPE string;
DEFINE FIELD experience ON TABLE cv TYPE array;
DEFINE FIELD education ON TABLE cv TYPE array;
DEFINE FIELD skills ON TABLE cv TYPE array;
DEFINE FIELD created ON TABLE cv TYPE datetime VALUE $before OR time::now();
DEFINE FIELD updated ON TABLE cv TYPE datetime VALUE time::now();

DEFINE INDEX user ON TABLE cv COLUMNS user;