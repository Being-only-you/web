DEFINE TABLE waitlist SCHEMAFULL
  PERMISSIONS
    FOR select, update, delete, create WHERE $scope = 'admin';

DEFINE FIELD created ON waitlist TYPE datetime VALUE $before OR time::now();
DEFINE FIELD email ON waitlist TYPE string ASSERT is::email($value);
DEFINE FIELD name ON waitlist TYPE string;
DEFINE FIELD updated ON waitlist TYPE datetime VALUE time::now();

DEFINE INDEX email ON TABLE waitlist COLUMNS email UNIQUE;