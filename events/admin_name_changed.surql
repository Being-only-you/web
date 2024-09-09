DEFINE EVENT name ON TABLE admin WHEN $before.name != $after.name AND $before.name != NONE AND $after.name != NONE THEN ( 
  CREATE event SET from=$before.name, to=$after.name, event="admin_name_changed", field=$after.id
);
