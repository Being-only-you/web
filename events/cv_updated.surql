DEFINE EVENT cv_updated ON TABLE cv WHEN $before != $after THEN (
  CREATE event SET
    from = $before,
    to = $after,
    event = "cv_updated",
    user = $after.user
);