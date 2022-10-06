file::delete("./proto/{{grpc_service_name}}.proto");

let is_use_telemetry = variable::get("is_use_telemetry");
let is_use_psql = variable::get("is_use_psql");
let is_use_nosql = variable::get("is_use_nosql");
let is_use_sb = variable::get("is_use_sb");

if !is_use_telemetry{
    file::delete("./src/grpc_server/telemetry.rs");
}

if !is_use_psql{
    file::delete("./src/db_repositories");
}

if is_use_nosql == "no"{
    file::delete("./src/nosql");
}

if is_use_sb != "subscriber" || is_use_sb != "both"   {
    file::delete("./src/sb/test_event/test_event_listener.rs");
}

if is_use_sb == "no"{
    file::delete("./src/sb");
}