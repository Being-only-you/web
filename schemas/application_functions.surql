DEFINE FUNCTION fn::initiate_application($applicant_id: record(user), $job_posting_id: record(job_posting), $cv_id: record(cv), $cover_letter: string) {
    LET $business = (SELECT business FROM job_posting WHERE id = $job_posting_id);
    LET $initial_stage = (SELECT name, order FROM application_stage_config WHERE business = $business ORDER BY order ASC LIMIT 1);

    CREATE application SET
        applicant = $applicant_id,
        job_posting = $job_posting_id,
        cv = $cv_id,
        cover_letter = $cover_letter,
        current_stage = $initial_stage.name,
        stages = [(SELECT name, order FROM application_stage_config WHERE business = $business ORDER BY order ASC)],
        stage_history = [{
            stage: $initial_stage.name,
            timestamp: time::now(),
            notes: "Application submitted"
        }],
        ats_score = fn::calculate_initial_ats_score($cv_id, $job_posting_id)
};

DEFINE FUNCTION fn::calculate_initial_ats_score($cv_id: record(cv), $job_posting_id: record(job_posting)) {
    LET $cv = (SELECT * FROM cv WHERE id = $cv_id);
    LET $job_posting = (SELECT * FROM job_posting WHERE id = $job_posting_id);
    
    LET $keyword_match_score = array::len(array::intersect($job_posting.keywords, $cv.keywords)) / array::len($job_posting.keywords);
    LET $skill_match_score = array::len(array::intersect($job_posting.skills, $cv.skills)) / array::len($job_posting.skills);
    
    RETURN ($keyword_match_score * 0.5 + $skill_match_score * 0.5) * 100
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