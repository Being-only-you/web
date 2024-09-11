DEFINE EVENT application_status_changed ON TABLE application WHEN $before.status != $after.status THEN (
  CREATE event SET
    from = $before,
    to = $after,
    event = "application_status_changed",
    user = $after.applicant,
    business = $after.job_posting.business
);