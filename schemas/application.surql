DEFINE TABLE application SCHEMAFULL
  PERMISSIONS
    FOR select WHERE ($scope = 'user' && applicant = $auth.id)
      OR ($scope = 'admin')
      OR ($scope = 'business' && job_posting.business = $auth.id)
    FOR update WHERE ($scope = 'admin')
      OR ($scope = 'business' && job_posting.business = $auth.id)
    FOR delete WHERE $scope = 'admin'
    FOR create WHERE $scope = 'user' OR $scope = 'admin';

DEFINE FIELD applicant ON application TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD job_posting ON application TYPE record(job_posting) ASSERT $value != NONE;
DEFINE FIELD cv ON application TYPE record(cv) ASSERT $value != NONE;
DEFINE FIELD cover_letter ON application TYPE string;
DEFINE FIELD current_stage ON application TYPE string;
DEFINE FIELD stages ON application TYPE array;
DEFINE FIELD stage_history ON application TYPE array;
DEFINE FIELD ats_score ON application TYPE float;
DEFINE FIELD created_at ON application TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON application TYPE datetime DEFAULT time::now();

DEFINE INDEX applicant_job ON application FIELDS applicant, job_posting UNIQUE;
DEFINE INDEX application_current_stage ON application FIELDS current_stage;