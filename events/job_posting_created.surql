DEFINE EVENT job_posting_created ON TABLE job_posting WHEN $event = "CREATE" THEN (
  CREATE event SET
    to = $after,
    event = "job_posting_created",
    business = $after.business
);