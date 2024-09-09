DEFINE EVENT email ON TABLE admin WHEN $before.email != $after.email AND $before.email != NONE AND $after.email != NONE THEN ( 
  CREATE event SET from=$before.email, to=$after.email, event="admin_email_changed", field=$after.id
);