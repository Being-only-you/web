DEFINE EVENT application_stage_changed ON TABLE application WHEN $before.current_stage != $after.current_stage THEN (
  CREATE event SET
    type = "application_stage_changed",
    application = $after.id,
    from_stage = $before.current_stage,
    to_stage = $after.current_stage,
    applicant = $after.applicant,
    job_posting = $after.job_posting,
    business = $after.job_posting.business,
    timestamp = time::now()
);