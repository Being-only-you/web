DEFINE EVENT password ON TABLE user WHEN $before.password != $after.password AND $before.password != NONE AND $after.id != NONE THEN ( 
  CREATE event SET event="user_password_changed", field=$after.id
);
