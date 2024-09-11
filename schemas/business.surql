DEFINE TABLE business SCHEMAFULL
  PERMISSIONS
    FOR select, update WHERE ($scope = 'admin')
      OR ($scope = 'business' && id = $auth.id)
    FOR delete WHERE $scope = 'admin'
    FOR create WHERE $scope = 'admin';

DEFINE FIELD name ON TABLE business TYPE string;
DEFINE FIELD description ON TABLE business TYPE string;
DEFINE FIELD industry ON TABLE business TYPE string;
DEFINE FIELD size ON TABLE business TYPE string;
DEFINE FIELD location ON TABLE business TYPE string;
DEFINE FIELD website ON TABLE business TYPE string;
DEFINE FIELD hiring_process ON TABLE business TYPE array;
DEFINE FIELD ats_settings ON TABLE business TYPE object;
DEFINE FIELD created ON TABLE business TYPE datetime VALUE $before OR time::now();
DEFINE FIELD updated ON TABLE business TYPE datetime VALUE time::now();

DEFINE INDEX name ON TABLE business COLUMNS name UNIQUE;