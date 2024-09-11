DEFINE FUNCTION fn::initiate_application($applicant_id: record(user), $job_posting_id: record(job_posting), $cv_id: record(cv), $cover_letter: string) {
    LET $business = (SELECT business FROM job_posting WHERE id = $job_posting_id);
    LET $initial_stage = (SELECT name FROM application_stage_config WHERE business = $business ORDER BY order ASC LIMIT 1);

    CREATE application SET
        applicant = $applicant_id,
        job_posting = $job_posting_id,
        cv = $cv_id,
        cover_letter = $cover_letter,
        current_stage = $initial_stage.name,
        stages = [(SELECT name FROM application_stage_config WHERE business = $business ORDER BY order ASC)],
        stage_history = [{
            stage: $initial_stage.name,
            timestamp: time::now(),
            notes: "Application submitted"
        }],
        ats_score = fn::calculate_initial_ats_score($cv_id, $job_posting_id)
};

DEFINE FUNCTION fn::move_to_next_stage($application_id: record(application), $notes: string) {
    LET $application = (SELECT * FROM application WHERE id = $application_id);
    LET $current_stage_index = array::index_of($application.stages, $application.current_stage);
    LET $next_stage = $application.stages[$current_stage_index + 1];

    IF $next_stage != NONE THEN
        UPDATE $application_id SET
            current_stage = $next_stage,
            stage_history += [{
                stage: $next_stage,
                timestamp: time::now(),
                notes: $notes
            }]
    END;

    RETURN $next_stage
};

DEFINE FUNCTION fn::calculate_initial_ats_score($cv_id: record(cv), $job_posting_id: record(job_posting)) {
    LET $cv = (SELECT * FROM cv WHERE id = $cv_id);
    LET $job_posting = (SELECT * FROM job_posting WHERE id = $job_posting_id);
    
    LET $keyword_match_score = array::len(array::intersect($job_posting.keywords, $cv.keywords)) / array::len($job_posting.keywords);
    LET $skill_match_score = array::len(array::intersect($job_posting.skills, $cv.skills)) / array::len($job_posting.skills);
    
    RETURN ($keyword_match_score * 0.5 + $skill_match_score * 0.5) * 100
};

DEFINE FUNCTION fn::update_ats_score($application_id: record(application)) {
    LET $application = (SELECT * FROM application WHERE id = $application_id);
    LET $initial_score = fn::calculate_initial_ats_score($application.cv, $application.job_posting);
    LET $stage_progress = array::index_of($application.stages, $application.current_stage) / array::len($application.stages);
    
    LET $updated_score = $initial_score * (1 + $stage_progress * 0.5);
    
    UPDATE $application_id SET ats_score = $updated_score;
    
    RETURN $updated_score
};

DEFINE FUNCTION fn::get_similar_applications($application_id: record(application), $limit: number) {
    LET $application = (SELECT * FROM application WHERE id = $application_id);
    LET $cv = (SELECT * FROM cv WHERE id = $application.cv);
    
    RETURN (
        SELECT *,
        vector::similarity(cv.similarity_vector, $cv.similarity_vector) AS similarity
        FROM application
        WHERE id != $application_id
        AND job_posting.business = $application.job_posting.business
        ORDER BY similarity DESC
        LIMIT $limit
    )
};

DEFINE FUNCTION fn::get_application_timeline($application_id: record(application)) {
    LET $application = (SELECT * FROM application WHERE id = $application_id);
    
    RETURN (
        SELECT 
            stage_history,
            stages,
            current_stage
        FROM $application
    )
};