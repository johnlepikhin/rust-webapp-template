
let db_plugin = variable::get("db-plugin");
db_plugin.replace("-", "_");
variable::set("db_plugin", db_plugin);

let user_core_plugin = variable::get("user-core-plugin");
user_core_plugin.replace("-", "_");
variable::set("user_core_plugin", user_core_plugin);
