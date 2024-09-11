DEFINE TABLE cv SCHEMAFULL
  PERMISSIONS
    FOR select WHERE ($scope = 'user' && user = $auth.id)
      OR $scope = 'admin'
      OR ($scope = 'business' && id IN (SELECT cv FROM application WHERE job_posting.business = $auth.id))
    FOR update WHERE ($scope = 'user' && user = $auth.id) OR $scope = 'admin'
    FOR delete WHERE ($scope = 'user' && user = $auth.id) OR $scope = 'admin'
    FOR create WHERE $scope = 'user' OR $scope = 'admin';

DEFINE FIELD user ON cv TYPE record(user) ASSERT $value != NONE;
DEFINE FIELD full_name ON cv TYPE string;
DEFINE FIELD email ON cv TYPE string ASSERT is::email($value);
DEFINE FIELD phone ON cv TYPE string;
DEFINE FIELD summary ON cv TYPE string;
DEFINE FIELD experience ON cv TYPE array;
DEFINE FIELD education ON cv TYPE array;
DEFINE FIELD skills ON cv TYPE array;
DEFINE FIELD certifications ON cv TYPE array;
DEFINE FIELD languages ON cv TYPE array;
DEFINE FIELD projects ON cv TYPE array;
DEFINE FIELD publications ON cv TYPE array;
DEFINE FIELD references ON cv TYPE array;
DEFINE FIELD custom_sections ON cv TYPE object;
DEFINE FIELD keywords ON cv TYPE array;
DEFINE FIELD similarity_vector ON cv TYPE vector(1536);
DEFINE FIELD document_url ON cv TYPE string;
DEFINE FIELD created_at ON cv TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON cv TYPE datetime DEFAULT time::now();
DEFINE FIELD version ON cv TYPE int VALUE $before.version + 1 OR 1;

DEFINE INDEX cv_user ON cv FIELDS user UNIQUE;
DEFINE INDEX cv_email ON cv FIELDS email;
DEFINE INDEX cv_skills ON cv FIELDS skills;
DEFINE INDEX cv_similarity ON cv FIELDS similarity_vector VECTOR 1536 COSINE DIST;
DEFINE INDEX cv_keywords ON cv FIELDS keywords SEARCH ANALYZER ascii BM25;