
let db_plugin = variable::get("db-plugin");

db_plugin.replace("-", "_");

variable::set("db_plugin", db_plugin);
