-- Existing functions
DEFINE FUNCTION fn::can_view_post($user_id: string, $post_id: record(post)) {
    LET $post = (SELECT * FROM type::table($post_id) WHERE id = $post_id);
    RETURN $post.profile_type = 'professional' OR 
           ($post.profile_type = 'personal' AND 
            $post.author IN (SELECT personal_connections FROM user WHERE id = $user_id));
}

DEFINE FUNCTION fn::can_view_comment($user_id: string, $comment_id: record(comment)) {
    LET $comment = (SELECT * FROM type::table($comment_id) WHERE id = $comment_id);
    RETURN fn::can_view_post($user_id, $comment.post);
}

DEFINE FUNCTION fn::get_viewable_posts($user_id: string) {
    RETURN (
        SELECT * FROM post 
        WHERE profile_type = 'professional' OR 
              (profile_type = 'personal' AND 
               author IN (SELECT personal_connections FROM user WHERE id = $user_id))
    );
}

DEFINE FUNCTION fn::get_viewable_comments($user_id: string) {
    RETURN (
        SELECT * FROM comment 
        WHERE fn::can_view_post($user_id, post)
    );
}

-- New CV-specific functions
DEFINE FUNCTION fn::update_cv_similarity_vector($cv_id: record(cv)) {
    LET $cv = (SELECT * FROM cv WHERE id = $cv_id);
    
    LET $vector = ml::encode_text(string::join([
        $cv.summary,
        string::join(array::map($cv.experience, function($exp) {
            return $exp.title + ' ' + $exp.company + ' ' + $exp.description;
        }), ' '),
        string::join(array::map($cv.education, function($edu) {
            return $edu.degree + ' ' + $edu.institution;
        }), ' '),
        string::join($cv.skills, ' '),
        string::join($cv.certifications, ' '),
        string::join($cv.languages, ' '),
        string::join(array::map($cv.projects, function($proj) {
            return $proj.name + ' ' + $proj.description;
        }), ' '),
        string::join(array::map($cv.publications, function($pub) {
            return $pub.title + ' ' + $pub.publisher;
        }), ' ')
    ], ' '));
    
    UPDATE $cv_id SET similarity_vector = $vector;
    
    RETURN $vector;
};

DEFINE FUNCTION fn::calculate_cv_job_match_score($cv_id: record(cv), $job_posting_id: record(job_posting)) {
    LET $cv = (SELECT * FROM cv WHERE id = $cv_id);
    LET $job_posting = (SELECT * FROM job_posting WHERE id = $job_posting_id);
    
    LET $keyword_match_score = array::len(array::intersect($job_posting.keywords, $cv.keywords)) / array::len($job_posting.keywords);
    
    LET $experience_score = math::min(array::len($cv.experience) / 5, 1);
    
    LET $education_score = array::len(array::filter($cv.education, function($edu) {
        return $edu.level IN ['bachelor', 'master', 'phd'];
    })) / 3;
    
    LET $skill_match_score = array::len(array::intersect($job_posting.requirements, $cv.skills)) / array::len($job_posting.requirements);
    
    LET $match_score = ($keyword_match_score * 0.3 + $experience_score * 0.2 + $education_score * 0.2 + $skill_match_score * 0.3) * 100;
    
    RETURN $match_score;
};

DEFINE FUNCTION fn::find_similar_cvs($cv_id: record(cv), $limit: number) {
    LET $cv = (SELECT * FROM cv WHERE id = $cv_id);
    
    RETURN (
        SELECT *,
        vector::similarity(similarity_vector, $cv.similarity_vector) AS similarity
        FROM cv
        WHERE id != $cv_id
        ORDER BY similarity DESC
        LIMIT $limit
    );
};

DEFINE FUNCTION fn::recommend_jobs_for_cv($cv_id: record(cv), $limit: number) {
    LET $cv = (SELECT * FROM cv WHERE id = $cv_id);
    
    RETURN (
        SELECT *,
        fn::calculate_cv_job_match_score($cv_id, id) AS match_score
        FROM job_posting
        WHERE status = 'published'
        ORDER BY match_score DESC
        LIMIT $limit
    );
};

DEFINE FUNCTION fn::update_cv_keywords($cv_id: record(cv)) {
    LET $cv = (SELECT * FROM cv WHERE id = $cv_id);
    
    LET $text = string::join([
        $cv.summary,
        string::join(array::map($cv.experience, function($exp) {
            return $exp.title + ' ' + $exp.company + ' ' + $exp.description;
        }), ' '),
        string::join(array::map($cv.education, function($edu) {
            return $edu.degree + ' ' + $edu.institution;
        }), ' '),
        string::join($cv.skills, ' '),
        string::join($cv.certifications, ' '),
        string::join($cv.languages, ' '),
        string::join(array::map($cv.projects, function($proj) {
            return $proj.name + ' ' + $proj.description;
        }), ' '),
        string::join(array::map($cv.publications, function($pub) {
            return $pub.title + ' ' + $pub.publisher;
        }), ' ')
    ], ' ');
    
    LET $keywords = ml::extract_keywords($text, 20);
    
    UPDATE $cv_id SET keywords = $keywords;
    
    RETURN $keywords;
};