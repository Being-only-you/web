DEFINE EVENT password ON TABLE admin WHEN $before.password != $after.password AND $before.password != NONE AND $after.id != NONE THEN ( 
  CREATE event SET event="admin_password_changed", field=$after.id
);