DEFINE TABLE job_posting SCHEMAFULL
  PERMISSIONS
    FOR select WHERE true
    FOR update, delete WHERE ($scope = 'admin')
      OR ($scope = 'business' && business = $auth.id)
    FOR create WHERE $scope = 'admin' OR $scope = 'business';

DEFINE FIELD business ON job_posting TYPE record(business) ASSERT $value != NONE;
DEFINE FIELD title ON job_posting TYPE string;
DEFINE FIELD description ON job_posting TYPE string;
DEFINE FIELD requirements ON job_posting TYPE array;
DEFINE FIELD responsibilities ON job_posting TYPE array;
DEFINE FIELD location ON job_posting TYPE string;
DEFINE FIELD salary_range ON job_posting TYPE object;
DEFINE FIELD job_type ON job_posting TYPE string;
DEFINE FIELD department ON job_posting TYPE string;
DEFINE FIELD industry ON job_posting TYPE string;
DEFINE FIELD experience_level ON job_posting TYPE string;
DEFINE FIELD education_level ON job_posting TYPE string;
DEFINE FIELD skills ON job_posting TYPE array;
DEFINE FIELD keywords ON job_posting TYPE array;
DEFINE FIELD status ON job_posting TYPE string ASSERT $value INSIDE ['draft', 'published', 'closed'];
DEFINE FIELD created_at ON job_posting TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON job_posting TYPE datetime DEFAULT time::now();
DEFINE FIELD similarity_vector ON job_posting TYPE vector(1536);

DEFINE INDEX business_title ON job_posting FIELDS business, title;
DEFINE INDEX job_posting_keywords ON job_posting FIELDS keywords SEARCH ANALYZER ascii BM25;
DEFINE INDEX job_posting_skills ON job_posting FIELDS skills;
DEFINE INDEX job_posting_similarity ON job_posting FIELDS similarity_vector VECTOR 1536 COSINE DIST;