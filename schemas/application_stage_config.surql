DEFINE TABLE application_stage_config SCHEMAFULL
  PERMISSIONS
    FOR select WHERE true
    FOR update, delete WHERE ($scope = 'admin')
      OR ($scope = 'business' && business = $auth.id)
    FOR create WHERE $scope = 'admin' OR $scope = 'business';

DEFINE FIELD business ON application_stage_config TYPE record(business) ASSERT $value != NONE;
DEFINE FIELD name ON application_stage_config TYPE string;
DEFINE FIELD order ON application_stage_config TYPE int;
DEFINE FIELD description ON application_stage_config TYPE string;
DEFINE FIELD required_fields ON application_stage_config TYPE array;
DEFINE FIELD optional_fields ON application_stage_config TYPE array;
DEFINE FIELD created_at ON application_stage_config TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON application_stage_config TYPE datetime DEFAULT time::now();

DEFINE INDEX business_stage_order ON application_stage_config FIELDS business, order UNIQUE;